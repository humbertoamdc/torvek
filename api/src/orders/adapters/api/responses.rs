use serde_derive::{Deserialize, Serialize};

use crate::orders::domain::order::Order;

#[derive(Deserialize, Serialize, Debug)]
pub struct CreateOrdersResponse {
    pub id: String,
    pub upload_url: String,
}

impl CreateOrdersResponse {
    pub const fn new(id: String, upload_url: String) -> Self {
        Self { id, upload_url }
    }
}

#[derive(Deserialize, Serialize, Debug)]
pub struct QueryOrdersForClientResponse {
    pub orders: Vec<Order>,
}

impl QueryOrdersForClientResponse {
    pub const fn new(orders: Vec<Order>) -> Self {
        Self { orders }
    }
}
