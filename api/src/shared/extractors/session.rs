use crate::auth::models::session::Session;
use async_trait::async_trait;
use axum::extract::FromRequestParts;
use http::request::Parts;
use http::StatusCode;

#[derive(Clone, Debug)]
pub struct AdminSession(pub Session);
#[derive(Clone, Debug)]
pub struct CustomerSession(pub Session);

#[async_trait]
impl<S> FromRequestParts<S> for CustomerSession
where
    S: Send + Sync,
{
    type Rejection = (StatusCode, &'static str);

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        let customer_session = parts
            .extensions
            .get::<CustomerSession>()
            .cloned()
            .ok_or((StatusCode::UNAUTHORIZED, "Missing customer session"))?;

        if !customer_session.0.active {
            return Err((StatusCode::UNAUTHORIZED, "Expired session"));
        }

        Ok(customer_session)
    }
}

#[async_trait]
impl<S> FromRequestParts<S> for AdminSession
where
    S: Send + Sync,
{
    type Rejection = (StatusCode, &'static str);

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        let admin_session = parts
            .extensions
            .get::<AdminSession>()
            .cloned()
            .ok_or((StatusCode::UNAUTHORIZED, "Missing admin session"))?;

        if !admin_session.0.active {
            return Err((StatusCode::UNAUTHORIZED, "Expired session"));
        }

        Ok(admin_session)
    }
}
