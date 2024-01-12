use crate::api::common::{into_json, Result};
use crate::api::models::orders::{
    CreateOrdersRequest, CreateOrdersResponse, QueryOrdersForClientResponse, UpdateOrderRequest,
};
use crate::env;
use api_boundary::orders::requests::CreateDrawingUploadUrlRequest;
use api_boundary::orders::responses::CreateDrawingUploadUrlResponse;
use gloo_net::http::Request;
use leptos::wasm_bindgen::JsValue;
use leptos::web_sys::{File, RequestCredentials};
use serde::de::DeserializeOwned;

#[derive(Clone, Copy)]
pub struct OrdersClient {
    url: &'static str,
}

impl OrdersClient {
    pub const fn new() -> Self {
        Self { url: env::API_URL }
    }

    pub async fn create_orders(
        &self,
        body: CreateOrdersRequest,
    ) -> Result<Vec<CreateOrdersResponse>> {
        let url = format!("{}/orders", self.url);
        let request = Request::post(&url)
            .credentials(RequestCredentials::Include)
            .json(&body)?;

        self.send(request).await
    }

    pub async fn upload_file_with_presigned_url(
        &self,
        file: File,
        presigned_url: String,
    ) -> Result<()> {
        Request::put(presigned_url.as_str())
            .body(JsValue::from(file))?
            .send()
            .await?;

        Ok(())
    }

    pub async fn query_orders_for_client(
        &self,
        client_id: String,
    ) -> Result<QueryOrdersForClientResponse> {
        let url = format!("{}/clients/{}/orders", self.url, client_id);
        let request = Request::get(&url)
            .credentials(RequestCredentials::Include)
            .build()?;

        self.send(request).await
    }

    pub async fn update_order(&self, body: UpdateOrderRequest) -> Result<()> {
        let url = format!("{}/orders", self.url);
        let request = Request::patch(&url)
            .credentials(RequestCredentials::Include)
            .json(&body)?;

        self.send(request).await
    }

    pub async fn create_drawing_upload_url(
        &self,
        body: CreateDrawingUploadUrlRequest,
    ) -> Result<CreateDrawingUploadUrlResponse> {
        let url = format!("{}/orders/drawing_upload_url", self.url);
        let request = Request::post(&url)
            .credentials(RequestCredentials::Include)
            .json(&body)?;

        self.send(request).await
    }

    async fn send<T: DeserializeOwned>(&self, req: Request) -> Result<T> {
        let response = req.send().await?;
        into_json(response).await
    }
}
