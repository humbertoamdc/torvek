use std::sync::Arc;

use axum::async_trait;

use crate::auth::models::requests::RegisterClientRequest;
use crate::auth::models::session::{MetadataAdmin, SessionWithToken};
use crate::services::identity_manager::IdentityManager;
use crate::services::stripe_client::StripeClient;
use crate::shared;
use shared::Result;
use shared::UseCase;

pub struct RegisterClientUseCase {
    identity_manager: Arc<dyn IdentityManager>,
    stripe_client: Arc<dyn StripeClient>,
}

impl RegisterClientUseCase {
    pub fn new(
        identity_manager: Arc<dyn IdentityManager>,
        stripe_client: Arc<dyn StripeClient>,
    ) -> Self {
        Self {
            identity_manager,
            stripe_client,
        }
    }
}

#[async_trait]
impl UseCase<RegisterClientRequest, SessionWithToken> for RegisterClientUseCase {
    async fn execute(&self, request: RegisterClientRequest) -> Result<SessionWithToken> {
        let mut session_with_token = self.identity_manager.register_user(request.clone()).await?;

        let identity = self
            .identity_manager
            .get_identity(session_with_token.session.identity.id)
            .await?;
        session_with_token.session.identity = identity.clone();

        let stripe_customer = self
            .stripe_client
            .create_customer(request.name, request.email)
            .await?;

        let admin_metadata = MetadataAdmin {
            stripe_customer_id: stripe_customer.id.to_string(),
        };
        self.identity_manager
            .update_admin_metadata(&identity.id, admin_metadata)
            .await?;

        Ok(session_with_token)
    }
}
