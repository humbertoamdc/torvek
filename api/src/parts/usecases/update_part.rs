use crate::parts::models::dynamodb_requests::UpdatablePart;
use crate::parts::models::inputs::UpdatePartInput;
use crate::parts::models::part::Part;
use crate::quotations::models::quotation::QuotationStatus;
use crate::repositories::parts::PartsRepository;
use crate::repositories::quotations::QuotationsRepository;
use crate::shared::error::Error;
use crate::shared::{Result, UseCase};
use async_trait::async_trait;
use std::sync::Arc;

pub struct UpdatePart {
    parts_repository: Arc<dyn PartsRepository>,
    quotations_repository: Arc<dyn QuotationsRepository>,
}

impl UpdatePart {
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
impl UseCase<UpdatePartInput, Part> for UpdatePart {
    async fn execute(&self, input: UpdatePartInput) -> Result<Part> {
        let quotation = self
            .quotations_repository
            .get(input.project_id.clone(), input.quotation_id.clone())
            .await?;

        // Check that the quotation is in an updatable status and change status to created after making an update.
        match quotation.status {
            QuotationStatus::Created => (),
            QuotationStatus::PendingReview | QuotationStatus::PendingPayment => {
                let _ = self
                    .quotations_repository
                    .update(
                        input.project_id.clone(),
                        input.quotation_id.clone(),
                        Some(QuotationStatus::Created),
                    )
                    .await?;
            }
            QuotationStatus::Payed => return Err(Error::UpdatePartAfterPayingQuotation),
        }

        let updatable_part = UpdatablePart::from(&input);

        self.parts_repository.update(updatable_part).await
    }
}
