use serde_derive::{Deserialize, Serialize};
use stripe::Metadata;

#[derive(Deserialize, Serialize, Debug)]
pub struct CreateQuotationRequest {
    pub client_id: String,
    pub project_id: String,
}
impl CreateQuotationRequest {
    pub const fn new(client_id: String, project_id: String) -> Self {
        Self {
            client_id,
            project_id,
        }
    }
}

#[derive(Deserialize, Serialize, Debug)]
pub struct QueryQuotationsForProjectRequest {
    pub client_id: String,
    pub project_id: String,
}
impl QueryQuotationsForProjectRequest {
    pub const fn new(client_id: String, project_id: String) -> Self {
        Self {
            client_id,
            project_id,
        }
    }
}

#[derive(Debug)]
pub enum WebhookRequestError {
    MissingMetadata,
    MissingField,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct ConfirmQuotationPaymentWebhookRequest {
    pub client_id: String,
    pub project_id: String,
    pub quotation_id: String,
}
impl TryFrom<Option<Metadata>> for ConfirmQuotationPaymentWebhookRequest {
    type Error = WebhookRequestError;

    fn try_from(metadata: Option<Metadata>) -> Result<Self, Self::Error> {
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
