use crate::parts::models::Part;
use serde_derive::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct CreateCheckoutSessionRequest {
    pub client_id: String,
    pub project_id: String,
    pub quotation_id: String,
    pub parts_data: Vec<CreateCheckoutSessionPartData>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct CreateCheckoutSessionPartData {
    pub name: String,
    pub process: String,
    pub material: String,
    pub tolerance: String,
    pub quantity: u64,
    pub sub_total: u64,
}

impl From<&Part> for CreateCheckoutSessionPartData {
    fn from(part: &Part) -> Self {
        Self {
            name: part.model_file.name.clone(),
            process: part.process.clone(),
            material: part.material.clone(),
            tolerance: part.tolerance.clone(),
            quantity: part.quantity,
            sub_total: part.sub_total.unwrap(),
        }
    }
}
