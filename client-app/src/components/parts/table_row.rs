use crate::api::models::auth::UserInfo;
use crate::api::parts::PartsClient;
use crate::components::parts::materials_dropdown::MaterialsDropdown;
use crate::components::parts::tolerance_dropdown::TolerancesDropdown;
use crate::models::reactive_part::ReactivePart;
use api_boundary::common::file::File;
use api_boundary::parts::requests::CreateDrawingUploadUrlRequest;
use api_boundary::parts::requests::UpdatePartRequest;
use leptos::*;
use rusty_money::{iso, Money};
use web_sys::HtmlInputElement;

#[component]
pub fn PartsTableRow(#[prop(into)] reactive_part: ReactivePart) -> impl IntoView {
    let part_id = reactive_part.id.clone();

    // -- context -- //

    let user_info = use_context::<RwSignal<UserInfo>>().expect("user info to be provided");

    // -- actions -- //
    let update_part = create_action(move |_| {
        let update_part_request = UpdatePartRequest {
            id: reactive_part.id.clone(),
            client_id: reactive_part.client_id.clone(),
            project_id: reactive_part.project_id.clone(),
            quotation_id: reactive_part.quotation_id.clone(),
            drawing_file: reactive_part.drawing_file.get_untracked(),
            process: Some(reactive_part.process.get_untracked()),
            material: Some(reactive_part.material.get_untracked()),
            tolerance: Some(reactive_part.tolerance.get_untracked()),
            quantity: Some(reactive_part.quantity.get_untracked()),
        };
        let parts_client = PartsClient::new();
        async move {
            let response = parts_client.update_part(update_part_request).await;

            match response {
                Ok(_) => (),
                Err(_) => (), // TODO: Handle error.
            }
        }
    });

    let upload_drawing_file = create_action(move |input_element: &HtmlInputElement| {
        let file = input_element.clone().files().unwrap().item(0).unwrap();
        let file_name = file.name();
        let file_url = match reactive_part.drawing_file.get_untracked() {
            Some(file) => Some(file.url),
            None => None,
        };

        let input_element = input_element.clone();
        let request = CreateDrawingUploadUrlRequest {
            client_id: user_info.get_untracked().id,
            file_name: file_name.clone(),
            file_url,
        };

        async move {
            let parts_client = PartsClient::new();
            match parts_client.create_drawing_upload_url(request).await {
                Ok(response) => {
                    let upload_file_response = parts_client
                        .upload_file_with_presigned_url(file, response.presigned_url.clone())
                        .await;

                    match upload_file_response {
                        Ok(_) => {
                            reactive_part
                                .drawing_file
                                .update(|f| *f = Some(File::new(file_name, response.url)));
                            update_part.dispatch(());
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
                <div class="flex items-center justify-left pl-6">
                    <div class="flex-shrink-0 w-10 h-10">
                        <img
                            class="w-full h-full rounded-full"
                            src="https://cdn.dribbble.com/userupload/11259598/file/original-70a5fe9cc326f004bb78e36ee5e9d8a7.png?resize=100x0"
                            alt="User Image"
                        />
                    </div>
                    <div class="ml-3">
                        <p class="text-gray-900 whitespace-no-wrap">
                            {reactive_part.model_file.get_untracked().name}
                        </p>
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
                            update_part.dispatch(());
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
                            update_part.dispatch(());
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
                            update_part.dispatch(())
                        }
                    />

                </div>
            </td>
            <td class="px-2 py-5 border-b border-gray-200 bg-white text-sm">
                <div class="flex justify-center">
                    <p class="text-gray-900 whitespace-no-wrap">
                        {move || {
                            match reactive_part.unit_price.get() {
                                Some(unit_price) => {
                                    Money::from_minor(unit_price as i64, iso::MXN).to_string()
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
                            match reactive_part.sub_total.get() {
                                Some(sub_total) => {
                                    Money::from_minor(sub_total as i64, iso::MXN).to_string()
                                }
                                None => String::from("N/A"),
                            }
                        }}

                    </p>
                </div>
            </td>
        </tr>
    }
}
