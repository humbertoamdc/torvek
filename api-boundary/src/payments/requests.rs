use std::collections::HashMap;

use serde_derive::{Deserialize, Serialize};

use crate::payments::errors::WebhookRequestError;

#[derive(Deserialize, Serialize, Debug)]
pub struct CreateCheckoutSessionRequest {
    pub client_id: String,
    pub project_id: String,
    pub quotation_id: String,
    pub selected_quotes_per_part: HashMap<String, String>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct CompleteCheckoutSessionWebhookRequest {
    pub client_id: String,
    pub project_id: String,
    pub quotation_id: String,
}

impl TryFrom<Option<HashMap<String, String>>> for CompleteCheckoutSessionWebhookRequest {
    type Error = WebhookRequestError;

    fn try_from(metadata: Option<HashMap<String, String>>) -> Result<Self, Self::Error> {
        match metadata {
            Some(metadata) => {
                let client_id = metadata
                    .get("client_id")
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

                Ok(Self {
                    client_id,
                    project_id,
                    quotation_id,
                })
            }
            None => Err(WebhookRequestError::MissingMetadata),
        }
    }
}
