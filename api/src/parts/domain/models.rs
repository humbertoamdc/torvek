use api_boundary::common::file::File;
use api_boundary::parts::models::Part;
use chrono::{DateTime, Utc};
use serde_derive::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct DynamodbPartItem {
    pub id: String,
    #[serde(rename = "client_id#project_id#quotation_id")]
    pub client_project_and_quotation_ids: String,
    pub file: File,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl Into<Part> for DynamodbPartItem {
    fn into(self) -> Part {
        let [client_id, project_id, quotation_id] = self
            .client_project_and_quotation_ids
            .split("#")
            .map(|id| id.to_string())
            .collect::<Vec<String>>()
            .try_into()
            .unwrap();

        Part {
            id: self.id,
            client_id,
            project_id,
            quotation_id,
            file: self.file,
            created_at: self.created_at,
            updated_at: self.updated_at,
        }
    }
}

impl From<Part> for DynamodbPartItem {
    fn from(part: Part) -> Self {
        Self {
            id: part.id,
            client_project_and_quotation_ids: format!(
                "{}#{}#{}",
                part.client_id, part.project_id, part.quotation_id
            ),
            file: part.file,
            created_at: part.created_at,
            updated_at: part.updated_at,
        }
    }
}
