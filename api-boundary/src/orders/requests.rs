use chrono::NaiveDate;
use serde_derive::{Deserialize, Serialize};

use crate::common::file::File;
use crate::common::money::Money;
use crate::orders::models::OrderStatus;

#[derive(Deserialize, Serialize, Debug)]
pub struct AdminCreateOrdersRequest {
    pub client_id: String,
    pub project_id: String,
    pub quotation_id: String,
    pub data: Vec<AdminCreateOrdersRequestData>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct AdminCreateOrdersRequestData {
    pub part_id: String,
    pub model_file: File,
    pub payment: Money,
    pub deadline: NaiveDate,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct QueryOrdersByStatusRequest {
    pub status: OrderStatus,
}
