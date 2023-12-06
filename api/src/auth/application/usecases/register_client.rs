use std::sync::Arc;

use axum::async_trait;

use crate::auth::adapters::api::requests::RegisterClientRequest;
use crate::auth::application::services::identity_manager::IdentityManager;
use crate::auth::application::usecases::interfaces::UseCase;
use crate::auth::domain::errors::AuthError;
use crate::auth::domain::session::SessionWithToken;
use crate::auth::domain::user::UserRole;

pub struct RegisterClientUseCase {
    identity_manager: Arc<dyn IdentityManager>,
}

impl RegisterClientUseCase {
    pub fn new(identity_manager: Arc<dyn IdentityManager>) -> Self {
        Self { identity_manager }
    }
}

#[async_trait]
impl UseCase<RegisterClientRequest, SessionWithToken, AuthError> for RegisterClientUseCase {
    async fn execute(&self, request: RegisterClientRequest) -> Result<SessionWithToken, AuthError> {
        let mut session_with_token = self.identity_manager.register_user(request).await?;
        let identity = self
            .identity_manager
            .set_user_role(&session_with_token.session.identity.id, UserRole::Client)
            .await?;
        session_with_token.session.identity = identity;
        Ok(session_with_token)
    }
}
