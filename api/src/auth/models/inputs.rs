use crate::auth::models::session::Role;
use serde_derive::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct RegisterUserInput {
    pub email: String,
    pub name: String,
    pub password: String,
    pub role: Role,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct LoginUserInput {
    pub email: String,
    pub password: String,
    pub role: Role,
}
