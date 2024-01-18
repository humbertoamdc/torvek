use crate::components::parts::table_row::PartsTableRow;
use crate::models::reactive_part::ReactivePart;
use api_boundary::parts::models::Part;
use leptos::*;

#[component]
pub fn PartsTable(#[prop(into)] parts: RwSignal<Vec<Part>>) -> impl IntoView {
    view! {
        <div class="inline-block min-w-full shadow rounded-lg overflow-hidden">
            <table class="min-w-full leading-normal">
                <thead>
                    <tr>
                        <For
                            each=move || {
                                [
                                    "Part",
                                    "Process",
                                    "Material",
                                    "Tolerance",
                                    "Quantity",
                                    "Unit Price",
                                    "Subtotal",
                                ]
                                    .into_iter()
                                    .enumerate()
                            }

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
                        each=move || parts.get().into_iter().enumerate()
                        key=|(_, part)| part.id.clone()
                        children=move |(_, part)| {
                            let reactive_part = ReactivePart::from(&part);
                            view! { <PartsTableRow reactive_part=reactive_part/> }
                        }
                    />

                </tbody>
            </table>

        </div>
    }
}
