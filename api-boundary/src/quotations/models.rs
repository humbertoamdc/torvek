use chrono::{DateTime, Utc};
use serde_derive::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Quotation {
    pub id: String,
    pub project_id: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
impl Quotation {
    pub fn new(project_id: String) -> Self {
        let now = Utc::now();

        Self {
            id: Uuid::new_v4().to_string(),
            project_id,
            created_at: now,
            updated_at: now,
        }
    }
}
