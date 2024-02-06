use iso_currency::Currency;
use serde_derive::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct Money {
    amount: i64,
    currency: Currency,
}

impl Money {
    pub const fn new(amount: i64, currency: Currency) -> Self {
        Self { amount, currency }
    }
}
