use crate::app_state::AppState;
use crate::config::Config;
use crate::{auth, orders, parts, payments, projects, quotations};
use aws_config::BehaviorVersion;
use axum::Router;
use http::header::{CONTENT_TYPE, ORIGIN};
use http::{HeaderValue, Method};
use std::env;
use tower_http::compression::CompressionLayer;
use tower_http::cors::{AllowHeaders, AllowMethods, AllowOrigin, CorsLayer};

const CONFIG_BUCKET_STAGING: &str = "torvek-config-files-staging";
const CONFIG_KEY_STAGING: &str = "staging.toml";

const CONFIG_BUCKET_PROD: &str = "torvek-config-files";
const CONFIG_KEY_PROD: &str = "prod.toml";

pub async fn create_local_app() -> (Router, Config) {
    let config_path = "./env/dev.toml";
    let config_string = std::fs::read_to_string(config_path).expect("could not find config file");
    let app_config = Config::from(config_string.as_str());

    let app = create_app_from_config(&app_config).await;

    // Set up CORS
    let origins = vec![
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
    let cors_layer = cors_layer(origins);

    let app = app.layer::<CorsLayer>(cors_layer);

    (app, app_config)
}

pub async fn create_lambda_app() -> Router {
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

pub async fn create_app_from_config(config: &Config) -> Router {
    let app_state = AppState::from(config).await;
    create_base_router().with_state(app_state)
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

pub fn cors_layer(origins: Vec<HeaderValue>) -> CorsLayer {
    CorsLayer::new()
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
        .allow_origin::<AllowOrigin>(origins.into())
}
