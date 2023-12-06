use crate::api::models::orders::{ReactiveOrder, UpdateOrderRequest};
use crate::api::orders::OrdersApi;
use crate::components::orders::materials_dropdown::MaterialsDropdown;
use crate::components::orders::tolerances_dropdown::TolerancesDropdown;
use leptos::*;

#[component]
pub fn OrdersRow(#[prop(into)] reactive_order: ReactiveOrder) -> impl IntoView {
    let orders_client = use_context::<OrdersApi>().unwrap_or(OrdersApi::new());

    let update_order = create_action(move |_| {
        let order_id = reactive_order.id.clone();
        let client_id = reactive_order.client_id.clone();
        let update_orders_request = UpdateOrderRequest::new(
            order_id,
            client_id,
            Some(reactive_order.process.get_untracked()),
            Some(reactive_order.material.get_untracked()),
            Some(reactive_order.tolerance.get_untracked()),
            Some(reactive_order.quantity.get_untracked()),
        );
        async move {
            let response = orders_client.update_order(update_orders_request).await;

            match response {
                Ok(_) => (),
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
                    <div class="ml-3">
                        <p class="text-gray-900 whitespace-no-wrap">{reactive_order.file_name}</p>
                    </div>
                </div>
            </td>
            <td class="px-2 py-5 border-b border-gray-200 bg-white text-sm">
                <div class="flex justify-center">
                    <p class="text-gray-900 whitespace-no-wrap">
                        {reactive_order.process.get_untracked()}
                    </p>
                </div>
            </td>
            <td class="px-2 py-5 border-b border-gray-200 bg-white text-sm">
                <div class="flex justify-center">
                    <MaterialsDropdown
                        material=reactive_order.material
                        on_material_change=move |material| {
                            reactive_order.material.update(|m| *m = material);
                            update_order.dispatch(());
                        }
                    />

                </div>

            </td>
            <td class="px-2 py-5 border-b border-gray-200 bg-white text-sm">
                <div class="flex justify-center">
                    <p class="text-gray-900 whitespace-no-wrap"></p>
                    <TolerancesDropdown
                        tolerance=reactive_order.tolerance
                        on_tolerance_change=move |tolerance| {
                            reactive_order.tolerance.update(|t| *t = tolerance);
                            update_order.dispatch(());
                        }
                    />

                </div>
            </td>
            <td class="px-2 py-5 border-b border-gray-200 bg-white text-sm">
                <div class="flex justify-center">
                    <input
                        type="number"
                        id="quantity"
                        name="quantity"
                        min=1
                        class="w-20 px-3 py-2 text-center bg-white border border-gray-300 rounded-md text-sm focus:outline-none focus:ring-indigo-500 focus:border-indigo-500"
                        value=reactive_order.quantity.get_untracked()
                        on:change=move |ev| {
                            let quantity = event_target_value(&ev).parse::<u64>().unwrap();
                            reactive_order.quantity.update(|q| *q = quantity.clone());
                            update_order.dispatch(())
                        }
                    />

                </div>
            </td>
            <td class="px-2 py-5 border-b border-gray-200 bg-white text-sm">
                <div class="flex justify-center">
                    <p class="text-gray-900 whitespace-no-wrap">

                        {match reactive_order.unit_price.get_untracked() {
                            Some(unit_price) => format!("${unit_price}"),
                            None => String::from("N/A"),
                        }}

                    </p>
                </div>
            </td>
            <td class="px-2 py-5 border-b border-gray-200 bg-white text-sm">
                <div class="flex justify-center">
                    <p class="text-gray-900 whitespace-no-wrap">

                        {match reactive_order.sub_total.get_untracked() {
                            Some(sub_total) => format!("${sub_total}"),
                            None => String::from("N/A"),
                        }}

                    </p>
                </div>
            </td>
        </tr>
    }
}
