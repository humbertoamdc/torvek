use api_boundary::quotations::models::Quotation;
use leptos::*;

#[component]
pub fn QuotationButton(quotation: Quotation) -> impl IntoView {
    view! {
        <button
            class="rounded-md bg-neutral-50 hover:bg-neutral-200 w-48 h-48"
        >
            New Quotation
        </button>
    }
}
