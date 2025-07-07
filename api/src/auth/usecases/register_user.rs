use crate::auth::models::inputs::RegisterUserInput;
use crate::auth::models::session::{MetadataPublic, SessionWithToken};
use crate::services::identity_manager::IdentityManager;
use crate::services::stripe_client::StripeClient;
use crate::shared;
use async_trait::async_trait;
use shared::Result;
use shared::UseCase;
use std::sync::Arc;

pub struct RegisterUserUseCase {
    identity_manager: Arc<dyn IdentityManager>,
    stripe_client: Arc<dyn StripeClient>,
}

impl RegisterUserUseCase {
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
impl UseCase<RegisterUserInput, SessionWithToken> for RegisterUserUseCase {
    async fn execute(&self, input: RegisterUserInput) -> Result<SessionWithToken> {
        let (session_token, identity_id) =
            self.identity_manager.register_user(input.clone()).await?;

        let stripe_customer = self
            .stripe_client
            .create_customer(input.name, input.email)
            .await?;

        let public_metadata = MetadataPublic {
            stripe_customer_id: Some(stripe_customer.id.to_string()),
            role: input.role,
        };
        self.identity_manager
            .update_public_metadata(&identity_id, public_metadata)
            .await?;

        let session = self
            .identity_manager
            .get_session(session_token.clone())
            .await?;

        let session_with_token = SessionWithToken {
            session_token,
            session,
        };

        Ok(session_with_token)
    }
}
