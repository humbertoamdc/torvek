use std::sync::Arc;

use axum::async_trait;

use crate::auth::application::services::identity_manager::AdminIdentityManager;
use crate::auth::application::usecases::interfaces::UseCase;
use crate::auth::domain::errors::AuthError;

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
impl UseCase<String, (), AuthError> for AdminLogoutUseCase {
    async fn execute(&self, session_token: String) -> Result<(), AuthError> {
        self.admin_identity_manager
            .logout_admin(session_token)
            .await
    }
}
