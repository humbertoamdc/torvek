use std::collections::HashMap;
use std::sync::Arc;

use api_boundary::common::error::Error;
use axum::async_trait;
use chrono::Utc;

use api_boundary::orders::models::{Order, OrderStatus};
use api_boundary::parts::models::PartQuote;
use api_boundary::parts::requests::QueryPartsForQuotationRequest;

use crate::parts::usecases::query_parts_for_quotation::QueryPartsForQuotationUseCase;
use crate::payments::domain::requests::CompleteCheckoutSessionWebhookRequest;
use crate::payments::services::orders_creation::OrdersCreationService;
use crate::shared::{Result, UseCase};

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
impl UseCase<CompleteCheckoutSessionWebhookRequest, ()>
    for CreateOrdersAndConfirmQuotationPaymentUseCase
{
    async fn execute(&self, request: CompleteCheckoutSessionWebhookRequest) -> Result<()> {
        let query_parts_for_quotation_request = QueryPartsForQuotationRequest {
            quotation_id: request.quotation_id.clone(),
            with_quotation_subtotal: false,
        };
        let query_parts_for_quotation_response = self
            .query_parts_for_quotation_usecase
            .execute(query_parts_for_quotation_request)
            .await
            .map_err(|_| Error::UnknownError)?;

        let selected_part_quote_for_part = query_parts_for_quotation_response
            .parts
            .iter()
            .map(|part| {
                (
                    part.id.clone(),
                    // TODO: Safely unwrap.
                    part.part_quotes
                        .clone()
                        .expect("expecting part quotes")
                        .into_iter()
                        .find(|part_quote| {
                            part_quote.id == part.selected_part_quote_id.clone().unwrap()
                        })
                        .unwrap(),
                )
            })
            .collect::<HashMap<String, PartQuote>>();

        let orders = query_parts_for_quotation_response
            .parts
            .into_iter()
            .map(|part| {
                Order::new(
                    part.customer_id,
                    part.project_id,
                    part.quotation_id,
                    part.id.clone(),
                    selected_part_quote_for_part[&part.id].id.clone(),
                    Utc::now().naive_utc().date(), // TODO: This will come from quote price, which will be attached to part by id.
                    OrderStatus::PendingPricing,
                    request.shipping_recipient_name.clone(),
                    request.shipping_address.clone(),
                )
            })
            .collect();

        self.orders_creation_service
            .create_orders_and_update_quotation_status(
                request.project_id,
                request.quotation_id,
                orders,
            )
            .await
    }
}
