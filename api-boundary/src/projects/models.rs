use chrono::{DateTime, Utc};
use serde_derive::{Deserialize, Serialize};
use uuid::{ContextV7, Timestamp, Uuid};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Project {
    pub id: String,
    pub customer_id: String,
    pub name: String,
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
            created_at: now,
            updated_at: now,
        }
    }
}
