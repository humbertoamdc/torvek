use crate::services::identity_manager::IdentityManager;
use crate::shared;
use async_trait::async_trait;
use shared::Result;
use shared::UseCase;
use std::sync::Arc;

pub struct LogoutClientUseCase {
    identity_manager: Arc<dyn IdentityManager>,
}

impl LogoutClientUseCase {
    pub fn new(identity_manager: Arc<dyn IdentityManager>) -> Self {
        Self { identity_manager }
    }
}

#[async_trait]
impl UseCase<String, ()> for LogoutClientUseCase {
    async fn execute(&self, session_token: String) -> Result<()> {
        self.identity_manager.logout_user(session_token).await
    }
}
