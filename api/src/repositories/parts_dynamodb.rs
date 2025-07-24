use crate::parts::models::dynamodb_requests::{BatchDeletePartObject, UpdatablePart};
use crate::parts::models::part::{Part, PartQuote};
use crate::repositories::parts::{DynamodbPart, PartsRepository};
use crate::shared::error::Error;
use crate::shared::{CustomerId, PartId, PartQuoteId, QueryResponse, QuoteId, Result};
use crate::utils::dynamodb_key_codec::DynamodbKeyCodec;
use async_trait::async_trait;
use aws_sdk_dynamodb::types::{
    AttributeValue, DeleteRequest, KeysAndAttributes, PutRequest, ReturnValue, TransactWriteItem,
    Update, WriteRequest,
};
use chrono::Utc;
use serde_dynamo::aws_sdk_dynamodb_1::from_item;
use serde_dynamo::{from_items, to_item};
use serde_enum_str::Serialize_enum_str;
use std::collections::HashMap;

#[derive(Serialize_enum_str)]
enum TableIndex {
    #[serde(rename = "LSI1_QuoteAndCreationDateTime")]
    LSI1QuoteAndCreationDateTime,
}

#[derive(Clone)]
pub struct DynamodbParts {
    client: aws_sdk_dynamodb::Client,
    table: String,
}

impl DynamodbParts {
    pub fn new(client: aws_sdk_dynamodb::Client, table: String) -> Self {
        Self { client, table }
    }
}

#[async_trait]
impl PartsRepository for DynamodbParts {
    type TransactionItem = TransactWriteItem;

    async fn delete(&self, customer_id: CustomerId, part_id: PartId) -> Result<Part> {
        let response = self
            .client
            .delete_item()
            .table_name(&self.table)
            .key("pk", AttributeValue::S(customer_id))
            .key("sk", AttributeValue::S(part_id))
            .return_values(ReturnValue::AllOld)
            .send()
            .await;

        match response {
            Ok(output) => match output.attributes {
                Some(item) => match from_item::<DynamodbPart>(item) {
                    Ok(dynamodb_part) => dynamodb_part.try_into(),
                    Err(err) => {
                        tracing::error!("{err:?}");
                        Err(Error::UnknownError)
                    }
                },
                None => Err(Error::ItemNotFoundError),
            },
            Err(err) => {
                tracing::error!("{err:?}");
                Err(Error::UnknownError)
            }
        }
    }

    async fn get(&self, customer_id: CustomerId, part_id: PartId) -> Result<Part> {
        let response = self
            .client
            .get_item()
            .table_name(&self.table)
            .key(String::from("pk"), AttributeValue::S(customer_id))
            .key(String::from("sk"), AttributeValue::S(part_id))
            .send()
            .await;

        match response {
            Ok(output) => match output.item {
                Some(item) => match from_item::<DynamodbPart>(item) {
                    Ok(dynamodb_part) => dynamodb_part.try_into(),
                    Err(err) => {
                        tracing::error!("{err:?}");
                        Err(Error::UnknownError)
                    }
                },
                None => Err(Error::ItemNotFoundError),
            },
            Err(err) => {
                tracing::error!("{err:?}");
                Err(Error::UnknownError)
            }
        }
    }

    async fn query(
        &self,
        customer_id: CustomerId,
        quotation_id: QuoteId,
        cursor: Option<String>,
        limit: i32,
    ) -> Result<QueryResponse<Vec<Part>, String>> {
        let expression_attrubte_values = [
            (String::from(":customer_id"), AttributeValue::S(customer_id)),
            (
                String::from(":quotation_id"),
                AttributeValue::S(quotation_id),
            ),
        ]
        .into_iter()
        .collect();

        let response = self
            .client
            .query()
            .limit(limit)
            .index_name(TableIndex::LSI1QuoteAndCreationDateTime.to_string())
            .set_exclusive_start_key(DynamodbKeyCodec::decode_from_base64(cursor))
            .key_condition_expression("pk = :customer_id AND begins_with(lsi1_sk, :quotation_id)")
            .set_expression_attribute_values(Some(expression_attrubte_values))
            .table_name(&self.table)
            .send()
            .await;

        match response {
            Ok(output) => {
                let items = output.items().to_vec();
                match from_items::<_, DynamodbPart>(items) {
                    Ok(dynamodb_parts) => {
                        let mut parts = Vec::with_capacity(dynamodb_parts.len());
                        for dynamodb_part in dynamodb_parts {
                            parts.push(dynamodb_part.try_into()?);
                        }
                        Ok(QueryResponse {
                            data: parts,
                            cursor: DynamodbKeyCodec::encode_to_base64(output.last_evaluated_key()),
                        })
                    }
                    Err(err) => {
                        tracing::error!("{err:?}");
                        Err(Error::UnknownError)
                    }
                }
            }
            Err(err) => {
                tracing::error!("{err:?}");
                Err(Error::UnknownError)
            }
        }
    }

    async fn update(&self, updatable_part: UpdatablePart) -> Result<Part> {
        let mut update_expression = String::from("SET updated_at = :updated_at");
        let mut expression_attribute_values: HashMap<String, AttributeValue> = [(
            String::from(":updated_at"),
            AttributeValue::S(Utc::now().to_rfc3339()),
        )]
        .into_iter()
        .collect();

        if let Some(drawing_file) = updatable_part.drawing_file {
            update_expression.push_str(", drawing_file = :drawing_file");
            expression_attribute_values.insert(
                String::from(":drawing_file"),
                AttributeValue::M(to_item(drawing_file).unwrap()),
            );
        }
        if let Some(process) = updatable_part.process {
            update_expression.push_str(", process = :process");
            expression_attribute_values.insert(
                String::from(":process"),
                AttributeValue::S(process.to_string()),
            );
        }
        if let Some(attributes) = updatable_part.attributes {
            update_expression.push_str(", attributes = :attributes");
            expression_attribute_values.insert(
                String::from(":attributes"),
                AttributeValue::M(to_item(attributes).unwrap()),
            );
        }
        if let Some(quantity) = updatable_part.quantity {
            update_expression.push_str(", quantity = :quantity");
            expression_attribute_values.insert(
                String::from(":quantity"),
                AttributeValue::N(quantity.to_string()),
            );
        }

        update_expression.push_str(", selected_part_quote_id = :selected_part_quote_id");
        expression_attribute_values.insert(
            String::from(":selected_part_quote_id"),
            match updatable_part.selected_part_quote_id {
                Some(selected_part_quote_id) => {
                    AttributeValue::S(selected_part_quote_id.to_string())
                }
                None => AttributeValue::Null(true),
            },
        );

        if updatable_part.clear_part_quotes.unwrap_or(false) {
            update_expression.push_str(", part_quotes = :part_quotes");
            expression_attribute_values
                .insert(String::from(":part_quotes"), AttributeValue::Null(true));
        }

        let response = self
            .client
            .update_item()
            .table_name(&self.table)
            .key("pk", AttributeValue::S(updatable_part.customer_id))
            .key("sk", AttributeValue::S(updatable_part.id))
            .update_expression(update_expression)
            .set_expression_attribute_values(Some(expression_attribute_values))
            .return_values(ReturnValue::AllNew)
            .send()
            .await;

        match response {
            Ok(output) => match output.attributes {
                Some(item) => match from_item::<DynamodbPart>(item) {
                    Ok(dynamodb_part) => dynamodb_part.try_into(),
                    Err(err) => {
                        tracing::error!("{err:?}");
                        Err(Error::UnknownError)
                    }
                },
                None => Err(Error::ItemNotFoundError),
            },
            Err(err) => {
                tracing::error!("{err:?}");
                Err(Error::UnknownError)
            }
        }
    }

    async fn batch_create(&self, parts: Vec<Part>) -> Result<()> {
        let items: Vec<WriteRequest> = parts
            .into_iter()
            .map(|part| {
                let dynamodb_part = DynamodbPart::from(part);
                WriteRequest::builder()
                    .put_request(
                        PutRequest::builder()
                            .set_item(Some(
                                to_item(dynamodb_part).expect("error converting to dynamodb item"),
                            ))
                            .build()
                            .unwrap(),
                    )
                    .build()
            })
            .collect();

        let response = self
            .client
            .batch_write_item()
            .request_items(&self.table, items)
            .send()
            .await;

        match response {
            Ok(_) => Ok(()),
            Err(err) => {
                tracing::error!("{err:?}");
                Err(Error::UnknownError)
            }
        }
    }

    async fn batch_delete(&self, data: Vec<BatchDeletePartObject>) -> Result<()> {
        let write_requests = data
            .into_iter()
            .map(|delete_object| {
                WriteRequest::builder()
                    .delete_request(
                        DeleteRequest::builder()
                            .set_key(Some(HashMap::from([
                                (
                                    String::from("pk"),
                                    AttributeValue::S(delete_object.customer_id),
                                ),
                                (String::from("sk"), AttributeValue::S(delete_object.part_id)),
                            ])))
                            .build()
                            .unwrap(),
                    )
                    .build()
            })
            .collect();

        let response = self
            .client
            .batch_write_item()
            .request_items(&self.table, write_requests)
            .send()
            .await;

        match response {
            Ok(_) => Ok(()),
            Err(err) => {
                tracing::error!("{err:?}");
                Err(Error::UnknownError)
            }
        }
    }

    async fn batch_get(
        &self,
        customer_and_part_ids: Vec<(CustomerId, PartId)>,
    ) -> Result<Vec<Part>> {
        let keys_and_attributes = customer_and_part_ids
            .into_iter()
            .fold(
                KeysAndAttributes::builder(),
                |mut keys_and_attributes_builder, (customer_id, part_id)| {
                    keys_and_attributes_builder =
                        keys_and_attributes_builder.keys(HashMap::from([
                            (String::from("pk"), AttributeValue::S(customer_id)),
                            (String::from("sk"), AttributeValue::S(part_id)),
                        ]));

                    keys_and_attributes_builder
                },
            )
            .build()
            .expect("unable to build batch get request keys and attributes");

        let response = self
            .client
            .batch_get_item()
            .request_items(&self.table, keys_and_attributes)
            .send()
            .await;

        match response {
            Ok(output) => {
                let items = output
                    .responses
                    .unwrap()
                    .into_values()
                    .flatten()
                    .collect::<Vec<_>>();

                match from_items::<_, DynamodbPart>(items) {
                    Ok(dynamodb_parts) => {
                        let mut parts = Vec::with_capacity(dynamodb_parts.len());
                        for dynamodb_part in dynamodb_parts {
                            parts.push(dynamodb_part.try_into()?)
                        }
                        Ok(parts)
                    }
                    Err(err) => {
                        tracing::error!("{err:?}");
                        Err(Error::UnknownError)
                    }
                }
            }
            Err(err) => {
                tracing::error!("{err:?}");
                Err(Error::UnknownError)
            }
        }
    }

    fn transaction_create_part_quotes(
        &self,
        customer_id: CustomerId,
        part_id: PartId,
        selected_part_quote_id: PartQuoteId,
        part_quotes: Vec<PartQuote>,
    ) -> TransactWriteItem {
        let part_quote_items = part_quotes
            .into_iter()
            .map(|part_quote| {
                let item = serde_dynamo::aws_sdk_dynamodb_1::to_item(part_quote)
                    .expect("error converting to dynamodb item");
                AttributeValue::M(item)
            })
            .collect();

        TransactWriteItem::builder()
            .update(
                Update::builder()
                    .key("pk", AttributeValue::S(customer_id.clone()))
                    .key("sk", AttributeValue::S(part_id))
                    .table_name(&self.table)
                    .set_expression_attribute_values(Some(
                        [
                            (
                                String::from(":part_quotes"),
                                AttributeValue::L(part_quote_items),
                            ),
                            (
                                String::from(":selected_part_quote_id"),
                                AttributeValue::S(selected_part_quote_id),
                            ),
                        ]
                            .into_iter()
                            .collect::<HashMap<String, AttributeValue>>(),
                    ))
                    .update_expression("SET part_quotes = :part_quotes, selected_part_quote_id = :selected_part_quote_id")
                    .build()
                    .unwrap(),
            )
            .build()
    }
}
