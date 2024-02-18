use std::sync::Arc;

use axum::async_trait;

use api_boundary::parts::models::PartPriceOption;
use api_boundary::parts::requests::CreatePartPriceOptionsAndUpdateQuotationStatusRequest;

use crate::parts::domain::errors::PartsError;
use crate::parts::services::part_price_options_creation::PartPriceOptionsCreation;
use crate::shared::usecase::UseCase;

pub struct CreatePartPriceOptionsAndUpdateQuotationStatusUseCase {
    part_price_options_creation_service: Arc<dyn PartPriceOptionsCreation>,
}

impl CreatePartPriceOptionsAndUpdateQuotationStatusUseCase {
    pub fn new(part_price_options_creation_service: Arc<dyn PartPriceOptionsCreation>) -> Self {
        Self {
            part_price_options_creation_service,
        }
    }
}

#[async_trait]
impl UseCase<CreatePartPriceOptionsAndUpdateQuotationStatusRequest, (), PartsError>
    for CreatePartPriceOptionsAndUpdateQuotationStatusUseCase
{
    async fn execute(
        &self,
        request: CreatePartPriceOptionsAndUpdateQuotationStatusRequest,
    ) -> Result<(), PartsError> {
        let part_price_options = request
            .price_data
            .into_iter()
            .map(|price_data| {
                PartPriceOption::new(price_data.part_id, price_data.price, price_data.deadline)
            })
            .collect::<Vec<PartPriceOption>>();

        self.part_price_options_creation_service
            .create_part_price_options(
                request.client_id,
                request.project_id,
                request.quotation_id,
                part_price_options,
            )
            .await
    }
}
