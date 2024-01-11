use crate::orders::adapters::api::requests::{AdminUpdateOrderRequest, UpdateOrderRequest};
use chrono::{DateTime, Utc};
use serde_derive::{Deserialize, Serialize};
use serde_enum_str::{Deserialize_enum_str, Serialize_enum_str};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Order {
    pub id: String,
    pub client_id: String,
    pub file_name: String,
    pub file_url: String,
    pub drawing_file_name: Option<String>,
    pub drawing_file_url: Option<String>,
    pub order_status: OrderStatus,
    pub process: String,
    pub material: String,
    pub tolerance: String,
    pub quantity: u64,
    pub unit_price: Option<u64>,
    pub sub_total: Option<u64>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl Order {
    pub fn new(client_id: String, file_name: String, file_url: String) -> Self {
        let now = Utc::now();

        Self {
            id: Uuid::new_v4().to_string(),
            client_id,
            file_name,
            file_url,
            drawing_file_name: None,
            drawing_file_url: None,
            order_status: OrderStatus::PendingQuotation,
            process: String::from("CNC"),
            material: String::from("Aluminum 6061-T6"),
            tolerance: String::from("ISO 2768 Medium"),
            quantity: 1,
            unit_price: None,
            sub_total: None,
            created_at: now,
            updated_at: now,
        }
    }
}

#[derive(Serialize_enum_str, Deserialize_enum_str, Clone, Debug)]
#[serde(rename_all = "snake_case")]
pub enum OrderStatus {
    PendingQuotation,
    PendingPayment,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct UpdatableOrder {
    pub drawing_file_name: Option<String>,
    pub drawing_file_url: Option<String>,
    pub process: Option<String>,
    pub material: Option<String>,
    pub tolerance: Option<String>,
    pub quantity: Option<u64>,
    pub unit_price: Option<f64>,
    pub sub_total: Option<f64>,
}

impl From<&UpdateOrderRequest> for UpdatableOrder {
    fn from(request: &UpdateOrderRequest) -> Self {
        Self {
            drawing_file_name: request.drawing_file_name.clone(),
            drawing_file_url: request.drawing_file_url.clone(),
            process: request.process.clone(),
            material: request.material.clone(),
            tolerance: request.tolerance.clone(),
            quantity: request.quantity,
            unit_price: None,
            sub_total: None,
        }
    }
}

impl From<&AdminUpdateOrderRequest> for UpdatableOrder {
    fn from(request: &AdminUpdateOrderRequest) -> Self {
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
