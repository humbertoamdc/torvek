#[derive(thiserror::Error, Debug)]
pub enum OrdersError {
    #[error("error while querying orders")]
    QueryOrdersError,
    #[error("an unexpected error occurred")]
    UnknownError,
}
