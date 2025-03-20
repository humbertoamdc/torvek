use crate::auth::models::session::Session;
use crate::services::identity_manager::IdentityManager;
use crate::shared;
use async_trait::async_trait;
use shared::Result;
use shared::UseCase;
use std::sync::Arc;

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
