use chrono::{DateTime, Utc};
use serde_derive::{Deserialize, Serialize};
use serde_enum_str::{Deserialize_enum_str, Serialize_enum_str};
use uuid::{ContextV7, Timestamp, Uuid};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Quotation {
    pub id: String,
    pub customer_id: String,
    pub project_id: String,
    pub name: String,
    pub status: QuotationStatus,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
impl Quotation {
    pub fn new(customer_id: String, project_id: String, name: String) -> Self {
        let id = Uuid::new_v7(Timestamp::now(ContextV7::new()));
        let encoded_id = format!("quo_{}", bs58::encode(id).into_string());
        let now = Utc::now();

        Self {
            id: encoded_id,
            customer_id,
            project_id,
            name,
            status: QuotationStatus::Created,
            created_at: now,
            updated_at: now,
        }
    }
}

#[derive(Serialize_enum_str, Deserialize_enum_str, Clone, Debug, PartialEq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum QuotationStatus {
    Created,
    PendingPayment,
    Payed,
}
