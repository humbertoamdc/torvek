use api_boundary::quotations::models::Quotation;
use api_boundary::quotations::requests::CreateQuotationRequest;
use api_boundary::quotations::responses::QueryQuotationsForProjectResponse;
use gloo_net::http::Request;
use web_sys::RequestCredentials;

use crate::common::{send, Result};
#[derive(Clone, Copy)]
pub struct QuotationsClient {
    url: &'static str,
}

impl QuotationsClient {
    pub const fn new(url: &'static str) -> Self {
        Self { url }
    }

    pub async fn create_quotation(&self, body: CreateQuotationRequest) -> Result<()> {
        let url = format!("{}/quotations", self.url);
        let request = Request::post(&url)
            .credentials(RequestCredentials::Include)
            .json(&body)
            .unwrap();

        send(request).await
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
            .build()
            .unwrap();

        send(request).await
    }

    pub async fn get_quotation_by_id(
        &self,
        client_id: String,
        project_id: String,
        quotation_id: String,
    ) -> Result<Quotation> {
        let url = format!(
            "{}/clients/{client_id}/projects/{project_id}/quotations/{quotation_id}",
            self.url
        );
        let request = Request::get(&url)
            .credentials(RequestCredentials::Include)
            .build()
            .unwrap();

        send(request).await
    }
}
