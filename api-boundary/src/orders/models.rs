use chrono::{DateTime, NaiveDate, Utc};
use serde_derive::{Deserialize, Serialize};
use serde_enum_str::{Deserialize_enum_str, Serialize_enum_str};
use uuid::{ContextV7, Timestamp, Uuid};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Order {
    pub id: String,
    pub customer_id: String,
    pub project_id: String,
    pub quotation_id: String,
    pub part_id: String,
    pub part_quote_id: String,
    pub deadline: NaiveDate,
    pub status: OrderStatus,
    pub shipping_recipient_name: String,
    pub shipping_address: Address,
    pub is_open: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl Order {
    pub fn new(
        customer_id: String,
        project_id: String,
        quotation_id: String,
        part_id: String,
        part_quote_id: String,
        deadline: NaiveDate,
        status: OrderStatus,
        shipping_recipient_name: String,
        shipping_address: Address,
    ) -> Self {
        let now = Utc::now();
        let id = Uuid::new_v7(Timestamp::now(ContextV7::new()));
        let encoded_id = format!("ord_{}", bs58::encode(id).into_string());

        Self {
            id: encoded_id,
            customer_id,
            project_id,
            quotation_id,
            part_id,
            part_quote_id,
            deadline,
            status,
            shipping_recipient_name,
            shipping_address,
            is_open: Some(String::from("1")),
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
    Ready,
    Shipped,
    Delivered,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct Address {
    /// City, district, suburb, town, or village.
    pub city: Option<String>,
    /// Two-letter country code ([ISO 3166-1 alpha-2](https://en.wikipedia.org/wiki/ISO_3166-1_alpha-2)).
    pub country: Option<String>,
    /// Address line 1 (e.g., street, PO Box, or company name).
    pub line1: Option<String>,
    /// Address line 2 (e.g., apartment, suite, unit, or building).
    pub line2: Option<String>,
    /// ZIP or postal code.
    pub postal_code: Option<String>,
    /// State, county, province, or region.
    pub state: Option<String>,
}
