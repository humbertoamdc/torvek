pub mod create_project;
pub mod query_projects_for_client;

use axum::async_trait;

#[async_trait]
pub trait UseCase<Request, Response, Error> {
    async fn execute(&self, request: Request) -> Result<Response, Error>;
}
