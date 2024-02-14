use leptos::*;

use api_boundary::common::money::Money;
use api_boundary::orders::models::Order;

#[component]
pub fn AddOrderPayoutsTableRow(#[prop(into)] order: Order) -> impl IntoView {
    // -- signals -- //

    let payout = create_rw_signal(None::<Money>);

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
                        <p class="text-gray-900 whitespace-no-wrap">{order.model_file.name}</p>
                    </div>
                </div>
            </td>
            <td class="px-2 py-5 border-b border-gray-200 bg-white text-sm">
                <div class="flex justify-center">
                    <p class="text-gray-900 whitespace-no-wrap">{order.status.to_string()}</p>
                </div>
            </td>
            <td class="px-2 py-5 border-b border-gray-200 bg-white text-sm">
                <div class="flex justify-center">
                    <p class="text-gray-900 whitespace-no-wrap">{order.deadline.to_string()}</p>
                </div>
            </td>
            <td class="px-2 py-5 border-b border-gray-200 bg-white text-sm">
                <div class="flex justify-center">
                    <p class="text-gray-900 whitespace-no-wrap">
                        <input
                            type="number"
                            id="payout"
                            name="payout"
                            min=1
                            class="w-32 px-3 py-2 text-center bg-white border border-gray-300 rounded-md text-sm focus:outline-none focus:ring-indigo-500 focus:border-indigo-500"
                            placeholder="N/A"
                            value=move || {
                                if payout.get().is_some() {
                                    Some(payout.get().unwrap().amount as f64 / 100.0)
                                } else {
                                    None
                                }
                            }

                            on:change=move |ev| {
                                let payment_amount = (event_target_value(&ev)).parse::<f64>();
                                log::info!("{payment_amount:?}");
                                match payment_amount {
                                    Ok(amount) => {
                                        payout
                                            .update(|p| {
                                                let amount = (amount * 100.0) as i64;
                                                *p = Some(Money::new(amount, iso_currency::Currency::MXN));
                                            })
                                    }
                                    Err(_) => payout.update(|p| *p = None),
                                };
                            }
                        />

                    </p>
                </div>
                <div class="flex justify-center">

                    {move || {
                        if payout.get().is_some() {
                            rusty_money::Money::from_minor(
                                    payout.get().unwrap().amount,
                                    rusty_money::iso::MXN,
                                )
                                .to_string()
                        } else {
                            String::from("$")
                        }
                    }}

                </div>
            </td>
            <td class="px-2 py-5 border-b border-gray-200 bg-white text-sm">
                <div class="flex justify-center">
                    <button
                        type="submit"
                        class="rounded-md bg-indigo-600 px-6 py-3 text-sm font-semibold text-white shadow-sm hover:bg-indigo-500 focus-visible:outline focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-indigo-600"
                    >
                        // hidden=move || !is_ready()
                        // on:click=move |_| create_orders_from_parts.dispatch(())

                        Submit
                    </button>
                </div>
            </td>
        </tr>
    }
}
