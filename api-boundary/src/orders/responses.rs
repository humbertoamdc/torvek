use serde_derive::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct CreateDrawingUploadUrlResponse {
    pub url: String,
}

impl CreateDrawingUploadUrlResponse {
    pub const fn new(url: String) -> Self {
        Self { url }
    }
}
