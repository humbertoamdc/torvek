use crate::orders::models::order::{Order, OrderStatus};
use crate::repositories::orders::{DynamodbOrder, OrdersRepository, QueryBy};
use crate::shared::error::Error;
use crate::shared::{CustomerId, OrderId, PartId, ProjectId, QueryResponse, QuoteId, Result};
use crate::utils::dynamodb_key_codec::DynamodbKeyCodec;
use async_trait::async_trait;
use aws_sdk_dynamodb::operation::query::builders::QueryFluentBuilder;
use aws_sdk_dynamodb::types::AttributeValue;
use chrono::{DateTime, Utc};
use serde_dynamo::from_items;
use serde_enum_str::Serialize_enum_str;
use std::collections::HashMap;

#[derive(Serialize_enum_str)]
enum TableIndex {
    #[serde(rename = "LSI1_CreationDateTime")]
    LSI1CreationDateTime,
    #[serde(rename = "LSI2_ProjectAndQuoteAndPart")]
    LSI2ProjectAndQuoteAndPart,
    #[serde(rename = "GSI1_OrderStatus")]
    GSI1OrderStatus,
    #[serde(rename = "GSI1_OrderIsOpen")]
    GSI2IsOpen,
}

#[derive(Clone)]
pub struct DynamodbOrders {
    client: aws_sdk_dynamodb::Client,
    table: String,
}

impl DynamodbOrders {
    pub fn new(client: aws_sdk_dynamodb::Client, table: String) -> Self {
        Self { client, table }
    }
}

#[async_trait]
impl OrdersRepository for DynamodbOrders {
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
    ) -> Result<QueryResponse<Vec<Order>, String>> {
        let mut query = {
            match query_by {
                QueryBy::Customer => {
                    let customer_id = customer_id
                        .ok_or(Error::MissingRequiredParameter(String::from("customer_id")))?;

                    if let Some(project_id) = project_id {
                        self.project_quote_part_query(project_id, quote_id, part_id)
                    } else if let Some(status) = status {
                        self.status_query(customer_id, status)
                    } else {
                        self.datetime_range_query(customer_id, from, to)
                    }
                }
                QueryBy::IsOpen => self.is_open_query(),
            }
        };

        query = query
            .table_name(&self.table)
            .limit(limit)
            .set_exclusive_start_key(DynamodbKeyCodec::decode_from_base64(cursor))
            .scan_index_forward(false);

        let response = query.send().await;

        match response {
            Ok(output) => {
                let items = output.items().to_vec();
                match from_items::<_, DynamodbOrder>(items) {
                    Ok(dynamodb_orders) => {
                        let mut orders = Vec::with_capacity(dynamodb_orders.len());
                        for dynamodb_order in dynamodb_orders {
                            orders.push(dynamodb_order.try_into()?);
                        }
                        Ok(QueryResponse {
                            data: orders,
                            cursor: DynamodbKeyCodec::encode_to_base64(output.last_evaluated_key()),
                        })
                    }
                    Err(err) => {
                        tracing::error!("{:?}", err);
                        Err(Error::UnknownError)
                    }
                }
            }
            Err(err) => {
                tracing::error!("{:?}", err);
                Err(Error::UnknownError)
            }
        }
    }

    async fn update(&self, _customer_id: CustomerId, _order_id: OrderId) -> Result<()> {
        todo!("Not implemented")
    }
}

impl DynamodbOrders {
    fn project_quote_part_query(
        &self,
        project_id: ProjectId,
        quote_id: Option<QuoteId>,
        part_id: Option<PartId>,
    ) -> QueryFluentBuilder {
        let mut lsi2_sk = project_id;

        for item_option in [quote_id, part_id] {
            match item_option {
                Some(item) => lsi2_sk.push_str(format!("&{item}").as_str()),
                None => break,
            }
        }

        self.client
            .query()
            .index_name(TableIndex::LSI2ProjectAndQuoteAndPart.to_string())
            .key_condition_expression("begins_with(lsi2_sk, :lsi2_sk)")
            .expression_attribute_values(":lsi2_sk", AttributeValue::S(lsi2_sk))
    }

    fn status_query(&self, customer_id: CustomerId, status: OrderStatus) -> QueryFluentBuilder {
        let prefix = format!("{status}");

        let expression_attribute_values: HashMap<String, AttributeValue> = [
            (String::from(":customer_id"), AttributeValue::S(customer_id)),
            (String::from(":prefix"), AttributeValue::S(prefix)),
        ]
        .into_iter()
        .collect();

        self.client
            .query()
            .index_name(TableIndex::GSI1OrderStatus.to_string())
            .key_condition_expression("pk = :customer_id AND begins_with(gsi1_sk, :prefix)")
            .set_expression_attribute_values(Some(expression_attribute_values))
    }

    fn datetime_range_query(
        &self,
        customer_id: CustomerId,
        from: Option<DateTime<Utc>>,
        to: Option<DateTime<Utc>>,
    ) -> QueryFluentBuilder {
        let lower_bound = from.unwrap_or(DateTime::<Utc>::UNIX_EPOCH).to_rfc3339();
        let upper_bound = to.unwrap_or(Utc::now()).to_rfc3339();

        let expression_attribute_values: HashMap<String, AttributeValue> = [
            (String::from(":customer_id"), AttributeValue::S(customer_id)),
            (String::from(":lower_bound"), AttributeValue::S(lower_bound)),
            (String::from(":upper_bound"), AttributeValue::S(upper_bound)),
        ]
        .into_iter()
        .collect();

        self.client
            .query()
            .index_name(TableIndex::LSI1CreationDateTime.to_string())
            .key_condition_expression(
                "pk = :customer_id AND lsi1_sk BETWEEN :lower_bound AND :upper_bound",
            )
            .set_expression_attribute_values(Some(expression_attribute_values))
    }

    fn is_open_query(&self) -> QueryFluentBuilder {
        self.client
            .query()
            .index_name(TableIndex::GSI2IsOpen.to_string())
            .key_condition_expression("gsi2_pk = :is_open")
            .expression_attribute_values(":is_open", AttributeValue::S(String::from("true")))
    }
}
