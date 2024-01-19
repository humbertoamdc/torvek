use crate::parts::domain::dynamodb_part_item::DynamodbPartItem;
use crate::parts::domain::errors::PartsError;
use crate::parts::domain::updatable_part::UpdatablePart;
use crate::parts::repositories::parts::PartsRepository;
use api_boundary::parts::models::{Part, PartStatus};
use aws_sdk_dynamodb::types::{AttributeValue, PutRequest, WriteRequest};
use axum::async_trait;
use chrono::Utc;
use serde_dynamo::{from_items, to_item};
use std::collections::HashMap;

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
    async fn create_parts(&self, parts: Vec<Part>) -> Result<(), PartsError> {
        let items: Vec<WriteRequest> = parts
            .into_iter()
            .map(|part| {
                let part = DynamodbPartItem::from(part);
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
            Err(_) => Err(PartsError::PartsBatchCreateError),
        }
    }

    async fn query_parts_for_quotation(
        &self,
        client_id: String,
        project_id: String,
        quotation_id: String,
    ) -> Result<Vec<Part>, PartsError> {
        let client_project_and_quotation_ids = format!("{client_id}#{project_id}#{quotation_id}");

        // TODO: Get ordered by date.
        let response = self
            .client
            .query()
            .key_condition_expression("#client_project_and_quotation_ids = :value")
            .expression_attribute_values(
                ":value",
                AttributeValue::S(client_project_and_quotation_ids),
            )
            .expression_attribute_names(
                "#client_project_and_quotation_ids",
                "client_id#project_id#quotation_id",
            )
            .table_name(&self.table)
            .send()
            .await;

        match response {
            Ok(output) => {
                let items = output.items().to_vec();
                match from_items(items) {
                    Ok(dynamodb_parts) => {
                        let parts = dynamodb_parts
                            .into_iter()
                            .map(|p: DynamodbPartItem| p.into())
                            .collect();

                        Ok(parts)
                    }
                    Err(_) => Err(PartsError::UnknownError),
                }
            }
            Err(_) => Err(PartsError::QueryPartsError),
        }
    }

    async fn query_orders_by_status(&self, status: PartStatus) -> Result<Vec<Part>, PartsError> {
        let response = self
            .client
            .query()
            .table_name(&self.table)
            .index_name("PartsByStatus")
            .key_condition_expression("#status = :value")
            .expression_attribute_values(":value", AttributeValue::S(status.to_string()))
            .expression_attribute_names("#status", "status")
            .send()
            .await;

        match response {
            Ok(output) => {
                let items = output.items().to_vec();
                match from_items(items) {
                    Ok(dynamodb_parts) => {
                        let parts = dynamodb_parts
                            .into_iter()
                            .map(|p: DynamodbPartItem| p.into())
                            .collect();

                        Ok(parts)
                    }
                    Err(_) => Err(PartsError::UnknownError),
                }
            }
            Err(_) => Err(PartsError::QueryPartsError),
        }
    }

    async fn update_part(
        &self,
        updatable_part: UpdatablePart,
        status: PartStatus,
    ) -> Result<(), PartsError> {
        let mut update_expression = String::from("SET ");
        let mut expression_attribute_values = HashMap::new();
        let mut expression_attribute_names = HashMap::new();

        update_expression.push_str("updated_at = :updated_at, ");
        expression_attribute_values.insert(
            ":updated_at".to_string(),
            AttributeValue::S(Utc::now().to_string()),
        );

        update_expression.push_str("#status = :status, ");
        expression_attribute_values
            .insert(":status".to_string(), AttributeValue::S(status.to_string()));
        expression_attribute_names.insert("#status".to_string(), "status".to_string());

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
        if let Some(unit_price) = updatable_part.unit_price {
            update_expression.push_str("unit_price = :unit_price, ");
            expression_attribute_values.insert(
                ":unit_price".to_string(),
                AttributeValue::N(unit_price.to_string()),
            );
        }
        if let Some(sub_total) = updatable_part.sub_total {
            update_expression.push_str("sub_total = :sub_total, ");
            expression_attribute_values.insert(
                ":sub_total".to_string(),
                AttributeValue::N(sub_total.to_string()),
            );
        }

        // Remove trailing comma and space
        if !update_expression.is_empty() {
            update_expression.pop();
            update_expression.pop();
        }

        let client_project_and_quotation_ids = format!(
            "{}#{}#{}",
            updatable_part.client_id, updatable_part.project_id, updatable_part.quotation_id
        );
        let response = self
            .client
            .update_item()
            .table_name(&self.table)
            .key(
                "client_id#project_id#quotation_id",
                AttributeValue::S(client_project_and_quotation_ids),
            )
            .key("id", AttributeValue::S(updatable_part.id))
            .update_expression(update_expression)
            .set_expression_attribute_values(Some(expression_attribute_values))
            .set_expression_attribute_names(Some(expression_attribute_names))
            .send()
            .await;

        match response {
            Ok(_) => Ok(()),
            Err(_) => Err(PartsError::UpdatePartError),
        }
    }
}
