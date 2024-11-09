use crate::shared::{QueryResponse, Result};
use api_boundary::common::error::Error;
use aws_sdk_dynamodb::operation::delete_item::DeleteItemError;
use aws_sdk_dynamodb::types::{AttributeValue, DeleteRequest, WriteRequest};
use axum::async_trait;
use chrono::Utc;
use serde_dynamo::aws_sdk_dynamodb_1::from_item;
use serde_dynamo::{from_items, to_item};
use std::collections::HashMap;

use crate::quotations::domain::dynamodb_requests::BatchDeleteQuotationObject;
use crate::repositories::quotations::QuotationsRepository;
use crate::utils::dynamodb_key_codec::DynamodbKeyCodec;
use api_boundary::quotations::models::{Quotation, QuotationStatus};

static QUOTATIONS_BY_STATUS_INDEX: &'static str = "QuotationsByStatus";

#[derive(Clone)]
pub struct DynamodbQuotations {
    client: aws_sdk_dynamodb::Client,
    table: String,
}

impl DynamodbQuotations {
    pub fn new(client: aws_sdk_dynamodb::Client, table: String) -> Self {
        Self { client, table }
    }
}

#[async_trait]
impl QuotationsRepository for DynamodbQuotations {
    async fn create_quotation(&self, quotation: Quotation) -> Result<()> {
        let item = to_item(quotation).expect("error converting to dynamodb item");
        let response = self
            .client
            .put_item()
            .set_item(Some(item))
            .table_name(&self.table)
            .send()
            .await;

        match response {
            Ok(_) => Ok(()),
            Err(_) => Err(Error::UnknownError),
        }
    }

    async fn query_quotations_for_project(
        &self,
        project_id: String,
        page_limit: i32,
        cursor: Option<String>,
    ) -> Result<QueryResponse<Vec<Quotation>, String>> {
        let response = self
            .client
            .query()
            .table_name(&self.table)
            .limit(page_limit)
            .set_exclusive_start_key(DynamodbKeyCodec::decode_from_base64(cursor))
            .key_condition_expression("project_id = :value")
            .expression_attribute_values(":value", AttributeValue::S(project_id))
            .scan_index_forward(false)
            .send()
            .await;

        match response {
            Ok(output) => {
                let items = output.items().to_vec();
                match from_items(items) {
                    Ok(quotations) => Ok(QueryResponse {
                        data: quotations,
                        cursor: DynamodbKeyCodec::encode_to_base64(output.last_evaluated_key()),
                    }),
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

    async fn query_quotations_by_status(
        &self,
        status: QuotationStatus,
        page_limit: i32,
        cursor: Option<String>,
    ) -> Result<QueryResponse<Vec<Quotation>, String>> {
        let response = self
            .client
            .query()
            .table_name(&self.table)
            .index_name(QUOTATIONS_BY_STATUS_INDEX)
            .limit(page_limit)
            .set_exclusive_start_key(DynamodbKeyCodec::decode_from_base64(cursor))
            .key_condition_expression("#status = :value")
            .expression_attribute_values(":value", AttributeValue::S(status.to_string()))
            .expression_attribute_names("#status", "status")
            .send()
            .await;

        match response {
            Ok(output) => {
                let items = output.items().to_vec();
                match from_items(items) {
                    Ok(quotations) => Ok(QueryResponse {
                        data: quotations,
                        cursor: DynamodbKeyCodec::encode_to_base64(output.last_evaluated_key()),
                    }),
                    Err(_) => Err(Error::UnknownError),
                }
            }
            Err(_) => Err(Error::UnknownError),
        }
    }

    async fn get_quotation_by_id(
        &self,
        project_id: String,
        quotation_id: String,
    ) -> Result<Quotation> {
        let response = self
            .client
            .get_item()
            .table_name(&self.table)
            .set_key(Some(HashMap::from([
                (String::from("project_id"), AttributeValue::S(project_id)),
                (String::from("id"), AttributeValue::S(quotation_id)),
            ])))
            .send()
            .await;

        match response {
            Ok(output) => match output.item {
                Some(item) => match from_item::<Quotation>(item) {
                    Ok(quotation) => Ok(quotation),
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

    async fn update_quotation_status(
        &self,
        project_id: String,
        quotation_id: String,
        status: QuotationStatus,
    ) -> Result<()> {
        let response = self
            .client
            .update_item()
            .table_name(&self.table)
            .key("project_id", AttributeValue::S(project_id))
            .key("id", AttributeValue::S(quotation_id))
            .update_expression("SET updated_at = :updated_at, #status = :status")
            .set_expression_attribute_names(Some(HashMap::from([(
                String::from("#status"),
                String::from("status"),
            )])))
            .set_expression_attribute_values(Some(HashMap::from([
                (
                    String::from(":updated_at"),
                    AttributeValue::S(Utc::now().to_rfc3339()),
                ),
                (
                    String::from(":status"),
                    AttributeValue::S(status.to_string()),
                ),
            ])))
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

    async fn try_delete_quotation(&self, project_id: String, quotation_id: String) -> Result<()> {
        let response = self
            .client
            .delete_item()
            .table_name(&self.table)
            .key("project_id", AttributeValue::S(project_id))
            .key("id", AttributeValue::S(quotation_id))
            .condition_expression("#status <> :payed")
            .set_expression_attribute_names(Some(HashMap::from([(
                String::from("#status"),
                String::from("status"),
            )])))
            .set_expression_attribute_values(Some(HashMap::from([(
                String::from(":payed"),
                AttributeValue::S(QuotationStatus::Payed.to_string()),
            )])))
            .send()
            .await;

        match response {
            Ok(_) => Ok(()),
            Err(err) => match err.as_service_error() {
                Some(service_error) => match service_error {
                    DeleteItemError::ConditionalCheckFailedException(conditional_check_error) => {
                        tracing::error!("{conditional_check_error:?}");
                        Err(Error::DeletePayedQuotation)
                    }
                    delete_item_error => {
                        tracing::error!("{delete_item_error:?}");
                        Err(Error::UnknownError)
                    }
                },
                None => {
                    tracing::error!("{err:?}");
                    Err(Error::UnknownError)
                }
            },
        }
    }

    async fn batch_delete_parts(&self, data: Vec<BatchDeleteQuotationObject>) -> Result<()> {
        let write_requests = data
            .into_iter()
            .map(|delete_object| {
                WriteRequest::builder()
                    .delete_request(
                        DeleteRequest::builder()
                            .set_key(Some(HashMap::from([
                                (
                                    String::from("project_id"),
                                    AttributeValue::S(delete_object.project_id),
                                ),
                                (
                                    String::from("id"),
                                    AttributeValue::S(delete_object.quotation_id),
                                ),
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
}
