use axum::async_trait;
use ory_kratos_client::apis::configuration::{ApiKey, Configuration};
use ory_kratos_client::apis::frontend_api::{
    create_native_login_flow, create_native_registration_flow, perform_native_logout, to_session,
    update_login_flow, update_registration_flow, UpdateLoginFlowError, UpdateRegistrationFlowError,
};
use ory_kratos_client::apis::identity_api::patch_identity;
use ory_kratos_client::apis::Error;
use ory_kratos_client::models::ui_text::TypeEnum;
use ory_kratos_client::models::{JsonPatch, LoginFlow, RegistrationFlow, UiText};
use serde_json::json;

use crate::auth::adapters::api::requests::{LoginClientRequest, RegisterClientRequest};
use crate::auth::adapters::spi::identity_manager::ory_mappers::{
    OryLoginRequestMapper, OryLogoutRequestMapper, OryRegisterRequestMapper,
};
use crate::auth::application::services::identity_manager::IdentityManager;
use crate::auth::domain::errors::AuthError;
use crate::auth::domain::session::{Identity, Session, SessionWithToken};
use crate::auth::domain::user::UserRole;

#[derive(Clone)]
pub struct OryIdentityManager {
    config: Configuration,
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
        }
    }
}

#[async_trait]
impl IdentityManager for OryIdentityManager {
    async fn register_user(
        &self,
        request: RegisterClientRequest,
    ) -> Result<SessionWithToken, AuthError> {
        // TODO: Handle errors and cases where user is already registered.
        let registration_flow = self.init_registration_flow().await?;

        self.execute_registration_flow(&registration_flow.id, request)
            .await
    }

    async fn login_user(&self, request: LoginClientRequest) -> Result<SessionWithToken, AuthError> {
        // TODO: Handle errors and cases where user is already registered.
        let login_flow = self.init_login_flow().await?;

        self.execute_login_flow(&login_flow.id, request).await
    }

    async fn logout_user(&self, session_token: String) -> Result<(), AuthError> {
        let response = perform_native_logout(
            &self.config,
            &OryLogoutRequestMapper::api_to_spi(session_token),
        )
        .await;

        match response {
            Ok(_) => Ok(()),
            // TODO: Handle error.
            Err(_) => Err(AuthError::UnknownError),
        }
    }

    async fn get_session(&self, session_token: String) -> Result<Session, AuthError> {
        let response = to_session(&self.config, Some(&session_token), None, None).await;

        match response {
            Ok(session) => {
                let serialized = serde_json::to_string(&session).unwrap();
                let session = serde_json::from_str::<Session>(&serialized).unwrap();
                Ok(session)
            }
            // TODO: Handle error.
            Err(_) => Err(AuthError::UnknownError),
        }
    }

    async fn set_user_role(
        &self,
        identity_id: &str,
        role: UserRole,
    ) -> Result<Identity, AuthError> {
        let patches = vec![JsonPatch {
            from: None,
            op: String::from("add"),
            path: String::from("/metadata_public"),
            value: Some(json!({"user": {"role": role}})),
        }];

        let response = patch_identity(&self.config, identity_id, Some(patches)).await;

        match response {
            Ok(ory_identity) => {
                let serialized = serde_json::to_string(&ory_identity).unwrap();
                let identity = serde_json::from_str::<Identity>(&serialized).unwrap();
                Ok(identity)
            }
            // TODO: Handle error.
            Err(_) => Err(AuthError::UnknownError),
        }
    }
}

impl OryIdentityManager {
    async fn init_registration_flow(&self) -> Result<RegistrationFlow, AuthError> {
        let response = create_native_registration_flow(&self.config, None, None).await;

        match response {
            Ok(registration_flow) => Ok(registration_flow),
            Err(_) => Err(AuthError::InitializingRegistrationFlowError),
        }
    }

    async fn execute_registration_flow(
        &self,
        flow_id: &str,
        request: RegisterClientRequest,
    ) -> Result<SessionWithToken, AuthError> {
        let response = update_registration_flow(
            &self.config,
            flow_id,
            &OryRegisterRequestMapper::api_to_spi(request),
            None,
        )
        .await;

        match response {
            Ok(successful_native_login) => {
                let serialized = serde_json::to_string(&successful_native_login).unwrap();
                let auth_session = serde_json::from_str::<SessionWithToken>(&serialized).unwrap();
                Ok(auth_session)
            }
            Err(Error::ResponseError(response_content)) => {
                log::error!("Response content {response_content:#?}");
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

                Err(Self::match_error(&error_messages))
            }
            Err(_) => Err(AuthError::UnknownError),
        }
    }

    async fn init_login_flow(&self) -> Result<LoginFlow, AuthError> {
        let response =
            create_native_login_flow(&self.config, None, None, None, None, None, None).await;

        match response {
            Ok(login_flow) => Ok(login_flow),
            Err(err) => {
                log::error!("{:#?}", err);
                Err(AuthError::InitializingLoginFlowError)
            }
        }
    }

    async fn execute_login_flow(
        &self,
        flow_id: &str,
        request: LoginClientRequest,
    ) -> Result<SessionWithToken, AuthError> {
        let response = update_login_flow(
            &self.config,
            flow_id,
            &OryLoginRequestMapper::api_to_spi(request),
            None,
            None,
        )
        .await;

        match response {
            Ok(successful_native_login) => {
                let serialized = serde_json::to_string(&successful_native_login).unwrap();
                let auth_session = serde_json::from_str::<SessionWithToken>(&serialized).unwrap();
                Ok(auth_session)
            }
            Err(Error::ResponseError(response_content)) => {
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

                Err(Self::match_error(&error_messages))
            }
            Err(_) => Err(AuthError::UnknownError),
        }
    }

    fn match_error(error_messages: &Vec<UiText>) -> AuthError {
        log::error!("{error_messages:#?}");
        // TODO: Map error ids to a custom enum.
        match Self::extract_error_id(&error_messages) {
            4000006 => AuthError::InvalidCredentialsLoginError,
            4000028 => AuthError::EmailTakenRegistrationError,
            4000034 => AuthError::BreachedPasswordRegistrationError,
            _ => AuthError::UnknownError,
        }
    }

    fn extract_error_id(error_messages: &Vec<UiText>) -> i64 {
        error_messages
            .iter()
            .find(|msg| msg._type == TypeEnum::Error)
            .map(|msg| msg.id)
            .unwrap_or(0)
    }
}
