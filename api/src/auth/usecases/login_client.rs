use std::sync::Arc;

use axum::async_trait;

use crate::auth::models::requests::LoginClientRequest;
use crate::auth::models::session::SessionWithToken;
use crate::services::identity_manager::IdentityManager;
use crate::shared;
use shared::Result;
use shared::UseCase;

pub struct LoginClientUseCase {
    identity_manager: Arc<dyn IdentityManager>,
}

impl LoginClientUseCase {
    pub fn new(identity_manager: Arc<dyn IdentityManager>) -> Self {
        Self { identity_manager }
    }
}

#[async_trait]
impl UseCase<LoginClientRequest, SessionWithToken> for LoginClientUseCase {
    async fn execute(&self, request: LoginClientRequest) -> Result<SessionWithToken> {
        self.identity_manager.login_user(request).await
    }
}
