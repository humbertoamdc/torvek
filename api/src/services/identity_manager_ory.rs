use crate::auth::models::session::{
    Identity, IdentityId, MetadataPublic, Session, SessionToken, SessionWithToken,
};
use crate::services::identity_manager::IdentityManager;
use crate::shared;
use crate::shared::error::Error;
use async_trait::async_trait;
use moka::future::Cache;
use ory_kratos_client::apis::configuration::{ApiKey, Configuration};
use ory_kratos_client::apis::frontend_api::{
    create_native_login_flow, create_native_registration_flow, perform_native_logout, to_session,
    update_login_flow, update_registration_flow, UpdateLoginFlowError, UpdateRegistrationFlowError,
};
use ory_kratos_client::apis::identity_api::{get_identity, patch_identity};
use ory_kratos_client::models::ui_text::TypeEnum;
use ory_kratos_client::models::{
    JsonPatch, LoginFlow, PerformNativeLogoutBody, RegistrationFlow, UiText, UpdateLoginFlowBody,
    UpdateLoginFlowWithPasswordMethod, UpdateRegistrationFlowBody,
    UpdateRegistrationFlowWithPasswordMethod,
};
use serde_json::json;
use shared::Result;
use std::sync::Arc;
use tokio::sync::RwLock;

static CACHE_LIMIT: u64 = 10_000;

#[derive(Clone)]
pub struct OryIdentityManager {
    config: Configuration,
    cache: Arc<RwLock<Cache<SessionToken, Session>>>,
}

impl OryIdentityManager {
    pub fn new(base_path: String, client: reqwest::Client, api_key: String) -> Self {
        let api_key = ApiKey {
            prefix: Some(String::from("Bearer")),
            key: api_key,
        };

        Self {
            config: Configuration {
                base_path,
                user_agent: None,
                client,
                basic_auth: None,
                oauth_access_token: None,
                bearer_access_token: None,
                api_key: Some(api_key),
            },
            cache: Arc::new(RwLock::new(Cache::new(CACHE_LIMIT))),
        }
    }
}

#[async_trait]
impl IdentityManager for OryIdentityManager {
    async fn register(
        &self,
        email: String,
        password: String,
        metadata: MetadataPublic,
    ) -> Result<SessionWithToken> {
        // TODO: Handle errors and cases where user is already registered.
        let registration_flow = self.init_registration_flow().await?;

        self.execute_registration_flow(&registration_flow.id, email, password, metadata)
            .await
    }

    async fn login(&self, email: String, password: String) -> Result<SessionWithToken> {
        // TODO: Handle errors and cases where user is already registered.
        let login_flow = self.init_login_flow().await?;

        self.execute_login_flow(&login_flow.id, email, password)
            .await
    }

    async fn logout(&self, session_token: SessionToken) -> Result<()> {
        let request = PerformNativeLogoutBody { session_token };
        let response = perform_native_logout(&self.config, request).await;

        match response {
            Ok(_) => Ok(()),
            // TODO: Handle error.
            Err(err) => {
                tracing::error!("Failed to logout user. {err:?}");
                Err(Error::UnknownError)
            }
        }
    }

    async fn get_session(&self, session_token: SessionToken) -> Result<Session> {
        if let Some(session) = self.cache.read().await.get(&session_token).await {
            return Ok(session.clone());
        }

        let result = to_session(&self.config, Some(&session_token), None, None).await;

        match result {
            Ok(session) => {
                let serialized = serde_json::to_string(&session).unwrap();
                let session = serde_json::from_str::<Session>(&serialized).unwrap();

                self.cache
                    .write()
                    .await
                    .insert(session_token, session.clone())
                    .await;

                Ok(session)
            }
            // TODO: Handle error.
            Err(err) => {
                tracing::error!("Failed to get session. {err:?}");
                Err(Error::Unauthorized)
            }
        }
    }

    async fn get_identity(&self, identity_id: IdentityId) -> Result<Identity> {
        let result = get_identity(&self.config, &identity_id, None).await;

        match result {
            Ok(identity) => {
                let serialized = serde_json::to_string(&identity).unwrap();
                let identity = serde_json::from_str::<Identity>(&serialized).unwrap();
                Ok(identity)
            }
            Err(err) => {
                tracing::error!("Failed to get identity. {err:?}");
                Err(Error::UnknownError)
            }
        }
    }
}

impl OryIdentityManager {
    async fn init_registration_flow(&self) -> Result<RegistrationFlow> {
        let response = create_native_registration_flow(&self.config, None, None, None).await;

        match response {
            Ok(registration_flow) => Ok(registration_flow),
            Err(err) => {
                tracing::error!("Failed to create registration flow. {err:?}");
                Err(Error::UnknownError)
            }
        }
    }

    async fn execute_registration_flow(
        &self,
        flow_id: &str,
        email: String,
        password: String,
        metadata: MetadataPublic,
    ) -> Result<SessionWithToken> {
        let request = UpdateRegistrationFlowWithPasswordMethod {
            csrf_token: None,
            method: String::from("password"),
            password,
            traits: json!({"email": email }),
            transient_payload: None,
        };

        let response = update_registration_flow(
            &self.config,
            flow_id,
            UpdateRegistrationFlowBody::Password(Box::new(request)),
            None,
        )
        .await;

        match response {
            Ok(successful_native_registration) => {
                self.update_public_metadata(&successful_native_registration.identity.id, metadata)
                    .await?;

                let session_token = successful_native_registration.session_token.unwrap();
                let session = self.get_session(session_token.clone()).await?;

                let session_with_token = SessionWithToken {
                    session_token,
                    session,
                };

                Ok(session_with_token)
            }
            Err(ory_kratos_client::apis::Error::ResponseError(response_content)) => {
                let error_messages = response_content
                    .entity
                    .map(|update_registration_flow_error| {
                        if let UpdateRegistrationFlowError::Status400(registration_flow_error) =
                            update_registration_flow_error
                        {
                            registration_flow_error.ui.messages.unwrap_or_default()
                        } else {
                            vec![]
                        }
                    })
                    .unwrap_or_default();

                Err(Self::match_error(error_messages))
            }
            Err(err) => {
                tracing::error!("Failed to update registration flow. {err:?}");
                Err(Error::UnknownError)
            }
        }
    }

    async fn init_login_flow(&self) -> Result<LoginFlow> {
        let response =
            create_native_login_flow(&self.config, None, None, None, None, None, None, None).await;

        match response {
            Ok(login_flow) => Ok(login_flow),
            Err(err) => {
                tracing::error!("Failed to init login flow. {:?}", err);
                Err(Error::UnknownError)
            }
        }
    }

    async fn execute_login_flow(
        &self,
        flow_id: &str,
        email: String,
        password: String,
    ) -> Result<SessionWithToken> {
        let request = UpdateLoginFlowWithPasswordMethod {
            csrf_token: None,
            identifier: email,
            method: String::from("password"),
            password,
            password_identifier: None,
            transient_payload: None,
        };

        let response = update_login_flow(
            &self.config,
            flow_id,
            UpdateLoginFlowBody::Password(Box::new(request)),
            None,
            None,
        )
        .await;

        match response {
            Ok(successful_native_login) => {
                let serialized = serde_json::to_string(&successful_native_login).unwrap();
                let session_with_token =
                    serde_json::from_str::<SessionWithToken>(&serialized).unwrap();
                Ok(session_with_token)
            }
            Err(ory_kratos_client::apis::Error::ResponseError(response_content)) => {
                let error_messages = response_content
                    .entity
                    .map(|update_login_flow_error| {
                        if let UpdateLoginFlowError::Status400(login_flow_error) =
                            update_login_flow_error
                        {
                            login_flow_error.ui.messages.unwrap_or_default()
                        } else {
                            vec![]
                        }
                    })
                    .unwrap_or_default();

                Err(Self::match_error(error_messages))
            }
            Err(err) => {
                tracing::error!("Failed to update login flow. {err:?}");
                Err(Error::UnknownError)
            }
        }
    }

    async fn update_public_metadata(
        &self,
        identity_id: &IdentityId,
        metadata: MetadataPublic,
    ) -> Result<Identity> {
        let patches = vec![JsonPatch {
            from: None,
            op: String::from("add"),
            path: String::from("/metadata_public"),
            value: Some(Some(json!(serde_json::to_value(&metadata).unwrap()))),
        }];

        let response = patch_identity(&self.config, identity_id, Some(patches)).await;

        match response {
            Ok(ory_identity) => {
                let serialized = serde_json::to_string(&ory_identity).unwrap();
                let identity = serde_json::from_str::<Identity>(&serialized).unwrap();
                Ok(identity)
            }
            // TODO: Handle error.
            Err(err) => {
                tracing::error!("Failed to update public metadata. {err:?}");
                Err(Error::UnknownError)
            }
        }
    }

    fn match_error(error_messages: Vec<UiText>) -> Error {
        tracing::error!("{error_messages:?}");
        match Self::extract_error_id(error_messages) {
            4000006 => Error::InvalidCredentialsLoginError,
            4000028 => Error::EmailTakenRegistrationError,
            4000034 => Error::BreachedPasswordRegistrationError,
            _ => Error::UnknownError,
        }
    }

    fn extract_error_id(error_messages: Vec<UiText>) -> i64 {
        error_messages
            .iter()
            .find(|msg| msg.r#type == TypeEnum::Error)
            .map(|msg| msg.id)
            .unwrap_or(0)
    }
}
