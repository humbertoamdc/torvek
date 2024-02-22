use gloo_net::http::Request;
use web_sys::RequestCredentials;

use api_boundary::parts::requests::CreatePartQuotesRequest;
use api_boundary::parts::responses::QueryPartsForQuotationResponse;

use crate::common::{send, Result};

#[derive(Copy, Clone)]
pub struct PartsClient {
    url: &'static str,
}

impl PartsClient {
    pub const fn new(url: &'static str) -> Self {
        Self { url }
    }

    pub async fn admin_create_part_quotes(&self, request: CreatePartQuotesRequest) -> Result<()> {
        let url = format!("{}/admin/part_quotes", self.url);
        let request = Request::post(&url)
            .credentials(RequestCredentials::Include)
            .json(&request)?;

        send(request).await
    }

    pub async fn query_parts_for_quotation(
        &self,
        client_id: String,
        project_id: String,
        quotation_id: String,
    ) -> Result<QueryPartsForQuotationResponse> {
        let url = format!(
            "{}/clients/{client_id}/projects/{project_id}/quotations/{quotation_id}/parts",
            self.url
        );
        let request = Request::get(&url)
            .credentials(RequestCredentials::Include)
            .build()?;

        send(request).await
    }
}
