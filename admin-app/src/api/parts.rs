use crate::api::common::{into_json, Result};
use crate::env;
use api_boundary::parts::requests::AdminUpdatePartRequest;
use api_boundary::parts::responses::AdminQueryPartsByStatusResponse;
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

    pub async fn query_parts_by_status(
        &self,
        status: String,
    ) -> Result<AdminQueryPartsByStatusResponse> {
        let url = format!("{}/admin/parts", self.url);
        let request = Request::get(&url)
            .query([("status", status)])
            .credentials(RequestCredentials::Include)
            .build()?;

        self.send(request).await
    }

    pub async fn update_part(&self, body: AdminUpdatePartRequest) -> Result<()> {
        let url = format!("{}/admin/parts", self.url);
        let request = Request::patch(&url)
            .credentials(RequestCredentials::Include)
            .json(&body)?;

        self.send(request).await
    }

    async fn send<T: DeserializeOwned>(&self, req: Request) -> crate::api::common::Result<T> {
        let response = req.send().await?;
        into_json(response).await
    }
}
