use crate::projects::models::project::Project;
use crate::shared::{QueryResponse, Result};
use async_trait::async_trait;

#[async_trait]
pub trait ProjectsRepository: Send + Sync + 'static {
    async fn create(&self, project: Project) -> Result<()>;
    /// Delete project ONLY if it is not in `LOCKED` status.
    async fn delete(&self, customer_id: String, project_id: String) -> Result<()>;
    async fn get(&self, customer_id: String, project_id: String) -> Result<Project>;
    async fn query(
        &self,
        customer_id: String,
        cursor: Option<String>,
        limit: i32,
    ) -> Result<QueryResponse<Vec<Project>, String>>;
}
