use crate::common::{send, Result};
use api_boundary::orders::requests::AdminCreateOrderRequest;
use gloo_net::http::Request;
use web_sys::RequestCredentials;

#[derive(Copy, Clone)]
pub struct OrdersClient {
    url: &'static str,
}

impl OrdersClient {
    pub const fn new(url: &'static str) -> Self {
        Self { url }
    }

    pub async fn admin_create_order(&self, request: AdminCreateOrderRequest) -> Result<()> {
        let url = format!("{}/admin/orders", self.url);
        let request = Request::post(&url)
            .credentials(RequestCredentials::Include)
            .json(&request)?;

        send(request).await
    }
}
