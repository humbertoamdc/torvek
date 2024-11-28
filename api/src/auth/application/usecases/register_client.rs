use std::sync::Arc;

use axum::async_trait;

use crate::auth::adapters::api::requests::RegisterClientRequest;
use crate::auth::application::services::identity_manager::IdentityManager;
use crate::auth::domain::session::{MetadataAdmin, SessionWithToken};
use crate::auth::domain::user::UserRole;
use crate::services::payment_processor::PaymentsProcessor;
use crate::shared;
use shared::Result;
use shared::UseCase;

pub struct RegisterClientUseCase {
    identity_manager: Arc<dyn IdentityManager>,
    payments_processor: Arc<dyn PaymentsProcessor>,
}

impl RegisterClientUseCase {
    pub fn new(
        identity_manager: Arc<dyn IdentityManager>,
        payments_processor: Arc<dyn PaymentsProcessor>,
    ) -> Self {
        Self {
            identity_manager,
            payments_processor,
        }
    }
}

#[async_trait]
impl UseCase<RegisterClientRequest, SessionWithToken> for RegisterClientUseCase {
    async fn execute(&self, request: RegisterClientRequest) -> Result<SessionWithToken> {
        let mut session_with_token = self.identity_manager.register_user(request.clone()).await?;
        let identity = self
            .identity_manager
            .set_user_role(&session_with_token.session.identity.id, UserRole::Client)
            .await?;
        session_with_token.session.identity = identity.clone();

        // TODO: Add name in request and use common error.
        let stripe_customer = self
            .payments_processor
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
