use crate::projects::domain::errors::ProjectError;
use crate::projects::repositories::projects::ProjectsRepository;
use api_boundary::projects::models::Project;
use axum::async_trait;
use serde_dynamo::to_item;

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
    async fn create_project(&self, project: Project) -> Result<(), ProjectError> {
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
            Err(_) => Err(ProjectError::CreateProjectError),
        }
    }
}
