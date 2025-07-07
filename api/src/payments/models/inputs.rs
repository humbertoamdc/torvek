use crate::auth::models::session::Identity;
use crate::orders::models::order::Address;
use serde_derive::{Deserialize, Serialize};
use stripe::CheckoutSession;

#[derive(Debug)]
pub enum WebhookRequestError {
    MissingShippingDetails,
    MissingShippingRecipientName,
    MissingShippingAddress,
    MissingMetadata,
    MissingField,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct CreateCheckoutSessionInput {
    pub identity: Identity,
    pub project_id: String,
    pub quotation_id: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct CompleteCheckoutSessionWebhookRequest {
    pub customer_id: String,
    pub project_id: String,
    pub quotation_id: String,
    pub shipping_recipient_name: String,
    pub shipping_address: Address,
}

impl TryFrom<CheckoutSession> for CompleteCheckoutSessionWebhookRequest {
    type Error = WebhookRequestError;

    fn try_from(session: CheckoutSession) -> Result<Self, Self::Error> {
        let metadata = session
            .metadata
            .ok_or(WebhookRequestError::MissingMetadata)?;
        let shipping_details = session
            .shipping_details
            .ok_or(WebhookRequestError::MissingShippingDetails)?;

        let shipping_details_address = shipping_details
            .address
            .ok_or(WebhookRequestError::MissingShippingAddress)?;

        let customer_id = metadata
            .get("customer_id")
            .ok_or(WebhookRequestError::MissingField)?
            .clone();
        let project_id = metadata
            .get("project_id")
            .ok_or(WebhookRequestError::MissingField)?
            .clone();
        let quotation_id = metadata
            .get("quotation_id")
            .ok_or(WebhookRequestError::MissingField)?
            .clone();

        let shipping_recipient_name = shipping_details
            .name
            .ok_or(WebhookRequestError::MissingShippingRecipientName)?;

        let shipping_address = Address {
            city: shipping_details_address.city,
            country: shipping_details_address.country,
            line1: shipping_details_address.line1,
            line2: shipping_details_address.line2,
            postal_code: shipping_details_address.postal_code,
            state: shipping_details_address.state,
        };

        Ok(Self {
            customer_id,
            project_id,
            quotation_id,
            shipping_recipient_name,
            shipping_address,
        })
    }
}
