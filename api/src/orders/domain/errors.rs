#[derive(thiserror::Error, Debug)]
pub enum OrdersError {
    #[error("error while generating signed url")]
    PresignedUrlGenerationError,
    #[error("error while querying orders from the database")]
    QueryOrdersError,
    #[error("error while writing orders to the database")]
    OrdersBatchCreateError,
    #[error("error while updating order")]
    UpdateOrderError,
    #[error("error parsing order item from db item")]
    ConversionError,
    // #[error("an unexpected error occurred")]
    // UnknownError,
}
