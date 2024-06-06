use chrono::Utc;
use leptos::*;

use api_boundary::parts::models::PartQuote;

#[component]
pub fn PartQuoteCard(
    #[prop(into)] part_quote: PartQuote,
    #[prop(into)] is_selected: RwSignal<bool>,
    #[prop(into)] on_select: Callback<RwSignal<bool>>,
) -> impl IntoView {
    let button_class = move || {
        let mut class =
            String::from("grow flex justify-between items-center rounded-md border p-3 ");
        if is_selected.get() {
            class.push_str("border-blue-500 bg-blue-50");
        }
        class
    };

    view! {
        <button class=move || button_class() on:click=move |_| on_select.call(is_selected)>
            <div>
                <p class="text-sm font-bold text-left text-gray-700">"Expedite"</p>
                <p class="text-sm font-semibold text-gray-500">
                    {(part_quote.deadline - Utc::now().date_naive()).num_days()} " business days"
                </p>
            </div>
            <div>
                <p class="text-sm font-semibold text-right text-gray-500">
                    {part_quote.unit_price.to_string()} " each"
                </p>
                <p class="text-2xl font-bold text-gray-700">{part_quote.sub_total.to_string()}</p>
            </div>
        </button>
    }
}
