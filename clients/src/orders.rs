use gloo_net::http::Request;
use web_sys::RequestCredentials;

use api_boundary::orders::requests::AdminUpdateOrderPayoutRequest;
use api_boundary::orders::responses::QueryOpenOrdersResponse;

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

    pub async fn query_open_orders(&self) -> Result<QueryOpenOrdersResponse> {
        let url = format!("{}/admin/orders/open", self.url);
        let request = Request::get(&url)
            .credentials(RequestCredentials::Include)
            .build()
            .unwrap();

        send(request).await
    }
}
