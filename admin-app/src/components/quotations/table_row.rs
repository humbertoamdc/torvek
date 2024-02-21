use chrono::NaiveDate;
use leptos::*;

use api_boundary::common::money::Money;
use api_boundary::parts::models::Part;
use api_boundary::quotations::models::Quotation;
use clients::parts::PartsClient;

#[derive(Debug, Clone)]
pub struct PartToOrderData {
    pub part: Part,
    pub payment: RwSignal<Option<Money>>,
    pub deadline: RwSignal<Option<NaiveDate>>,
}

#[component]
pub fn QuotationsRow(#[prop(into)] quotation: Quotation) -> impl IntoView {
    let (quotation, _) = create_signal(quotation);

    // -- clients -- //

    let parts_client = use_context::<PartsClient>().unwrap();

    // -- signals -- //

    let expanded = create_rw_signal(false);
    let parts_to_orders_data_for_quotation = create_rw_signal(Vec::<PartToOrderData>::default());

    let query_parts_for_quotation = create_action(move |_| {
        async move {
            if parts_to_orders_data_for_quotation
                .get_untracked()
                .is_empty()
            {
                let result = parts_client
                    .query_parts_for_quotation(
                        quotation.get_untracked().client_id,
                        quotation.get_untracked().project_id,
                        quotation.get_untracked().id,
                    )
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
                    "Quotation with ID: " {quotation.get_untracked().id}
                </p>

            </button>
        // <Show
        // when=move || expanded.get()
        // fallback=|| {
        // view! {}
        // }
        // >

        // <PartToOrderTable
        // quotation=quotation.get_untracked()
        // parts_to_orders_data=parts_to_orders_data_for_quotation
        // remove_quotation
        // />
        // </Show>
        </div>
    }
}
