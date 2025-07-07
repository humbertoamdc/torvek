use crate::auth::models::requests::AdminLoginRequest;
use crate::auth::models::session::{Session, SessionWithToken};
use crate::services::identity_manager::AdminIdentityManager;
use crate::shared;
use crate::shared::error::Error;
use async_trait::async_trait;
use ory_kratos_client::apis::configuration::Configuration;
use ory_kratos_client::apis::frontend_api::{
    create_native_login_flow, perform_native_logout, to_session, update_login_flow,
    UpdateLoginFlowError,
};
use ory_kratos_client::models::ui_text::TypeEnum;
use ory_kratos_client::models::UpdateLoginFlowBody::UpdateLoginFlowWithPasswordMethod;
use ory_kratos_client::models::{LoginFlow, PerformNativeLogoutBody, UiText};
use shared::Result;

#[derive(Clone)]
pub struct OryAdminIdentityManager {
    config: Configuration,
}

impl OryAdminIdentityManager {
    pub fn new(base_path: String, client: reqwest::Client) -> Self {
        Self {
            config: Configuration {
                base_path,
                user_agent: None,
                client,
                basic_auth: None,
                oauth_access_token: None,
                bearer_access_token: None,
                api_key: None,
            },
        }
    }
}

#[async_trait]
impl AdminIdentityManager for OryAdminIdentityManager {
    async fn login_admin(&self, request: AdminLoginRequest) -> Result<SessionWithToken> {
        let login_flow = self.init_admin_login_flow().await?;

        self.execute_admin_login_flow(&login_flow.id, request).await
    }

    async fn logout_admin(&self, session_token: String) -> Result<()> {
        let request = PerformNativeLogoutBody { session_token };

        let response = perform_native_logout(&self.config, &request).await;

        match response {
            Ok(_) => Ok(()),
            Err(err) => {
                tracing::error!("{err:?}");
                Err(Error::UnknownError)
            }
        }
    }

    async fn get_admin_session(&self, session_token: String) -> Result<Session> {
        let response = to_session(&self.config, Some(&session_token), None, None).await;

        match response {
            Ok(session) => {
                let serialized = serde_json::to_string(&session).unwrap();
                let session = serde_json::from_str::<Session>(&serialized).unwrap();
                Ok(session)
            }
            // TODO: Handle error.
            Err(err) => {
                tracing::error!("{err:?}");
                Err(Error::UnknownError)
            }
        }
    }
}

impl OryAdminIdentityManager {
    async fn init_admin_login_flow(&self) -> Result<LoginFlow> {
        let response =
            create_native_login_flow(&self.config, None, None, None, None, None, None).await;

        match response {
            Ok(login_flow) => Ok(login_flow),
            Err(err) => {
                tracing::error!("{err:?}");
                Err(Error::UnknownError)
            }
        }
    }

    async fn execute_admin_login_flow(
        &self,
        flow_id: &str,
        request: AdminLoginRequest,
    ) -> Result<SessionWithToken> {
        let request = UpdateLoginFlowWithPasswordMethod {
            csrf_token: Some(request.csrf_token),
            identifier: request.email,
            password: request.password,
            password_identifier: None,
        };

        let response = update_login_flow(&self.config, flow_id, &request, None, None).await;

        match response {
            Ok(successful_native_login) => {
                let serialized = serde_json::to_string(&successful_native_login).unwrap();
                let auth_session = serde_json::from_str::<SessionWithToken>(&serialized).unwrap();
                Ok(auth_session)
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

                Err(Self::match_error(&error_messages))
            }
            Err(err) => {
                tracing::error!("{err:?}");
                Err(Error::UnknownError)
            }
        }
    }
    fn match_error(error_messages: &Vec<UiText>) -> Error {
        tracing::error!("{error_messages:#?}");
        // TODO: Map error ids to a custom enum.
        match Self::extract_error_id(&error_messages) {
            4000006 => Error::InvalidCredentialsLoginError,
            _ => Error::UnknownError,
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
