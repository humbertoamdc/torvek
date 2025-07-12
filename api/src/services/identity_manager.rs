use crate::auth::models::session::{
    Identity, IdentityId, MetadataPublic, Session, SessionToken, SessionWithToken,
};
use crate::shared;
use async_trait::async_trait;
use shared::Result;

#[async_trait]
pub trait IdentityManager: Send + Sync + 'static {
    async fn register(
        &self,
        email: String,
        password: String,
        metadata: MetadataPublic,
    ) -> Result<SessionWithToken>;
    async fn login(&self, email: String, password: String) -> Result<SessionWithToken>;
    async fn logout(&self, session_token: SessionToken) -> Result<()>;
    async fn get_session(&self, session_token: SessionToken) -> Result<Session>;
    async fn get_identity(&self, identity_id: IdentityId) -> Result<Identity>;
}
