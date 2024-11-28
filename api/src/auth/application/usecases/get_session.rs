use std::sync::Arc;

use axum::async_trait;

use crate::auth::application::services::identity_manager::IdentityManager;
use crate::auth::domain::session::Session;
use crate::shared;
use shared::Result;
use shared::UseCase;

pub struct GetSessionUseCase {
    identity_manager: Arc<dyn IdentityManager>,
}

impl GetSessionUseCase {
    pub fn new(identity_manager: Arc<dyn IdentityManager>) -> Self {
        Self { identity_manager }
    }
}

#[async_trait]
impl UseCase<String, Session> for GetSessionUseCase {
    async fn execute(&self, session_token: String) -> Result<Session> {
        self.identity_manager.get_session(session_token).await
    }
}
