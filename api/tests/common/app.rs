use api::app::{cors_layer, create_app_from_config};
use api::config::Config;
use axum::Router;
use axum_test::TestServer;
use http::HeaderValue;
use std::sync::Once;
use tower_http::cors::CorsLayer;

static TRACING_INIT: Once = Once::new();

pub async fn init_test_server() -> TestServer {
    let (app, app_config) = create_test_app().await;

    // Set up CORS for tests
    let origins = vec![
        format!("http://{}:8080", app_config.app.domain)
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

async fn create_test_app() -> (Router, Config) {
    // Initialize tracing exactly once
    TRACING_INIT.call_once(|| {
        tracing_subscriber::fmt::init();
    });

    let config_path = "./env/test.toml";
    let config_string = std::fs::read_to_string(config_path).expect("could not find config file");
    let app_config = Config::from(config_string.as_str());

    let app = create_app_from_config(&app_config).await;
    (app, app_config)
}
