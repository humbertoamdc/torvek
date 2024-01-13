pub mod create_quotation;
pub mod query_quotations_for_project;

use axum::async_trait;

#[async_trait]
pub trait UseCase<Request, Response, Error> {
    async fn execute(&self, request: Request) -> Result<Response, Error>;
}
