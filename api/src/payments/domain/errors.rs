#[derive(thiserror::Error, Debug)]
pub enum PaymentsError {
    #[error("error while querying parts for quotation")]
    QueryPartsError,
    #[error("error while creating orders and confirming quotation payment transaction")]
    CreateOrdersAndConfirmQuotationPaymentTransactionError,
}
