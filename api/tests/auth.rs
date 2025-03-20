mod common;

mod register_customer {
    use crate::common::app::init_test_server;
    use api::auth::models::requests::RegisterClientRequest;
    use http::StatusCode;

    #[tokio::test]
    async fn it_should_create_session_on_registration() {
        let server = init_test_server().await;

        let id = uuid::Uuid::new_v4();
        let register_request = RegisterClientRequest {
            email: format!("{:?}@test.com", id),
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
}

mod customer_login {
    use crate::common::app::init_test_server;
    use crate::common::user_generator::generate_customer;
    use api::auth::models::requests::LoginClientRequest;
    use http::StatusCode;

    #[tokio::test]
    async fn it_should_create_session_on_login() {
        let server = init_test_server().await;

        let user = generate_customer().await;
        let login_request = LoginClientRequest {
            email: user.email,
            password: user.password,
        };

        let response = server.post("/api/v1/login").json(&login_request).await;

        assert_eq!(response.status_code(), StatusCode::OK);
        assert!(
            response.maybe_cookie("customer_session_token").is_some(),
            "Expected session cookie to be set"
        );
    }
}

mod get_active_session {
    use crate::common::app::init_test_server;
    use crate::common::user_generator::generate_customer;
    use api::auth::models::responses::GetSessionResponse;
    use cookie::Cookie;

    #[tokio::test]
    async fn it_should_get_active_session() {
        let mut server = init_test_server().await;

        let user = generate_customer().await;

        server.add_cookie(Cookie::new(
            "customer_session_token",
            user.session_token.clone(),
        ));

        let session = server
            .get("/api/v1/session")
            .await
            .json::<GetSessionResponse>();

        assert_eq!(session.email, user.email,);
    }
}

mod customer_logout {
    use crate::common::app::init_test_server;
    use crate::common::user_generator::generate_customer;
    use cookie::Cookie;
    use http::StatusCode;

    #[tokio::test]
    async fn it_should_logout_user_and_unset_the_session_token() {
        let mut server = init_test_server().await;

        let user = generate_customer().await;

        server.add_cookie(Cookie::new(
            "customer_session_token",
            user.session_token.clone(),
        ));

        let response = server.post("/api/v1/logout").await;

        assert_eq!(response.status_code(), StatusCode::NO_CONTENT);
        assert!(
            response
                .maybe_cookie("customer_session_token")
                .unwrap()
                .value()
                .is_empty(),
            "Expected session cookie to be empty"
        );
    }
}
