use http::StatusCode;
use leptos::html::Canvas;
use leptos::wasm_bindgen::JsCast;
use leptos::*;
use leptos_use::use_element_visibility;
use std::thread::sleep;
use std::time::Duration;
use web_sys::{HtmlCanvasElement, HtmlInputElement};
#[cfg(target_arch = "wasm32")]
use winit::platform::web::WindowBuilderExtWebSys;
use winit::window::WindowBuilder;

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
    #[prop(into)] insert_window: Callback<(WindowBuilder, three_d_asset::Model)>,
) -> impl IntoView {
    let part_id = reactive_part.id.clone();

    // -- clients -- //

    let parts_client = use_context::<PartsClient>().unwrap();

    // -- context -- //

    let user_info = use_context::<RwSignal<UserInfo>>().expect("user info to be provided");

    // -- signals -- //

    let (canvas_id, _) = create_signal(format!("canvas-part-{part_id}"));
    let canvas_ref = create_node_ref::<Canvas>();
    let is_visible = use_element_visibility(canvas_ref);

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

    let load_part_model: Action<(), three_d_asset::Model> = create_action(move |_| async move {
        loop {
            let resp = parts_client
                .get_file_from_presigned_url(
                    reactive_part
                        .render_file
                        .get_untracked()
                        .presigned_url
                        .unwrap(),
                )
                .await;

            if resp.is_ok() && resp.unwrap().status() == StatusCode::OK {
                break;
            }
            gloo_timers::future::TimeoutFuture::new(1_000).await;
        }

        // TODO: Use presigned url to render file. We are double fetching the file, we can use the result obtained
        //       in the loop and create the RawAssets manually since using the `load_async` function is not working
        //       with presigned urls.
        let mut result =
            three_d_asset::io::load_async(&[reactive_part.render_file.get_untracked().url]).await;

        result.unwrap().deserialize("/").unwrap()
    });
    load_part_model.dispatch(());

    // -- effects -- //

    let _ = create_effect(move |_| {
        if load_part_model.value().get().is_some() {
            let window = web_sys::window().expect("should have a window in this context");
            let document = window.document().expect("window should have a document");

            let canvas = document
                .get_element_by_id(&canvas_id.get_untracked())
                .expect("Document should have a canvas with the specified ID")
                .dyn_into::<HtmlCanvasElement>()
                .map_err(|_| ())
                .expect("Element with specified ID should be a canvas");

            let window_builder = WindowBuilder::new()
                .with_canvas(Some(canvas))
                .with_inner_size(winit::dpi::LogicalSize::new(288, 288))
                .with_prevent_default(true);

            let model = load_part_model.value().get().unwrap();

            insert_window.call((window_builder, model));
        }
    });

    view! {
        <div class="flex shadow my-2 p-4 h-80 bg-white rounded-xl overflow-hidden space-x-4">
            <div class="flex flex-col items-center justify-left">
                <div class="flex-shrink-0 w-72 space-y-1">
                    <canvas id=canvas_id class="rounded" ref=canvas_ref></canvas>
                </div>
            </div>
            <div class="flex-col grow grow">
                <div class="flex space-x-2">
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
                            <div class="ml-1">{reactive_part.model_file.get_untracked().name}</div>

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
                <div class="flex items-baseline mt-1">
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
                        let on_select = move |_| {
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
