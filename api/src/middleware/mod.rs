use crate::app_state::AppState;
use crate::auth;
use crate::auth::controllers::{ADMIN_SESSION_TOKEN, CUSTOMER_SESSION_TOKEN};
use crate::shared::extractors::session::{AdminSession, CustomerSession};
use axum::response::{IntoResponse, Response};
use http::{header, HeaderMap, HeaderValue, Request};
use lambda_http::tower::Layer;
use lambda_http::Service;
use std::convert::Infallible;
use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};

#[derive(Clone)]
pub struct SessionAuthLayer {
    state: AppState,
}

impl SessionAuthLayer {
    pub const fn new(state: AppState) -> Self {
        Self { state }
    }
}

impl<S> Layer<S> for SessionAuthLayer {
    type Service = SessionAuth<S>;

    fn layer(&self, inner: S) -> Self::Service {
        SessionAuth::new(inner, self.state.clone())
    }
}

#[derive(Clone)]
pub struct SessionAuth<S> {
    inner: S,
    state: AppState,
}

impl<S> SessionAuth<S> {
    pub const fn new(inner: S, state: AppState) -> Self {
        Self { inner, state }
    }

    fn get_session_tokens(&self, header_map: &HeaderMap) -> Vec<String> {
        let mut tokens = Vec::new();

        if !header_map.contains_key(header::COOKIE) {
            return tokens;
        }

        for cookie in self.get_cookie_str(header_map).split(';') {
            let (token_name, token_value) = cookie.trim().split_once('=').unwrap();
            if token_name == CUSTOMER_SESSION_TOKEN || token_name == ADMIN_SESSION_TOKEN {
                tokens.push(token_value.to_owned());
            }
        }
        tokens
    }

    fn get_cookie_str(&self, header_map: &HeaderMap<HeaderValue>) -> String {
        if let Some(cookie_header) = header_map.get("cookie") {
            if let Ok(cookie_str) = cookie_header.to_str() {
                return cookie_str.to_owned();
            }
        }
        String::default()
    }
}

impl<S, Body> Service<Request<Body>> for SessionAuth<S>
where
    S: Service<Request<Body>, Error = Infallible> + Clone + Send + 'static,
    S::Response: IntoResponse + 'static,
    Body: Send + 'static,
    S::Future: Send + 'static,
{
    type Response = Response;
    type Error = Infallible;
    type Future =
        Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>> + Send + 'static>>;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.inner.poll_ready(cx)
    }

    fn call(&mut self, mut req: Request<Body>) -> Self::Future {
        let session_tokens = self.get_session_tokens(req.headers());
        let mut inner = self.inner.clone();
        let state = self.state.clone();

        Box::pin(async move {
            for token in session_tokens {
                if let Ok(session) = state.auth.identity_manager.get_session(token).await {
                    match session.clone().identity.metadata_public.role {
                        auth::models::session::Role::Admin => {
                            req.extensions_mut().insert(AdminSession(session));
                        }
                        auth::models::session::Role::Customer => {
                            req.extensions_mut().insert(CustomerSession(session));
                        }
                    };
                }
            }

            let res = inner.call(req).await?;
            Ok(res.into_response())
        })
    }
}
