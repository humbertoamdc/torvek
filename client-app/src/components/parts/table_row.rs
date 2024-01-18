use crate::api::models::auth::UserInfo;
use crate::components::parts::materials_dropdown::MaterialsDropdown;
use crate::components::parts::tolerance_dropdown::TolerancesDropdown;
use crate::models::reactive_part::ReactivePart;
use leptos::*;
use web_sys::HtmlInputElement;

#[component]
pub fn PartsTableRow(#[prop(into)] reactive_part: ReactivePart) -> impl IntoView {
    let part_id = reactive_part.id.clone();

    // -- context -- //

    let user_info = use_context::<RwSignal<UserInfo>>().expect("user info to be provided");

    view! {
        <tr>
            <td class="px-2 py-5 border-b border-gray-200 bg-white text-sm">
                <div class="flex items-center justify-left pl-6">
                    <div class="flex-shrink-0 w-10 h-10">
                        <img
                            class="w-full h-full rounded-full"
                            src="https://cdn.dribbble.com/userupload/11259598/file/original-70a5fe9cc326f004bb78e36ee5e9d8a7.png?resize=100x0"
                            alt="User Image"
                        />
                    </div>
                    <div class="ml-3">
                        <p class="text-gray-900 whitespace-no-wrap">{reactive_part.model_file.get_untracked().name}</p>
                        <p
                            class="text-gray-900 whitespace-no-wrap"
                            for=format!("drawing-file-{}", part_id)
                        >
                            <label for=format!("drawing-file-{}", part_id)>
                                <input
                                    id=format!("drawing-file-{}", part_id)

                                    type="file"
                                    class="hidden"
                                    accept=".pdf"
                                    on:change=move |ev| {
                                        let input_element = event_target::<HtmlInputElement>(&ev);
                                        // upload_drawing_file.dispatch(input_element);
                                    }
                                />

                                <span class="inline-flex items-center rounded-xl bg-red-50 hover:bg-red-100 px-3 py-1 my-1 text-xs font-medium text-red-600 ring-1 ring-inset ring-red-600/10 cursor-pointer ">
                                    <img
                                        style="width: 18px; height: 18px;"
                                        src="https://icons.veryicon.com/png/o/transport/traffic-2/pdf-34.png"
                                        alt="User Image"
                                    />
                                    {move || {
                                        match reactive_part.drawing_file.get() {
                                            Some(drawing_file) => drawing_file.name,
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
                        {reactive_part.process.get_untracked()}
                    </p>
                </div>
            </td>
            <td class="px-2 py-5 border-b border-gray-200 bg-white text-sm">
                <div class="flex justify-center">
                    <MaterialsDropdown
                        material=reactive_part.material
                        on_material_change=move |material| {
                            reactive_part.material.update(|m| *m = material);
                            // update_order.dispatch(());
                        }
                    />

                </div>
            </td>
            <td class="px-2 py-5 border-b border-gray-200 bg-white text-sm">
                <div class="flex justify-center">
                    <p class="text-gray-900 whitespace-no-wrap"></p>
                    <TolerancesDropdown
                        tolerance=reactive_part.tolerance
                        on_tolerance_change=move |tolerance| {
                            reactive_part.tolerance.update(|t| *t = tolerance);
                            // update_order.dispatch(());
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
                        value=reactive_part.quantity.get_untracked()
                        on:change=move |ev| {
                            let quantity = event_target_value(&ev).parse::<u64>().unwrap();
                            reactive_part.quantity.update(|q| *q = quantity.clone());
                            // update_order.dispatch(())
                        }
                    />

                </div>
            </td>
            <td class="px-2 py-5 border-b border-gray-200 bg-white text-sm">
                <div class="flex justify-center">
                    <p class="text-gray-900 whitespace-no-wrap">
                        {move || {
                            match reactive_part.unit_price.get() {
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
                            match reactive_part.sub_total.get() {
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
