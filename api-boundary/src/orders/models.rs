use crate::common::file::File;
use crate::common::money::Money;
use chrono::{DateTime, NaiveDate, Utc};
use serde_derive::{Deserialize, Serialize};
use serde_enum_str::{Deserialize_enum_str, Serialize_enum_str};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Order {
    pub id: String,
    pub part_id: String,
    pub model_file: File,
    pub payment: Money,
    pub deadline: NaiveDate,
    pub status: OrderStatus,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl Order {
    pub fn new(
        part_id: String,
        model_file: File,
        payment: Money,
        deadline: NaiveDate,
        status: OrderStatus,
    ) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4().to_string(),
            part_id,
            model_file,
            payment,
            deadline,
            status,
            created_at: now,
            updated_at: now,
        }
    }
}

#[derive(Serialize_enum_str, Deserialize_enum_str, Clone, Debug, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum OrderStatus {
    Open,
    InProgress,
    ReadyForShipment,
    Shipped,
    Delivered,
}
