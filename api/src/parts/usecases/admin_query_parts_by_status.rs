use crate::parts::domain::errors::PartsError;
use crate::parts::repositories::parts::PartsRepository;
use crate::parts::usecases::UseCase;
use api_boundary::parts::requests::AdminQueryPartsByStatusRequest;
use api_boundary::parts::responses::AdminQueryPartsByStatusResponse;
use axum::async_trait;
use std::sync::Arc;

pub struct AdminQueryPartsByStatusUseCase {
    parts_repository: Arc<dyn PartsRepository>,
}

impl AdminQueryPartsByStatusUseCase {
    pub const fn new(parts_repository: Arc<dyn PartsRepository>) -> Self {
        Self { parts_repository }
    }
}

#[async_trait]
impl UseCase<AdminQueryPartsByStatusRequest, AdminQueryPartsByStatusResponse, PartsError>
    for AdminQueryPartsByStatusUseCase
{
    async fn execute(
        &self,
        request: AdminQueryPartsByStatusRequest,
    ) -> Result<AdminQueryPartsByStatusResponse, PartsError> {
        let parts = self
            .parts_repository
            .query_orders_by_status(request.status)
            .await?;

        Ok(AdminQueryPartsByStatusResponse::new(parts))
    }
}
