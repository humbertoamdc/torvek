use crate::app_state::AppState;
use crate::auth::models::inputs::{LoginUserInput, RegisterUserInput};
use crate::auth::models::mappers::GetSessionResponseMapper;
use crate::auth::models::session::Role;
use crate::auth::usecases::get_session::GetSession;
use crate::auth::usecases::login::Login;
use crate::auth::usecases::logout::Logout;
use crate::auth::usecases::register::Register;
use crate::shared::error::Error;
use crate::shared::into_error_response::IntoError;
use crate::shared::UseCase;
use axum::extract::State;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::Json;
use axum_extra::extract::cookie::Cookie;
use axum_extra::extract::CookieJar;
use serde_derive::{Deserialize, Serialize};
use time::OffsetDateTime;

pub static CUSTOMER_SESSION_TOKEN: &'static str = "x-customer-session";
pub static ADMIN_SESSION_TOKEN: &'static str = "x-admin-session";

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct RegisterClientRequest {
    pub email: String,
    pub name: String,
    pub password: String,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct LoginUserRequest {
    pub email: String,
    pub password: String,
}

pub async fn register_customer(
    cookies: CookieJar,
    State(app_state): State<AppState>,
    Json(request): Json<RegisterClientRequest>,
) -> impl IntoResponse {
    let input = RegisterUserInput {
        email: request.email,
        name: request.name,
        password: request.password,
        role: Role::Customer,
    };
    let usecase = Register::new(app_state.auth.ory_kratos, app_state.payments.stripe_client);
    let result = usecase.execute(input).await;

    match result {
        Ok(auth_session) => Ok((
            StatusCode::NO_CONTENT,
            cookies.add(
                auth_session
                    .session_cookie(
                        CUSTOMER_SESSION_TOKEN,
                        app_state.env.secure_session_cookie(),
                        app_state.domain,
                    )
                    .into_owned(),
            ),
        )),
        Err(err) => Err(err.into_error_response()),
    }
}

pub async fn login(
    cookies: CookieJar,
    State(app_state): State<AppState>,
    Json(request): Json<LoginUserRequest>,
) -> impl IntoResponse {
    let input = LoginUserInput {
        email: request.email,
        password: request.password,
        role: Role::Customer,
    };
    let usecases = Login::new(app_state.auth.ory_kratos);
    let result = usecases.execute(input).await;

    match result {
        Ok(auth_session) => Ok((
            StatusCode::OK,
            cookies.add(
                auth_session
                    .session_cookie(
                        CUSTOMER_SESSION_TOKEN,
                        app_state.env.secure_session_cookie(),
                        app_state.domain,
                    )
                    .into_owned(),
            ),
        )),
        Err(err) => Err(err.into_error_response()),
    }
}

pub async fn get_session(
    cookies: CookieJar,
    State(app_state): State<AppState>,
) -> impl IntoResponse {
    let usecase = GetSession::new(app_state.auth.ory_kratos);
    let session_cookie = cookies.get(CUSTOMER_SESSION_TOKEN);

    let result = match session_cookie {
        Some(session_cookie) => usecase.execute(session_cookie.value().to_string()).await,
        None => Err(Error::UnknownError),
    };

    match result {
        Ok(session_information) => Ok(http::Response::builder()
            .status(StatusCode::OK)
            .body(
                serde_json::to_string(&GetSessionResponseMapper::to_api(session_information))
                    .unwrap(),
            )
            .unwrap()),
        // TODO: Handle error.
        Err(err) => Err(err.into_error_response()),
    }
}

pub async fn logout(cookies: CookieJar, State(app_state): State<AppState>) -> impl IntoResponse {
    let usecase = Logout::new(app_state.auth.ory_kratos);
    let session_cookie = cookies.get(CUSTOMER_SESSION_TOKEN);

    let result = match session_cookie {
        Some(session_cookie) => usecase.execute(session_cookie.value().to_string()).await,
        // TODO: Handle error.
        None => Err(Error::UnknownError),
    };

    match result {
        Ok(_) => {
            let cookie = Cookie::build((CUSTOMER_SESSION_TOKEN, ""))
                .path("/")
                .expires(OffsetDateTime::now_utc())
                .build();

            Ok((StatusCode::NO_CONTENT, cookies.add(cookie)))
        }
        Err(err) => Err(err.into_error_response()),
    }
}

pub async fn admin_login(
    cookies: CookieJar,
    State(app_state): State<AppState>,
    Json(request): Json<LoginUserRequest>,
) -> impl IntoResponse {
    let input = LoginUserInput {
        email: request.email,
        password: request.password,
        role: Role::Admin,
    };
    let usecases = Login::new(app_state.auth.ory_kratos);
    let result = usecases.execute(input).await;

    match result {
        Ok(auth_session) => Ok((
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
        )),
        Err(err) => Err(err.into_error_response()),
    }
}

pub async fn get_admin_session(
    cookies: CookieJar,
    State(app_state): State<AppState>,
) -> impl IntoResponse {
    let usecase = GetSession::new(app_state.auth.ory_kratos);
    let session_cookie = cookies.get(ADMIN_SESSION_TOKEN);

    let result = match session_cookie {
        Some(session_cookie) => usecase.execute(session_cookie.value().to_string()).await,
        None => Err(Error::UnknownError),
    };

    match result {
        Ok(session_information) => Ok(http::Response::builder()
            .status(StatusCode::OK)
            .body(
                serde_json::to_string(&GetSessionResponseMapper::to_api(session_information))
                    .unwrap(),
            )
            .unwrap()),
        // TODO: Handle error.
        Err(err) => Err(err.into_error_response()),
    }
}

pub async fn admin_logout(
    cookies: CookieJar,
    State(app_state): State<AppState>,
) -> impl IntoResponse {
    let usecase = Logout::new(app_state.auth.ory_kratos);
    let session_cookie = cookies.get(ADMIN_SESSION_TOKEN);

    let result = match session_cookie {
        Some(session_cookie) => usecase.execute(session_cookie.value().to_string()).await,
        // TODO: Handle error.
        None => Err(Error::UnknownError),
    };

    match result {
        Ok(_) => {
            let cookie = Cookie::build((ADMIN_SESSION_TOKEN, ""))
                .path("/")
                .expires(OffsetDateTime::now_utc())
                .build();

            Ok((StatusCode::NO_CONTENT, cookies.add(cookie)))
        }
        Err(err) => Err(err.into_error_response()),
    }
}
