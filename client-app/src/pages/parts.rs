use leptos::*;
use leptos_router::*;
use thaw::{Breadcrumb, BreadcrumbItem, Button, Upload};
use web_sys::FileList;

use api_boundary::parts::models::Part;
use api_boundary::parts::requests::CreatePartsRequest;
use api_boundary::payments::requests::CreateCheckoutSessionRequest;
use api_boundary::quotations::models::{Quotation, QuotationStatus};

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

    let quotation = create_rw_signal(None::<Quotation>);
    let parts = create_rw_signal(Vec::<Part>::default());
    let checkout_button_disabled = Signal::derive(move || {
        quotation.get_untracked().is_none()
            || quotation.get_untracked().unwrap().status != QuotationStatus::PendingPayment
            || parts.get().is_empty()
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

    let create_parts = create_action(move |file_list: &FileList| {
        let file_list = file_list.clone();
        let mut file_names: Vec<String> = Vec::with_capacity(file_list.length() as usize);
        for i in 0..file_list.length() {
            if let Some(file) = file_list.item(i) {
                file_names.push(file.name());
            }
        }

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
            query_parts_callback.call(());
        }
    });

    let create_checkout_session = create_action(move |_| async move {
        let payments_client = PaymentsClient::new();
        let request = CreateCheckoutSessionRequest {
            client_id: user_info.get_untracked().id,
            project_id: project_id().unwrap(),
            quotation_id: quotation_id().unwrap(),
        };

        let response = payments_client.create_checkout_session(request).await;
        match response {
            Ok(response) => {
                let _ = window().location().set_href(&response.url);
            }
            Err(_) => (), // TODO: Handle error.
        }
    });

    // -- derived signals -- //

    let is_creating_checkout_session =
        Signal::derive(move || create_checkout_session.pending().get());

    query_parts.dispatch(());

    view! {
        <Breadcrumb>
            <BreadcrumbItem>
                <button on:click=move |_| {
                    let navigate = use_navigate();
                    navigate("/projects", Default::default())
                }>"Projects"</button>
            </BreadcrumbItem>
            <BreadcrumbItem>
                <button on:click=move |_| {
                    let navigate = use_navigate();
                    let path = format!("/projects/{}/quotations", project_id().unwrap());
                    navigate(&path, Default::default())
                }>"Quotations"</button>
            </BreadcrumbItem>
            <BreadcrumbItem>"Parts"</BreadcrumbItem>
        </Breadcrumb>

        <header class="flex justify-between items-center py-4">
            <h1 class="text-3xl font-bold text-gray-900">Parts</h1>
        </header>

        <div class="flex justify-between">

            <Upload
                accept=".stp,.step"
                multiple=true
                custom_request=move |file_list| create_parts.dispatch(file_list)
            >
                <Button>"Create Parts"</Button>
            </Upload>

            <Button
                loading=is_creating_checkout_session
                disabled=checkout_button_disabled
                on_click=move |_| create_checkout_session.dispatch(())
            >
                "Checkout"
            </Button>
        </div>

        <div class="mt-8"></div>
        <PartsTable parts=parts/>
    }
}
