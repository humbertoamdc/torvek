use serde_derive::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct CreatePartsRequest {
    pub client_id: String,
    pub project_id: String,
    pub quotation_id: String,
    pub file_names: Vec<String>,
}
