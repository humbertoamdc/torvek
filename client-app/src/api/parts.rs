use crate::api::common::{into_json, Result};
use crate::env;
use api_boundary::parts::requests::CreateDrawingUploadUrlRequest;
use api_boundary::parts::requests::{CreatePartsRequest, UpdatePartRequest};
use api_boundary::parts::responses::CreateDrawingUploadUrlResponse;
use api_boundary::parts::responses::{CreatePartsResponse, QueryPartsForQuotationResponse};
use gloo_net::http::Request;
use leptos::wasm_bindgen::JsValue;
use serde::de::DeserializeOwned;
use web_sys::{File, RequestCredentials};

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

    pub async fn upload_file_with_presigned_url(
        &self,
        file: File,
        presigned_url: String,
    ) -> Result<()> {
        Request::put(presigned_url.as_str())
            .body(JsValue::from(file))?
            .send()
            .await?;

        Ok(())
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

        self.send(request).await
    }

    pub async fn update_part(&self, body: UpdatePartRequest) -> Result<()> {
        let url = format!("{}/parts", self.url);
        let request = Request::patch(&url)
            .credentials(RequestCredentials::Include)
            .json(&body)?;

        self.send(request).await
    }

    pub async fn create_drawing_upload_url(
        &self,
        body: CreateDrawingUploadUrlRequest,
    ) -> Result<CreateDrawingUploadUrlResponse> {
        let url = format!("{}/parts/drawing_upload_url", self.url);
        let request = Request::post(&url)
            .credentials(RequestCredentials::Include)
            .json(&body)?;

        self.send(request).await
    }

    async fn send<T: DeserializeOwned>(&self, req: Request) -> Result<T> {
        let response = req.send().await?;
        into_json(response).await
    }
}
