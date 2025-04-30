use crate::app_state::AppState;
use crate::auth::controllers::CUSTOMER_SESSION_TOKEN;
use crate::projects::controllers::{
    create_project, delete_project, get_project_by_id, query_projects_for_client,
};
use axum::routing::{delete, get, post};
use axum::Router;
use http::{HeaderMap, HeaderValue, Request};
use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};
use tower::{Layer, Service};

pub fn create_router(state: AppState) -> Router<AppState> {
    Router::new()
        .route("/projects", post(create_project))
        .route(
            "/customers/:customer_id/projects",
            get(query_projects_for_client),
        )
        .route(
            "/customers/:customer_id/projects/:project_id",
            get(get_project_by_id),
        )
        .route(
            "/customers/:customer_id/projects/:project_id",
            delete(delete_project),
        )
        .layer(SessionAuthLayer::new(state))
}

#[derive(Clone)]
struct SessionAuthLayer {
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
struct SessionAuth<S> {
    inner: S,
    state: AppState,
}

impl<S> SessionAuth<S> {
    pub const fn new(inner: S, state: AppState) -> Self {
        Self { inner, state }
    }

    fn get_session_token(&self, header_map: &HeaderMap) -> Option<String> {
        for cookie in self.get_cookie_str(header_map).split(';') {
            let (token_name, token_value) = cookie.trim().split_once('=').unwrap();
            if token_name == CUSTOMER_SESSION_TOKEN {
                return Some(token_value.to_owned());
            }
        }
        None
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
    S: Service<Request<Body>>,
    S::Future: Send + 'static,
{
    type Response = S::Response;
    type Error = S::Error;
    type Future =
        Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>> + Send + 'static>>;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.inner.poll_ready(cx).map_err(Into::into)
    }

    fn call(&mut self, req: Request<Body>) -> Self::Future {
        let session_token = self.get_session_token(req.headers()).unwrap(); // TODO: Remove unwrap
        let inner = self.inner.call(req);
        let state = self.state.clone();

        Box::pin(async move {
            match state.auth.identity_manager.get_session(session_token).await {
                Ok(session) => {
                    tracing::info!("{session:#?}");
                }
                Err(e) => {
                    tracing::error!("{e}");
                }
            }
            inner.await
        })
    }
}

pub type BoxFuture<'a, T> = Pin<Box<dyn Future<Output = T> + Send + 'a>>;
