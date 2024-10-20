use crate::orders::repositories::orders::OrdersRepository;
use crate::parts::usecases::query_parts_for_quotation::QueryPartsForQuotationUseCase;
use crate::shared::{Result, UseCase};
use api_boundary::orders::requests::QueryOrdersForCustomerRequest;
use api_boundary::orders::responses::{
    QueryOrdersForCustomerResponse, QueryOrdersForCustomerResponseData,
};
use api_boundary::parts::models::Part;
use api_boundary::parts::requests::QueryPartsForQuotationRequest;
use axum::async_trait;
use std::collections::HashMap;
use std::sync::Arc;

pub struct QueryOrdersForCustomer {
    orders_repository: Arc<dyn OrdersRepository>,
    query_parts_for_quotation_usecase: QueryPartsForQuotationUseCase,
}

impl QueryOrdersForCustomer {
    pub fn new(
        orders_repository: Arc<dyn OrdersRepository>,
        query_parts_for_quotation_usecase: QueryPartsForQuotationUseCase,
    ) -> Self {
        Self {
            orders_repository,
            query_parts_for_quotation_usecase,
        }
    }
}

#[async_trait]
impl UseCase<QueryOrdersForCustomerRequest, QueryOrdersForCustomerResponse>
    for QueryOrdersForCustomer
{
    async fn execute(
        &self,
        request: QueryOrdersForCustomerRequest,
    ) -> Result<QueryOrdersForCustomerResponse> {
        let orders = self
            .orders_repository
            .query_orders_for_customer(request.customer_id)
            .await?;

        let mut parts_map = HashMap::<String, Part>::new();
        if request.with_part_data && !orders.is_empty() {
            let quotation_id = orders.first().unwrap().quotation_id.clone();
            let query_parts_for_quotation_request = QueryPartsForQuotationRequest {
                quotation_id,
                with_quotation_subtotal: false,
            };

            parts_map = self
                .query_parts_for_quotation_usecase
                .execute(query_parts_for_quotation_request)
                .await?
                .parts
                .into_iter()
                .map(|part| (part.id.clone(), part))
                .collect::<HashMap<String, Part>>();
        }

        let data = orders
            .into_iter()
            .map(|order| {
                let part_id = order.part_id.clone();
                let part = parts_map.remove(&part_id).unwrap();
                QueryOrdersForCustomerResponseData {
                    order,
                    part: Some(part),
                }
            })
            .collect();

        Ok(QueryOrdersForCustomerResponse { data })
    }
}
