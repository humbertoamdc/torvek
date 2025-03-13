use std::env;
use std::net::SocketAddr;

use aws_config::BehaviorVersion;
use axum::Router;
use axum_test::TestServer;
use http::header::{CONTENT_TYPE, ORIGIN};
use http::{HeaderValue, Method};
use tower_http::compression::CompressionLayer;
use tower_http::cors::{AllowHeaders, AllowMethods, AllowOrigin, CorsLayer};

use crate::app_state::AppState;
use crate::config::Config;

mod app_state;
mod auth;
mod config;
mod orders;
mod parts;
mod payments;
mod projects;
mod quotations;
mod repositories;
mod services;
mod shared;
mod utils;

const CONFIG_BUCKET_STAGING: &str = "torvek-config-files-staging";
const CONFIG_KEY_STAGING: &str = "staging.toml";

const CONFIG_BUCKET_PROD: &str = "torvek-config-files";
const CONFIG_KEY_PROD: &str = "prod.toml";

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    match env::var("RUN_MODE")
        .unwrap_or(String::from("local"))
        .as_str()
    {
        "lambda" => run_lambda().await,
        _ => run_local().await,
    };
}

fn create_base_router() -> Router<AppState> {
    Router::new()
        .nest("/api/v1", auth::routes::create_router())
        .nest("/api/v1", orders::routes::create_router())
        .nest("/api/v1", projects::routes::create_router())
        .nest("/api/v1", quotations::routes::create_router())
        .nest("/api/v1", parts::routes::create_router())
        .nest("/api/v1", payments::routes::create_router())
        .layer(CompressionLayer::new().gzip(true).deflate(true))
}

async fn run_local() {
    let (app, app_config) = create_local_app().await;

    // Set up CORS
    let origins = [
        format!("http://{}:8080", app_config.app.domain)
            .parse::<HeaderValue>()
            .unwrap(),
        format!("http://{}:8081", app_config.app.domain)
            .parse::<HeaderValue>()
            .unwrap(),
        format!("http://{}:8082", app_config.app.domain)
            .parse::<HeaderValue>()
            .unwrap(),
    ];
    let cors_layer = CorsLayer::new()
        .allow_headers::<AllowHeaders>([CONTENT_TYPE, ORIGIN].into())
        .allow_methods::<AllowMethods>(
            [
                Method::GET,
                Method::POST,
                Method::PATCH,
                Method::PUT,
                Method::DELETE,
            ]
            .into(),
        )
        .allow_credentials(true)
        .allow_origin::<AllowOrigin>(origins.into());

    let app = app.layer::<CorsLayer>(cors_layer.into());

    // Run
    let addr = SocketAddr::from(([127, 0, 0, 1], app_config.app.port));
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    tracing::info!("listening on {addr}");
    axum::serve(
        listener,
        app.into_make_service_with_connect_info::<SocketAddr>(),
    )
    .await
    .unwrap()
}

async fn run_lambda() {
    let app = create_lambda_app().await;

    let app = tower::ServiceBuilder::new()
        .layer(axum_aws_lambda::LambdaLayer::default())
        .service(app);

    let _ = lambda_http::run(app).await;
}

async fn create_app_from_config(config: &Config) -> Router {
    let app_state = AppState::from(config).await;
    create_base_router().with_state(app_state)
}

async fn create_local_app() -> (Router, Config) {
    let config_path = "./env/dev.toml";
    let config_string = std::fs::read_to_string(config_path).expect("could not find config file");
    let app_config = Config::from(config_string.as_str());

    let app = create_app_from_config(&app_config).await;
    (app, app_config)
}

async fn create_lambda_app() -> Router {
    let (config_bucket, config_key) = match env::var("ENV").unwrap_or(String::from("prod")).as_str()
    {
        "prod" => (CONFIG_BUCKET_PROD, CONFIG_KEY_PROD),
        _ => (CONFIG_BUCKET_STAGING, CONFIG_KEY_STAGING),
    };

    let shared_config = aws_config::defaults(BehaviorVersion::latest()).load().await;
    let s3_config = aws_sdk_s3::config::Builder::from(&shared_config).build();
    let s3_client = aws_sdk_s3::Client::from_conf(s3_config);
    let result = s3_client
        .get_object()
        .bucket(config_bucket)
        .key(config_key)
        .send()
        .await
        .expect("error getting config from S3");
    let bytes = result
        .body
        .collect()
        .await
        .expect("error parsing body")
        .into_bytes();

    let config_string = std::str::from_utf8(&bytes).expect("error parsing body");
    let app_config = Config::from(config_string);

    create_app_from_config(&app_config).await
}

#[cfg(test)]
async fn new_test_app() -> TestServer {
    let (app, app_config) = create_test_app().await;

    // Set up CORS for tests
    let origins = [
        format!("http://{}:8080", app_config.app.domain)
            .parse::<HeaderValue>()
            .unwrap(),
        String::from("http://localhost")
            .parse::<HeaderValue>()
            .unwrap(),
    ];

    let cors_layer = CorsLayer::new()
        .allow_headers::<AllowHeaders>([CONTENT_TYPE, ORIGIN].into())
        .allow_methods::<AllowMethods>(
            [
                Method::GET,
                Method::POST,
                Method::PATCH,
                Method::PUT,
                Method::DELETE,
            ]
            .into(),
        )
        .allow_credentials(true)
        .allow_origin::<AllowOrigin>(origins.into());

    let app = app.layer::<CorsLayer>(cors_layer.into());

    TestServer::builder()
        .save_cookies()
        .expect_success_by_default()
        .mock_transport()
        .build(app)
        .unwrap()
}

#[cfg(test)]
async fn create_test_app() -> (Router, Config) {
    tracing_subscriber::fmt::init();

    let config_path = "./env/test.toml";
    let config_string = std::fs::read_to_string(config_path).expect("could not find config file");
    let app_config = Config::from(config_string.as_str());

    let app = create_app_from_config(&app_config).await;
    (app, app_config)
}

#[cfg(test)]
mod tests {
    use crate::auth::models::requests::RegisterClientRequest;
    use crate::new_test_app;
    use http::StatusCode;

    #[tokio::test]
    async fn test_register() {
        let server = new_test_app().await;

        // Create registration request
        let register_request = RegisterClientRequest {
            email: String::from("testd632fddfsd7263454@example.com"),
            password: String::from("password"), // More complex password to pass validation
            name: String::from("Test Name"),
        };

        let response = server
            .post("/api/v1/register")
            .json(&register_request)
            .await;

        assert_eq!(response.status_code(), StatusCode::NO_CONTENT);
        assert!(
            response.maybe_cookie("customer_session_token").is_some(),
            "Expected session cookie to be set"
        );
    }
}
