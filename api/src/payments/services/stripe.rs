use stripe::{
    CheckoutSession, CheckoutSessionMode, Client, CreateCheckoutSession,
    CreateCheckoutSessionLineItems, CreateCheckoutSessionLineItemsPriceData,
    CreateCheckoutSessionLineItemsPriceDataProductData, Currency, StripeError,
};

use api_boundary::orders::requests::StripeCreateOrdersRequestData;
use api_boundary::payments::requests::{
    CreateCheckoutSessionPartData, CreateCheckoutSessionRequest,
};

const CLIENT_ID: &'static str = "client_id";
const PROJECT_ID: &'static str = "project_id";
const QUOTATION_ID: &'static str = "quotation_id";
const DATA: &'static str = "data";

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
        let line_items = Self::line_items_from_parts_data(&request.data);
        let success_url = format!(
            "{}/projects/{}/quotations/{}/parts",
            self.success_url, request.project_id, request.quotation_id
        );

        let mut params = CreateCheckoutSession::new();
        params.line_items = Some(line_items);
        params.success_url = Some(&success_url);
        params.mode = Some(CheckoutSessionMode::Payment);
        let metadata = stripe::Metadata::from([
            (String::from(CLIENT_ID), request.client_id),
            (String::from(PROJECT_ID), request.project_id),
            (String::from(QUOTATION_ID), request.quotation_id),
            (
                String::from(DATA),
                serde_json::to_string(&Self::create_orders_data_from_parts_data(&request.data))
                    .unwrap(),
            ),
        ]);
        params.metadata = Some(metadata);

        let result = CheckoutSession::create(&self.client, params).await?;

        Ok(result.url.unwrap())
    }

    fn line_items_from_parts_data(
        parts_data: &Vec<CreateCheckoutSessionPartData>,
    ) -> Vec<CreateCheckoutSessionLineItems> {
        parts_data.iter().map(|part_data| CreateCheckoutSessionLineItems {
            adjustable_quantity: None,
            dynamic_tax_rates: None,
            price: None,
            price_data: Some(CreateCheckoutSessionLineItemsPriceData {
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

    fn create_orders_data_from_parts_data(
        parts_data: &Vec<CreateCheckoutSessionPartData>,
    ) -> Vec<StripeCreateOrdersRequestData> {
        parts_data
            .iter()
            .map(|part_data| StripeCreateOrdersRequestData {
                part_id: part_data.part_id.clone(),
                model_file: part_data.model_file.clone(),
                drawing_file: part_data.drawing_file.clone(),
                deadline: part_data.deadline,
            })
            .collect()
    }
}
