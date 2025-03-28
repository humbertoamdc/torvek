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

#[derive(Deserialize)]
pub struct QueryOrdersForCustomerQueryParameters {
    pub with_part_data: Option<bool>,
    pub cursor: Option<String>,
    pub limit: Option<i32>,
}
#[derive(Deserialize, Serialize, Debug)]
pub struct QueryOrdersForCustomerRequest {
    pub customer_id: String,
    pub with_part_data: bool,
    pub cursor: Option<String>,
    pub limit: i32,
}
impl QueryOrdersForCustomerRequest {
    pub fn new(customer_id: String, params: QueryOrdersForCustomerQueryParameters) -> Self {
        let max_limit = 10;
        Self {
            customer_id,
            with_part_data: params.with_part_data.unwrap_or(false),
            cursor: params.cursor,
            limit: params.limit.unwrap_or(max_limit),
        }
    }
}

#[derive(Deserialize, Serialize, Debug)]
pub struct AdminUpdateOrderPayoutRequest {
    pub order_id: String,
    pub payout: Money,
}
