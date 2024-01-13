use crate::quotations::models::Quotation;
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
