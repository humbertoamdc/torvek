use crate::auth::models::inputs::LoginUserInput;
use crate::auth::models::session::SessionWithToken;
use crate::services::identity_manager::IdentityManager;
use crate::shared;
use crate::shared::error::Error;
use async_trait::async_trait;
use shared::Result;
use shared::UseCase;
use std::sync::Arc;

pub struct Login {
    identity_manager: Arc<dyn IdentityManager>,
}

impl Login {
    pub fn new(identity_manager: Arc<dyn IdentityManager>) -> Self {
        Self { identity_manager }
    }
}

#[async_trait]
impl UseCase<LoginUserInput, SessionWithToken> for Login {
    async fn execute(&self, input: LoginUserInput) -> Result<SessionWithToken> {
        let session_with_token = self
            .identity_manager
            .login(input.email, input.password)
            .await?;
        let session_role = session_with_token
            .session
            .identity
            .metadata_public
            .clone()
            .role;

        if session_role != input.role {
            self.identity_manager
                .logout(session_with_token.session_token)
                .await?;
            return Err(Error::InvalidCredentialsLoginError);
        }

        Ok(session_with_token)
    }
}
