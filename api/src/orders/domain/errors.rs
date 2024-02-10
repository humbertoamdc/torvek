#[derive(thiserror::Error, Debug)]
pub enum OrdersError {
    #[error("error while creating orders")]
    CreateOrdersError,
    // #[error("an unexpected error occurred")]
    // UnknownError,
}
