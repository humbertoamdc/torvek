use gloo_net::http::Request;
use serde::de::DeserializeOwned;
use web_sys::RequestCredentials;

use api_boundary::quotations::models::QuotationStatus;
use api_boundary::quotations::responses::AdminQueryQuotationsByStatusResponse;

use crate::api::common::{into_json, Result};

#[derive(Clone, Copy)]
pub struct QuotationsClient {
    url: &'static str,
}

impl QuotationsClient {
    pub const fn new(url: &'static str) -> Self {
        Self { url }
    }

    pub async fn query_quotations_by_status(
        &self,
        status: QuotationStatus,
    ) -> Result<AdminQueryQuotationsByStatusResponse> {
        let url = format!("{}/admin/quotations", self.url);
        let request = Request::get(&url)
            .query([("status", status.to_string())])
            .credentials(RequestCredentials::Include)
            .build()?;

        self.send(request).await
    }

    async fn send<T: DeserializeOwned>(&self, req: Request) -> Result<T> {
        let response = req.send().await?;
        into_json(response).await
    }
}
