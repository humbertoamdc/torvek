use crate::api::models::auth::UserInfo;
use crate::api::quotations::QuotationsClient;
use crate::components::quotations::quotation_button::QuotationButton;
use api_boundary::quotations::models::Quotation;
use api_boundary::quotations::requests::CreateQuotationRequest;
use leptos::*;
use leptos_router::*;

#[derive(Params, PartialEq)]
struct QuotationsParams {
    project_id: Option<String>,
}

#[component]
pub fn QuotationsContainer() -> impl IntoView {
    view! { <Outlet/> }
}

#[component]
pub fn Quotations() -> impl IntoView {
    // -- context -- //

    let user_info = use_context::<RwSignal<UserInfo>>().expect("user info to be provided");

    // -- signals -- //

    let quotations = create_rw_signal(Vec::<Quotation>::default());

    // -- params -- //

    let params = use_params::<QuotationsParams>();
    let project_id = move || {
        params.with_untracked(|params| {
            params
                .as_ref()
                .map(|params| params.project_id.clone())
                .unwrap_or_default()
        })
    };

    // -- actions -- //

    let query_quotations = create_action(move |_| {
        async move {
            let quotations_client = QuotationsClient::new();
            let result = quotations_client
                .query_quotations_for_project(
                    user_info.get_untracked().id,
                    project_id().unwrap_or_default(),
                )
                .await;

            match result {
                Ok(response) => quotations.update(|p| *p = response.quotations),
                Err(_) => (), // TODO: Handle error.
            }
        }
    });

    let query_quotations_callback = Callback::<()>::new(move |_| {
        query_quotations.dispatch(());
    });

    let create_quotation = create_action(move |_| {
        let request = CreateQuotationRequest::new(
            user_info.get_untracked().id,
            project_id().unwrap_or_default(),
        );
        async move {
            let quotations_client = QuotationsClient::new();
            let result = quotations_client.create_quotation(request).await;

            match result {
                Ok(_) => query_quotations_callback.call(()),
                Err(_) => (), // TODO: Handle error.
            }
        }
    });

    query_quotations.dispatch(());

    view! {
        <header class="flex justify-between items-center py-4">
            <h1 class="text-3xl font-bold text-gray-900">Quotations</h1>
        </header>

        <button
            type="submit"
            class="flex justify-center rounded-md bg-indigo-600 px-3 py-1.5 text-sm font-semibold text-white shadow-sm hover:bg-indigo-500 focus-visible:outline focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-indigo-600"
            on:click=move |_| {
                create_quotation.dispatch(());
            }
        >

            "New Quotation"
        </button>

        <div class="mt-8 flex flex-wrap gap-4">
            <For
                each=move || quotations.get().into_iter()
                key=|project| project.id.clone()
                children=move |quotation| {
                    view! { <QuotationButton quotation=quotation/> }
                }
            />

        </div>
    }
}
