use std::collections::HashMap;

use chrono::{NaiveDate, Utc};
use serde_derive::{Deserialize, Serialize};

use crate::common::file::File;
use crate::parts::models::Part;
use crate::payments::errors::WebhookRequestError;

#[derive(Deserialize, Serialize, Debug)]
pub struct CreateCheckoutSessionRequest {
    pub client_id: String,
    pub project_id: String,
    pub quotation_id: String,
    pub data: Vec<CreateCheckoutSessionPartData>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct CreateCheckoutSessionPartData {
    pub part_id: String,
    pub name: String,
    pub model_file: File,
    pub drawing_file: Option<File>,
    // TODO: Send part id instead of part fields and retrieve the part data on the backend.
    //       Do this for security reasons. We don't want the client to be able to send fake values
    //       for these fields, specially for money related fields.
    pub process: String,
    pub material: String,
    pub tolerance: String,
    pub quantity: u64,
    pub sub_total: u64,
    pub deadline: NaiveDate,
}

impl From<&Part> for CreateCheckoutSessionPartData {
    fn from(part: &Part) -> Self {
        Self {
            part_id: part.id.clone(),
            name: part.model_file.name.clone(),
            model_file: part.model_file.clone(),
            drawing_file: part.drawing_file.clone(),
            process: part.process.clone(),
            material: part.material.clone(),
            tolerance: part.tolerance.clone(),
            quantity: part.quantity,
            sub_total: part.sub_total.unwrap(),
            deadline: Utc::now().naive_utc().date(), // TODO: Use the deadline from quotation price.
        }
    }
}

#[derive(Deserialize, Serialize, Debug)]
pub struct CompleteCheckoutSessionWebhookRequest {
    pub client_id: String,
    pub project_id: String,
    pub quotation_id: String,
    pub data: Vec<CompleteCheckoutSessionWebhookData>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct CompleteCheckoutSessionWebhookData {
    pub part_id: String,
    pub model_file: File,
    pub drawing_file: Option<File>,
    pub deadline: NaiveDate,
}

impl TryFrom<Option<HashMap<String, String>>> for CompleteCheckoutSessionWebhookRequest {
    type Error = WebhookRequestError;

    fn try_from(metadata: Option<HashMap<String, String>>) -> Result<Self, Self::Error> {
        println!("Getting");
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
                let data = serde_json::from_str::<Vec<CompleteCheckoutSessionWebhookData>>(
                    &metadata
                        .get("data")
                        .ok_or(WebhookRequestError::MissingField)?
                        .clone(),
                )
                .map_err(|_| WebhookRequestError::MissingField)?;

                Ok(Self {
                    client_id,
                    project_id,
                    quotation_id,
                    data,
                })
            }
            None => Err(WebhookRequestError::MissingMetadata),
        }
    }
}
