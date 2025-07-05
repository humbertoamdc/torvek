use axum::routing::get;
use axum::routing::post;
use axum::Router;

use crate::app_state::AppState;
use crate::auth::controllers::{
    admin_login, admin_logout, get_admin_session, get_session, login, logout, register_client,
};

pub fn create_public_router() -> Router<AppState> {
    Router::new()
        .route("/register", post(register_client))
        .route("/login", post(login))
        .route("/accounts/admins/login", post(admin_login))
}

pub fn create_private_router() -> Router<AppState> {
    Router::new()
        .route("/logout", post(logout))
        .route("/session", get(get_session))
        .route("/accounts/admins/session", get(get_admin_session))
        .route("/accounts/admins/logout", post(admin_logout))
}
