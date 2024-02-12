use leptos::*;
use leptos_router::*;
use web_sys::HtmlInputElement;

use api_boundary::parts::models::{Part, PartStatus};
use api_boundary::parts::requests::CreatePartsRequest;
use api_boundary::payments::requests::{
    CreateCheckoutSessionPartData, CreateCheckoutSessionRequest,
};

use crate::api::models::auth::UserInfo;
use crate::api::parts::PartsClient;
use crate::api::payments::PaymentsClient;
use crate::components::parts::table::PartsTable;

#[derive(Params, PartialEq)]
struct PartsParams {
    project_id: Option<String>,
    quotation_id: Option<String>,
}

#[component]
pub fn PartsContainer() -> impl IntoView {
    view! { <Outlet/> }
}

#[component]
pub fn Parts() -> impl IntoView {
    // -- context -- //

    let user_info = use_context::<RwSignal<UserInfo>>().expect("user info to be provided");

    // -- signals -- //

    let parts = create_rw_signal(Vec::<Part>::default());
    let checkout_button_disabled = Signal::derive(move || {
        let parts_awaiting_pricing = parts
            .get()
            .iter()
            .map(|part| part.status.clone())
            .collect::<Vec<PartStatus>>()
            .contains(&PartStatus::AwaitingPricing);

        parts.get().is_empty() || parts_awaiting_pricing
    });

    // -- params -- //

    let params = use_params::<PartsParams>();
    let project_id = move || {
        params.with_untracked(|params| {
            params
                .as_ref()
                .map(|params| params.project_id.clone())
                .unwrap_or_default()
        })
    };

    let quotation_id = move || {
        params.with_untracked(|params| {
            params
                .as_ref()
                .map(|params| params.quotation_id.clone())
                .unwrap_or_default()
        })
    };

    // -- action -- //

    let query_parts = create_action(move |_| async move {
        let parts_client = PartsClient::new();
        let result = parts_client
            .query_parts_for_quotation(
                user_info.get_untracked().id,
                project_id().unwrap_or_default(),
                quotation_id().unwrap_or_default(),
            )
            .await;

        match result {
            Ok(response) => parts.update(|p| *p = response.parts),
            Err(_) => (), // TODO: Handle error.
        }
    });

    let query_parts_callback = Callback::<()>::new(move |_| {
        query_parts.dispatch(());
    });

    let create_parts = create_action(move |input_element: &HtmlInputElement| {
        let file_list = input_element.clone().files().unwrap();
        let mut file_names: Vec<String> = Vec::with_capacity(file_list.length() as usize);
        for i in 0..file_list.length() {
            if let Some(file) = file_list.item(i) {
                file_names.push(file.name());
            }
        }

        let input_element = input_element.clone();
        let request = CreatePartsRequest::new(
            String::from(user_info.get_untracked().id),
            project_id().unwrap_or_default(),
            quotation_id().unwrap_or_default(),
            file_names.to_owned(),
        );
        async move {
            let parts_client = PartsClient::new();
            match parts_client.create_parts(request).await {
                Ok(response) => {
                    for i in 0..file_list.length() {
                        if let Some(file) = file_list.item(i) {
                            parts_client
                                .upload_file_with_presigned_url(
                                    file,
                                    response.upload_urls[i as usize].clone(),
                                )
                                .await
                                .expect("error while uploading file with presigned url");
                        }
                    }
                }
                Err(_) => (), // TODO: Handle error.
            }
            input_element.set_value("");
            query_parts_callback.call(());
        }
    });

    let create_checkout_session = create_action(move |_| async move {
        let payments_client = PaymentsClient::new();
        let request = CreateCheckoutSessionRequest {
            client_id: user_info.get_untracked().id,
            project_id: project_id().unwrap(),
            quotation_id: quotation_id().unwrap(),
            data: parts
                .get_untracked()
                .iter()
                .map(|part| CreateCheckoutSessionPartData::from(part))
                .collect(),
        };

        let response = payments_client.create_checkout_session(request).await;
        match response {
            Ok(response) => {
                let _ = window().location().set_href(&response.url);
            }
            Err(_) => (), // TODO: Handle error.
        }
    });

    query_parts.dispatch(());

    view! {
        <header class="flex justify-between items-center py-4">
            <h1 class="text-3xl font-bold text-gray-900">Parts</h1>
        </header>

        <div class="flex justify-between">
            <label
                for="dropzone-file"
                class="justify-center rounded-md bg-indigo-600 d px-3 py-2 text-sm font-semibold text-white shadow-sm hover:bg-indigo-500 focus-visible:outline focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-indigo-600 hover:cursor-pointer"
            >
                <input
                    id="dropzone-file"
                    type="file"
                    class="hidden"
                    accept=".stp,.step"
                    multiple
                    on:change=move |ev| {
                        let input_element = event_target::<HtmlInputElement>(&ev);
                        create_parts.dispatch(input_element);
                    }
                />

                Create Parts
            </label>
            <button
                type="submit"
                class="flex justify-center rounded-md bg-indigo-600 px-3 py-1.5 text-sm font-semibold text-white shadow-sm hover:bg-indigo-500 focus-visible:outline focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-indigo-600"
                disabled=checkout_button_disabled
                on:click=move |_| {
                    create_checkout_session.dispatch(());
                }
            >

                "Checkout"
            </button>
        </div>

        <div class="mt-8"></div>
        <PartsTable parts=parts/>
    }
}
