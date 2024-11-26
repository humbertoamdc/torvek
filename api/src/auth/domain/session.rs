use axum_extra::extract::cookie::{Cookie, SameSite};
use chrono::{DateTime, Utc};
use serde_derive::{Deserialize, Serialize};

use crate::auth::domain::user::User;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct SessionWithToken {
    #[serde(rename = "session_token")]
    pub session_token: String,
    #[serde(rename = "session")]
    pub session: Session,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Session {
    pub active: bool,
    pub expires_at: DateTime<Utc>,
    pub identity: Identity,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Identity {
    pub id: String,
    pub traits: Traits,
    pub metadata_admin: Option<MetadataAdmin>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Traits {
    pub email: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct MetadataPublic {
    pub user: User,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct MetadataAdmin {
    pub stripe_customer_id: String,
}

impl SessionWithToken {
    pub fn session_cookie(&self, name: &'static str, secure: bool, domain: String) -> Cookie {
        let cookie = Cookie::build((name, &self.session_token))
            .secure(secure)
            .http_only(true)
            .domain(domain)
            .same_site(SameSite::Strict)
            .path("/")
            .build();

        cookie
    }
}
