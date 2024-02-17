use gloo_net::http::Request;
use serde::de::DeserializeOwned;
use web_sys::RequestCredentials;

use api_boundary::parts::responses::QueryPartsForQuotationResponse;

use crate::api::common::{into_json, Result};
use crate::env::API_URL;

#[derive(Clone, Copy)]
pub struct PartsClient {
    url: &'static str,
}

impl PartsClient {
    pub const fn new() -> Self {
        Self { url: API_URL }
    }

    pub async fn query_parts_for_quotation(
        &self,
        client_id: String,
        project_id: String,
        quotation_id: String,
    ) -> Result<QueryPartsForQuotationResponse> {
        // TODO: Create and use endpoint for admin.
        let url = format!(
            "{}/clients/{client_id}/projects/{project_id}/quotations/{quotation_id}/parts",
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
