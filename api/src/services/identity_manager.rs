use crate::auth::models::requests::{AdminLoginRequest, LoginClientRequest, RegisterClientRequest};
use crate::auth::models::session::{Identity, MetadataPublic, Session, SessionWithToken};
use crate::shared;
use async_trait::async_trait;
use shared::Result;

#[async_trait]
pub trait IdentityManager: Send + Sync + 'static {
    async fn register_user(&self, request: RegisterClientRequest) -> Result<SessionWithToken>;
    async fn login_user(&self, request: LoginClientRequest) -> Result<SessionWithToken>;
    async fn logout_user(&self, session_token: String) -> Result<()>;
    async fn get_session(&self, session_token: String) -> Result<Session>;
    async fn get_identity(&self, identity_id: String) -> Result<Identity>;
    async fn update_public_metadata(
        &self,
        identity_id: &str,
        metadata: MetadataPublic,
    ) -> Result<Identity>;
}

#[async_trait]
pub trait AdminIdentityManager: Send + Sync + 'static {
    async fn login_admin(&self, request: AdminLoginRequest) -> Result<SessionWithToken>;
    async fn logout_admin(&self, session_token: String) -> Result<()>;
    async fn get_admin_session(&self, session_token: String) -> Result<Session>;
}
