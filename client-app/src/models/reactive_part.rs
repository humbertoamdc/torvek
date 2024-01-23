use api_boundary::common::file::File;
use api_boundary::parts::models::Part;
use leptos::{create_rw_signal, RwSignal};

#[derive(Debug, Clone, PartialEq)]
pub struct ReactivePart {
    pub id: String,
    pub client_id: String,
    pub project_id: String,
    pub quotation_id: String,
    pub model_file: RwSignal<File>,
    pub drawing_file: RwSignal<Option<File>>,
    pub process: RwSignal<String>,
    pub material: RwSignal<String>,
    pub tolerance: RwSignal<String>,
    pub quantity: RwSignal<u64>,
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
            model_file: create_rw_signal(part.model_file.clone()),
            drawing_file: create_rw_signal(part.drawing_file.clone()),
            process: create_rw_signal(part.process.clone()),
            material: create_rw_signal(part.material.clone()),
            tolerance: create_rw_signal(part.tolerance.clone()),
            quantity: create_rw_signal(part.quantity),
            unit_price: create_rw_signal(part.unit_price),
            sub_total: create_rw_signal(part.sub_total),
        }
    }
}
