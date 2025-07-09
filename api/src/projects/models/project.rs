use chrono::{DateTime, Utc};
use serde_derive::{Deserialize, Serialize};
use serde_enum_str::{Deserialize_enum_str, Serialize_enum_str};
use uuid::{ContextV7, Timestamp, Uuid};

pub type ProjectId = String;
pub type CustomerId = String;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Project {
    pub id: ProjectId,
    pub customer_id: CustomerId,
    pub name: String,
    pub status: ProjectStatus,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
impl Project {
    pub fn new(customer_id: String, name: String) -> Self {
        let id = Uuid::new_v7(Timestamp::now(ContextV7::new()));
        let encoded_id = format!("pro_{}", bs58::encode(id).into_string());
        let now = Utc::now();

        Self {
            id: encoded_id,
            customer_id,
            name,
            status: ProjectStatus::Created,
            created_at: now,
            updated_at: now,
        }
    }
}

#[derive(Serialize_enum_str, Deserialize_enum_str, Clone, Debug, PartialEq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ProjectStatus {
    Created,
    Locked,
}
