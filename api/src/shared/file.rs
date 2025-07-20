use serde_derive::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Default)]
pub struct File {
    pub name: String,
    pub key: String,
}

impl File {
    pub const fn new(name: String, key: String) -> Self {
        File { name, key }
    }
}
