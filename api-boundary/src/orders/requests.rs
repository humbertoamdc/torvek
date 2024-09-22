use chrono::NaiveDate;
use serde_derive::{Deserialize, Serialize};

use crate::common::file::File;
use crate::common::money::Money;

#[derive(Deserialize, Serialize, Debug)]
pub struct AdminCreateOrdersRequest {
    pub customer_id: String,
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
pub struct QueryOpenOrdersRequest {}

#[derive(Deserialize, Serialize, Debug)]
pub struct QueryOrdersForCustomerRequest {
    pub customer_id: String,
    pub with_part_data: bool,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct AdminUpdateOrderPayoutRequest {
    pub order_id: String,
    pub payout: Money,
}
