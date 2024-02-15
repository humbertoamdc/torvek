use leptos::*;

use api_boundary::orders::models::Order;

use crate::components::orders::add_order_payouts_table_row::AddOrderPayoutsTableRow;

#[component]
pub fn AddOrderPayoutsTable(#[prop(into)] orders: RwSignal<Vec<Order>>) -> impl IntoView {
    view! {
        <div class="inline-block min-w-full shadow rounded-lg overflow-hidden">
            <table class="min-w-full leading-normal">
                <thead>
                    <tr>
                        <For
                            each=move || {
                                ["Name", "Status", "Payed", "Deadline", "Payout", ""]
                                    .into_iter()
                                    .enumerate()
                            }

                            key=|(_, column_name)| column_name.to_string()
                            children=move |(_, column_name)| {
                                view! {
                                    <th class="px-2 py-3 border-b-2 border-gray-200 bg-gray-100 text-left text-xs font-semibold text-gray-600 uppercase tracking-wider">
                                        <div class="flex justify-center">{column_name}</div>
                                    </th>
                                }
                            }
                        />

                    </tr>
                </thead>
                <tbody>
                    <For
                        each=move || orders.get().into_iter().enumerate()
                        key=|(_, order)| order.id.clone()
                        children=move |(idx, order)| {
                            let remove_self_from_orders_callback = Callback::new(move |_| {
                                orders
                                    .update(|orders| {
                                        orders.remove(idx);
                                    });
                            });
                            view! {
                                <AddOrderPayoutsTableRow order remove_self_from_orders_callback/>
                            }
                        }
                    />

                </tbody>
            </table>
        </div>
    }
}
