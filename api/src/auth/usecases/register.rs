use crate::auth::models::inputs::RegisterUserInput;
use crate::auth::models::session::{MetadataPublic, SessionWithToken};
use crate::services::identity_manager::IdentityManager;
use crate::services::stripe_client::StripeClient;
use crate::shared;
use async_trait::async_trait;
use shared::Result;
use shared::UseCase;
use std::sync::Arc;

pub struct Register {
    identity_manager: Arc<dyn IdentityManager>,
    stripe_client: Arc<dyn StripeClient>,
}

impl Register {
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
impl UseCase<RegisterUserInput, SessionWithToken> for Register {
    async fn execute(&self, input: RegisterUserInput) -> Result<SessionWithToken> {
        let stripe_customer = self
            .stripe_client
            .create_customer(input.name.clone(), input.email.clone())
            .await?;

        let metadata = MetadataPublic {
            stripe_customer_id: Some(stripe_customer.id.to_string()),
            role: input.role.clone(),
        };

        self.identity_manager
            .register(input.email, input.password, metadata)
            .await
    }
}
