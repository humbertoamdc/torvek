use leptos::*;
use thaw::{Button, ButtonSize};

use api_boundary::parts::models::Part;

use crate::components::parts::table_row::PartsRow;

#[component]
pub fn PartsTable(#[prop(into)] parts: RwSignal<Vec<Part>>) -> impl IntoView {
    // -- derived signals -- //

    let submit_is_disabled =
        Signal::derive(move || !parts.get().iter().all(|part| part.unit_price.is_some()));

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
                                    "",
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
                        children=move |(i, part)| {
                            view! {
                                <PartsRow
                                    part=part.clone()
                                    on_change=move |unit_price: u64| {
                                        parts.update(|parts| parts[i].unit_price = Some(unit_price));
                                    }
                                />
                            }
                        }
                    />

                </tbody>
            </table>
        </div>
        <Button
            class="mt-4 self-end"
            size=ButtonSize::Large
            disabled=submit_is_disabled
            on_click=move |_| {
                // TODO: Send updated parts
            }
        >
            "Submit"
        </Button>
    }
}
