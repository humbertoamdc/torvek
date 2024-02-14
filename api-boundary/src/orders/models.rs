use chrono::{DateTime, NaiveDate, Utc};
use serde_derive::{Deserialize, Serialize};
use serde_enum_str::{Deserialize_enum_str, Serialize_enum_str};
use uuid::Uuid;

use crate::common::file::File;
use crate::common::money::Money;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Order {
    pub id: String,
    pub part_id: String,
    pub model_file: File,
    pub drawing_file: Option<File>,
    pub quantity: u64,
    pub payout: Option<Money>,
    pub deadline: NaiveDate,
    pub status: OrderStatus,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl Order {
    pub fn new(
        part_id: String,
        model_file: File,
        drawing_file: Option<File>,
        quantity: u64,
        payout: Option<Money>,
        deadline: NaiveDate,
        status: OrderStatus,
    ) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4().to_string(),
            part_id,
            model_file,
            drawing_file,
            quantity,
            payout,
            deadline,
            status,
            created_at: now,
            updated_at: now,
        }
    }
}

#[derive(Serialize_enum_str, Deserialize_enum_str, Clone, Debug, PartialEq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum OrderStatus {
    PendingPricing,
    Open,
    InProgress,
    ReadyForShipment,
    Shipped,
    Delivered,
}
