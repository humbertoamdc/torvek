use api_boundary::common::file::File;
use api_boundary::parts::requests::UpdatePartRequest;
use serde_derive::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub struct UpdatablePart {
    pub id: String,
    pub customer_id: String,
    pub quotation_id: String,
    pub drawing_file: Option<File>,
    pub process: Option<String>,
    pub material: Option<String>,
    pub tolerance: Option<String>,
    pub quantity: Option<u64>,
    pub selected_part_quote_id: Option<String>,
    pub clear_part_quotes: Option<bool>,
}

impl UpdatablePart {
    pub fn partial_new(quotation_id: String, part_id: String) -> Self {
        let mut updatable_part = Self::default();
        updatable_part.id = part_id;
        updatable_part.quotation_id = quotation_id;
        updatable_part
    }
}

impl From<&UpdatePartRequest> for UpdatablePart {
    fn from(request: &UpdatePartRequest) -> Self {
        Self {
            id: request.part_id.clone(),
            customer_id: request.customer_id.clone(),
            quotation_id: request.quotation_id.clone(),
            drawing_file: request.drawing_file.clone(),
            process: request.process.clone(),
            material: request.material.clone(),
            tolerance: request.tolerance.clone(),
            quantity: request.quantity,
            selected_part_quote_id: None,
            clear_part_quotes: Some(true),
        }
    }
}
