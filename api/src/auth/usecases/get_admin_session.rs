use crate::auth::models::session::Session;
use crate::services::identity_manager::AdminIdentityManager;
use crate::shared;
use async_trait::async_trait;
use shared::Result;
use shared::UseCase;
use std::sync::Arc;

pub struct GetAdminSessionUseCase {
    admin_identity_manager: Arc<dyn AdminIdentityManager>,
}

impl GetAdminSessionUseCase {
    pub fn new(admin_identity_manager: Arc<dyn AdminIdentityManager>) -> Self {
        Self {
            admin_identity_manager,
        }
    }
}

#[async_trait]
impl UseCase<String, Session> for GetAdminSessionUseCase {
    async fn execute(&self, session_token: String) -> Result<Session> {
        self.admin_identity_manager
            .get_admin_session(session_token)
            .await
    }
}
