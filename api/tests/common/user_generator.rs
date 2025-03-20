use crate::common::app::get_app_state;
use api::auth::models::requests::RegisterClientRequest;

pub struct TestUser {
    pub email: String,
    pub password: String,
    pub session_token: String,
}

pub async fn generate_customer() -> TestUser {
    let id = uuid::Uuid::new_v4();
    let email = format!("{id:?}@test.com");
    let password = String::from("password");

    // Create registration request
    let register_request = RegisterClientRequest {
        email: email.clone(),
        password: password.clone(),
        name: String::from("Test Name"),
    };

    let app_state = get_app_state().await;
    let session_token = app_state
        .auth
        .identity_manager
        .register_user(register_request)
        .await
        .expect("Failed to generate user")
        .session_token;

    TestUser {
        email,
        password,
        session_token,
    }
}
