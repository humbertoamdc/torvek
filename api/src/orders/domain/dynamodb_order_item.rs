use api_boundary::orders::models::{Address, Order, OrderStatus};
use chrono::{DateTime, NaiveDate, Utc};
use serde_derive::{Deserialize, Serialize};
use std::str::FromStr;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct DynamodbOrderItem {
    customer_id: String,
    #[serde(rename = "status#id")]
    status_and_id: String,
    pub project_id: String,
    pub quotation_id: String,
    pub part_id: String,
    pub part_quote_id: String,
    pub deadline: NaiveDate,
    pub is_open: Option<String>,
    pub shipping_recipient_name: String,
    pub shipping_address: Address,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl Into<Order> for DynamodbOrderItem {
    fn into(self) -> Order {
        let [status, id] = self
            .status_and_id
            .split("#")
            .map(|element| element.to_string())
            .collect::<Vec<String>>()
            .try_into()
            .unwrap();

        Order {
            id,
            status: OrderStatus::from_str(&status).unwrap(),
            customer_id: self.customer_id,
            project_id: self.project_id,
            quotation_id: self.quotation_id,
            part_id: self.part_id,
            part_quote_id: self.part_quote_id,
            deadline: self.deadline,
            is_open: self.is_open,
            shipping_recipient_name: self.shipping_recipient_name,
            shipping_address: self.shipping_address,
            created_at: self.created_at,
            updated_at: self.updated_at,
        }
    }
}

impl From<Order> for DynamodbOrderItem {
    fn from(order: Order) -> Self {
        Self {
            customer_id: order.customer_id,
            status_and_id: format!("{}#{}", order.status, order.id),
            project_id: order.project_id,
            quotation_id: order.quotation_id,
            part_id: order.part_id,
            part_quote_id: order.part_quote_id,
            deadline: order.deadline,
            is_open: order.is_open,
            shipping_recipient_name: order.shipping_recipient_name,
            shipping_address: order.shipping_address,
            created_at: order.created_at,
            updated_at: order.updated_at,
        }
    }
}
