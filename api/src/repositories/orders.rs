use crate::orders::models::order::{Address, Order, OrderStatus};
use crate::shared::error::Error;
use crate::shared::error::Error::UnknownError;
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
    type TransactionItem;
    async fn query(
        &self,
        customer_id: Option<CustomerId>,
        project_id: Option<ProjectId>,
        quote_id: Option<QuoteId>,
        part_id: Option<PartId>,
        status: Option<OrderStatus>,
        from: Option<DateTime<Utc>>,
        to: Option<DateTime<Utc>>,
        query_by: QueryBy,
        cursor: Option<String>,
        limit: i32,
    ) -> Result<QueryResponse<Vec<Order>, String>>;
    async fn update(&self, customer_id: CustomerId, order_id: OrderId) -> Result<()>;
    fn transaction_create(&self, order: Order) -> Self::TransactionItem;
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct DynamodbOrder {
    pub pk: CustomerId,
    pub sk: OrderId,
    /// created_at&order_id
    pub lsi1_sk: String,
    /// project_id&quote_id&part_id&order_id
    pub lsi2_sk: String,
    /// status&created_at&order_id
    pub gsi1_sk: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// is_open
    pub gsi2_pk: Option<String>,
    /// created_at&order_id
    pub gsi2_sk: String,
    pub part_quote_id: PartQuoteId,
    pub deadline: NaiveDate,
    pub shipping_recipient_name: String,
    pub shipping_address: Address,
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

        let gsi1_sk_attributes = self
            .gsi1_sk
            .split(ATTRIBUTES_SEPARATOR)
            .collect::<Vec<&str>>();
        let lsi2_sk_attributes = self
            .lsi2_sk
            .split(ATTRIBUTES_SEPARATOR)
            .collect::<Vec<&str>>();

        if let [sk_status, sk_created_at, _] = &gsi1_sk_attributes[..] {
            if let Ok(parsed_status) = sk_status.parse() {
                status = Some(parsed_status);
            }
            created_at = Some(DateTime::<Utc>::from_str(sk_created_at).unwrap());
        }

        if let [sk_project_id, sk_quote_id, sk_part_id, _] = &lsi2_sk_attributes[..] {
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
            "{}{ATTRIBUTES_SEPARATOR}{}{ATTRIBUTES_SEPARATOR}{}{ATTRIBUTES_SEPARATOR}{}",
            value.project_id, value.quotation_id, value.part_id, value.id
        );
        let gsi1_sk = format!(
            "{}{ATTRIBUTES_SEPARATOR}{}{ATTRIBUTES_SEPARATOR}{}",
            value.status,
            value.created_at.to_rfc3339(),
            value.id
        );
        let gsi2_sk = format!(
            "{}{ATTRIBUTES_SEPARATOR}{}",
            value.created_at.to_rfc3339(),
            value.id,
        );

        let gsi2_pk = if value.status == OrderStatus::Open {
            Some(String::from("true"))
        } else {
            None
        };

        Self {
            pk: value.customer_id,
            sk: value.id,
            lsi1_sk,
            lsi2_sk,
            gsi1_sk,
            gsi2_pk,
            gsi2_sk,
            part_quote_id: value.part_quote_id,
            deadline: value.deadline,
            shipping_recipient_name: value.shipping_recipient_name,
            shipping_address: value.shipping_address,
            update_at: value.updated_at,
        }
    }
}
