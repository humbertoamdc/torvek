use serde_derive::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct File {
    pub name: String,
    pub key: String,
}
