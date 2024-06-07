use std::collections::HashMap;

use leptos::*;
use leptos_router::*;
use thaw::{Breadcrumb, BreadcrumbItem, Button, Upload};
use web_sys::FileList;

use api_boundary::parts::models::{Part, PartQuote};
use api_boundary::parts::requests::{CreatePartsRequest, QueryPartQuotesForPartsRequest};
use api_boundary::payments::requests::CreateCheckoutSessionRequest;
use api_boundary::quotations::models::Quotation;
use clients::parts::PartsClient;
use clients::quotations::QuotationsClient;

use crate::api::models::auth::UserInfo;
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
    // -- clients -- //

    let parts_client = use_context::<PartsClient>().unwrap();
    let quotations_client = use_context::<QuotationsClient>().unwrap();

    // -- context -- //

    let user_info = use_context::<RwSignal<UserInfo>>().expect("user info to be provided");

    // -- signals -- //

    let quotation = create_rw_signal(None::<Quotation>);
    let parts = create_rw_signal(Vec::<Part>::default());
    let selected_quote_per_part =
        create_rw_signal(HashMap::<String, RwSignal<Option<String>>>::new());
    let part_quotes_by_part =
        create_rw_signal(HashMap::<String, RwSignal<Vec<PartQuote>>>::default());
    let checkout_button_disabled = Signal::derive(move || {
        selected_quote_per_part.get().is_empty()
            || selected_quote_per_part
                .get()
                .iter()
                .any(|(_, selected_quote)| selected_quote.get().is_none())
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

    let _query_quotation = create_action(move |_| {
        let client_id = user_info.get_untracked().id;
        let project_id = project_id().unwrap();
        let quotation_id = quotation_id().unwrap();
        async move {
            match quotations_client
                .get_quotation_by_id(client_id, project_id, quotation_id)
                .await
            {
                Ok(quotation_response) => {
                    quotation.update(|quotation| *quotation = Some(quotation_response))
                }
                Err(_) => (),
            }
        }
    })
    .dispatch(());

    let query_part_quotes_for_parts = create_action(move |_| {
        let part_ids = parts
            .get_untracked()
            .into_iter()
            .map(|part| part.id)
            .collect::<Vec<String>>();
        let request = QueryPartQuotesForPartsRequest { part_ids };

        async move {
            let result = parts_client.query_part_quotes_for_parts(request).await;

            match result {
                Ok(response) => {
                    part_quotes_by_part.update(|part_quotes_by_part| {
                        response.part_quotes_by_part_id.into_iter().for_each(
                            |(part_id, part_quotes)| {
                                part_quotes_by_part
                                    .get(&part_id)
                                    .unwrap()
                                    .update(|local_part_quotes| *local_part_quotes = part_quotes);
                            },
                        );
                    });
                }
                Err(_) => (),
            }
        }
    });

    let query_parts = create_action(move |_| async move {
        let result = parts_client
            .query_parts_for_quotation(
                user_info.get_untracked().id,
                project_id().unwrap_or_default(),
                quotation_id().unwrap_or_default(),
            )
            .await;

        match result {
            Ok(response) => {
                part_quotes_by_part.update(|part_quotes_by_part| {
                    *part_quotes_by_part = response
                        .parts
                        .iter()
                        .map(|part| (part.id.clone(), create_rw_signal(Vec::default())))
                        .collect();
                });
                selected_quote_per_part.update(|selected_quote_per_part| {
                    *selected_quote_per_part = response
                        .parts
                        .iter()
                        .map(|part| (part.id.clone(), create_rw_signal(None)))
                        .collect()
                });
                parts.update(|p| *p = response.parts.clone());
                query_part_quotes_for_parts.dispatch(());
            }
            Err(_) => (), // TODO: Handle error.
        }
    });

    let query_parts_callback = Callback::<()>::new(move |_| {
        query_parts.dispatch(());
    });

    let create_parts = create_action(move |file_list: &FileList| {
        let file_list = file_list.clone();
        let mut file_names: Vec<String> = Vec::with_capacity(file_list.length() as usize);
        let mut files = Vec::with_capacity(file_list.length() as usize);
        for i in 0..file_list.length() {
            if let Some(file) = file_list.item(i) {
                file_names.push(file.name());
                files.push(file.clone())
            }
        }

        let request = CreatePartsRequest::new(
            String::from(user_info.get_untracked().id),
            project_id().unwrap_or_default(),
            quotation_id().unwrap_or_default(),
            file_names.to_owned(),
        );

        async move {
            match parts_client.create_parts(request).await {
                Ok(response) => {
                    for (i, file) in files.into_iter().enumerate() {
                        parts_client
                            .upload_file_with_presigned_url(
                                file,
                                response.upload_urls[i as usize].clone(),
                            )
                            .await
                            .expect("error while uploading file with presigned url");
                    }
                }
                Err(_) => (), // TODO: Handle error.
            }
            query_parts_callback.call(());
        }
    });

    let create_checkout_session = create_action(move |_| async move {
        let payments_client = PaymentsClient::new();
        let selected_quotes_per_part = selected_quote_per_part
            .get_untracked()
            .iter()
            .map(|(part_id, quote_id)| (part_id.clone(), quote_id.get_untracked().unwrap()))
            .collect();
        let request = CreateCheckoutSessionRequest {
            client_id: user_info.get_untracked().id,
            project_id: project_id().unwrap(),
            quotation_id: quotation_id().unwrap(),
            selected_quotes_per_part,
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
            <h1 class="text-4xl font-bold text-gray-900">
                {
                    move || {
                        match quotation.get() {
                            Some(quotation) => quotation.name,
                            None => String::default(),
                        }
                    }
                }
            </h1>
        </header>

        <header class="flex justify-between items-center py-4">
            <h1 class="text-2xl font-bold text-gray-900">Parts</h1>
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
        <PartsTable parts part_quotes_by_part selected_quote_per_part quotation/>
    }
}
