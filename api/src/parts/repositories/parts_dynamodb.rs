use std::collections::HashMap;

use api_boundary::common::error::Error;
use aws_sdk_dynamodb::types::{AttributeValue, PutRequest, ReturnValue, WriteRequest};
use axum::async_trait;
use chrono::Utc;
use serde_dynamo::aws_sdk_dynamodb_1::from_item;
use serde_dynamo::{from_items, to_item};

use api_boundary::parts::models::Part;

use crate::parts::domain::updatable_part::UpdatablePart;
use crate::parts::repositories::parts::PartsRepository;

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
    async fn get_part(&self, quotation_id: String, part_id: String) -> Result<Part, Error> {
        let response = self
            .client
            .get_item()
            .table_name(&self.table)
            .set_key(Some(HashMap::from([
                (
                    String::from("quotation_id"),
                    AttributeValue::S(quotation_id),
                ),
                (String::from("id"), AttributeValue::S(part_id)),
            ])))
            .send()
            .await;

        match response {
            Ok(output) => match output.item {
                Some(item) => match from_item::<Part>(item) {
                    Ok(part) => Ok(part),
                    Err(err) => {
                        log::error!("{err:?}");
                        Err(Error::UnknownError)
                    }
                },
                None => Err(Error::PartItemNotFound),
            },
            Err(err) => {
                log::error!("{err:?}");
                Err(Error::UnknownError)
            }
        }
    }

    async fn create_parts(&self, parts: Vec<Part>) -> Result<(), Error> {
        let items: Vec<WriteRequest> = parts
            .into_iter()
            .map(|part| {
                WriteRequest::builder()
                    .put_request(
                        PutRequest::builder()
                            .set_item(Some(
                                to_item(part).expect("error converting to dynamodb item"),
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
                log::error!("{err:?}");
                Err(Error::UnknownError)
            }
        }
    }

    async fn query_parts_for_quotation(&self, quotation_id: String) -> Result<Vec<Part>, Error> {
        let response = self
            .client
            .query()
            .key_condition_expression("quotation_id = :value")
            .expression_attribute_values(":value", AttributeValue::S(quotation_id))
            .table_name(&self.table)
            .send()
            .await;

        match response {
            Ok(output) => {
                let items = output.items().to_vec();
                match from_items(items) {
                    Ok(parts) => Ok(parts),
                    Err(err) => {
                        log::error!("{err:?}");
                        Err(Error::UnknownError)
                    }
                }
            }
            Err(err) => {
                log::error!("{err:?}");
                Err(Error::UnknownError)
            }
        }
    }

    async fn update_part(&self, updatable_part: UpdatablePart) -> Result<Part, Error> {
        let mut update_expression = String::from("SET ");
        let mut expression_attribute_values = HashMap::new();

        update_expression.push_str("updated_at = :updated_at, ");
        expression_attribute_values.insert(
            ":updated_at".to_string(),
            AttributeValue::S(Utc::now().to_rfc3339()),
        );

        if let Some(drawing_file) = updatable_part.drawing_file {
            update_expression.push_str("drawing_file = :drawing_file, ");
            expression_attribute_values.insert(
                ":drawing_file".to_string(),
                AttributeValue::M(to_item(drawing_file).unwrap()),
            );
        }
        if let Some(process) = updatable_part.process {
            update_expression.push_str("process = :process, ");
            expression_attribute_values.insert(":process".to_string(), AttributeValue::S(process));
        }
        if let Some(material) = updatable_part.material {
            update_expression.push_str("material = :material, ");
            expression_attribute_values
                .insert(":material".to_string(), AttributeValue::S(material));
        }
        if let Some(tolerance) = updatable_part.tolerance {
            update_expression.push_str("tolerance = :tolerance, ");
            expression_attribute_values
                .insert(":tolerance".to_string(), AttributeValue::S(tolerance));
        }
        if let Some(quantity) = updatable_part.quantity {
            update_expression.push_str("quantity = :quantity, ");
            expression_attribute_values.insert(
                ":quantity".to_string(),
                AttributeValue::N(quantity.to_string()),
            );
        }
        // TODO: We actually want to always update the `selected_part_quote_id`. We will also update
        //       the part quotes to be nil. This is a bigger change because we have to also update the
        //       quotation status.
        if let Some(selected_part_quote_id) = updatable_part.selected_part_quote_id {
            update_expression.push_str("selected_part_quote_id = :selected_part_quote_id, ");
            expression_attribute_values.insert(
                ":selected_part_quote_id".to_string(),
                AttributeValue::S(selected_part_quote_id.to_string()),
            );
        }

        // Remove trailing comma and space
        if !update_expression.is_empty() {
            update_expression.pop();
            update_expression.pop();
        }

        let response = self
            .client
            .update_item()
            .table_name(&self.table)
            .key(
                "quotation_id",
                AttributeValue::S(updatable_part.quotation_id),
            )
            .key("id", AttributeValue::S(updatable_part.id))
            .update_expression(update_expression)
            .set_expression_attribute_values(Some(expression_attribute_values))
            .return_values(ReturnValue::AllNew)
            .send()
            .await;

        match response {
            Ok(output) => match output.attributes {
                Some(item) => match from_item::<Part>(item) {
                    Ok(part) => Ok(part),
                    Err(err) => {
                        log::error!("{err:?}");
                        Err(Error::UnknownError)
                    }
                },
                None => Err(Error::PartItemNotFound),
            },
            Err(err) => {
                log::error!("{err:?}");
                Err(Error::UnknownError)
            }
        }
    }
}
