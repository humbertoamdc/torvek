use crate::api::quotations::QuotationsClient;
use api_boundary::quotations::requests::CreateQuotationRequest;
use leptos::*;
use leptos_router::use_params;
use leptos_router::IntoParam;
use leptos_router::Params;

#[derive(Params, PartialEq)]
struct QuotationsParams {
    project_id: Option<String>,
}

#[component]
pub fn Quotations() -> impl IntoView {
    // -- params -- //

    let params = use_params::<QuotationsParams>();
    let project_id = move || {
        params.with(|params| {
            params
                .as_ref()
                .map(|params| params.project_id.clone())
                .unwrap_or_default()
        })
    };

    // -- actions -- //

    let create_quotation = create_action(move |_| {
        let request = CreateQuotationRequest::new(project_id().unwrap_or_default());
        async move {
            let quotations_client = QuotationsClient::new();
            let result = quotations_client.create_quotation(request).await;

            match result {
                Ok(_) => (),
                Err(_) => (), // TODO: Handle error.
            }
        }
    });

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
    }
}
