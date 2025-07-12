use crate::shared::{CustomerId, QuoteId};

pub struct BatchDeleteQuotationObject {
    pub quotation_id: QuoteId,
    pub customer_id: CustomerId,
}
