use crate::orders::models::order::{Order, OrderStatus};
use crate::parts::models::part::PartQuote;
use crate::payments::models::inputs::CompleteCheckoutSessionWebhookRequest;
use crate::payments::services::orders_creation::OrdersCreationService;
use crate::repositories::parts::PartsRepository;
use crate::shared::{Result, UseCase};
use crate::utils::workdays::Workdays;
use async_trait::async_trait;
use chrono::Utc;
use std::collections::HashMap;
use std::sync::Arc;

pub struct CreateOrdersAndConfirmQuotationPaymentUseCase {
    orders_creation_service: Arc<dyn OrdersCreationService>,
    parts_repository: Arc<dyn PartsRepository>,
}

impl CreateOrdersAndConfirmQuotationPaymentUseCase {
    pub fn new(
        orders_creation_service: Arc<dyn OrdersCreationService>,
        parts_repository: Arc<dyn PartsRepository>,
    ) -> Self {
        Self {
            orders_creation_service,
            parts_repository,
        }
    }
}

#[async_trait]
impl UseCase<CompleteCheckoutSessionWebhookRequest, ()>
    for CreateOrdersAndConfirmQuotationPaymentUseCase
{
    async fn execute(&self, request: CompleteCheckoutSessionWebhookRequest) -> Result<()> {
        let query_parts_for_quotation_response = self
            .parts_repository
            .query_parts_for_quotation(request.quotation_id.clone(), None, 100)
            .await?;

        let selected_part_quote_for_part = query_parts_for_quotation_response
            .data
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
            .data
            .into_iter()
            .map(|part| {
                let part_quote = selected_part_quote_for_part[&part.id].clone();
                let now = Utc::now().naive_utc().date();
                let deadline = Workdays::add_workdays(now, part_quote.workdays_to_complete);
                Order::new(
                    part.customer_id,
                    part.project_id,
                    part.quotation_id,
                    part.id.clone(),
                    selected_part_quote_for_part[&part.id].id.clone(),
                    deadline,
                    OrderStatus::PendingPricing,
                    request.shipping_recipient_name.clone(),
                    request.shipping_address.clone(),
                )
            })
            .collect();

        self.orders_creation_service
            .create_orders_and_update_quotation_status(
                request.customer_id,
                request.project_id,
                request.quotation_id,
                orders,
            )
            .await
    }
}
