use axum::async_trait;

use crate::auth::adapters::api::requests::{
    AdminLoginRequest, LoginClientRequest, RegisterClientRequest,
};
use crate::auth::domain::errors::AuthError;
use crate::auth::domain::session::{Identity, Session, SessionWithToken};
use crate::auth::domain::user::UserRole;

#[async_trait]
pub trait IdentityManager: Send + Sync + 'static {
    async fn register_user(
        &self,
        request: RegisterClientRequest,
    ) -> Result<SessionWithToken, AuthError>;
    async fn login_user(&self, request: LoginClientRequest) -> Result<SessionWithToken, AuthError>;
    async fn logout_user(&self, session_token: String) -> Result<(), AuthError>;
    async fn get_session(&self, session_token: String) -> Result<Session, AuthError>;
    async fn set_user_role(&self, identity_id: &str, role: UserRole)
        -> Result<Identity, AuthError>;
}

#[async_trait]
pub trait AdminIdentityManager: Send + Sync + 'static {
    async fn login_admin(&self, request: AdminLoginRequest) -> Result<SessionWithToken, AuthError>;
    async fn logout_admin(&self, session_token: String) -> Result<(), AuthError>;
    async fn get_admin_session(&self, session_token: String) -> Result<Session, AuthError>;
}
