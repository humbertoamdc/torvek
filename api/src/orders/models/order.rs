use crate::shared::{CustomerId, OrderId, PartId, PartQuoteId, ProjectId, QuoteId};
use chrono::{DateTime, NaiveDate, Utc};
use serde_derive::{Deserialize, Serialize};
use serde_enum_str::{Deserialize_enum_str, Serialize_enum_str};
use uuid::{ContextV7, Timestamp, Uuid};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Order {
    pub id: OrderId,
    pub customer_id: CustomerId,
    pub project_id: ProjectId,
    pub quotation_id: QuoteId,
    pub part_id: PartId,
    pub part_quote_id: PartQuoteId,
    pub deadline: NaiveDate,
    pub status: OrderStatus,
    pub shipping_recipient_name: String,
    pub shipping_address: Address,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl Order {
    pub fn new(
        customer_id: CustomerId,
        project_id: ProjectId,
        quotation_id: QuoteId,
        part_id: PartId,
        part_quote_id: PartQuoteId,
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
            created_at: now,
            updated_at: now,
        }
    }
}

#[derive(Serialize_enum_str, Deserialize_enum_str, Clone, Debug, PartialEq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum OrderStatus {
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
