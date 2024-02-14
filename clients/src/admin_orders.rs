use gloo_net::http::Request;
use web_sys::RequestCredentials;

use api_boundary::orders::models::OrderStatus;
use api_boundary::orders::requests::AdminCreateOrdersRequest;
use api_boundary::orders::responses::QueryOrdersByStatusResponse;

use crate::common::{send, Result};

#[derive(Copy, Clone)]
pub struct AdminOrdersClient {
    url: &'static str,
}

impl AdminOrdersClient {
    pub const fn new(url: &'static str) -> Self {
        Self { url }
    }

    // TODO: Delete
    pub async fn create_order(&self, request: AdminCreateOrdersRequest) -> Result<()> {
        let url = format!("{}/admin/orders", self.url);
        let request = Request::post(&url)
            .credentials(RequestCredentials::Include)
            .json(&request)?;

        send(request).await
    }

    pub async fn query_orders_by_status(
        &self,
        status: OrderStatus,
    ) -> Result<QueryOrdersByStatusResponse> {
        let url = format!("{}/orders", self.url);
        let request = Request::get(&url)
            .query([("status", status.to_string())])
            .credentials(RequestCredentials::Include)
            .build()?;

        send(request).await
    }
}
