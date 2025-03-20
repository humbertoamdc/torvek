use crate::auth::models::requests::AdminLoginRequest;
use crate::auth::models::session::SessionWithToken;
use crate::services::identity_manager::AdminIdentityManager;
use crate::shared;
use async_trait::async_trait;
use shared::Result;
use shared::UseCase;
use std::sync::Arc;

pub struct AdminLoginUseCase {
    admin_identity_manager: Arc<dyn AdminIdentityManager>,
}

impl AdminLoginUseCase {
    pub fn new(admin_identity_manager: Arc<dyn AdminIdentityManager>) -> Self {
        Self {
            admin_identity_manager,
        }
    }
}

#[async_trait]
impl UseCase<AdminLoginRequest, SessionWithToken> for AdminLoginUseCase {
    async fn execute(&self, request: AdminLoginRequest) -> Result<SessionWithToken> {
        self.admin_identity_manager.login_admin(request).await
    }
}
