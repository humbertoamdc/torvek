#[derive(thiserror::Error, Debug)]
pub enum AuthError {
    #[error("email is already in use")]
    EmailTakenRegistrationError,
    #[error("invalid credentials")]
    InvalidCredentialsLoginError,
    #[error("error while initializing login flow")]
    InitializingLoginFlowError,
    #[error("error while initializing registration flow")]
    InitializingRegistrationFlowError,
    #[error("password has been found in data breaches")]
    BreachedPasswordRegistrationError,
    #[error("an unexpected error occurred")]
    UnknownError,
}
