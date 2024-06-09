use std::sync::Arc;

use api_boundary::parts::errors::PartsError;
use axum::async_trait;

use api_boundary::parts::models::PartQuote;
use api_boundary::parts::requests::CreatePartQuotesRequest;

use crate::parts::services::part_quotes_creation::PartQuotesCreation;
use crate::shared::usecase::UseCase;

pub struct CreatePartQuotesUseCase {
    part_quotes_creation_service: Arc<dyn PartQuotesCreation>,
}

impl CreatePartQuotesUseCase {
    pub fn new(part_quotes_creation_service: Arc<dyn PartQuotesCreation>) -> Self {
        Self {
            part_quotes_creation_service,
        }
    }
}

#[async_trait]
impl UseCase<CreatePartQuotesRequest, (), PartsError> for CreatePartQuotesUseCase {
    async fn execute(&self, request: CreatePartQuotesRequest) -> Result<(), PartsError> {
        let part_quotes = request
            .data
            .into_iter()
            .map(|quote_data| {
                PartQuote::new(
                    quote_data.part_id,
                    quote_data.unit_price,
                    quote_data.sub_total,
                    quote_data.deadline,
                )
            })
            .collect::<Vec<PartQuote>>();

        self.part_quotes_creation_service
            .create_part_quotes_and_update_quotation_status(
                request.client_id,
                request.project_id,
                request.quotation_id,
                part_quotes,
            )
            .await
    }
}
