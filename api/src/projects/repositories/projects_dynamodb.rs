use crate::projects::domain::errors::ProjectsError;
use crate::projects::repositories::projects::ProjectsRepository;
use api_boundary::projects::models::Project;
use aws_sdk_dynamodb::types::AttributeValue;
use axum::async_trait;
use serde_dynamo::{from_items, to_item};

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
    async fn create_project(&self, project: Project) -> Result<(), ProjectsError> {
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
            Err(_) => Err(ProjectsError::CreateProjectError),
        }
    }

    async fn query_projects_for_client(
        &self,
        client_id: String,
    ) -> Result<Vec<Project>, ProjectsError> {
        // TODO: Get ordered by date.
        let response = self
            .client
            .query()
            .key_condition_expression("client_id = :client_id")
            .expression_attribute_values(":client_id", AttributeValue::S(client_id))
            .table_name(&self.table)
            .send()
            .await;

        match response {
            Ok(output) => {
                let items = output.items().to_vec();
                match from_items(items) {
                    Ok(projects) => Ok(projects),
                    Err(_) => Err(ProjectsError::UnknownError),
                }
            }
            Err(_) => Err(ProjectsError::QueryProjectsError),
        }
    }
}
