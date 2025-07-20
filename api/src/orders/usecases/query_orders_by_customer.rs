use crate::orders::models::inputs::QueryOrdersForCustomerInput;
use crate::orders::models::responses::{
    QueryOrdersForCustomerResponse, QueryOrdersForCustomerResponseData,
};
use crate::parts::models::part::Part;
use crate::repositories::orders::{OrdersRepository, QueryBy};
use crate::repositories::parts::PartsRepository;
use crate::services::object_storage::ObjectStorage;
use crate::shared::{Result, UseCase};
use async_trait::async_trait;
use std::collections::HashMap;
use std::sync::Arc;
use std::time::Duration;

static PRESIGNED_URLS_GET_DURATION_SECONDS: u64 = 3600;

pub struct QueryOrdersByCustomer<O, P>
where
    O: OrdersRepository,
    P: PartsRepository,
{
    orders_repository: Arc<O>,
    parts_repository: Arc<P>,
    object_storage: Arc<dyn ObjectStorage>,
}

impl<O, P> QueryOrdersByCustomer<O, P>
where
    O: OrdersRepository,
    P: PartsRepository,
{
    pub fn new(
        orders_repository: Arc<O>,
        parts_repository: Arc<P>,
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
impl<O, P> UseCase<QueryOrdersForCustomerInput, QueryOrdersForCustomerResponse>
    for QueryOrdersByCustomer<O, P>
where
    O: OrdersRepository,
    P: PartsRepository,
{
    async fn execute(
        &self,
        input: QueryOrdersForCustomerInput,
    ) -> Result<QueryOrdersForCustomerResponse> {
        let response = self
            .orders_repository
            .query(
                Some(input.identity.id),
                None,
                None,
                None,
                None,
                None,
                None,
                QueryBy::Customer,
                input.cursor,
                input.limit,
            )
            .await?;

        let mut parts_map = HashMap::<String, Part>::new();
        if input.with_part_data && !response.data.is_empty() {
            let order_and_part_ids = response
                .data
                .iter()
                .map(|order| (order.customer_id.clone(), order.part_id.clone()))
                .collect();

            let mut parts = self.parts_repository.batch_get(order_and_part_ids).await?;

            parts_map = parts
                .into_iter()
                .map(|part| (part.id.clone(), part))
                .collect::<HashMap<String, Part>>();
        }

        let data = response
            .data
            .into_iter()
            .map(|order| {
                let part_id = order.part_id.clone();
                let part = parts_map.remove(&part_id);
                QueryOrdersForCustomerResponseData { order, part }
            })
            .collect();

        Ok(QueryOrdersForCustomerResponse {
            data,
            cursor: response.cursor,
        })
    }
}
