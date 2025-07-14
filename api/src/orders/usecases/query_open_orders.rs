use crate::orders::models::responses::QueryOpenOrdersResponse;
use crate::repositories::orders::{OrdersRepository, QueryBy};
use crate::shared::{Result, UseCase};
use async_trait::async_trait;
use std::sync::Arc;

pub struct QueryOpenOrders<O>
where
    O: OrdersRepository,
{
    orders_repository: Arc<O>,
}

impl<O> QueryOpenOrders<O>
where
    O: OrdersRepository,
{
    pub fn new(orders_repository: Arc<O>) -> Self {
        Self { orders_repository }
    }
}

#[async_trait]
impl<O> UseCase<(), QueryOpenOrdersResponse> for QueryOpenOrders<O>
where
    O: OrdersRepository,
{
    async fn execute(&self, _: ()) -> Result<QueryOpenOrdersResponse> {
        let response = self
            .orders_repository
            .query(
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                QueryBy::IsOpen,
                None,
                100,
            )
            .await?;

        Ok(QueryOpenOrdersResponse {
            orders: response.data,
            cursor: response.cursor,
        })
    }
}
