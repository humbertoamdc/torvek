use ory_kratos_client::models::Identity;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct User {
    pub id: String,
    pub traits: Traits,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Traits {
    pub email: String,
    pub name: Name,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Name {
    pub first: String,
    pub last: String,
}

impl From<Identity> for User {
    fn from(identity: Identity) -> Self {
        let traits = serde_json::from_value::<Traits>(identity.traits.unwrap()).unwrap();
        Self {
            id: identity.id,
            traits,
        }
    }
}
