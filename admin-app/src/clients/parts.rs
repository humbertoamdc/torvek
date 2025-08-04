use crate::clients::common::{send, Result};
use crate::models::money::Money;
use crate::models::part::Part;
use gloo_net::http::Request;
use serde_derive::{Deserialize, Serialize};
use web_sys::RequestCredentials;

#[derive(Deserialize, Serialize, Debug)]
pub struct CreatePartQuotesRequest {
    pub customer_id: String,
    pub project_id: String,
    pub quotation_id: String,
    pub data: Vec<CreatePartQuotesRequestData>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct CreatePartQuotesRequestData {
    pub part_id: String,
    pub unit_price: Money,
    pub sub_total: Money,
    pub workdays_to_complete: u64,
    pub quantity: u64,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct GeneratePresignedUrlRequest {
    pub key: String,
    pub operation: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct QueryPartsForQuotationResponse {
    pub parts: Vec<Part>,
    pub quotation_subtotal: Option<Money>,
    pub cursor: Option<String>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct GeneratePresignedUrlResponse {
    pub presigned_url: String,
}

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
            .json(&request)
            .unwrap();

        send(request).await
    }

    pub async fn admin_query_parts_for_quotation(
        &self,
        customer_id: String,
        quotation_id: String,
    ) -> Result<QueryPartsForQuotationResponse> {
        let url = format!(
            "{}/admin/customers/{customer_id}/quotations/{quotation_id}/parts",
            self.url
        );
        let request = Request::get(&url)
            .credentials(RequestCredentials::Include)
            .build()
            .unwrap();

        send(request).await
    }

    pub async fn admin_generate_presigned_url(
        &self,
        request: GeneratePresignedUrlRequest,
    ) -> Result<GeneratePresignedUrlResponse> {
        let url = format!("{}/admin/presigned_url", self.url);
        let request = Request::post(&url)
            .credentials(RequestCredentials::Include)
            .json(&request)
            .unwrap();

        send(request).await
    }
}
