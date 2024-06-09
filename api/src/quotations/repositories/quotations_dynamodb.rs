use api_boundary::quotations::errors::QuotationsError;
use aws_sdk_dynamodb::types::AttributeValue;
use axum::async_trait;
use serde_dynamo::aws_sdk_dynamodb_1::from_item;
use serde_dynamo::{from_items, to_item};
use std::collections::HashMap;

use api_boundary::quotations::models::{Quotation, QuotationStatus};

use crate::quotations::domain::models::DynamodbQuotationItem;
use crate::quotations::repositories::quotations::QuotationsRepository;

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
    async fn create_quotation(&self, quotation: Quotation) -> Result<(), QuotationsError> {
        let dynamodb_quotation = DynamodbQuotationItem::from(quotation);
        let item = to_item(dynamodb_quotation).expect("error converting to dynamodb item");
        let response = self
            .client
            .put_item()
            .set_item(Some(item))
            .table_name(&self.table)
            .send()
            .await;

        match response {
            Ok(_) => Ok(()),
            Err(_) => Err(QuotationsError::UnknownError),
        }
    }

    async fn query_quotations_for_project(
        &self,
        client_id: String,
        project_id: String,
    ) -> Result<Vec<Quotation>, QuotationsError> {
        let client_id_and_project_id = format!("{client_id}#{project_id}");

        // TODO: Get ordered by date.
        let response = self
            .client
            .query()
            .table_name(&self.table)
            .key_condition_expression("#client_id_project_id = :value")
            .expression_attribute_values(":value", AttributeValue::S(client_id_and_project_id))
            .expression_attribute_names("#client_id_project_id", "client_id#project_id")
            .send()
            .await;

        match response {
            Ok(output) => {
                let items = output.items().to_vec();
                match from_items(items) {
                    Ok(dynamodb_quotations) => {
                        let quotations = dynamodb_quotations
                            .into_iter()
                            .map(|q: DynamodbQuotationItem| q.into())
                            .collect();

                        Ok(quotations)
                    }
                    Err(err) => {
                        log::error!("{err:?}");
                        Err(QuotationsError::UnknownError)
                    }
                }
            }
            Err(err) => {
                log::error!("{err:?}");
                Err(QuotationsError::UnknownError)
            }
        }
    }

    async fn query_quotations_by_status(
        &self,
        status: QuotationStatus,
    ) -> Result<Vec<Quotation>, QuotationsError> {
        let response = self
            .client
            .query()
            .table_name(&self.table)
            .index_name(QUOTATIONS_BY_STATUS_INDEX)
            .key_condition_expression("#status = :value")
            .expression_attribute_values(":value", AttributeValue::S(status.to_string()))
            .expression_attribute_names("#status", "status")
            .send()
            .await;

        match response {
            Ok(output) => {
                let items = output.items().to_vec();
                match from_items(items) {
                    Ok(dynamodb_quotations) => {
                        let quotations = dynamodb_quotations
                            .into_iter()
                            .map(|q: DynamodbQuotationItem| q.into())
                            .collect();

                        Ok(quotations)
                    }
                    Err(_) => Err(QuotationsError::UnknownError),
                }
            }
            Err(_) => Err(QuotationsError::UnknownError),
        }
    }

    async fn get_quotation_by_id(
        &self,
        client_id: String,
        project_id: String,
        quotation_id: String,
    ) -> Result<Quotation, QuotationsError> {
        let client_id_and_project_id = format!("{client_id}#{project_id}");

        let response = self
            .client
            .get_item()
            .table_name(&self.table)
            .set_key(Some(HashMap::from([
                (
                    String::from("client_id#project_id"),
                    AttributeValue::S(client_id_and_project_id),
                ),
                (String::from("id"), AttributeValue::S(quotation_id)),
            ])))
            .send()
            .await;

        match response {
            Ok(output) => match output.item {
                Some(item) => match from_item::<DynamodbQuotationItem>(item) {
                    Ok(dynamodb_quotation) => Ok(dynamodb_quotation.into()),
                    Err(err) => {
                        log::error!("{err:?}");
                        Err(QuotationsError::UnknownError)
                    }
                },
                None => Err(QuotationsError::GetQuotationItemNotFoundError),
            },
            Err(err) => {
                log::error!("{err:?}");
                Err(QuotationsError::UnknownError)
            }
        }
    }
}
