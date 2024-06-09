use std::collections::HashMap;
use stripe::{
    CheckoutSession, CheckoutSessionMode, Client, CreateCheckoutSession,
    CreateCheckoutSessionLineItems, CreateCheckoutSessionLineItemsPriceData,
    CreateCheckoutSessionLineItemsPriceDataProductData, Currency,
};

use api_boundary::parts::models::{Part, PartQuote};
use api_boundary::payments::errors::PaymentsError;

const CLIENT_ID: &'static str = "client_id";
const PROJECT_ID: &'static str = "project_id";
const QUOTATION_ID: &'static str = "quotation_id";

#[derive(Clone)]
pub struct StripePaymentsProcessor {
    client: Client,
    success_url: String,
}

impl StripePaymentsProcessor {
    pub fn new(client: Client, success_url: String) -> Self {
        Self {
            client,
            success_url,
        }
    }

    pub async fn create_checkout_session(
        &self,
        client_id: String,
        project_id: String,
        quotation_id: String,
        parts: Vec<Part>,
        selected_quote_per_part: HashMap<String, PartQuote>,
    ) -> Result<String, PaymentsError> {
        let line_items = Self::line_items_from_parts_data(&parts, selected_quote_per_part);
        let success_url = format!(
            "{}/projects/{}/quotations/{}/parts",
            self.success_url, project_id, quotation_id
        );

        let mut params = CreateCheckoutSession::new();
        params.line_items = Some(line_items);
        params.success_url = Some(&success_url);
        params.mode = Some(CheckoutSessionMode::Payment);
        let metadata = stripe::Metadata::from([
            (String::from(CLIENT_ID), client_id),
            (String::from(PROJECT_ID), project_id),
            (String::from(QUOTATION_ID), quotation_id),
        ]);
        params.metadata = Some(metadata);

        let result = CheckoutSession::create(&self.client, params).await;

        match result {
            Ok(checkout_session) => Ok(checkout_session.url.unwrap()),
            Err(err) => {
                log::error!("{err:?}");
                Err(PaymentsError::UnknownError)
            }
        }
    }

    fn line_items_from_parts_data(
        parts: &Vec<Part>,
        selected_quote_per_part: HashMap<String, PartQuote>,
    ) -> Vec<CreateCheckoutSessionLineItems> {
        parts.iter().map(|part| CreateCheckoutSessionLineItems {
            adjustable_quantity: None,
            dynamic_tax_rates: None,
            price: None,
            price_data: Some(CreateCheckoutSessionLineItemsPriceData {
                currency: Currency::MXN,
                product: None,
                product_data: Some(CreateCheckoutSessionLineItemsPriceDataProductData {
                    description: Some(format!(
                        "Process: {} / Material: {} / Tolerance: {}",
                        part.process, part.material, part.tolerance
                    )),
                    images: Some(vec![
                        "https://cdn.dribbble.com/userupload/11259598/file/original-70a5fe9cc326f004bb78e36ee5e9d8a7.png?resize=100x".to_string()
                    ]),
                    metadata: None,
                    name: part.model_file.name.clone(),
                    tax_code: None,
                }),
                recurring: None,
                tax_behavior: None,
                unit_amount: Some(selected_quote_per_part[&part.id].unit_price.amount),
                unit_amount_decimal: None,
            }),
            quantity: Some(part.quantity),
            tax_rates: None,
        }).collect()
    }
}
