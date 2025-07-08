use crate::auth::models::session::{Identity, MetadataPublic, Session, SessionWithToken};
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
    async fn logout(&self, session_token: String) -> Result<()>;
    async fn get_session(&self, session_token: String) -> Result<Session>;
    async fn get_identity(&self, identity_id: String) -> Result<Identity>;
}
