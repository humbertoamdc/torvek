use crate::repositories::projects::ProjectsRepository;
use crate::shared::{QueryResponse, Result};
use crate::utils::dynamodb_key_codec::DynamodbKeyCodec;
use api_boundary::common::error::Error;
use api_boundary::projects::models::{Project, ProjectStatus};
use aws_sdk_dynamodb::operation::delete_item::DeleteItemError;
use aws_sdk_dynamodb::types::AttributeValue;
use axum::async_trait;
use serde_dynamo::aws_sdk_dynamodb_1::from_item;
use serde_dynamo::{from_items, to_item};
use std::collections::HashMap;

#[derive(Clone)]
pub struct DynamodbProjects {
    client: aws_sdk_dynamodb::Client,
    table: String,
}

impl DynamodbProjects {
    pub fn new(client: aws_sdk_dynamodb::Client, table: String) -> Self {
        Self { client, table }
    }
}

#[async_trait]
impl ProjectsRepository for DynamodbProjects {
    async fn create_project(&self, project: Project) -> Result<()> {
        let item = to_item(project).expect("error converting to dynamodb item");
        let response = self
            .client
            .put_item()
            .set_item(Some(item))
            .table_name(&self.table)
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

    async fn query_projects_for_client(
        &self,
        customer_id: String,
        page_limit: i32,
        cursor: Option<String>,
    ) -> Result<QueryResponse<Vec<Project>, String>> {
        let response = self
            .client
            .query()
            .limit(page_limit)
            .set_exclusive_start_key(DynamodbKeyCodec::decode_from_base64(cursor))
            .table_name(&self.table)
            .key_condition_expression("customer_id = :customer_id")
            .expression_attribute_values(":customer_id", AttributeValue::S(customer_id))
            .scan_index_forward(false)
            .send()
            .await;

        match response {
            Ok(output) => {
                let items = output.items().to_vec();
                match from_items(items) {
                    Ok(projects) => Ok(QueryResponse {
                        data: projects,
                        cursor: DynamodbKeyCodec::encode_to_base64(output.last_evaluated_key()),
                    }),
                    Err(_) => Err(Error::UnknownError),
                }
            }
            Err(_) => Err(Error::UnknownError),
        }
    }

    async fn get_project_by_id(&self, customer_id: String, project_id: String) -> Result<Project> {
        let response = self
            .client
            .get_item()
            .table_name(&self.table)
            .set_key(Some(HashMap::from([
                (String::from("customer_id"), AttributeValue::S(customer_id)),
                (String::from("id"), AttributeValue::S(project_id)),
            ])))
            .send()
            .await;

        match response {
            Ok(output) => match output.item {
                Some(item) => match from_item::<Project>(item) {
                    Ok(project) => Ok(project),
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

    async fn try_delete_project(&self, customer_id: String, project_id: String) -> Result<()> {
        let response = self
            .client
            .delete_item()
            .table_name(&self.table)
            .key("customer_id", AttributeValue::S(customer_id))
            .key("id", AttributeValue::S(project_id))
            .condition_expression("#status <> :locked")
            .set_expression_attribute_names(Some(HashMap::from([(
                String::from("#status"),
                String::from("status"),
            )])))
            .set_expression_attribute_values(Some(HashMap::from([(
                String::from(":locked"),
                AttributeValue::S(ProjectStatus::Locked.to_string()),
            )])))
            .send()
            .await;

        match response {
            Ok(_) => Ok(()),
            Err(err) => match err.as_service_error() {
                Some(service_error) => match service_error {
                    DeleteItemError::ConditionalCheckFailedException(conditional_check_error) => {
                        tracing::error!("{conditional_check_error:?}");
                        Err(Error::DeleteLockedProject)
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
}
