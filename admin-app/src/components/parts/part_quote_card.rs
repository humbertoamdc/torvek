use chrono::NaiveDate;
use leptos::*;
use thaw::{DatePicker, Input, InputPrefix};

use api_boundary::common::money::Money;

#[component]
pub fn PartQuoteCard(
    #[prop(into)] price_option: RwSignal<Option<Money>>,
    #[prop(into)] deadline_option: RwSignal<Option<NaiveDate>>,
) -> impl IntoView {
    // -- signals -- //

    let price = create_rw_signal(String::default());

    // -- callbacks -- //

    let allow_price_value = move |value: String| {
        let is_valid_input = value
            .chars()
            .all(|c| c.is_digit(10) || c == ',' || c == '.');
        let is_valid_amount = value
            .split(".")
            .nth(0)
            .unwrap()
            .chars()
            .filter(|c| c.is_digit(10))
            .collect::<String>()
            .len()
            < 9;
        let has_valid_cents = value.split(".").nth(1).unwrap_or_default().len() <= 2;

        value.is_empty() || (is_valid_input && is_valid_amount && has_valid_cents)
    };
    let set_price_value = move |_| {
        if price.get().is_empty() {
            return;
        }

        // Format input
        let formatted_price = (price
            .get()
            .split(",")
            .collect::<Vec<&str>>()
            .join("")
            .parse::<f64>()
            .unwrap()
            * 100.0) as i64;

        let mut money = Money::new(formatted_price, iso_currency::Currency::MXN).to_string();
        money.remove(0);

        price.update(|p| *p = money);
        price_option.update(|p| *p = Some(Money::new(formatted_price, iso_currency::Currency::MXN)))
    };

    view! {
        <div class="grow flex justify-between items-center rounded-md border p-3">
            <DatePicker value=deadline_option/>
            <Input
                class="w-36"
                value=price
                placeholder="$100,000.00"
                allow_value=allow_price_value
                on_blur=set_price_value
            >
                <InputPrefix slot>
                    {move || if price.get().is_empty() { "" } else { "$" }}
                </InputPrefix>
            </Input>
        </div>
    }
}
