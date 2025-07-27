use crate::parts::models::dynamodb_requests::UpdatablePart;
use crate::parts::models::inputs::UpdatePartInput;
use crate::parts::models::part::Part;
use crate::quotations::models::quotation::QuoteStatus;
use crate::repositories::parts::PartsRepository;
use crate::repositories::quotes::QuotesRepository;
use crate::shared::error::Error;
use crate::shared::{Result, UseCase};
use async_trait::async_trait;
use std::sync::Arc;

pub struct UpdatePart<Q, P>
where
    Q: QuotesRepository,
    P: PartsRepository,
{
    parts_repository: Arc<P>,
    quotations_repository: Arc<Q>,
}

impl<Q, P> UpdatePart<Q, P>
where
    Q: QuotesRepository,
    P: PartsRepository,
{
    pub const fn new(parts_repository: Arc<P>, quotations_repository: Arc<Q>) -> Self {
        Self {
            parts_repository,
            quotations_repository,
        }
    }
}

#[async_trait]
impl<Q, P> UseCase<UpdatePartInput, Part> for UpdatePart<Q, P>
where
    Q: QuotesRepository,
    P: PartsRepository,
{
    async fn execute(&self, input: UpdatePartInput) -> Result<Part> {
        let quotation = self
            .quotations_repository
            .get(input.identity.id.clone(), input.quotation_id.clone())
            .await?;

        // Check that the quotation is in an updatable status and change status to created after making an update.
        match quotation.status {
            QuoteStatus::Created => (),
            QuoteStatus::PendingReview | QuoteStatus::PendingPayment => {
                let _ = self
                    .quotations_repository
                    .update_status(
                        input.identity.id.clone(),
                        input.project_id.clone(),
                        input.quotation_id.clone(),
                        Some(QuoteStatus::Created),
                    )
                    .await?;
            }
            QuoteStatus::Payed => return Err(Error::UpdatePartAfterPayingQuotation),
        }

        let mut updatable_part = UpdatablePart::from(&input);
        updatable_part.clear_part_quotes = Some(true);

        self.parts_repository.update(updatable_part).await
    }
}
