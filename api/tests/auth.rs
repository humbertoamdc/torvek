#[path = "common/mod.rs"]
mod common;

mod register_customer {
    use crate::common::app::init_test_server;
    use api::auth::controllers::CUSTOMER_SESSION_TOKEN;
    use api::auth::models::inputs::RegisterUserInput;
    use api::auth::models::session::Role;
    use http::StatusCode;

    #[tokio::test]
    async fn it_should_create_session_on_registration() {
        let server = init_test_server().await;

        let id = uuid::Uuid::new_v4();
        let register_request = RegisterUserInput {
            email: format!("{:?}@test.com", id),
            password: String::from("password"),
            name: String::from("Test Name"),
            role: Role::Customer,
        };

        let response = server
            .post("/api/v1/accounts/customers/register")
            .json(&register_request)
            .await;

        assert_eq!(response.status_code(), StatusCode::NO_CONTENT);
        assert!(
            response.maybe_cookie(CUSTOMER_SESSION_TOKEN).is_some(),
            "Expected session cookie to be set"
        );
    }
}

mod customer_login {
    use crate::common::app::init_test_server;
    use crate::common::user_generator::TestUser;
    use api::auth::controllers::CUSTOMER_SESSION_TOKEN;
    use api::auth::models::inputs::LoginUserInput;
    use api::auth::models::session::Role;
    use http::StatusCode;

    #[tokio::test]
    async fn it_should_create_session_on_login() {
        let server = init_test_server().await;

        let user = TestUser::new().await;
        let login_request = LoginUserInput {
            email: user.email,
            password: user.password,
            role: Role::Customer,
        };

        let response = server
            .post("/api/v1/accounts/customers/login")
            .json(&login_request)
            .await;

        assert_eq!(response.status_code(), StatusCode::OK);
        assert!(
            response.maybe_cookie(CUSTOMER_SESSION_TOKEN).is_some(),
            "Expected session cookie to be set"
        );
    }
}

mod get_active_session {
    use crate::common::app::init_test_server;
    use crate::common::user_generator::TestUser;
    use api::auth::controllers::CUSTOMER_SESSION_TOKEN;
    use api::auth::models::responses::GetSessionResponse;
    use cookie::Cookie;

    #[tokio::test]
    async fn it_should_get_active_session() {
        let mut server = init_test_server().await;

        let user = TestUser::new().await;

        server.add_cookie(Cookie::new(CUSTOMER_SESSION_TOKEN, user.session_token));

        let session = server
            .get("/api/v1/accounts/customers/session")
            .await
            .json::<GetSessionResponse>();

        assert_eq!(session.email, user.email,);
    }
}

mod customer_logout {
    use crate::common::app::init_test_server;
    use crate::common::user_generator::TestUser;
    use api::auth::controllers::CUSTOMER_SESSION_TOKEN;
    use cookie::Cookie;
    use http::StatusCode;

    #[tokio::test]
    async fn it_should_logout_user_and_unset_the_session_token() {
        let mut server = init_test_server().await;

        let user = TestUser::new().await;

        server.add_cookie(Cookie::new(CUSTOMER_SESSION_TOKEN, user.session_token));

        let response = server.post("/api/v1/accounts/customers/logout").await;

        assert_eq!(response.status_code(), StatusCode::NO_CONTENT);
        assert!(
            response
                .maybe_cookie(CUSTOMER_SESSION_TOKEN)
                .unwrap()
                .value()
                .is_empty(),
            "Expected session cookie to be empty"
        );
    }
}
