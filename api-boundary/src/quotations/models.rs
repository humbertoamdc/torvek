use chrono::{DateTime, Utc};
use names::Generator;
use serde_derive::{Deserialize, Serialize};
use serde_enum_str::{Deserialize_enum_str, Serialize_enum_str};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Quotation {
    pub id: String,
    pub client_id: String,
    pub project_id: String,
    pub name: String,
    pub status: QuotationStatus,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
impl Quotation {
    pub fn new(client_id: String, project_id: String) -> Self {
        let id = Uuid::new_v4().to_string();
        let name = Generator::default().next().unwrap();
        let now = Utc::now();

        Self {
            id,
            client_id,
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
