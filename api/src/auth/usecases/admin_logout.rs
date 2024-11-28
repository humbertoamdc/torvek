use std::sync::Arc;

use axum::async_trait;

use crate::services::identity_manager::AdminIdentityManager;
use crate::shared;
use shared::Result;
use shared::UseCase;

pub struct AdminLogoutUseCase {
    admin_identity_manager: Arc<dyn AdminIdentityManager>,
}

impl AdminLogoutUseCase {
    pub fn new(admin_identity_manager: Arc<dyn AdminIdentityManager>) -> Self {
        Self {
            admin_identity_manager,
        }
    }
}

#[async_trait]
impl UseCase<String, ()> for AdminLogoutUseCase {
    async fn execute(&self, session_token: String) -> Result<()> {
        self.admin_identity_manager
            .logout_admin(session_token)
            .await
    }
}
