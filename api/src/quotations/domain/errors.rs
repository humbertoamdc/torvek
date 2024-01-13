#[derive(thiserror::Error, Debug)]
pub enum QuotationsError {
    #[error("error while creating quotation")]
    CreateQuotationError,
}
