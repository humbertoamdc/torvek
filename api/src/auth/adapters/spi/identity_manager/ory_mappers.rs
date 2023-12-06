use ory_kratos_client::models::UpdateLoginFlowBody::UpdateLoginFlowWithPasswordMethod;
use ory_kratos_client::models::UpdateRegistrationFlowBody::UpdateRegistrationFlowWithPasswordMethod;
use ory_kratos_client::models::{
    PerformNativeLogoutBody, UpdateLoginFlowBody, UpdateRegistrationFlowBody,
};
use serde_json::json;

use crate::auth::adapters::api::requests::{LoginClientRequest, RegisterClientRequest};

pub struct OryLoginRequestMapper {}

impl OryLoginRequestMapper {
    pub fn api_to_spi(entity: LoginClientRequest) -> UpdateLoginFlowBody {
        UpdateLoginFlowWithPasswordMethod {
            csrf_token: None,
            identifier: entity.email,
            password: entity.password,
            password_identifier: None,
        }
    }
}

pub struct OryRegisterRequestMapper {}

impl OryRegisterRequestMapper {
    pub fn api_to_spi(entity: RegisterClientRequest) -> UpdateRegistrationFlowBody {
        UpdateRegistrationFlowWithPasswordMethod {
            csrf_token: None,
            password: entity.password,
            traits: json!({"email": entity.email }),
            transient_payload: None,
        }
    }
}

pub struct OryLogoutRequestMapper {}

impl OryLogoutRequestMapper {
    pub fn api_to_spi(session_token: String) -> PerformNativeLogoutBody {
        PerformNativeLogoutBody { session_token }
    }
}
