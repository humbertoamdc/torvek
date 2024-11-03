use crate::repositories::projects::ProjectsRepository;
use crate::shared::Result;
use api_boundary::common::error::Error;
use api_boundary::projects::models::Project;
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

    async fn query_projects_for_client(&self, customer_id: String) -> Result<Vec<Project>> {
        let response = self
            .client
            .query()
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
                    Ok(projects) => Ok(projects),
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
}
