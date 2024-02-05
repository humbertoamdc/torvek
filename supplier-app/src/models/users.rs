use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct User {
    pub email: String,
    pub name: Name,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Name {
    pub first: String,
    pub last: String,
}
