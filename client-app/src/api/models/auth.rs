use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Credentials {
    pub email: String,
    pub password: String,
}

#[derive(Clone, Serialize, Deserialize, Debug, Default)]
pub struct UserInfo {
    pub id: String,
    pub email: String,
}
