use api_boundary::common::file::File;
use api_boundary::parts::models::Part;
use leptos::{create_rw_signal, RwSignal};

#[derive(Debug, Clone, PartialEq)]
pub struct ReactivePart {
    pub id: String,
    pub client_id: String,
    pub project_id: String,
    pub quotation_id: String,
    pub model_file: File,
    pub drawing_file: Option<File>,
    pub process: String,
    pub material: String,
    pub tolerance: String,
    pub quantity: u64,
    pub unit_price: RwSignal<Option<u64>>,
    pub sub_total: RwSignal<Option<u64>>,
}

impl From<&Part> for ReactivePart {
    fn from(part: &Part) -> Self {
        Self {
            id: part.id.clone(),
            client_id: part.client_id.clone(),
            project_id: part.project_id.clone(),
            quotation_id: part.quotation_id.clone(),
            model_file: part.model_file.clone(),
            drawing_file: part.drawing_file.clone(),
            process: part.process.clone(),
            material: part.material.clone(),
            tolerance: part.tolerance.clone(),
            quantity: part.quantity,
            unit_price: create_rw_signal(part.unit_price),
            sub_total: create_rw_signal(part.sub_total),
        }
    }
}
