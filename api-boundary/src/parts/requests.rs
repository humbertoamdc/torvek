use serde_derive::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct CreatePartsRequest {
    pub client_id: String,
    pub project_id: String,
    pub quotation_id: String,
    pub file_names: Vec<String>,
}
impl CreatePartsRequest {
    pub const fn new(
        client_id: String,
        project_id: String,
        quotation_id: String,
        file_names: Vec<String>,
    ) -> Self {
        Self {
            client_id,
            project_id,
            quotation_id,
            file_names,
        }
    }
}
