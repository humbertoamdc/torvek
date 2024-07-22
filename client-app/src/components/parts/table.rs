use http::StatusCode;
use std::collections::HashMap;

use leptos::*;
use winit::window::WindowBuilder;

use crate::components::parts::part_visualizer::part_window;
use api_boundary::parts::models::{Part, PartQuote};
use api_boundary::quotations::models::Quotation;
use clients::parts::PartsClient;

use crate::components::parts::table_row::PartsTableRow;
use crate::models::reactive_part::ReactivePart;

#[component]
pub fn PartsTable(
    #[prop(into)] parts: RwSignal<Vec<Part>>,
    #[prop(into)] part_quotes_by_part: RwSignal<HashMap<String, RwSignal<Vec<PartQuote>>>>,
    #[prop(into)] selected_quote_per_part: RwSignal<HashMap<String, RwSignal<Option<String>>>>,
    #[prop(into)] quotation: RwSignal<Option<Quotation>>,
) -> impl IntoView {
    // -- clients -- //

    let parts_client = use_context::<PartsClient>().unwrap();

    // -- signals -- //

    let window_builders_and_models =
        create_rw_signal(Vec::<(WindowBuilder, three_d_asset::Model)>::new());
    let part_models = create_rw_signal(Vec::<RwSignal<Option<three_d_asset::Model>>>::new());

    // -- callbacks -- //

    let insert_window = move |tup: (WindowBuilder, three_d_asset::Model)| {
        window_builders_and_models
            .update(|window_builders_and_models| window_builders_and_models.push(tup));
    };

    // -- actions -- //

    let _load_part_models = create_action(move |_| async move {
        loop {
            for (i, part_model) in part_models.get_untracked().into_iter().enumerate() {
                // TODO: Use presigned url to render file. We are double fetching the file, we can use the result obtained
                //       in the loop and create the RawAssets manually since using the `load_async` function is not working
                //       with presigned urls.
                let url = parts.get_untracked()[i].render_file.url.clone();
                // .replace(".glb", ".stl");
                let resp = parts_client.get_file_from_presigned_url(url.clone()).await;
                if resp.is_ok() && resp.unwrap().status() == StatusCode::OK {
                    let result = three_d_asset::io::load_async(&[url]).await;
                    part_model.update(|part_model| {
                        *part_model = Some(result.unwrap().deserialize("/").unwrap())
                    });
                }
            }
            gloo_timers::future::TimeoutFuture::new(1_000).await;
        }
    })
    .dispatch(());

    view! {
        <div class="flex flex-col w-full overflow-hidden">

            <For
                each=move || parts.get().into_iter().enumerate()
                key=|(_, part)| part.id.clone()
                children=move |(_, part)| {
                    let reactive_part = ReactivePart::from(&part);
                    let part_quotes = part_quotes_by_part.get()[&part.id];
                    let selected_part_quote = selected_quote_per_part.get()[&part.id];
                    let part_model = create_rw_signal(None::<three_d_asset::Model>);
                    part_models.update(|part_models| part_models.push(part_model));
                    view! {
                        <PartsTableRow
                            reactive_part
                            part_model
                            part_quotes
                            insert_window
                            selected_part_quote
                            quotation
                        />
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
