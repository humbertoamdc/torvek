use std::sync::Arc;

use api_boundary::parts::errors::PartsError;
use axum::async_trait;

use api_boundary::parts::requests::UpdatePartRequest;
use api_boundary::quotations::models::QuotationStatus;
use api_boundary::quotations::requests::GetQuotationByIdRequest;

use crate::parts::domain::updatable_part::UpdatablePart;
use crate::parts::repositories::parts::PartsRepository;
use crate::quotations::usecases::get_quotation_by_id::GetQuotationByIdUseCase;
use crate::shared::usecase::UseCase;

pub struct UpdatePartUseCase {
    parts_repository: Arc<dyn PartsRepository>,
    get_quotation_by_id_use_case: GetQuotationByIdUseCase,
}

impl UpdatePartUseCase {
    pub const fn new(
        parts_repository: Arc<dyn PartsRepository>,
        get_quotation_by_id_use_case: GetQuotationByIdUseCase,
    ) -> Self {
        Self {
            parts_repository,
            get_quotation_by_id_use_case,
        }
    }
}

#[async_trait]
impl UseCase<UpdatePartRequest, (), PartsError> for UpdatePartUseCase {
    async fn execute(&self, request: UpdatePartRequest) -> Result<(), PartsError> {
        let get_quotation_request = GetQuotationByIdRequest {
            customer_id: request.customer_id.clone(),
            project_id: request.project_id.clone(),
            quotation_id: request.quotation_id.clone(),
        };
        let quotation = self
            .get_quotation_by_id_use_case
            .execute(get_quotation_request)
            .await
            .map_err(|_| PartsError::UnknownError)?; // TODO: Handle error properly.

        if quotation.status == QuotationStatus::Payed {
            return Err(PartsError::UpdatePartAfterPayingQuotation);
        }

        let updatable_part = UpdatablePart::from(&request);

        self.parts_repository.update_part(updatable_part).await
    }
}
