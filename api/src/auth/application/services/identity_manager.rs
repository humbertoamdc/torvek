use axum::async_trait;

use crate::auth::adapters::api::requests::{
    AdminLoginRequest, LoginClientRequest, RegisterClientRequest,
};
use crate::auth::domain::session::{Identity, MetadataAdmin, Session, SessionWithToken};
use crate::auth::domain::user::UserRole;
use crate::shared;
use shared::Result;

#[async_trait]
pub trait IdentityManager: Send + Sync + 'static {
    async fn register_user(&self, request: RegisterClientRequest) -> Result<SessionWithToken>;
    async fn login_user(&self, request: LoginClientRequest) -> Result<SessionWithToken>;
    async fn logout_user(&self, session_token: String) -> Result<()>;
    async fn get_session(&self, session_token: String) -> Result<Session>;
    async fn get_identity(&self, identity_id: String) -> Result<Identity>;
    async fn set_user_role(&self, identity_id: &str, role: UserRole) -> Result<Identity>;
    async fn update_admin_metadata(
        &self,
        identity_id: &str,
        metadata: MetadataAdmin,
    ) -> Result<Identity>;
}

#[async_trait]
pub trait AdminIdentityManager: Send + Sync + 'static {
    async fn login_admin(&self, request: AdminLoginRequest) -> Result<SessionWithToken>;
    async fn logout_admin(&self, session_token: String) -> Result<()>;
    async fn get_admin_session(&self, session_token: String) -> Result<Session>;
}
