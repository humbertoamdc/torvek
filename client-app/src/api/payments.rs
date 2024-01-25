use crate::api::common::{into_json, Result};
use crate::env;
use api_boundary::payments::requests::CreateCheckoutSessionRequest;
use api_boundary::payments::responses::CreateCheckoutSessionResponse;
use gloo_net::http::Request;
use serde::de::DeserializeOwned;
use web_sys::RequestCredentials;

#[derive(Clone, Copy)]
pub struct PaymentsClient {
    url: &'static str,
}

impl PaymentsClient {
    pub const fn new() -> Self {
        Self { url: env::API_URL }
    }

    pub async fn create_checkout_session(
        &self,
        body: CreateCheckoutSessionRequest,
    ) -> Result<CreateCheckoutSessionResponse> {
        let url = format!("{}/payments/create_checkout_session", self.url);
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
