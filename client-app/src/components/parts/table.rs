use std::collections::HashMap;

use leptos::*;

use api_boundary::parts::models::{Part, PartQuote};

use crate::components::parts::table_row::PartsTableRow;
use crate::models::reactive_part::ReactivePart;

#[component]
pub fn PartsTable(
    #[prop(into)] parts: RwSignal<Vec<Part>>,
    #[prop(into)] part_quotes_by_part: RwSignal<HashMap<String, RwSignal<Vec<PartQuote>>>>,
) -> impl IntoView {
    view! {
        <div class="inline-block w-full shadow rounded-lg overflow-hidden">
            <table class="w-full">
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
                                    <th class="px-2 py-3 border-b-1 border-gray-600 bg-gray-300 text-left text-xs font-semibold text-gray-800 uppercase tracking-wider">
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
                            let part_quotes = part_quotes_by_part.get()[&part.id];
                            view! { <PartsTableRow reactive_part part_quotes/> }
                        }
                    />

                </tbody>
            </table>

        </div>
    }
}
