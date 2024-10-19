use stripe::{
    CheckoutSession, CheckoutSessionBillingAddressCollection, CheckoutSessionMode, Client,
    CreateCheckoutSession, CreateCheckoutSessionLineItems, CreateCheckoutSessionLineItemsPriceData,
    CreateCheckoutSessionLineItemsPriceDataProductData,
    CreateCheckoutSessionShippingAddressCollection,
    CreateCheckoutSessionShippingAddressCollectionAllowedCountries, Currency,
};

use api_boundary::parts::models::Part;
use api_boundary::payments::errors::PaymentsError;

const CUSTOMER_ID: &'static str = "customer_id";
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
        customer_id: String,
        project_id: String,
        quotation_id: String,
        parts: Vec<Part>,
    ) -> Result<String, PaymentsError> {
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
                log::error!("{err:?}");
                Err(PaymentsError::UnknownError)
            }
        }
    }

    fn line_items_from_parts_data(parts: &Vec<Part>) -> Vec<CreateCheckoutSessionLineItems> {
        parts.iter().map(|part| {
            let selected_part_quote = part
                .part_quotes
                .clone()
                .expect("expecting part quotes")
                .into_iter()
                .find(|part_quote| part_quote.id == part.selected_part_quote_id.clone().unwrap())
                .expect("could not find a selected quote for part");

            CreateCheckoutSessionLineItems {
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
                    unit_amount: Some(selected_part_quote.unit_price.amount),
                    unit_amount_decimal: None,
                }),
                quantity: Some(part.quantity),
                tax_rates: None,
            }
        }).collect()
    }
}
