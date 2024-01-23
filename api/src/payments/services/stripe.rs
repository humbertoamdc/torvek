use api_boundary::payments::requests::CreateCheckoutSessionRequest;
use stripe::{
    CheckoutSession, CheckoutSessionMode, Client, CreateCheckoutSession,
    CreateCheckoutSessionLineItems, CreateCheckoutSessionLineItemsPriceData,
    CreateCheckoutSessionLineItemsPriceDataProductData, Currency, StripeError,
};

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
        request: CreateCheckoutSessionRequest,
    ) -> Result<String, StripeError> {
        let line_items = Self::line_items(&request);

        let mut params = CreateCheckoutSession::new();
        params.line_items = Some(line_items);
        params.success_url = Some(&self.success_url);
        params.mode = Some(CheckoutSessionMode::Payment);

        let result = CheckoutSession::create(&self.client, params).await?;

        Ok(result.url.unwrap())
    }

    fn line_items(request: &CreateCheckoutSessionRequest) -> Vec<CreateCheckoutSessionLineItems> {
        request.parts_data.iter().map(|part_data| CreateCheckoutSessionLineItems {
            adjustable_quantity: None,
            dynamic_tax_rates: None,
            price: None,
            price_data:  Some(CreateCheckoutSessionLineItemsPriceData {
                currency: Currency::MXN,
                product: None,
                product_data: Some(CreateCheckoutSessionLineItemsPriceDataProductData {
                    description: Some(format!(
                        "Process: {} / Material: {} / Tolerance: {}",
                        part_data.process, part_data.material, part_data.tolerance
                    )),
                    images: Some(vec![
                        "https://cdn.dribbble.com/userupload/11259598/file/original-70a5fe9cc326f004bb78e36ee5e9d8a7.png?resize=100x".to_string()
                    ]),
                    metadata: None,
                    name: part_data.name.clone(),
                    tax_code: None,
                }),
                recurring: None,
                tax_behavior: None,
                unit_amount: Some(part_data.sub_total as i64),
                unit_amount_decimal: None,
            }),
            quantity: Some(part_data.quantity),
            tax_rates: None,
        }).collect()
    }
}
