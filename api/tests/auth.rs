mod common;

use crate::common::app::init_test_server;
use api::auth::models::requests::RegisterClientRequest;
use http::StatusCode;

static ID: std::sync::LazyLock<uuid::Uuid> = std::sync::LazyLock::new(uuid::Uuid::new_v4);

#[tokio::test]
async fn test_register() {
    let server = init_test_server().await;

    // Create registration request
    let register_request = RegisterClientRequest {
        email: format!("{:?}@test.com", &*ID),
        password: String::from("password"),
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
