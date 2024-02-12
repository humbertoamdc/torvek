use std::sync::Arc;

use axum::async_trait;

use api_boundary::orders::models::{Order, OrderStatus};
use api_boundary::payments::requests::CompleteCheckoutSessionWebhookRequest;

use crate::payments::domain::errors::PaymentsError;
use crate::payments::services::orders_creation::OrdersCreationService;
use crate::shared::usecase::UseCase;

pub struct CreateOrdersAndConfirmQuotationPaymentUseCase {
    orders_creation_service: Arc<dyn OrdersCreationService>,
}

impl CreateOrdersAndConfirmQuotationPaymentUseCase {
    pub fn new(orders_creation_service: Arc<dyn OrdersCreationService>) -> Self {
        Self {
            orders_creation_service,
        }
    }
}

#[async_trait]
impl UseCase<CompleteCheckoutSessionWebhookRequest, (), PaymentsError>
    for CreateOrdersAndConfirmQuotationPaymentUseCase
{
    async fn execute(
        &self,
        request: CompleteCheckoutSessionWebhookRequest,
    ) -> Result<(), PaymentsError> {
        let orders = request
            .data
            .into_iter()
            .map(|order_data| {
                Order::new(
                    order_data.part_id,
                    order_data.model_file,
                    None,
                    order_data.deadline,
                    OrderStatus::Open,
                )
            })
            .collect();

        self.orders_creation_service
            .create_orders_and_update_quotation_status(
                request.client_id,
                request.project_id,
                request.quotation_id,
                orders,
            )
            .await
    }
}
