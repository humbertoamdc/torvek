use std::env;
use std::net::SocketAddr;

use aws_config::BehaviorVersion;
use axum::Router;
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

mod shared;

#[tokio::main]
async fn main() {
    env_logger::init();

    let app = Router::new()
        .nest("/api/v1", auth::routes::create_router())
        .nest("/api/v1", orders::routes::create_router())
        .nest("/api/v1", projects::routes::create_router())
        .nest("/api/v1", quotations::routes::create_router())
        .nest("/api/v1", parts::routes::create_router())
        .nest("/api/v1", payments::routes::create_router())
        .layer(CompressionLayer::new().gzip(true).deflate(true));

    match env::var("RUN_MODE")
        .unwrap_or(String::from("local"))
        .as_str()
    {
        "lambda" => run_lambda(app).await,
        _ => run_local(app).await,
    };
}

async fn run_local(app: Router<AppState>) {
    // Constants
    let config_path = "./env/dev.toml";

    // Parse config
    let config_string = std::fs::read_to_string(config_path).expect("could not find config file");
    let app_config = Config::from(config_string.as_str());
    let app_state = AppState::from(&app_config).await;

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
            [Method::GET, Method::POST, Method::PATCH, Method::PUT].into(),
        )
        .allow_credentials(true)
        .allow_origin::<AllowOrigin>(origins.into());

    // Setup
    let app = app
        .layer::<CorsLayer>(cors_layer.into())
        .with_state(app_state);

    // Run
    let addr = SocketAddr::from(([127, 0, 0, 1], app_config.app.port));
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    log::info!("listening on {addr}");
    axum::serve(listener, app).await.unwrap()
}

async fn run_lambda(app: Router<AppState>) {
    // Constants
    let config_bucket = "unnamed-config-files";
    let config_key = "prod.toml";

    // Retrieve config from S3
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

    // Parse config
    let config_string = std::str::from_utf8(&bytes).expect("error parsing body");
    let app_config = Config::from(config_string);
    let app_state = AppState::from(&app_config).await;

    // Setup
    let app = tower::ServiceBuilder::new()
        .layer(axum_aws_lambda::LambdaLayer::default())
        .service(app.with_state(app_state));

    // Run
    let _ = lambda_http::run(app).await;
}
