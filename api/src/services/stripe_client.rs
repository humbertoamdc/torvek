use crate::shared;
use api_boundary::parts::models::Part;
use async_trait::async_trait;
use axum::body::Bytes;
use serde_derive::{Deserialize, Serialize};
use shared::Result;
use stripe::{Currency, Customer};

#[async_trait]
pub trait StripeClient: Send + Sync + 'static {
    async fn create_customer(&self, name: String, email: String) -> Result<Customer>;
    async fn create_product(&self, name: String, id: String) -> Result<()>;
    async fn create_quote(
        &self,
        stripe_customer_id: String,
        line_items: Vec<QuoteLineItem>,
    ) -> Result<StripeQuote>;
    async fn finalize_quote(&self, stripe_quote_id: String) -> Result<()>;
    async fn download_quote_pdf(&self, stripe_quote_id: String) -> Result<Bytes>;
    async fn create_checkout_session(
        &self,
        customer_id: String,
        project_id: String,
        quotation_id: String,
        parts: Vec<Part>,
    ) -> Result<String>;
}

#[derive(Deserialize, Serialize, Debug)]
pub struct StripeQuote {
    pub id: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Quote {
    /// Stipe's customer id.
    pub customer: String,
    pub line_items: Vec<QuoteLineItem>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct QuoteLineItem {
    pub price_data: PriceData,
    pub quantity: u64,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct PriceData {
    pub currency: Currency,
    /// Stripe's product id which is the same as the part id.
    pub product: String,
    pub unit_amount: u64,
}
