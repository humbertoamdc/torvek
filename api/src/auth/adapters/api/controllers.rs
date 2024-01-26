use axum::extract::State;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::Json;
use axum_extra::extract::cookie::Cookie;
use axum_extra::extract::CookieJar;
use cookie::time::OffsetDateTime;

use crate::app_state::AppState;
use crate::auth::adapters::api::mappers::GetSessionResponseMapper;
use crate::auth::adapters::api::requests::{
    AdminLoginRequest, LoginClientRequest, RegisterClientRequest,
};
use crate::auth::application::usecases::admin_login::AdminLoginUseCase;
use crate::auth::application::usecases::admin_logout::AdminLogoutUseCase;
use crate::auth::application::usecases::get_admin_session::GetAdminSessionUseCase;
use crate::auth::application::usecases::get_session::GetSessionUseCase;
use crate::auth::application::usecases::interfaces::UseCase;
use crate::auth::application::usecases::login_client::LoginClientUseCase;
use crate::auth::application::usecases::logout_client::LogoutClientUseCase;
use crate::auth::application::usecases::register_client::RegisterClientUseCase;
use crate::auth::domain::errors::AuthError;

static CLIENT_SESSION_TOKEN: &'static str = "client_session_token";
static ADMIN_SESSION_TOKEN: &'static str = "admin_session_token";

pub async fn register_client(
    cookies: CookieJar,
    State(app_state): State<AppState>,
    Json(request): Json<RegisterClientRequest>,
) -> impl IntoResponse {
    let usecase = RegisterClientUseCase::new(app_state.auth.identity_manager);
    let result = usecase.execute(request).await;

    match result {
        Ok(auth_session) => (
            StatusCode::NO_CONTENT,
            cookies.add(
                auth_session
                    .session_cookie(
                        CLIENT_SESSION_TOKEN,
                        app_state.env.secure_session_cookie(),
                        app_state.domain,
                    )
                    .into_owned(),
            ),
        ),
        Err(_) => (StatusCode::BAD_REQUEST, cookies),
    }
}

pub async fn login(
    cookies: CookieJar,
    State(app_state): State<AppState>,
    Json(request): Json<LoginClientRequest>,
) -> impl IntoResponse {
    let usecases = LoginClientUseCase::new(app_state.auth.identity_manager);
    let result = usecases.execute(request).await;

    match result {
        Ok(auth_session) => (
            StatusCode::OK,
            cookies.add(
                auth_session
                    .session_cookie(
                        CLIENT_SESSION_TOKEN,
                        app_state.env.secure_session_cookie(),
                        app_state.domain,
                    )
                    .into_owned(),
            ),
        ),
        Err(_) => (StatusCode::BAD_REQUEST, cookies),
    }
}

pub async fn get_session(
    cookies: CookieJar,
    State(app_state): State<AppState>,
) -> impl IntoResponse {
    let usecase = GetSessionUseCase::new(app_state.auth.identity_manager);
    let session_cookie = cookies.get(CLIENT_SESSION_TOKEN);

    let result = match session_cookie {
        Some(session_cookie) => usecase.execute(session_cookie.value().to_string()).await,
        None => Err(AuthError::UnknownError),
    };

    match result {
        Ok(session_information) => http::Response::builder()
            .status(StatusCode::OK)
            .body(
                serde_json::to_string(&GetSessionResponseMapper::to_api(session_information))
                    .unwrap(),
            )
            .unwrap(),
        // TODO: Handle error.
        Err(_) => http::Response::builder()
            .status(StatusCode::UNAUTHORIZED)
            .body("".to_string())
            .unwrap(),
    }
}

pub async fn logout(cookies: CookieJar, State(app_state): State<AppState>) -> impl IntoResponse {
    let usecase = LogoutClientUseCase::new(app_state.auth.identity_manager);
    let session_cookie = cookies.get(CLIENT_SESSION_TOKEN);

    let result = match session_cookie {
        Some(session_cookie) => usecase.execute(session_cookie.value().to_string()).await,
        // TODO: Handle error.
        None => Err(AuthError::UnknownError),
    };

    match result {
        Ok(_) => {
            let cookie = Cookie::build(CLIENT_SESSION_TOKEN, "")
                .path("/")
                .expires(OffsetDateTime::now_utc())
                .finish();

            (StatusCode::NO_CONTENT, cookies.add(cookie))
        }
        Err(_) => (StatusCode::BAD_REQUEST, CookieJar::new()),
    }
}

pub async fn admin_login(
    cookies: CookieJar,
    State(app_state): State<AppState>,
    Json(request): Json<AdminLoginRequest>,
) -> impl IntoResponse {
    let usecases = AdminLoginUseCase::new(app_state.auth.admin_identity_manager);
    let result = usecases.execute(request).await;

    match result {
        Ok(auth_session) => (
            StatusCode::OK,
            cookies.add(
                auth_session
                    .session_cookie(
                        ADMIN_SESSION_TOKEN,
                        app_state.env.secure_session_cookie(),
                        app_state.domain,
                    )
                    .into_owned(),
            ),
        ),
        Err(_) => (StatusCode::BAD_REQUEST, cookies),
    }
}

pub async fn get_admin_session(
    cookies: CookieJar,
    State(app_state): State<AppState>,
) -> impl IntoResponse {
    let usecase = GetAdminSessionUseCase::new(app_state.auth.admin_identity_manager);
    let session_cookie = cookies.get(ADMIN_SESSION_TOKEN);

    let result = match session_cookie {
        Some(session_cookie) => usecase.execute(session_cookie.value().to_string()).await,
        None => Err(AuthError::UnknownError),
    };

    match result {
        Ok(session_information) => http::Response::builder()
            .status(StatusCode::OK)
            .body(
                serde_json::to_string(&GetSessionResponseMapper::to_api(session_information))
                    .unwrap(),
            )
            .unwrap(),
        // TODO: Handle error.
        Err(_) => http::Response::builder()
            .status(StatusCode::UNAUTHORIZED)
            .body("".to_string())
            .unwrap(),
    }
}

pub async fn admin_logout(
    cookies: CookieJar,
    State(app_state): State<AppState>,
) -> impl IntoResponse {
    let usecase = AdminLogoutUseCase::new(app_state.auth.admin_identity_manager);
    let session_cookie = cookies.get(ADMIN_SESSION_TOKEN);

    let result = match session_cookie {
        Some(session_cookie) => usecase.execute(session_cookie.value().to_string()).await,
        // TODO: Handle error.
        None => Err(AuthError::UnknownError),
    };

    match result {
        Ok(_) => {
            let cookie = Cookie::build(ADMIN_SESSION_TOKEN, "")
                .path("/")
                .expires(OffsetDateTime::now_utc())
                .finish();

            (StatusCode::NO_CONTENT, cookies.add(cookie))
        }
        Err(_) => (StatusCode::BAD_REQUEST, CookieJar::new()),
    }
}
