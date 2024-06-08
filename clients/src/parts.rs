use gloo_net::http::Request;
use gloo_net::Error;
use web_sys::wasm_bindgen::JsValue;
use web_sys::{File, RequestCredentials};

use api_boundary::parts::requests::{
    CreateDrawingUploadUrlRequest, CreatePartQuotesRequest, CreatePartsRequest,
    QueryPartQuotesForPartsRequest, QueryPartQuotesForPartsResponse, UpdatePartRequest,
};
use api_boundary::parts::responses::{
    CreateDrawingUploadUrlResponse, CreatePartsResponse, QueryPartsForQuotationResponse,
};

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
            .json(&request)
            .unwrap();

        send(request).await
    }

    pub async fn create_parts(&self, body: CreatePartsRequest) -> Result<CreatePartsResponse> {
        let url = format!("{}/parts", self.url);
        let request = Request::post(&url)
            .credentials(RequestCredentials::Include)
            .json(&body)
            .unwrap();

        send(request).await
    }

    pub async fn get_file_from_presigned_url(
        &self,
        presigned_url: String,
    ) -> std::result::Result<gloo_net::http::Response, Error> {
        Request::get(presigned_url.as_str()).send().await
    }

    pub async fn upload_file_with_presigned_url(
        &self,
        file: File,
        presigned_url: String,
    ) -> Result<()> {
        Request::put(presigned_url.as_str())
            .body(JsValue::from(file))
            .unwrap()
            .send()
            .await
            .unwrap();

        Ok(())
    }

    pub async fn update_part(&self, body: UpdatePartRequest) -> Result<()> {
        let url = format!("{}/parts", self.url);
        let request = Request::patch(&url)
            .credentials(RequestCredentials::Include)
            .json(&body)
            .unwrap();

        send(request).await
    }

    pub async fn create_drawing_upload_url(
        &self,
        body: CreateDrawingUploadUrlRequest,
    ) -> Result<CreateDrawingUploadUrlResponse> {
        let url = format!("{}/parts/drawing_upload_url", self.url);
        let request = Request::post(&url)
            .credentials(RequestCredentials::Include)
            .json(&body)
            .unwrap();

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
            .build()
            .unwrap();

        send(request).await
    }

    pub async fn query_part_quotes_for_parts(
        &self,
        body: QueryPartQuotesForPartsRequest,
    ) -> Result<QueryPartQuotesForPartsResponse> {
        let url = format!("{}/parts/quotes", self.url);
        let request = Request::post(&url)
            .credentials(RequestCredentials::Include)
            .json(&body)
            .unwrap();

        send(request).await
    }
}
