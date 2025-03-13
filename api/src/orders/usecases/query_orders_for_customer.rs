use crate::repositories::orders::OrdersRepository;
use crate::repositories::parts::PartsRepository;
use crate::services::object_storage::ObjectStorage;
use crate::shared::{Result, UseCase};
use api_boundary::orders::requests::QueryOrdersForCustomerRequest;
use api_boundary::orders::responses::{
    QueryOrdersForCustomerResponse, QueryOrdersForCustomerResponseData,
};
use api_boundary::parts::models::Part;
use async_trait::async_trait;
use std::collections::HashMap;
use std::sync::Arc;
use std::time::Duration;

static PRESIGNED_URLS_GET_DURATION_SECONDS: u64 = 3600;

pub struct QueryOrdersForCustomer {
    orders_repository: Arc<dyn OrdersRepository>,
    parts_repository: Arc<dyn PartsRepository>,
    object_storage: Arc<dyn ObjectStorage>,
}

impl QueryOrdersForCustomer {
    pub fn new(
        orders_repository: Arc<dyn OrdersRepository>,
        parts_repository: Arc<dyn PartsRepository>,
        object_storage: Arc<dyn ObjectStorage>,
    ) -> Self {
        Self {
            orders_repository,
            parts_repository,
            object_storage,
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
            let order_and_part_ids = orders
                .iter()
                .map(|order| (order.quotation_id.clone(), order.part_id.clone()))
                .collect();

            let mut parts = self
                .parts_repository
                .get_parts_batch(order_and_part_ids)
                .await?;

            for part in parts.iter_mut() {
                part.render_file.presigned_url = Some(
                    self.object_storage
                        .get_object_presigned_url(
                            &part.render_file.url,
                            Duration::from_secs(PRESIGNED_URLS_GET_DURATION_SECONDS),
                        )
                        .await?,
                );
            }

            parts_map = parts
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
