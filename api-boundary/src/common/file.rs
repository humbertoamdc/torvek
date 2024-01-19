use serde_derive::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct File {
    pub name: String,
    pub url: String,
}

impl File {
    pub const fn new(name: String, url: String) -> Self {
        File { name, url }
    }
}
