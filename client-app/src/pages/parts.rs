use crate::api::models::auth::UserInfo;
use crate::api::models::orders::CreateOrdersRequest;
use crate::api::orders::OrdersClient;
use crate::api::parts::PartsClient;
use crate::api::quotations::QuotationsClient;
use api_boundary::parts::requests::CreatePartsRequest;
use api_boundary::quotations::requests::CreateQuotationRequest;
use leptos::*;
use leptos_router::*;
use web_sys::HtmlInputElement;

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
                            // TODO: Upload files using presigned urls.
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
                Err(err) => log::error!("{err:?}"),
            }
            input_element.set_value("");
        }
    });

    view! {
        <header class="flex justify-between items-center py-4">
            <h1 class="text-3xl font-bold text-gray-900">Parts</h1>
        </header>

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
    }
}
