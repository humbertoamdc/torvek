#[derive(thiserror::Error, Debug)]
pub enum QuotationsError {
    #[error("error while creating quotation")]
    CreateQuotationError,
    #[error("error while querying quotations")]
    QueryQuotationsError,
    #[error("an unexpected error occurred")]
    UnknownError,
}
