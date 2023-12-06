use serde_derive::{Deserialize, Serialize};

use crate::orders::domain::order::OrderStatus;

#[derive(Deserialize, Serialize, Debug)]
pub struct CreateOrdersRequest {
    pub client_id: String,
    pub file_names: Vec<String>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct QueryOrdersForClientRequest {
    pub client_id: String,
}

impl QueryOrdersForClientRequest {
    pub const fn new(client_id: String) -> Self {
        Self { client_id }
    }
}

#[derive(Deserialize, Serialize, Debug)]
pub struct AdminQueryOrdersByStatusRequest {
    pub order_status: OrderStatus,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct UpdateOrderRequest {
    pub order_id: String,
    pub client_id: String,
    pub process: Option<String>,
    pub material: Option<String>,
    pub tolerance: Option<String>,
    pub quantity: Option<u64>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct AdminUpdateOrderRequest {
    pub order_id: String,
    pub client_id: String,
    pub unit_price: Option<f64>,
    pub sub_total: Option<f64>,
}
