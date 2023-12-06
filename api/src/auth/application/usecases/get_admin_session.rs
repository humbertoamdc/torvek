use std::sync::Arc;

use axum::async_trait;

use crate::auth::application::services::identity_manager::AdminIdentityManager;
use crate::auth::application::usecases::interfaces::UseCase;
use crate::auth::domain::errors::AuthError;
use crate::auth::domain::session::Session;

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
impl UseCase<String, Session, AuthError> for GetAdminSessionUseCase {
    async fn execute(&self, session_token: String) -> Result<Session, AuthError> {
        self.admin_identity_manager
            .get_admin_session(session_token)
            .await
    }
}
