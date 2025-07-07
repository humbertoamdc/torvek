use gloo_net::http::Request;
use serde::de::DeserializeOwned;
use serde_derive::{Deserialize, Serialize};
use web_sys::RequestCredentials;

use crate::api::common::{into_json, Result};
use crate::models::quotation::{Quotation, QuotationStatus};

#[derive(Deserialize, Serialize, Debug)]
pub struct AdminQueryQuotationsByStatusResponse {
    pub quotations: Vec<Quotation>,
}

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
