use chrono::{DateTime, NaiveDate, Utc};
use serde_derive::{Deserialize, Serialize};
use uuid::{ContextV7, Timestamp, Uuid};

use crate::common::file::File;
use crate::common::money::Money;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Part {
    pub id: String,
    pub customer_id: String,
    pub project_id: String,
    pub quotation_id: String,
    pub model_file: File,
    pub render_file: File,
    pub drawing_file: Option<File>,
    pub process: String,   // TODO: Extract to enum in api-boundary.
    pub material: String,  // TODO: Extract to enum in api-boundary.
    pub tolerance: String, // TODO: Extract to enum in api-boudnary.
    pub quantity: u64,
    pub part_quotes: Option<Vec<PartQuote>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
impl Part {
    pub fn new(
        customer_id: String,
        project_id: String,
        quotation_id: String,
        model_file: File,
        render_file: File,
    ) -> Self {
        let now = Utc::now();
        let id = Uuid::new_v7(Timestamp::now(ContextV7::new()));
        let encoded_id = format!("part_{}", bs58::encode(id).into_string());

        Self {
            id: encoded_id,
            customer_id,
            project_id,
            quotation_id,
            model_file,
            render_file,
            drawing_file: None,
            process: String::from("CNC"),
            material: String::from("Aluminum 6061-T6"),
            tolerance: String::from("+/- .005\" (+/- 0.13mm)"),
            quantity: 1,
            part_quotes: None,
            created_at: now,
            updated_at: now,
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct PartQuote {
    pub id: String,
    pub part_id: String,
    pub unit_price: Money,
    pub sub_total: Money,
    pub deadline: NaiveDate,
    pub selected: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl PartQuote {
    pub fn new(part_id: String, unit_price: Money, sub_total: Money, deadline: NaiveDate) -> Self {
        let now = Utc::now();
        let id = Uuid::new_v7(Timestamp::now(ContextV7::new()));
        let encoded_id = format!("pq_{}", bs58::encode(id).into_string());

        Self {
            id: encoded_id,
            part_id,
            unit_price,
            sub_total,
            deadline,
            selected: true,
            created_at: now,
            updated_at: now,
        }
    }
}
