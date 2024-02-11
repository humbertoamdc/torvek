use serde_derive::{Deserialize, Serialize};

use crate::orders::models::Order;

#[derive(Deserialize, Serialize, Debug)]
pub struct CreateDrawingUploadUrlResponse {
    pub url: String,
}

impl CreateDrawingUploadUrlResponse {
    pub const fn new(url: String) -> Self {
        Self { url }
    }
}

#[derive(Deserialize, Serialize, Debug)]
pub struct QueryOrdersByStatusResponse {
    pub orders: Vec<Order>,
}
