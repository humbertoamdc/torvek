use std::collections::HashMap;

use leptos::*;
use winit::window::WindowBuilder;

use crate::components::parts::part_visualizer::part_window;
use api_boundary::parts::models::{Part, PartQuote};

use crate::components::parts::table_row::PartsTableRow;
use crate::models::reactive_part::ReactivePart;

#[component]
pub fn PartsTable(
    #[prop(into)] parts: RwSignal<Vec<Part>>,
    #[prop(into)] part_quotes_by_part: RwSignal<HashMap<String, RwSignal<Vec<PartQuote>>>>,
    #[prop(into)] selected_quote_per_part: RwSignal<HashMap<String, RwSignal<Option<String>>>>,
) -> impl IntoView {
    // -- signals -- //

    let window_builders_and_models =
        create_rw_signal(Vec::<(WindowBuilder, three_d_asset::Model)>::new());

    // -- callbacks -- //

    let insert_window = move |tup: (WindowBuilder, three_d_asset::Model)| {
        window_builders_and_models
            .update(|window_builders_and_models| window_builders_and_models.push(tup));
    };

    view! {
        <div class="flex flex-col w-full overflow-hidden">

            <For
                each=move || parts.get().into_iter().enumerate()
                key=|(_, part)| part.id.clone()
                children=move |(_, part)| {
                    let reactive_part = ReactivePart::from(&part);
                    let part_quotes = part_quotes_by_part.get()[&part.id];
                    let selected_part_quote = selected_quote_per_part.get()[&part.id];
                    view! {
                        <PartsTableRow reactive_part part_quotes insert_window selected_part_quote/>
                    }
                }
            />

            {move || {
                let window_builders_and_models = window_builders_and_models.get();
                if !parts.get().is_empty() && window_builders_and_models.len() == parts.get().len()
                {
                    part_window(window_builders_and_models);
                }
            }}

        </div>
    }
}
