use crate::orders::models::order::{Address, Order, OrderStatus};
use crate::shared::error::Error;
use crate::shared::error::Error::UnknownError;
use crate::shared::money::Money;
use crate::shared::{
    CustomerId, OrderId, PartId, PartQuoteId, ProjectId, QueryResponse, QuoteId, Result,
};
use async_trait::async_trait;
use chrono::{DateTime, NaiveDate, Utc};
use serde_derive::{Deserialize, Serialize};
use std::str::FromStr;

pub const ATTRIBUTES_SEPARATOR: &str = "&";

pub enum QueryBy {
    Customer,
    IsOpen,
}

#[async_trait]
pub trait OrdersRepository: Send + Sync + 'static {
    async fn query(
        &self,
        customer_id: Option<String>,
        query_by: QueryBy,
        cursor: Option<String>,
        limit: i32,
    ) -> Result<QueryResponse<Vec<Order>, String>>;
    async fn update(&self, order_id: String, payout: Option<Money>) -> Result<()>;
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct DynamodbOrder {
    pub pk: CustomerId,
    pub sk: OrderId,
    /// created_at&order_id
    pub lsi1_sk: String,
    /// status&created_at&order_id
    pub lsi2_sk: String,
    /// project_id&quote_id&part_id&order_id
    pub lsi3_sk: String,
    pub part_quote_id: PartQuoteId,
    pub deadline: NaiveDate,
    pub shipping_recipient_name: String,
    pub shipping_address: Address,
    pub is_open: Option<bool>,
    pub update_at: DateTime<Utc>,
}

impl TryInto<Order> for DynamodbOrder {
    type Error = Error;

    fn try_into(self) -> std::result::Result<Order, Self::Error> {
        let mut status = None::<OrderStatus>;
        let mut project_id = None::<ProjectId>;
        let mut quote_id = None::<QuoteId>;
        let mut part_id = None::<PartId>;
        let mut created_at = None::<DateTime<Utc>>;

        let lsi2_sk_attributes = self
            .lsi2_sk
            .split(ATTRIBUTES_SEPARATOR)
            .collect::<Vec<&str>>();
        let lsi3_sk_attributes = self
            .lsi3_sk
            .split(ATTRIBUTES_SEPARATOR)
            .collect::<Vec<&str>>();

        if let [sk_status, sk_created_at] = &lsi2_sk_attributes[..] {
            if let Ok(parsed_status) = sk_status.parse() {
                status = Some(parsed_status);
            }
            created_at = Some(DateTime::<Utc>::from_str(sk_created_at).unwrap());
        }

        if let [sk_project_id, sk_quote_id, sk_part_id] = &lsi3_sk_attributes[..] {
            project_id = Some(sk_project_id.to_string());
            quote_id = Some(sk_quote_id.to_string());
            part_id = Some(sk_part_id.to_string());
        }

        let item = Order {
            id: self.sk.clone(),
            customer_id: self.pk,
            project_id: project_id.ok_or_else(|| {
                tracing::error!(
                    "project id is required but not found for order with id {}",
                    self.sk
                );
                UnknownError
            })?,
            quotation_id: quote_id.ok_or_else(|| {
                tracing::error!(
                    "quote id is required but not found for order with id {}",
                    self.sk
                );
                UnknownError
            })?,
            part_id: part_id.ok_or_else(|| {
                tracing::error!(
                    "part id is required but not found for order with id {}",
                    self.sk
                );
                UnknownError
            })?,
            part_quote_id: self.part_quote_id,
            deadline: self.deadline,
            status: status.ok_or_else(|| {
                tracing::error!(
                    "status is required but not found for order with id {}",
                    self.sk
                );
                UnknownError
            })?,
            shipping_recipient_name: self.shipping_recipient_name,
            shipping_address: self.shipping_address,
            is_open: self.is_open,
            created_at: created_at.ok_or_else(|| {
                tracing::error!(
                    "created_at required but not found for order with id {}",
                    self.sk
                );
                UnknownError
            })?,
            updated_at: self.update_at,
        };

        Ok(item)
    }
}

impl From<Order> for DynamodbOrder {
    fn from(value: Order) -> Self {
        let lsi1_sk = format!(
            "{}{ATTRIBUTES_SEPARATOR}{}",
            value.created_at.to_rfc3339(),
            value.id,
        );
        let lsi2_sk = format!(
            "{}{ATTRIBUTES_SEPARATOR}{}{ATTRIBUTES_SEPARATOR}{}",
            value.status,
            value.created_at.to_rfc3339(),
            value.id
        );
        let lsi3_sk = format!(
            "{}{ATTRIBUTES_SEPARATOR}{}{ATTRIBUTES_SEPARATOR}{}{ATTRIBUTES_SEPARATOR}{}",
            value.project_id, value.quotation_id, value.part_id, value.id
        );

        Self {
            pk: value.customer_id,
            sk: value.id,
            lsi1_sk,
            lsi2_sk,
            lsi3_sk,
            part_quote_id: value.part_quote_id,
            deadline: value.deadline,
            shipping_recipient_name: value.shipping_recipient_name,
            shipping_address: value.shipping_address,
            is_open: value.is_open,
            update_at: value.updated_at,
        }
    }
}
