use std::sync::Arc;

use axum::async_trait;

use crate::auth::adapters::api::requests::AdminLoginRequest;
use crate::auth::application::services::identity_manager::AdminIdentityManager;
use crate::auth::application::usecases::interfaces::UseCase;
use crate::auth::domain::errors::AuthError;
use crate::auth::domain::session::SessionWithToken;

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
impl UseCase<AdminLoginRequest, SessionWithToken, AuthError> for AdminLoginUseCase {
    async fn execute(&self, request: AdminLoginRequest) -> Result<SessionWithToken, AuthError> {
        self.admin_identity_manager.login_admin(request).await
    }
}
