use crate::common::app::get_app_state;
use api::auth::models::inputs::RegisterUserInput;
use api::auth::models::session::Role;
use api::auth::usecases::register::Register;
use api::shared::UseCase;

pub struct TestUser {
    pub email: String,
    pub password: String,
    pub session_token: String,
}

impl TestUser {
    pub async fn new() -> Self {
        let id = uuid::Uuid::new_v4();
        let email = format!("{id:?}@test.com");
        let password = String::from("password");

        let register_request = RegisterUserInput {
            email: email.clone(),
            password: password.clone(),
            name: String::from("Test Name"),
            role: Role::Customer,
        };

        let app_state = get_app_state().await;
        let register_client_usecase = Register::new(
            app_state.auth.ory_kratos.clone(),
            app_state.payments.stripe_client.clone(),
        );
        let session_with_token = register_client_usecase
            .execute(register_request)
            .await
            .expect("Failed to register user.");

        Self {
            email,
            password,
            session_token: session_with_token.session_token,
        }
    }
}
