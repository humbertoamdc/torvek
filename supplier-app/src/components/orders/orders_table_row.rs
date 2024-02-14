use leptos::*;

use api_boundary::orders::models::Order;

#[component]
pub fn OrdersRow(#[prop(into)] order: Order) -> impl IntoView {
    view! {
        <tr>
            <td class="px-2 py-5 border-b border-gray-200 bg-white text-sm">
                <div class="flex justify-center">
                    <p class="text-gray-900 whitespace-no-wrap">Order file</p>
                </div>
            </td>
            <td class="px-2 py-5 border-b border-gray-200 bg-white text-sm">
                <div class="flex justify-center">
                    <p class="text-gray-900 whitespace-no-wrap">
                        {move || {
                            rusty_money::Money::from_minor(
                                    order.payout.clone().unwrap().amount,
                                    rusty_money::iso::MXN,
                                )
                                .to_string()
                        }}

                    </p>
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
        </tr>
    }
}
