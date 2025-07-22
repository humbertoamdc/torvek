use crate::parts::models::part::Part;
use crate::shared::file::File;
use crate::shared::money::Money;
use serde_derive::{Deserialize, Serialize};

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
pub struct UploadDrawingResponse {
    pub upload_url: String,
    pub file: File,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct QueryPartsForQuotationResponse {
    pub parts: Vec<Part>,
    pub quotation_subtotal: Option<Money>,
    pub cursor: Option<String>,
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

#[derive(Deserialize, Serialize, Debug)]
pub struct UpdateSelectedPartQuoteResponse {
    pub part: Part,
}
