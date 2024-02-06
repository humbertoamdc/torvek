#[derive(thiserror::Error, Debug)]
pub enum OrdersError {
    #[error("error while creating order")]
    CreateOrderError,
    // #[error("an unexpected error occurred")]
    // UnknownError,
}
