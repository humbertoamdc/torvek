use leptos::*;
use web_sys::HtmlInputElement;

use api_boundary::common::file::File;
use api_boundary::parts::models::PartQuote;
use api_boundary::parts::requests::CreateDrawingUploadUrlRequest;
use api_boundary::parts::requests::UpdatePartRequest;
use clients::parts::PartsClient;

use crate::api::models::auth::UserInfo;
use crate::components::parts::materials_dropdown::MaterialsDropdown;
use crate::components::parts::part_quote_card::PartQuoteCard;
use crate::components::parts::tolerance_dropdown::TolerancesDropdown;
use crate::models::reactive_part::ReactivePart;

#[component]
pub fn PartsTableRow(
    #[prop(into)] reactive_part: ReactivePart,
    #[prop(into)] part_quotes: RwSignal<Vec<PartQuote>>,
) -> impl IntoView {
    let part_id = reactive_part.id.clone();

    // -- clients -- //

    let parts_client = use_context::<PartsClient>().unwrap();

    // -- context -- //

    let user_info = use_context::<RwSignal<UserInfo>>().expect("user info to be provided");

    // -- signals -- //

    let selected_part_quote_cards = create_rw_signal(Vec::<RwSignal<bool>>::new());

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
        <div class="flex shadow my-2 p-4 h-80 bg-white rounded-xl overflow-hidden space-x-2">
            <div class="flex flex-col items-center justify-left">
                <div class="flex-shrink-0 w-80">
                    <img
                        class="object-scale-down"
                        src="https://cdn.dribbble.com/userupload/11259598/file/original-70a5fe9cc326f004bb78e36ee5e9d8a7.png?resize=320x0"
                        alt="User Image"
                    />
                    <div class="flex ml-3 space-x-2">
                        <label for=format!("model-file-{}", part_id)>
                            <input
                                id=format!("model-file-{}", part_id)

                                type="file"
                                class="hidden"
                                accept=".stp,.step"
                                on:change=move |_| {}
                            />

                            <span class="inline-flex items-center rounded-xl bg-gray-100 hover:bg-red-100 px-3 py-1 my-1 text-xs font-medium text-gray-600 ring-1 ring-inset hover:ring-gray-600 cursor-pointer ">
                                <img
                                    style="width: 18px; height: 18px;"
                                    src="https://icons.veryicon.com/png/o/construction-tools/cloud-device/spare-part-type-01.png"
                                    alt="User Image"
                                />
                                <div class="ml-1">
                                    {reactive_part.model_file.get_untracked().name}
                                </div>

                            </span>
                        </label>
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

                            <span class="inline-flex items-center rounded-xl bg-red-50 hover:bg-red-100 px-3 py-1 my-1 text-xs font-medium text-red-600 ring-1 ring-inset hover:ring-red-600 cursor-pointer ">
                                <img
                                    style="width: 18px; height: 18px;"
                                    src="https://icons.veryicon.com/png/o/transport/traffic-2/pdf-34.png"
                                    alt="User Image"
                                />
                                <div class="ml-1">
                                    {move || {
                                        match reactive_part.drawing_file.get() {
                                            Some(drawing_file) => drawing_file.name,
                                            None => String::from("Pdf"),
                                        }
                                    }}

                                </div>

                            </span>
                        </label>

                    </div>
                </div>
            </div>
            <div class="flex-col grow grow">
                <div class="flex items-baseline">
                    <p class="font-bold text-base pr-2">"Process:"</p>
                    <p class="ml-4 text-md text-gray-900">{reactive_part.process}</p>
                </div>
                <div class="flex items-baseline mt-1">
                    <p class="font-bold text-base pr-2">"Material:"</p>
                    <div class="ml-3">
                        <MaterialsDropdown
                            material=reactive_part.material
                            on_material_change=move |material| {
                                reactive_part.material.update(|m| *m = material);
                                update_part.dispatch(());
                            }
                        />

                    </div>

                </div>
                <div class="flex items-baseline mt-1">
                    <p class="font-bold text-base pr-2">"Tolerance:"</p>
                    <TolerancesDropdown
                        tolerance=reactive_part.tolerance
                        on_tolerance_change=move |tolerance| {
                            reactive_part.tolerance.update(|t| *t = tolerance);
                            update_part.dispatch(());
                        }
                    />

                </div>
                <div class="flex items-baseline mt-1">
                    <p class="font-bold text-base pr-2">"Quantity:"</p>
                    <input
                        type="number"
                        id="quantity"
                        name="quantity"
                        min=1
                        class="w-20 ml-2 px-3 py-1.5 text-sm text-center bg-white border border-gray-300 rounded-md focus:outline-none focus:ring-indigo-500 focus:border-indigo-500"
                        value=reactive_part.quantity.get_untracked()
                        on:change=move |ev| {
                            let quantity = event_target_value(&ev).parse::<u64>().unwrap();
                            reactive_part.quantity.update(|q| *q = quantity.clone());
                            update_part.dispatch(())
                        }
                    />

                </div>
            </div>
            <div class="flex flex-col w-80 space-y-2">
                <For
                    each=move || part_quotes.get().into_iter()
                    key=|part_quote| part_quote.id.clone()
                    children=move |part_quote| {
                        let is_selected = create_rw_signal(false);
                        let on_select = move |selected| {
                            selected_part_quote_cards
                                .with(|selected_part_quote_cards| {
                                    selected_part_quote_cards
                                        .iter()
                                        .for_each(|selected_card| {
                                            selected_card.update(|selected_card| *selected_card = false)
                                        })
                                });
                            is_selected.update(|is_selected| *is_selected = true);
                        };
                        selected_part_quote_cards
                            .update(|selected_part_quote_cards| {
                                selected_part_quote_cards.push(is_selected)
                            });
                        view! { <PartQuoteCard part_quote is_selected on_select/> }
                    }
                />

            </div>
        </div>
    }
}
