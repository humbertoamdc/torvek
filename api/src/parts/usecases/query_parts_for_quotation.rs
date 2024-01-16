use crate::parts::domain::errors::PartsError;
use crate::parts::repositories::parts::PartsRepository;
use crate::parts::usecases::UseCase;
use api_boundary::parts::requests::QueryPartsForQuotationRequest;
use api_boundary::parts::responses::QueryPartsForQuotationResponse;
use axum::async_trait;
use std::sync::Arc;

pub struct QueryPartsForQuotationUseCase {
    parts_repository: Arc<dyn PartsRepository>,
}

impl QueryPartsForQuotationUseCase {
    pub fn new(parts_repository: Arc<dyn PartsRepository>) -> Self {
        Self { parts_repository }
    }
}

#[async_trait]
impl UseCase<QueryPartsForQuotationRequest, QueryPartsForQuotationResponse, PartsError>
    for QueryPartsForQuotationUseCase
{
    async fn execute(
        &self,
        request: QueryPartsForQuotationRequest,
    ) -> Result<QueryPartsForQuotationResponse, PartsError> {
        let parts = self
            .parts_repository
            .query_parts_for_quotation(request.client_id, request.project_id, request.quotation_id)
            .await?;

        Ok(QueryPartsForQuotationResponse::new(parts))
    }
}
