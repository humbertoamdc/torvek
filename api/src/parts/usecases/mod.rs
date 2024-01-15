use axum::async_trait;

pub mod create_parts;

#[async_trait]
pub trait UseCase<Request, Response, Error> {
    async fn execute(&self, request: Request) -> Result<Response, Error>;
}
