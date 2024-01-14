use crate::api::common::{into_json, Result};
use crate::env;
use api_boundary::quotations::requests::CreateQuotationRequest;
use api_boundary::quotations::responses::QueryQuotationsForProjectResponse;
use gloo_net::http::Request;
use serde::de::DeserializeOwned;
use web_sys::RequestCredentials;

#[derive(Clone, Copy)]
pub struct QuotationsClient {
    url: &'static str,
}

impl QuotationsClient {
    pub const fn new() -> Self {
        Self { url: env::API_URL }
    }

    pub async fn create_quotation(&self, body: CreateQuotationRequest) -> Result<()> {
        let url = format!("{}/quotations", self.url);
        let request = Request::post(&url)
            .credentials(RequestCredentials::Include)
            .json(&body)?;

        self.send(request).await
    }

    pub async fn query_quotations_for_project(
        &self,
        client_id: String,
        project_id: String,
    ) -> Result<QueryQuotationsForProjectResponse> {
        let url = format!(
            "{}/clients/{client_id}/projects/{project_id}/quotations",
            self.url
        );
        let request = Request::get(&url)
            .credentials(RequestCredentials::Include)
            .build()?;

        self.send(request).await
    }

    async fn send<T: DeserializeOwned>(&self, req: Request) -> Result<T> {
        let response = req.send().await?;
        into_json(response).await
    }
}
