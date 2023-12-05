use crate::api::models::orders::{Order, ReactiveOrder};
use crate::components::orders::table_row::OrdersRow;
use leptos::*;

#[component]
pub fn OrdersTable(#[prop(into)] client_orders: RwSignal<Vec<Order>>) -> impl IntoView {
    view! {
        <div class="inline-block min-w-full shadow rounded-lg overflow-hidden">
            <table class="min-w-full leading-normal">
                <thead>
                    <tr>
                        <For
                            each=move || {
                                [
                                    "Part",
                                    "Process",
                                    "Material",
                                    "Tolerance",
                                    "Quantity",
                                    "Unit Price",
                                    "Subtotal",
                                    "",
                                ]
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
                        each=move || client_orders.get().into_iter().enumerate()
                        key=|(_, order)| order.id.clone()
                        children=move |(_, order)| {
                            let reactive_order = ReactiveOrder::from(&order);
                            view! { <OrdersRow reactive_order=reactive_order/> }
                        }
                    />

                </tbody>
            </table>

        </div>
    }
}
