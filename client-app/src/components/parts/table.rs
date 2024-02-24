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
        <div class="flex flex-col w-full overflow-hidden">

            <For
                each=move || parts.get().into_iter().enumerate()
                key=|(_, part)| part.id.clone()
                children=move |(_, part)| {
                    let reactive_part = ReactivePart::from(&part);
                    let part_quotes = part_quotes_by_part.get()[&part.id];
                    view! { <PartsTableRow reactive_part part_quotes/> }
                }
            />

        </div>
    }
}
