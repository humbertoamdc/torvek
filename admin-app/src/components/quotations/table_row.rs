use crate::api::parts::PartsClient;
use crate::components::parts::part_to_order_table::PartToOrderTable;
use api_boundary::common::money::Money;
use api_boundary::parts::models::Part;
use api_boundary::quotations::models::Quotation;
use chrono::NaiveDate;
use leptos::*;

#[derive(Debug, Clone)]
pub struct PartToOrderData {
    pub part: Part,
    pub payment: RwSignal<Option<Money>>,
    pub deadline: RwSignal<Option<NaiveDate>>,
}

#[component]
pub fn QuotationsRow(
    #[prop(into)] quotation: Quotation,
    #[prop(into)] remove_quotation: Callback<()>,
) -> impl IntoView {
    let quotation_id = quotation.id.clone();

    // -- clients -- //

    let parts_client = use_context::<PartsClient>().unwrap_or(PartsClient::new());

    // -- signals -- //

    let expanded = create_rw_signal(false);
    let parts_to_orders_data_for_quotation = create_rw_signal(Vec::<PartToOrderData>::default());

    let query_parts_for_quotation = create_action(move |_| {
        let client_id = quotation.client_id.clone();
        let project_id = quotation.project_id.clone();
        let quotation_id = quotation.id.clone();

        async move {
            if parts_to_orders_data_for_quotation
                .get_untracked()
                .is_empty()
            {
                let result = parts_client
                    .query_parts_for_quotation(client_id, project_id, quotation_id)
                    .await;

                match result {
                    Ok(response) => {
                        let parts_to_orders_data = response
                            .parts
                            .into_iter()
                            .map(|part| PartToOrderData {
                                part,
                                payment: create_rw_signal(None),
                                deadline: create_rw_signal(None),
                            })
                            .collect::<Vec<PartToOrderData>>();

                        parts_to_orders_data_for_quotation.update(|p| *p = parts_to_orders_data);
                    }
                    Err(_) => (), // TODO: Handle error.
                }
            }
        }
    });

    view! {
        <div class="flex flex-col border-b border-gray-200 bg-white text-sm">
            <button
                class="grow"
                on:click=move |_| {
                    expanded.update(|e| *e = !*e);
                    query_parts_for_quotation.dispatch(());
                }
            >

                <p class="ml-4 mx-2 my-5 text-gray-900 whitespace-no-wrap">
                    "Quotation with ID: " {quotation_id}
                </p>

            </button>
            <Show
                when=move || expanded.get()
                fallback=|| {
                    view! {}
                }
            >

                <PartToOrderTable
                    parts_to_orders_data=parts_to_orders_data_for_quotation
                    remove_quotation
                />
            </Show>
        </div>
    }
}
