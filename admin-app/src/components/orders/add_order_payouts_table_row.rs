use leptos::*;

use crate::clients::orders::{AdminUpdateOrderPayoutRequest, OrdersClient};
use crate::models::money::Money;
use crate::models::order::Order;

#[component]
pub fn AddOrderPayoutsTableRow(
    #[prop(into)] order: Order,
    #[prop(into)] remove_self_from_orders_callback: Callback<()>,
) -> impl IntoView {
    // -- clients -- //

    let orders_client = use_context::<OrdersClient>().unwrap();

    // -- signals -- //

    let payout = create_rw_signal(None::<Money>);
    let is_ready = move || payout.get().is_some();

    // -- actions -- //
    let update_order_payout = create_action(move |_| {
        let request = AdminUpdateOrderPayoutRequest {
            order_id: order.id.clone(),
            payout: payout.get_untracked().unwrap(),
        };

        async move {
            let result = orders_client.admin_update_order_payout(request).await;
            match result {
                Ok(_) => remove_self_from_orders_callback.call(()),
                Err(_) => (),
            }
        }
    });

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
                    // <p class="text-gray-900 whitespace-no-wrap">{order.model_file.name}</p>
                    <div class="ml-3"></div>
                </div>
            </td>
            <td class="px-2 py-5 border-b border-gray-200 bg-white text-sm">
                <div class="flex justify-center">
                    <p class="text-gray-900 whitespace-no-wrap">{order.status.to_string()}</p>
                </div>
            </td>
            <td class="px-2 py-5 border-b border-gray-200 bg-white text-sm">
                <div class="flex justify-center">
                    <p class="text-gray-900 whitespace-no-wrap">PLACEHOLDER</p>
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
                            payout.get().unwrap().to_string()
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
                        disabled=move || !is_ready()
                        on:click=move |_| update_order_payout.dispatch(())
                    >

                        Submit
                    </button>
                </div>
            </td>
        </tr>
    }
}
