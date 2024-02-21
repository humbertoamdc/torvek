use chrono::NaiveDate;
use leptos::*;
use thaw::{DatePicker, Input, InputPrefix};

use api_boundary::common::money::Money;
use api_boundary::parts::models::Part;

#[component]
pub fn PartsRow(
    #[prop(into)] part: Part,
    #[prop(into)] price_options: Vec<RwSignal<Option<Money>>>,
    #[prop(into)] deadline_options: Vec<RwSignal<Option<NaiveDate>>>,
) -> impl IntoView {
    // -- signals -- //
    view! {
        <div class="flex shadow bg-white text-sm my-2 h-80 rounded-xl overflow-hidden p-4 space-x-2">
            <div class="flex flex-col items-center">
                <img
                    class="object-scale-down"
                    src="https://cdn.dribbble.com/userupload/11259598/file/original-70a5fe9cc326f004bb78e36ee5e9d8a7.png?resize=300x0"
                    alt="User Image"
                />
                <div class="ml-3">
                    <p class="text-gray-900 whitespace-no-wrap">{part.model_file.name}</p>
                </div>
            </div>
            <div class="flex-col grow">
                <div class="flex items-baseline">
                    <p class="font-bold text-base pr-2">"Process:"</p>
                    <p class="text-md text-gray-900">{part.process}</p>
                </div>
                <div class="flex items-baseline">
                    <p class="font-bold text-base pr-2">"Material:"</p>
                    <p class="text-md text-gray-900">{part.material}</p>
                </div>
                <div class="flex items-baseline">
                    <p class="font-bold text-base pr-2">"Tolerance:"</p>
                    <p class="text-md text-gray-900">{part.tolerance}</p>
                </div>
                <div class="flex items-baseline">
                    <p class="font-bold text-base pr-2">"Quantity:"</p>
                    <p class="text-md text-gray-900">{part.quantity}</p>
                </div>
            </div>
            <div class="flex flex-col w-96 space-y-2">
                <PartPriceOption price_option=price_options[0] deadline_option=deadline_options[0]/>
                <PartPriceOption price_option=price_options[1] deadline_option=deadline_options[1]/>
                <PartPriceOption price_option=price_options[2] deadline_option=deadline_options[2]/>
            </div>
        </div>
    }
}

#[component]
pub fn PartPriceOption(
    #[prop(into)] price_option: RwSignal<Option<Money>>,
    #[prop(into)] deadline_option: RwSignal<Option<NaiveDate>>,
) -> impl IntoView {
    // -- signals -- //

    let price = create_rw_signal(String::default());

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

        let mut money =
            rusty_money::Money::from_minor(formatted_price, rusty_money::iso::MXN).to_string();
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
