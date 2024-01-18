use crate::parts::models::Part;
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
pub struct QueryPartsForQuotationResponse {
    pub parts: Vec<Part>,
}
impl QueryPartsForQuotationResponse {
    pub fn new(parts: Vec<Part>) -> Self {
        Self { parts }
    }
}

#[derive(Deserialize, Serialize, Debug)]
pub struct CreateDrawingUploadUrlResponse {
    pub url: String,
}

impl CreateDrawingUploadUrlResponse {
    pub const fn new(url: String) -> Self {
        Self { url }
    }
}
