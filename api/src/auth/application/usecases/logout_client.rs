use std::sync::Arc;

use axum::async_trait;

use crate::auth::application::services::identity_manager::IdentityManager;
use crate::auth::application::usecases::interfaces::UseCase;
use crate::auth::domain::errors::AuthError;

pub struct LogoutClientUseCase {
    identity_manager: Arc<dyn IdentityManager>,
}

impl LogoutClientUseCase {
    pub fn new(identity_manager: Arc<dyn IdentityManager>) -> Self {
        Self { identity_manager }
    }
}

#[async_trait]
impl UseCase<String, (), AuthError> for LogoutClientUseCase {
    async fn execute(&self, session_token: String) -> Result<(), AuthError> {
        self.identity_manager.logout_user(session_token).await
    }
}
