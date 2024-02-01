use crate::api::parts::PartsClient;
use api_boundary::parts::models::Part;
use api_boundary::quotations::models::Quotation;
use leptos::*;

#[component]
pub fn QuotationsRow(#[prop(into)] quotation: Quotation) -> impl IntoView {
    // -- clients -- //

    let parts_client = use_context::<PartsClient>().unwrap_or(PartsClient::new());

    // -- signals -- //

    let expanded = create_rw_signal(false);
    let parts_for_quotation = create_rw_signal(Vec::<Part>::default());

    let query_parts_for_quotation = create_action(move |_| {
        let client_id = quotation.client_id.clone();
        let project_id = quotation.project_id.clone();
        let quotation_id = quotation.id.clone();

        async move {
            if parts_for_quotation.get_untracked().is_empty() {
                let result = parts_client
                    .query_parts_for_quotation(client_id, project_id, quotation_id)
                    .await;

                match result {
                    Ok(response) => parts_for_quotation.update(|p| *p = response.parts),
                    Err(_) => (), // TODO: Handle error.
                }
            }
        }
    });

    view! {
        <div class="flex border-b border-gray-200 bg-white text-sm">
            <button
                class="grow"
                on:click=move |_| {
                    expanded.update(|e| *e = !*e);
                    query_parts_for_quotation.dispatch(());
                }
            >

                <p class="ml-4 mx-2 my-5 text-gray-900 whitespace-no-wrap">Quotation ID</p>
                <Show
                    when=move || expanded.get()
                    fallback=|| {
                        view! {}
                    }
                >

                    <div class="flex justify-center items-center h-56 bg-red-300">
                        <h2>Here goes a table</h2>
                    </div>
                // <PartsTable parts=parts_for_quotation/>
                </Show>
            </button>
        </div>
    }
}
