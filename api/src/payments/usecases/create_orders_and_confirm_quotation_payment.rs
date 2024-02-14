use std::sync::Arc;

use axum::async_trait;
use chrono::Utc;

use api_boundary::orders::models::{Order, OrderStatus};
use api_boundary::parts::requests::QueryPartsForQuotationRequest;
use api_boundary::payments::requests::CompleteCheckoutSessionWebhookRequest;

use crate::parts::usecases::query_parts_for_quotation::QueryPartsForQuotationUseCase;
use crate::payments::domain::errors::PaymentsError;
use crate::payments::services::orders_creation::OrdersCreationService;
use crate::shared::usecase::UseCase;

pub struct CreateOrdersAndConfirmQuotationPaymentUseCase {
    orders_creation_service: Arc<dyn OrdersCreationService>,
    query_parts_for_quotation_usecase: QueryPartsForQuotationUseCase,
}

impl CreateOrdersAndConfirmQuotationPaymentUseCase {
    pub fn new(
        orders_creation_service: Arc<dyn OrdersCreationService>,
        query_parts_for_quotation_usecase: QueryPartsForQuotationUseCase,
    ) -> Self {
        Self {
            orders_creation_service,
            query_parts_for_quotation_usecase,
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
        let query_parts_for_quotation_request = QueryPartsForQuotationRequest::new(
            request.client_id.clone(),
            request.project_id.clone(),
            request.quotation_id.clone(),
        );
        let query_parts_for_quotation_response = self
            .query_parts_for_quotation_usecase
            .execute(query_parts_for_quotation_request)
            .await
            .map_err(
                |_| PaymentsError::QueryPartsError, // TODO: Change to use payment error
            )?;

        let orders = query_parts_for_quotation_response
            .parts
            .into_iter()
            .map(|part| {
                Order::new(
                    part.id,
                    part.model_file,
                    part.drawing_file,
                    part.quantity,
                    None,
                    Utc::now().naive_utc().date(), // TODO: This will come from quote price, which will be attached to part by id.
                    OrderStatus::PendingPricing,
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
