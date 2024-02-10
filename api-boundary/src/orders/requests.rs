use crate::common::file::File;
use crate::common::money::Money;
use chrono::NaiveDate;
use serde_derive::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct AdminCreateOrdersRequest {
    pub data: Vec<AdminCreateOrdersRequestData>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct AdminCreateOrdersRequestData {
    pub part_id: String,
    pub model_file: File,
    pub payment: Money,
    pub deadline: NaiveDate,
}
