use api_boundary::quotations::models::Quotation;
use leptos::*;
use leptos_router::use_navigate;

#[component]
pub fn QuotationButton(quotation: Quotation) -> impl IntoView {
    let navigate = use_navigate();
    let parts_for_quotation_url = format!(
        "/projects/{}/quotations/{}/parts",
        quotation.project_id, quotation.id
    );

    view! {
        <button
            class="rounded-md bg-neutral-50 hover:bg-neutral-200 w-48 h-48"
            on:click=move |_| navigate(&parts_for_quotation_url, Default::default())
        >
            New Quotation
        </button>
    }
}
