use serde_derive::{Deserialize, Serialize};

use crate::orders::models::Order;
use crate::parts::models::Part;

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
pub struct QueryOpenOrdersResponse {
    pub orders: Vec<Order>,
    pub cursor: Option<String>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct QueryOrdersForCustomerResponse {
    pub data: Vec<QueryOrdersForCustomerResponseData>,
    pub cursor: Option<String>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct QueryOrdersForCustomerResponseData {
    pub order: Order,
    pub part: Option<Part>,
}
