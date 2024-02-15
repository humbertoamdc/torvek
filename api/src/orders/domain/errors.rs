#[derive(thiserror::Error, Debug)]
pub enum OrdersError {
    #[error("error while querying orders")]
    QueryOrdersError,
    #[error("error while updating payout for order")]
    UpdateOrderPayoutError,
    #[error("an unexpected error occurred")]
    UnknownError,
}
