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
