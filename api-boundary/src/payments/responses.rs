use serde_derive::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct CreateCheckoutSessionResponse {
    pub url: String,
}
impl CreateCheckoutSessionResponse {
    pub const fn new(url: String) -> Self {
        Self { url }
    }
}
