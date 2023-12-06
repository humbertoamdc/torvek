use std::sync::Arc;

use axum::async_trait;

use crate::auth::application::services::identity_manager::IdentityManager;
use crate::auth::application::usecases::interfaces::UseCase;
use crate::auth::domain::errors::AuthError;
use crate::auth::domain::session::Session;

pub struct GetSessionUseCase {
    identity_manager: Arc<dyn IdentityManager>,
}

impl GetSessionUseCase {
    pub fn new(identity_manager: Arc<dyn IdentityManager>) -> Self {
        Self { identity_manager }
    }
}

#[async_trait]
impl UseCase<String, Session, AuthError> for GetSessionUseCase {
    async fn execute(&self, session_token: String) -> Result<Session, AuthError> {
        self.identity_manager.get_session(session_token).await
    }
}
