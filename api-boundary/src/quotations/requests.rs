use serde_derive::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct CreateQuotationRequest {
    pub project_id: String,
}
impl CreateQuotationRequest {
    pub const fn new(project_id: String) -> Self {
        Self { project_id }
    }
}
