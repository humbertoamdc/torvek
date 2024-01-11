use chrono::{DateTime, Utc};
use serde_derive::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Project {
    pub id: String,
    pub client_id: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
impl Project {
    pub fn new(client_id: String) -> Self {
        let now = Utc::now();

        Self {
            id: Uuid::new_v4().to_string(),
            client_id,
            created_at: now,
            updated_at: now,
        }
    }
}
