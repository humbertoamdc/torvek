use leptos::*;

use api_boundary::quotations::models::Quotation;

use crate::components::quotations::table_row::QuotationsRow;

#[component]
pub fn QuotationsTable(#[prop(into)] quotations: RwSignal<Vec<Quotation>>) -> impl IntoView {
    view! {
        <div class="inline-block min-w-full shadow rounded-lg overflow-hidden">
            <table class="min-w-full leading-normal">
                <tbody>
                    <For
                        each=move || quotations.get().into_iter().enumerate()
                        key=|(_, quotation)| quotation.id.clone()
                        children=move |(_, quotation)| {
                            view! { <QuotationsRow quotation/> }
                        }
                    />

                </tbody>
            </table>
        </div>
    }
}
