use crate::models::file::File;
use crate::models::part::{Part, PartAttributes, PartProcess};

pub mod api_error;
pub mod error;
pub mod file;
pub mod money;
pub mod order;
pub mod part;
pub mod quotation;

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
