use gloo_net::http::Request;
use web_sys::RequestCredentials;

use api_boundary::orders::models::OrderStatus;
use api_boundary::orders::requests::AdminUpdateOrderPayoutRequest;
use api_boundary::orders::responses::QueryOrdersByStatusResponse;

use crate::common::{send, Result};

#[derive(Copy, Clone)]
pub struct OrdersClient {
    url: &'static str,
}

impl OrdersClient {
    pub const fn new(url: &'static str) -> Self {
        Self { url }
    }

    pub async fn admin_update_order_payout(
        &self,
        request: AdminUpdateOrderPayoutRequest,
    ) -> Result<()> {
        let url = format!("{}/admin/orders/payout", self.url);
        let request = Request::patch(&url)
            .credentials(RequestCredentials::Include)
            .json(&request)
            .unwrap();

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
            .build()
            .unwrap();

        send(request).await
    }
}
