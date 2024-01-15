use crate::common::file::File;
use chrono::{DateTime, Utc};
use serde_derive::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Part {
    pub id: String,
    pub client_id: String,
    pub project_id: String,
    pub quotation_id: String,
    pub file: File,
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
            file,
            created_at: now,
            updated_at: now,
        }
    }
}
