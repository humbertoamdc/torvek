use serde_derive::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct CreateQuotationRequest {
    pub client_id: String,
    pub project_id: String,
}
impl CreateQuotationRequest {
    pub const fn new(client_id: String, project_id: String) -> Self {
        Self {
            client_id,
            project_id,
        }
    }
}

#[derive(Deserialize, Serialize, Debug)]
pub struct QueryQuotationsForProjectRequest {
    pub project_id: String,
}
impl QueryQuotationsForProjectRequest {
    pub const fn new(project_id: String) -> Self {
        Self { project_id }
    }
}
