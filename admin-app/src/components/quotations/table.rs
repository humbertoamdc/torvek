use crate::components::quotations::table_row::QuotationsRow;
use api_boundary::quotations::models::Quotation;
use leptos::*;

#[component]
pub fn QuotationsTable(#[prop(into)] quotations: RwSignal<Vec<Quotation>>) -> impl IntoView {
    view! {
        <div class="inline-block min-w-full shadow rounded-lg overflow-hidden">
            <table class="min-w-full leading-normal">
                <thead>
                    <tr>
                        <For
                            each=move || { ["Id"].into_iter().enumerate() }
                            key=|(_, column_name)| column_name.to_string()
                            children=move |(_, column_name)| {
                                view! {
                                    <th class="px-2 py-3 border-b-2 border-gray-200 bg-gray-100 text-left text-xs font-semibold text-gray-600 uppercase tracking-wider">
                                        <div class="flex justify-center">{column_name}</div>
                                    </th>
                                }
                            }
                        />

                    </tr>
                </thead>
                <tbody>
                    <For
                        each=move || quotations.get().into_iter().enumerate()
                        key=|(_, quotation)| quotation.id.clone()
                        children=move |(_, quotation)| {
                            view! { <QuotationsRow quotation=quotation/> }
                        }
                    />

                </tbody>
            </table>
        </div>
    }
}
