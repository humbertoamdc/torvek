use crate::api::common::{into_json, Result};
use crate::env;
use api_boundary::parts::requests::CreatePartsRequest;
use api_boundary::parts::responses::CreatePartsResponse;
use gloo_net::http::Request;
use serde::de::DeserializeOwned;
use web_sys::RequestCredentials;

#[derive(Clone, Copy)]
pub struct PartsClient {
    url: &'static str,
}

impl PartsClient {
    pub const fn new() -> Self {
        Self { url: env::API_URL }
    }

    pub async fn create_parts(&self, body: CreatePartsRequest) -> Result<CreatePartsResponse> {
        let url = format!("{}/parts", self.url);
        let request = Request::post(&url)
            .credentials(RequestCredentials::Include)
            .json(&body)?;

        self.send(request).await
    }

    async fn send<T: DeserializeOwned>(&self, req: Request) -> crate::api::common::Result<T> {
        let response = req.send().await?;
        into_json(response).await
    }
}
