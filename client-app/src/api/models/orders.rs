use chrono::{DateTime, Utc};
use leptos::{create_rw_signal, RwSignal};
use serde_derive::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct CreateOrdersRequest {
    client_id: String,
    file_names: Vec<String>,
}

impl CreateOrdersRequest {
    pub const fn new(client_id: String, file_names: Vec<String>) -> Self {
        Self {
            client_id,
            file_names,
        }
    }
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct UpdateOrderRequest {
    pub order_id: String,
    pub client_id: String,
    pub drawing_file_name: Option<String>,
    pub drawing_file_url: Option<String>,
    pub process: Option<String>,
    pub material: Option<String>,
    pub tolerance: Option<String>,
    pub quantity: Option<u64>,
}

impl UpdateOrderRequest {
    pub const fn new(
        order_id: String,
        client_id: String,
        drawing_file_name: Option<String>,
        drawing_file_url: Option<String>,
        process: Option<String>,
        material: Option<String>,
        tolerance: Option<String>,
        quantity: Option<u64>,
    ) -> Self {
        Self {
            order_id,
            client_id,
            drawing_file_name,
            drawing_file_url,
            process,
            material,
            tolerance,
            quantity,
        }
    }
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct CreateOrdersResponse {
    pub id: String,
    pub upload_url: String,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct QueryOrdersForClientResponse {
    pub orders: Vec<Order>,
}

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq, Default)]
pub struct Order {
    pub id: String,
    pub client_id: String,
    pub file_name: String,
    pub file_url: String,
    pub drawing_file_name: Option<String>,
    pub drawing_file_url: Option<String>,
    pub process: String,
    pub material: String,
    pub tolerance: String,
    pub quantity: u64,
    pub unit_price: Option<u64>,
    pub sub_total: Option<u64>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ReactiveOrder {
    pub id: String,
    pub client_id: String,
    pub file_name: String,
    pub file_url: RwSignal<String>,
    pub drawing_file_name: RwSignal<Option<String>>,
    pub drawing_file_url: RwSignal<Option<String>>,
    pub process: RwSignal<String>,
    pub material: RwSignal<String>,
    pub tolerance: RwSignal<String>,
    pub quantity: RwSignal<u64>,
    pub unit_price: RwSignal<Option<u64>>,
    pub sub_total: RwSignal<Option<u64>>,
}

impl From<&Order> for ReactiveOrder {
    fn from(order: &Order) -> Self {
        Self {
            id: order.id.clone(),
            client_id: order.client_id.clone(),
            file_name: order.file_name.clone(),
            file_url: create_rw_signal(order.file_url.clone()),
            drawing_file_name: create_rw_signal(order.drawing_file_name.clone()),
            drawing_file_url: create_rw_signal(order.drawing_file_url.clone()),
            process: create_rw_signal(order.process.clone()),
            material: create_rw_signal(order.material.clone()),
            tolerance: create_rw_signal(order.tolerance.clone()),
            quantity: create_rw_signal(order.quantity),
            unit_price: create_rw_signal(order.unit_price),
            sub_total: create_rw_signal(order.sub_total),
        }
    }
}
