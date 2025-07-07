use serde_derive::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct File {
    pub name: String,
    pub url: String,
    pub presigned_url: Option<String>,
}

impl File {
    pub const fn new(name: String, url: String) -> Self {
        File {
            name,
            url,
            presigned_url: None,
        }
    }
}
