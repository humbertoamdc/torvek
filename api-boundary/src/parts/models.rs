use chrono::{DateTime, Utc};
use serde_derive::{Deserialize, Serialize};
use serde_enum_str::{Deserialize_enum_str, Serialize_enum_str};
use uuid::Uuid;

use crate::common::file::File;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Part {
    pub id: String,
    pub client_id: String,
    pub project_id: String,
    pub quotation_id: String,
    pub model_file: File,
    pub drawing_file: Option<File>,
    pub status: PartStatus,
    pub process: String,   // TODO: Extract to enum in api-boundary.
    pub material: String,  // TODO: Extract to enum in api-boundary.
    pub tolerance: String, // TODO: Extract to enum in api-boudnary.
    pub quantity: u64,
    pub unit_price: Option<u64>,
    pub sub_total: Option<u64>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
impl Part {
    pub fn new(client_id: String, project_id: String, quotation_id: String, file: File) -> Self {
        let now = Utc::now();

        Self {
            id: Uuid::new_v4().to_string(),
            client_id,
            project_id,
            quotation_id,
            model_file: file,
            drawing_file: None,
            status: PartStatus::AwaitingPricing,
            process: String::from("CNC"),
            material: String::from("Aluminum 6061-T6"),
            tolerance: String::from("ISO 2768 Medium"),
            quantity: 1,
            unit_price: None,
            sub_total: None,
            created_at: now,
            updated_at: now,
        }
    }
}

#[derive(Serialize_enum_str, Deserialize_enum_str, Clone, Debug, PartialEq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum PartStatus {
    AwaitingPricing,
    Ready,
}
