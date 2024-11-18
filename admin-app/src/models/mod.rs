use api_boundary::common::file::File;
use api_boundary::parts::models::{Part, PartAttributes, PartProcess};

#[derive(Debug, Clone, PartialEq)]
pub struct ReactivePart {
    pub id: String,
    pub customer_id: String,
    pub project_id: String,
    pub quotation_id: String,
    pub model_file: File,
    pub drawing_file: Option<File>,
    pub process: PartProcess,
    pub attributes: PartAttributes,
    pub quantity: u64,
}

impl From<&Part> for ReactivePart {
    fn from(part: &Part) -> Self {
        Self {
            id: part.id.clone(),
            customer_id: part.customer_id.clone(),
            project_id: part.project_id.clone(),
            quotation_id: part.quotation_id.clone(),
            model_file: part.model_file.clone(),
            drawing_file: part.drawing_file.clone(),
            process: part.process.clone(),
            attributes: part.attributes.clone(),
            quantity: part.quantity,
        }
    }
}
