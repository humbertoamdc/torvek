use serde_derive::{Deserialize, Serialize};
use serde_enum_str::{Deserialize_enum_str, Serialize_enum_str};

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    pub app: App,
    pub auth: Auth,
    pub orders: Orders,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct App {
    pub env: Environment,
    pub port: u16,
    pub domain: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Auth {
    pub ory_clients_url: String,
    pub ory_clients_api_key: String,
    pub ory_admins_url: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Orders {
    pub s3_bucket: String,
    pub orders_table: String,
}

impl From<&str> for Config {
    fn from(config_string: &str) -> Self {
        toml::from_str::<Config>(config_string).expect("failed to parse config")
    }
}

#[derive(Serialize_enum_str, Deserialize_enum_str, Clone, Debug, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum Environment {
    Development,
    Production,
}

impl Environment {
    pub fn secure_session_cookie(&self) -> bool {
        *self != Environment::Development
    }
}
