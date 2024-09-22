use crate::common::money::Money;
use serde_derive::{Deserialize, Serialize};

use crate::parts::models::Part;

#[derive(Deserialize, Serialize, Debug)]
pub struct CreatePartsResponse {
    pub upload_urls: Vec<String>,
}
impl CreatePartsResponse {
    pub const fn new(upload_urls: Vec<String>) -> Self {
        Self { upload_urls }
    }
}

#[derive(Deserialize, Serialize, Debug)]
pub struct QueryPartsForQuotationResponse {
    pub parts: Vec<Part>,
    pub quotation_subtotal: Option<Money>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct CreateDrawingUploadUrlResponse {
    pub url: String,
    pub presigned_url: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct CreateModelUploadUrlResponse {
    pub url: String,
    pub presigned_url: String,
}

impl CreateDrawingUploadUrlResponse {
    pub const fn new(url: String, presigned_url: String) -> Self {
        Self { url, presigned_url }
    }
}

#[derive(Deserialize, Serialize, Debug)]
pub struct AdminQueryPartsByStatusResponse {
    pub parts: Vec<Part>,
}
impl AdminQueryPartsByStatusResponse {
    pub fn new(parts: Vec<Part>) -> Self {
        Self { parts }
    }
}

#[derive(Deserialize, Serialize, Debug)]
pub struct GetRenderFilePresignedUrlResponse {
    pub file_url: String,
}
impl GetRenderFilePresignedUrlResponse {
    pub const fn new(file_url: String) -> Self {
        Self { file_url }
    }
}
