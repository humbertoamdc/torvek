use api_boundary::quotations::models::Quotation;
use chrono::{DateTime, Utc};
use serde_derive::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct DynamodbQuotationItem {
    pub id: String,
    #[serde(rename = "client_id#project_id")]
    pub client_id_and_project_id: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl Into<Quotation> for DynamodbQuotationItem {
    fn into(self) -> Quotation {
        let [client_id, project_id] = self
            .client_id_and_project_id
            .split("#")
            .map(|id| id.to_string())
            .collect::<Vec<String>>()
            .try_into()
            .unwrap();

        Quotation {
            id: self.id,
            client_id,
            project_id,
            created_at: self.created_at,
            updated_at: self.updated_at,
        }
    }
}

impl From<Quotation> for DynamodbQuotationItem {
    fn from(quotation: Quotation) -> Self {
        Self {
            id: quotation.id,
            client_id_and_project_id: format!("{}#{}", quotation.client_id, quotation.project_id),
            created_at: quotation.created_at,
            updated_at: quotation.updated_at,
        }
    }
}
