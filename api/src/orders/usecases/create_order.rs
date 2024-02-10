use crate::orders::domain::errors::OrdersError;
use crate::orders::repositories::orders::OrdersRepository;
use crate::quotations::usecases::update_quotation_status::UpdateQuotationStatusUseCase;
use crate::shared::usecase::UseCase;
use api_boundary::orders::models::{Order, OrderStatus};
use api_boundary::orders::requests::AdminCreateOrdersRequest;
use api_boundary::quotations::models::QuotationStatus;
use api_boundary::quotations::requests::UpdateQuotationStatusRequest;
use axum::async_trait;
use std::sync::Arc;

pub struct AdminCreateOrderUseCase {
    orders_repository: Arc<dyn OrdersRepository>,
    update_quotation_status_usecase: UpdateQuotationStatusUseCase,
}

impl AdminCreateOrderUseCase {
    pub fn new(
        orders_repository: Arc<dyn OrdersRepository>,
        update_quotation_status_usecase: UpdateQuotationStatusUseCase,
    ) -> Self {
        Self {
            orders_repository,
            update_quotation_status_usecase,
        }
    }
}

#[async_trait]
impl UseCase<AdminCreateOrdersRequest, (), OrdersError> for AdminCreateOrderUseCase {
    async fn execute(&self, request: AdminCreateOrdersRequest) -> Result<(), OrdersError> {
        // TODO: Add validation for current quotation status.

        let orders = request
            .data
            .into_iter()
            .map(|order| {
                Order::new(
                    order.part_id,
                    order.model_file,
                    order.payment,
                    order.deadline,
                    OrderStatus::Open,
                )
            })
            .collect();

        self.orders_repository.create_orders(orders).await?;

        let update_quotation_status_request = UpdateQuotationStatusRequest {
            client_id: request.client_id,
            project_id: request.project_id,
            quotation_id: request.quotation_id,
            status: QuotationStatus::OrdersCreated,
        };
        // TODO: Handle error
        let _ = self
            .update_quotation_status_usecase
            .execute(update_quotation_status_request)
            .await;

        Ok(())
    }
}
