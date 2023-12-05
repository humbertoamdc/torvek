use crate::api::common::{into_json, Result};
use crate::api::models::orders::{QueryOrdersResponse, UpdateOrderRequest};
use crate::env;
use gloo_net::http::Request;
use serde::de::DeserializeOwned;
use web_sys::{ RequestCredentials};

#[derive(Clone, Copy)]
pub struct OrdersApi {
    url: &'static str,
}

impl OrdersApi {

    pub const fn new() -> Self {
        Self { url: env::API_URL }
    }


    pub async fn query_orders_by_status(
        &self,
        order_status: String,
    ) -> Result<QueryOrdersResponse> {
        let url = format!("{}/admin/orders", self.url);
        let request = Request::get(&url)
            .query([("order_status", order_status)])
            .credentials(RequestCredentials::Include)
            .build()?;

        self.send(request).await
    }

    pub async fn update_order(&self, body: UpdateOrderRequest) -> Result<()> {
        let url = format!("{}/admin/orders", self.url);
        let request = Request::patch(&url)
            .credentials(RequestCredentials::Include)
            .json(&body)?;

        self.send(request).await
    }

    async fn send<T: DeserializeOwned>(&self, req: Request) -> Result<T> {
        let response = req.send().await?;
        into_json(response).await
    }
}
