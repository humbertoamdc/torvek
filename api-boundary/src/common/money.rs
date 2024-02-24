use std::fmt::Display;

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

impl Display for Money {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let currency = match self.currency {
            Currency::USD => rusty_money::iso::USD,
            _ => rusty_money::iso::MXN,
        };
        write!(
            f,
            "{}",
            rusty_money::Money::from_minor(self.amount, currency).to_string()
        )
    }
}
