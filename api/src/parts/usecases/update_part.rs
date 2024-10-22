use std::sync::Arc;

use api_boundary::common::error::Error;
use api_boundary::parts::models::Part;
use axum::async_trait;

use api_boundary::parts::requests::UpdatePartRequest;
use api_boundary::quotations::models::QuotationStatus;

use crate::parts::domain::updatable_part::UpdatablePart;
use crate::repositories::parts::PartsRepository;
use crate::repositories::quotations::QuotationsRepository;
use crate::shared::{Result, UseCase};

pub struct UpdatePartUseCase {
    parts_repository: Arc<dyn PartsRepository>,
    quotations_repository: Arc<dyn QuotationsRepository>,
}

impl UpdatePartUseCase {
    pub const fn new(
        parts_repository: Arc<dyn PartsRepository>,
        quotations_repository: Arc<dyn QuotationsRepository>,
    ) -> Self {
        Self {
            parts_repository,
            quotations_repository,
        }
    }
}

#[async_trait]
impl UseCase<UpdatePartRequest, Part> for UpdatePartUseCase {
    async fn execute(&self, request: UpdatePartRequest) -> Result<Part> {
        let quotation = self
            .quotations_repository
            .get_quotation_by_id(request.project_id.clone(), request.quotation_id.clone())
            .await?;

        // Check that the quotation is in an updatable status and change status to created after making an update.
        match quotation.status {
            QuotationStatus::Created => (),
            QuotationStatus::PendingPayment => {
                self.quotations_repository
                    .update_quotation_status(
                        request.project_id.clone(),
                        request.quotation_id.clone(),
                        QuotationStatus::Created,
                    )
                    .await?
            }
            QuotationStatus::Payed => return Err(Error::UpdatePartAfterPayingQuotation),
        }

        let updatable_part = UpdatablePart::from(&request);

        self.parts_repository.update_part(updatable_part).await
    }
}
