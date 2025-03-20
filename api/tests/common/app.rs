use api::app::{cors_layer, create_app_from_config};
use api::app_state::AppState;
use api::config::Config;
use axum::Router;
use axum_test::TestServer;
use http::HeaderValue;
use std::env;
use std::sync::Once;
use tokio::sync::OnceCell;
use tower_http::cors::CorsLayer;

static TRACING_INIT: Once = Once::new();
static APP_STATE: OnceCell<AppState> = OnceCell::const_new();
static APP_CONFIG: std::sync::LazyLock<Config> = std::sync::LazyLock::new(|| {
    let config_path = "./env/test.toml";
    let config_string = std::fs::read_to_string(config_path).expect("could not find config file");

    Config::from(config_string.as_str())
});

pub async fn get_app_state() -> &'static AppState {
    APP_STATE
        .get_or_init(|| async { AppState::from(&APP_CONFIG).await })
        .await
}

pub async fn init_test_server() -> TestServer {
    let app = create_test_app().await;

    // Set up CORS for tests
    let origins = vec![
        format!("http://{}:8080", APP_CONFIG.app.domain)
            .parse::<HeaderValue>()
            .unwrap(),
        String::from("http://localhost")
            .parse::<HeaderValue>()
            .unwrap(),
    ];

    let cors_layer = cors_layer(origins);

    let app = app.layer::<CorsLayer>(cors_layer);

    TestServer::builder()
        .save_cookies()
        .expect_success_by_default()
        .mock_transport()
        .build(app)
        .unwrap()
}

async fn create_test_app() -> Router {
    // Initialize tracing exactly once
    TRACING_INIT.call_once(|| {
        tracing_subscriber::fmt::init();
    });

    env::set_var("AWS_ENDPOINT_URL", "http://127.0.0.1:4576");
    env::set_var(
        "AWS_S3_ENDPOINT_URL",
        "http://s3.localhost.localstack.cloud:4576",
    );

    create_app_from_config(&APP_CONFIG).await
}
