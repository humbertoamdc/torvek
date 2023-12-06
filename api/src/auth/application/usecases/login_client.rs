use std::sync::Arc;

use axum::async_trait;

use crate::auth::adapters::api::requests::LoginClientRequest;
use crate::auth::application::services::identity_manager::IdentityManager;
use crate::auth::application::usecases::interfaces::UseCase;
use crate::auth::domain::errors::AuthError;
use crate::auth::domain::session::SessionWithToken;

pub struct LoginClientUseCase {
    identity_manager: Arc<dyn IdentityManager>,
}

impl LoginClientUseCase {
    pub fn new(identity_manager: Arc<dyn IdentityManager>) -> Self {
        Self { identity_manager }
    }
}

#[async_trait]
impl UseCase<LoginClientRequest, SessionWithToken, AuthError> for LoginClientUseCase {
    async fn execute(&self, request: LoginClientRequest) -> Result<SessionWithToken, AuthError> {
        self.identity_manager.login_user(request).await
    }
}
