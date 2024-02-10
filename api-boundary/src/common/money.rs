use iso_currency::Currency;
use serde_derive::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct Money {
    pub amount: i64,
    pub currency: Currency,
}

impl Money {
    pub const fn new(amount: i64, currency: Currency) -> Self {
        Self { amount, currency }
    }
}

impl Default for Money {
    fn default() -> Self {
        Self {
            amount: 0,
            currency: Currency::MXN,
        }
    }
}
