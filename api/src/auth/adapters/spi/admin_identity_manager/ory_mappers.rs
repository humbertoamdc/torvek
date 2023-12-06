use ory_kratos_client::models::UpdateLoginFlowBody::UpdateLoginFlowWithPasswordMethod;
use ory_kratos_client::models::{PerformNativeLogoutBody, UpdateLoginFlowBody};

use crate::auth::adapters::api::requests::AdminLoginRequest;

pub struct OryAdminLoginRequestMapper {}

impl OryAdminLoginRequestMapper {
    pub fn api_to_spi(entity: AdminLoginRequest) -> UpdateLoginFlowBody {
        UpdateLoginFlowWithPasswordMethod {
            csrf_token: Some(entity.csrf_token),
            identifier: entity.email,
            password: entity.password,
            password_identifier: None,
        }
    }
}

pub struct OryAdminLogoutRequestMapper {}

impl OryAdminLogoutRequestMapper {
    pub fn api_to_spi(session_token: String) -> PerformNativeLogoutBody {
        PerformNativeLogoutBody { session_token }
    }
}
