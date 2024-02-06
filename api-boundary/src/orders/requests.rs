use crate::common::file::File;
use crate::common::money::Money;
use chrono::{DateTime, Utc};
use serde_derive::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct AdminCreateOrderRequest {
    pub part_id: String,
    pub model_file: File,
    pub payment: Money,
    pub deadline: DateTime<Utc>,
}
