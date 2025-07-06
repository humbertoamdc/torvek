use crate::parts::models::part::Part;
use crate::services::stripe_client::{Quote, QuoteLineItem, StripeClient, StripeQuote};
use crate::shared;
use api_boundary::common::error::Error;
use async_trait::async_trait;
use axum::body::Bytes;
use iso_currency::Currency;
use shared::Result;
use stripe::{
    CheckoutSession, CheckoutSessionBillingAddressCollection, CheckoutSessionMode, Client,
    CreateCheckoutSession, CreateCheckoutSessionLineItems, CreateCheckoutSessionLineItemsPriceData,
    CreateCheckoutSessionLineItemsPriceDataProductData,
    CreateCheckoutSessionShippingAddressCollection,
    CreateCheckoutSessionShippingAddressCollectionAllowedCountries, CreateCustomer, CreateProduct,
    Customer, Product,
};

const CUSTOMER_ID: &'static str = "customer_id";
const PROJECT_ID: &'static str = "project_id";
const QUOTATION_ID: &'static str = "quotation_id";

#[derive(Clone)]
pub struct Stripe {
    client: Client,
    files_client: reqwest::Client,
    success_url: String,
}

impl Stripe {
    pub fn new(client: Client, files_client: reqwest::Client, success_url: String) -> Self {
        Self {
            client,
            files_client,
            success_url,
        }
    }
}

#[async_trait]
impl StripeClient for Stripe {
    async fn create_customer(&self, name: String, email: String) -> Result<Customer> {
        let mut create_customer = CreateCustomer::new();
        create_customer.name = Some(&name);
        create_customer.email = Some(&email);

        let result = Customer::create(&self.client, create_customer).await;

        match result {
            Ok(customer) => Ok(customer),
            Err(err) => {
                tracing::error!("Failed to create stripe customer: {}", err);
                Err(Error::UnknownError)
            }
        }
    }

    async fn create_product(&self, name: String, id: String) -> Result<()> {
        let mut create_product = CreateProduct::new(&name);
        create_product.id = Some(&id);

        let result = Product::create(&self.client, create_product).await;

        match result {
            Ok(_) => Ok(()),
            Err(err) => {
                tracing::error!("Failed to create stripe product: {}", err);
                Err(Error::UnknownError)
            }
        }
    }

    async fn create_quote(
        &self,
        stripe_customer_id: String,
        line_items: Vec<QuoteLineItem>,
    ) -> Result<StripeQuote> {
        let quote = Quote {
            customer: stripe_customer_id,
            line_items,
        };

        let response = self
            .client
            .post_form::<StripeQuote, Quote>("/quotes", quote)
            .await;

        match response {
            Ok(response) => Ok(response),
            Err(err) => {
                tracing::error!("Failed to create stripe quote: {}", err);
                Err(Error::UnknownError)
            }
        }
    }

    async fn finalize_quote(&self, stripe_quote_id: String) -> Result<()> {
        let path = format!("quotes/{stripe_quote_id}/finalize");
        let response = self.client.post::<StripeQuote>(&path).await;

        match response {
            Ok(_) => Ok(()),
            Err(err) => {
                tracing::error!("Failed to finalize stripe quote: {}", err);
                Err(Error::UnknownError)
            }
        }
    }

    async fn download_quote_pdf(&self, stripe_quote_id: String) -> Result<Bytes> {
        let url = format!("https://files.stripe.com/v1/quotes/{stripe_quote_id}/pdf");
        let result = self.files_client.get(&url).send().await;

        match result {
            Ok(response) => match response.bytes().await {
                Ok(bytes) => Ok(bytes),
                Err(err) => {
                    tracing::error!("Error parsing pdf file to bytes: {}", err);
                    Err(Error::UnknownError)
                }
            },
            Err(err) => {
                tracing::error!("Failed to download stripe quote: {}", err);
                Err(Error::UnknownError)
            }
        }
    }

    async fn create_checkout_session(
        &self,
        customer_id: String,
        project_id: String,
        quotation_id: String,
        parts: Vec<Part>,
    ) -> Result<String> {
        let line_items = Self::line_items_from_parts_data(&parts);
        let success_url = format!("{}/orders", self.success_url,);

        let mut params = CreateCheckoutSession::new();
        params.line_items = Some(line_items);
        params.success_url = Some(&success_url);
        params.mode = Some(CheckoutSessionMode::Payment);
        params.billing_address_collection = Some(CheckoutSessionBillingAddressCollection::Required);
        params.shipping_address_collection = Some(CreateCheckoutSessionShippingAddressCollection {
            allowed_countries: vec![
                CreateCheckoutSessionShippingAddressCollectionAllowedCountries::Mx,
            ],
        });
        let metadata = stripe::Metadata::from([
            (String::from(CUSTOMER_ID), customer_id),
            (String::from(PROJECT_ID), project_id),
            (String::from(QUOTATION_ID), quotation_id),
        ]);
        params.metadata = Some(metadata);

        let result = CheckoutSession::create(&self.client, params).await;

        match result {
            Ok(checkout_session) => Ok(checkout_session.url.unwrap()),
            Err(err) => {
                tracing::error!("{err:?}");
                Err(Error::UnknownError)
            }
        }
    }
}

impl Stripe {
    fn line_items_from_parts_data(parts: &Vec<Part>) -> Vec<CreateCheckoutSessionLineItems> {
        parts
            .iter()
            .map(|part| {
                let selected_part_quote = part
                    .part_quotes
                    .clone()
                    .expect("expecting part quotes")
                    .into_iter()
                    .find(|part_quote| {
                        part_quote.id == part.selected_part_quote_id.clone().unwrap()
                    })
                    .expect("could not find a selected quote for part");

                CreateCheckoutSessionLineItems {
                    adjustable_quantity: None,
                    dynamic_tax_rates: None,
                    price: None,
                    price_data: Some(CreateCheckoutSessionLineItemsPriceData {
                        currency: match selected_part_quote.sub_total.currency.clone() {
                            Currency::MXN => stripe::Currency::MXN,
                            Currency::USD => stripe::Currency::USD,
                            _ => stripe::Currency::USD,
                        },
                        product: None,
                        product_data: Some(CreateCheckoutSessionLineItemsPriceDataProductData {
                            description: Some(part.attributes.to_string()),
                            images: None,
                            metadata: None,
                            name: part.model_file.name.clone(),
                            tax_code: None,
                        }),
                        recurring: None,
                        tax_behavior: None,
                        unit_amount: Some(selected_part_quote.unit_price.amount),
                        unit_amount_decimal: None,
                    }),
                    quantity: Some(part.quantity),
                    tax_rates: None,
                }
            })
            .collect()
    }
}
