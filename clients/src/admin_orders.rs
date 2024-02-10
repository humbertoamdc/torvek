use crate::common::{send, Result};
use api_boundary::orders::requests::AdminCreateOrdersRequest;
use gloo_net::http::Request;
use web_sys::RequestCredentials;

#[derive(Copy, Clone)]
pub struct AdminOrdersClient {
    url: &'static str,
}

impl AdminOrdersClient {
    pub const fn new(url: &'static str) -> Self {
        Self { url }
    }

    pub async fn create_order(&self, request: AdminCreateOrdersRequest) -> Result<()> {
        let url = format!("{}/admin/orders", self.url);
        let request = Request::post(&url)
            .credentials(RequestCredentials::Include)
            .json(&request)?;

        send(request).await
    }
}
