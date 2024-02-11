use leptos::*;

use api_boundary::orders::models::{Order, OrderStatus};
use clients::suppliers_orders::SuppliersOrdersClient;

use crate::components::orders::orders_table_row::OrdersRow;

#[component]
pub fn OrdersTable() -> impl IntoView {
    // -- clients -- //

    let orders_client = use_context::<SuppliersOrdersClient>().unwrap();

    // -- signals -- //

    let orders = create_rw_signal(Vec::<Order>::new());

    // -- actions -- //

    let query_orders_by_status = create_action(move |_| async move {
        let result = orders_client
            .query_orders_by_status(OrderStatus::Open)
            .await;

        match result {
            Ok(response) => {
                orders.update(move |o| *o = response.orders);
            }
            Err(_) => (),
        }
    });

    query_orders_by_status.dispatch(());
    view! {
        <div class="inline-block min-w-full shadow rounded-lg overflow-hidden">
            <table class="min-w-full leading-normal">
                <thead>
                    <tr>
                        <For
                            each=move || {
                                ["Model", "Payment", "Status", "Deadline"].into_iter().enumerate()
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
                        each=move || orders.get()
                        key=move |order| order.id.clone()
                        children=move |order| {
                            view! { <OrdersRow order/> }
                        }
                    />

                </tbody>
            </table>
        </div>
    }
}
