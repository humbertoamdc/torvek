use api_boundary::common::money::Money;
use api_boundary::parts::models::Part;
use chrono::NaiveDate;
use leptos::*;
use thaw::DatePicker;

#[component]
pub fn PartToOrderRow(
    #[prop(into)] part: Part,
    #[prop(into)] payment: RwSignal<Option<Money>>,
    #[prop(into)] deadline: RwSignal<Option<NaiveDate>>,
) -> impl IntoView {
    view! {
        <tr>
            <td class="px-2 py-5 border-b border-gray-200 bg-white text-sm">
                <div class="flex items-center">
                    <div class="flex-shrink-0 w-10 h-10">
                        <img
                            class="w-full h-full rounded-full"
                            src="https://cdn.dribbble.com/userupload/11259598/file/original-70a5fe9cc326f004bb78e36ee5e9d8a7.png?resize=100x0"
                            alt="User Image"
                        />
                    </div>
                    <div class="ml-3">
                        <p class="text-gray-900 whitespace-no-wrap">
                            {part.model_file.name.clone()}
                        </p>
                    </div>
                </div>
            </td>
            <td class="px-2 py-5 border-b border-gray-200 bg-white text-sm">
                <div class="flex justify-center">
                    <p class="text-gray-900 whitespace-no-wrap">{part.process.clone()}</p>
                </div>
            </td>
            <td class="px-2 py-5 border-b border-gray-200 bg-white text-sm">
                <div class="flex justify-center">{part.material.clone()}</div>

            </td>
            <td class="px-2 py-5 border-b border-gray-200 bg-white text-sm">
                <div class="flex justify-center">
                    <p class="text-gray-900 whitespace-no-wrap">{part.tolerance.clone()}</p>
                </div>
            </td>
            <td class="px-2 py-5 border-b border-gray-200 bg-white text-sm">
                <div class="flex justify-center">
                    <p class="text-gray-900 whitespace-no-wrap">{part.quantity}</p>
                </div>
            </td>
            <td class="px-2 py-5 border-b border-gray-200 bg-white text-sm">
                <div class="flex justify-center">
                    <p class="text-gray-900 whitespace-no-wrap">
                        {move || {
                            match part.unit_price {
                                Some(unit_price) => {
                                    rusty_money::Money::from_minor(
                                            unit_price as i64,
                                            rusty_money::iso::MXN,
                                        )
                                        .to_string()
                                }
                                None => String::from("N/A"),
                            }
                        }}

                    </p>
                </div>
            </td>
            <td class="px-2 py-5 border-b border-gray-200 bg-white text-sm">
                <div class="flex justify-center">
                    <p class="text-gray-900 whitespace-no-wrap">
                        {move || {
                            match part.sub_total {
                                Some(sub_total) => {
                                    rusty_money::Money::from_minor(
                                            sub_total as i64,
                                            rusty_money::iso::MXN,
                                        )
                                        .to_string()
                                }
                                None => String::from("N/A"),
                            }
                        }}

                    </p>
                </div>
            </td>
            <td class="px-2 py-5 border-b border-gray-200 bg-white text-sm">
                <div class="flex justify-center">
                    <p class="text-gray-900 whitespace-no-wrap">
                        <input
                            type="number"
                            id="quantity"
                            name="quantity"
                            min=1
                            class="w-32 px-3 py-2 text-center bg-white border border-gray-300 rounded-md text-sm focus:outline-none focus:ring-indigo-500 focus:border-indigo-500"
                            placeholder="N/A"
                            value=move || {
                                if payment.get().is_some() {
                                    Some(payment.get().unwrap().amount as f64 / 100.0)
                                } else {
                                    None
                                }
                            }

                            on:change=move |ev| {
                                let payment_amount = (event_target_value(&ev)).parse::<f64>();
                                match payment_amount {
                                    Ok(amount) => {
                                        payment
                                            .update(|p| {
                                                let amount = (amount * 100.0) as i64;
                                                *p = Some(Money::new(amount, iso_currency::Currency::MXN));
                                            })
                                    }
                                    Err(_) => payment.update(|p| *p = None),
                                };
                            }
                        />

                    </p>
                </div>
            </td>
            <td class="px-2 py-5 border-b border-gray-200 bg-white text-sm">
                <div class="flex justify-center">
                    <DatePicker value=deadline/>
                </div>
            </td>
        </tr>
    }
}
