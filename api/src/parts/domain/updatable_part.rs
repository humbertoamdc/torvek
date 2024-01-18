use crate::orders::domain::order::UpdatableOrder;
use api_boundary::common::file::File;
use api_boundary::parts::requests::{AdminUpdatePartRequest, UpdatePartRequest};
use serde_derive::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct UpdatablePart {
    pub id: String,
    pub client_id: String,
    pub project_id: String,
    pub quotation_id: String,
    pub drawing_file: Option<File>,
    pub process: Option<String>,
    pub material: Option<String>,
    pub tolerance: Option<String>,
    pub quantity: Option<u64>,
    pub unit_price: Option<f64>,
    pub sub_total: Option<f64>,
}

impl From<&UpdatePartRequest> for UpdatablePart {
    fn from(request: &UpdatePartRequest) -> Self {
        Self {
            id: request.id.clone(),
            client_id: request.client_id.clone(),
            project_id: request.client_id.clone(),
            quotation_id: request.client_id.clone(),
            drawing_file: request.drawing_file.clone(),
            process: request.process.clone(),
            material: request.material.clone(),
            tolerance: request.tolerance.clone(),
            quantity: request.quantity,
            unit_price: None,
            sub_total: None,
        }
    }
}

impl From<&AdminUpdatePartRequest> for UpdatableOrder {
    fn from(request: &AdminUpdatePartRequest) -> Self {
        Self {
            drawing_file_name: None,
            drawing_file_url: None,
            process: None,
            material: None,
            tolerance: None,
            quantity: None,
            unit_price: request.unit_price,
            sub_total: request.sub_total,
        }
    }
}
