use gloo_net::http::Request;
use serde_derive::{Deserialize, Serialize};
use web_sys::RequestCredentials;

use crate::clients::common::{send, Result};
use crate::models::money::Money;

#[derive(Copy, Clone)]
pub struct OrdersClient {
    url: &'static str,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct AdminUpdateOrderPayoutRequest {
    pub order_id: String,
    pub payout: Money,
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
}
