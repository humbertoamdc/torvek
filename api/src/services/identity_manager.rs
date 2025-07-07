use crate::auth::models::inputs::{LoginUserInput, RegisterUserInput};
use crate::auth::models::session::{
    Identity, IdentityId, MetadataPublic, Session, SessionToken, SessionWithToken,
};
use crate::shared;
use async_trait::async_trait;
use shared::Result;

#[async_trait]
pub trait IdentityManager: Send + Sync + 'static {
    async fn register_user(&self, input: RegisterUserInput) -> Result<(SessionToken, IdentityId)>;
    async fn login_user(&self, request: LoginUserInput) -> Result<SessionWithToken>;
    async fn logout_user(&self, session_token: String) -> Result<()>;
    async fn get_session(&self, session_token: String) -> Result<Session>;
    async fn get_identity(&self, identity_id: String) -> Result<Identity>;
    async fn update_public_metadata(
        &self,
        identity_id: &str,
        metadata: MetadataPublic,
    ) -> Result<Identity>;
}
