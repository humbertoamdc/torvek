use api::app::{create_lambda_app, create_local_app};
use std::env;
use std::net::SocketAddr;

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

async fn run_local() {
    let (app, app_config) = create_local_app().await;

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
