use crate::common::file::File;
use crate::common::money::Money;
use chrono::{DateTime, Utc};
use serde_derive::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct CreateOrderRequest {
    pub part_id: String,
    pub model_file: File,
    pub payment: Money,
    pub deadline: DateTime<Utc>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct CreateDrawingUploadUrlRequest {
    pub client_id: String,
    pub file_name: String,
    pub file_url: Option<String>,
}

impl CreateDrawingUploadUrlRequest {
    pub const fn new(client_id: String, file_name: String, file_url: Option<String>) -> Self {
        Self {
            client_id,
            file_name,
            file_url,
        }
    }
}
