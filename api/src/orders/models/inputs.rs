use crate::auth::models::session::Identity;
use crate::shared::file::File;
use crate::shared::money::Money;
use chrono::NaiveDate;
use serde_derive::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct AdminCreateOrdersRequestData {
    pub part_id: String,
    pub model_file: File,
    pub payment: Money,
    pub deadline: NaiveDate,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct QueryOrdersForCustomerInput {
    pub identity: Identity,
    pub with_part_data: bool,
    pub cursor: Option<String>,
    pub limit: i32,
}
