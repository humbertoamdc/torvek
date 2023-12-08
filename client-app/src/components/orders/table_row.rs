use crate::api::models::auth::UserInfo;
use crate::api::models::orders::{CreateOrdersRequest, ReactiveOrder, UpdateOrderRequest};
use crate::api::orders::OrdersApi;
use crate::components::orders::materials_dropdown::MaterialsDropdown;
use crate::components::orders::tolerances_dropdown::TolerancesDropdown;
use api_boundary::orders::requests::CreateDrawingUploadUrlRequest;
use leptos::*;
use web_sys::HtmlInputElement;

#[component]
pub fn OrdersRow(#[prop(into)] reactive_order: ReactiveOrder) -> impl IntoView {
    let order_id = reactive_order.id.clone();

    // -- context -- //

    let user_info = use_context::<RwSignal<UserInfo>>().expect("user info to be provided");
    let orders_client = use_context::<OrdersApi>().unwrap_or(OrdersApi::new());

    // -- actions -- //

    let update_order = create_action(move |_| {
        let order_id = reactive_order.id.clone();
        let client_id = reactive_order.client_id.clone();
        let update_orders_request = UpdateOrderRequest::new(
            order_id,
            client_id,
            reactive_order.drawing_file_name.get_untracked(),
            reactive_order.drawing_file_url.get_untracked(),
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

    let upload_drawing_file = create_action(move |input_element: &HtmlInputElement| {
        let file = input_element.clone().files().unwrap().item(0).unwrap();
        let file_name = file.name();

        let input_element = input_element.clone();
        let request = CreateDrawingUploadUrlRequest::new(
            user_info.get().id,
            file_name.clone(),
            reactive_order.drawing_file_url.get_untracked(),
        );

        async move {
            let orders_client = OrdersApi::new();
            match orders_client.create_drawing_upload_url(request).await {
                Ok(response) => {
                    let upload_file_response = orders_client
                        .upload_file_with_presigned_url(file, response.url.clone())
                        .await;

                    match upload_file_response {
                        Ok(_) => {
                            reactive_order
                                .drawing_file_name
                                .update(|f| *f = Some(file_name));
                            reactive_order
                                .drawing_file_url
                                .update(|f| *f = Some(response.url));
                            update_order.dispatch(());
                        }
                        Err(err) => log::error!("{err:?}"),
                    };
                }
                Err(err) => log::error!("{err:?}"),
            }
            input_element.set_value("");
        }
    });

    view! {
        <tr>
            <td class="px-2 py-5 border-b border-gray-200 bg-white text-sm">
                <div class="flex items-center justify-center">
                    <div class="flex-shrink-0 w-10 h-10">
                        <img
                            class="w-full h-full rounded-full"
                            src="https://cdn.dribbble.com/userupload/11259598/file/original-70a5fe9cc326f004bb78e36ee5e9d8a7.png?resize=100x0"
                            alt="User Image"
                        />
                    </div>
                    <div class="ml-3">
                        <p class="text-gray-900 whitespace-no-wrap">{reactive_order.file_name}</p>
                        <p class="text-gray-900 whitespace-no-wrap" for=format!("drawing-file-{}", order_id)>
                            <label
                                for=format!("drawing-file-{}", order_id)
                            >
                                <input
                                    id=format!("drawing-file-{}", order_id)

                                    type="file"
                                    class="hidden"
                                    accept=".pdf"
                                    on:change=move |ev| {
                                        let input_element = event_target::<HtmlInputElement>(&ev);
                                        upload_drawing_file.dispatch(input_element);
                                    }
                                />

                                <span class="inline-flex items-center rounded-xl bg-red-50 hover:bg-red-100 px-3 py-1 my-1 text-xs font-medium text-red-600 ring-1 ring-inset ring-red-600/10 cursor-pointer ">
                                    <img
                                        style="width: 18px; height: 18px;"
                                        src="https://icons.veryicon.com/png/o/transport/traffic-2/pdf-34.png"
                                        alt="User Image"
                                    />
                                    {move || {
                                        match reactive_order.drawing_file_name.get() {
                                            Some(drawing_file_name) => format!("{drawing_file_name}"),
                                            None => String::from("Pdf"),
                                        }
                                    }}

                                </span>
                            </label>

                        </p>

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
                        {move || {
                            match reactive_order.unit_price.get() {
                                Some(unit_price) => format!("${unit_price}"),
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
                            match reactive_order.sub_total.get() {
                                Some(sub_total) => format!("${sub_total}"),
                                None => String::from("N/A"),
                            }
                        }}

                    </p>
                </div>
            </td>
        </tr>
    }
}
