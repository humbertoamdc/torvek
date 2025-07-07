use crate::quotations::models::quotation::Quotation;
use crate::shared::money::Money;
use serde_derive::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct QueryQuotationsForProjectResponse {
    pub quotations: Vec<Quotation>,
}
impl QueryQuotationsForProjectResponse {
    pub const fn new(quotations: Vec<Quotation>) -> Self {
        Self { quotations }
    }
}

#[derive(Deserialize, Serialize, Debug)]
pub struct AdminQueryQuotationsByStatusResponse {
    pub quotations: Vec<Quotation>,
}
impl AdminQueryQuotationsByStatusResponse {
    pub const fn new(quotations: Vec<Quotation>) -> Self {
        Self { quotations }
    }
}

#[derive(Deserialize, Serialize, Debug)]
pub struct GetQuotationSubtotalResponse {
    pub quotation_subtotal: Option<Money>,
}
