use leptos::html::Div;
use leptos::*;
use leptos_use::use_element_visibility;

use api_boundary::parts::models::Part;
use api_boundary::quotations::models::Quotation;

use crate::api::parts::PartsClient;
use crate::components::parts::table::PartsTable;

#[component]
pub fn CreatedQuotationsTableRow(#[prop(into)] quotation: Quotation) -> impl IntoView {
    // -- clients -- //

    let parts_client = use_context::<PartsClient>().unwrap_or(PartsClient::new());

    // -- signals -- //
    let parts = create_rw_signal(Vec::<Part>::new());
    let parts_table_ref = create_node_ref::<Div>();
    let is_visible = use_element_visibility(parts_table_ref);

    // -- actions -- //
    let query_parts_for_quotation = create_action(move |_| {
        let client_id = quotation.client_id.clone();
        let project_id = quotation.project_id.clone();
        let quotation_id = quotation.id.clone();
        async move {
            let result = parts_client
                .query_parts_for_quotation(client_id, project_id, quotation_id)
                .await;

            match result {
                Ok(response) => {
                    parts.update(|p| *p = response.parts);
                }
                Err(_) => (), // TODO: Handle error.
            }
        }
    });

    // -- effects -- //

    let _ = create_effect(move |_| {
        if is_visible.get() && parts.get_untracked().is_empty() {
            query_parts_for_quotation.dispatch(());
        }
    });

    view! {
        <div class="flex flex-col" ref=parts_table_ref>
            <PartsTable parts/>
        </div>
    }
}
