use std::collections::HashMap;

use leptos::html::Div;
use leptos::*;
use leptos_use::use_element_visibility;
use thaw::{Button, ButtonSize};

use crate::clients::parts::{CreatePartQuotesRequest, CreatePartQuotesRequestData, PartsClient};
use crate::components::parts::part_quotes_table_row::PartQuotesTableRow;
use crate::models::money::Money;
use crate::models::part::Part;
use crate::models::quotation::Quotation;

#[component]
pub fn PartQuotesTable(
    #[prop(into)] quotation: Quotation,
    #[prop(into)] on_create: Callback<()>,
) -> impl IntoView {
    // -- variables --//

    let customer_id = quotation.customer_id.clone();
    let project_id = quotation.project_id.clone();
    let quotation_id = quotation.id.clone();

    // -- clients -- //

    let parts_client = use_context::<PartsClient>().unwrap();

    // -- signals -- //

    let parts = create_rw_signal(Vec::<Part>::new());
    let prices_options_list = create_rw_signal(Vec::<Vec<RwSignal<Option<Money>>>>::default());
    let workdays_to_complete_list = create_rw_signal(Vec::<Vec<RwSignal<u64>>>::default());
    let parts_table_ref = create_node_ref::<Div>();
    let is_visible = use_element_visibility(parts_table_ref);

    // -- actions -- //

    let query_parts_for_quotation = create_action(move |_| {
        let quotation_id = quotation.id.clone();
        async move {
            let result = parts_client
                .admin_query_parts_for_quotation(quotation_id)
                .await;

            match result {
                Ok(response) => {
                    parts.update(|parts| *parts = response.parts);
                }
                Err(_) => (), // TODO: Handle error.
            }
        }
    });

    let create_part_quotes = create_action(move |_| {
        let parts_prices_map = parts
            .get_untracked()
            .into_iter()
            .zip(prices_options_list.get_untracked())
            .map(|(part, price_options)| (part.id, price_options))
            .collect::<HashMap<String, Vec<RwSignal<Option<Money>>>>>();
        let parts_deadlines_map = parts
            .get_untracked()
            .into_iter()
            .zip(workdays_to_complete_list.get_untracked())
            .map(|(part, deadlines)| (part.id, deadlines))
            .collect::<HashMap<String, Vec<RwSignal<u64>>>>();

        let mut price_data: Vec<CreatePartQuotesRequestData> = Vec::new();

        parts.get_untracked().into_iter().for_each(|part| {
            parts_prices_map
                .get(&part.id)
                .unwrap()
                .into_iter()
                .zip(parts_deadlines_map.get(&part.id).unwrap())
                .for_each(|(price_option, workdays_to_complete)| {
                    let sub_total = price_option.get_untracked().unwrap();
                    let mut unit_price = sub_total.clone();
                    unit_price.amount = sub_total.amount / part.quantity as i64;

                    price_data.push(CreatePartQuotesRequestData {
                        part_id: part.id.clone(),
                        unit_price,
                        sub_total,
                        workdays_to_complete: workdays_to_complete.get_untracked(),
                        quantity: part.quantity,
                    });
                });
        });

        let request = CreatePartQuotesRequest {
            customer_id: customer_id.clone(),
            project_id: project_id.clone(),
            quotation_id: quotation_id.clone(),
            data: price_data,
        };

        async move {
            let result = parts_client.admin_create_part_quotes(request).await;

            match result {
                Ok(_) => on_create.call(()),
                Err(_) => (),
            }
        }
    });

    // -- effects -- //

    let _ = create_effect(move |_| {
        if is_visible.get() && parts.get_untracked().is_empty() {
            query_parts_for_quotation.dispatch(());
        }
    });

    // -- derived signals -- //

    let submit_is_disabled = Signal::derive(move || {
        parts.get().is_empty()
            || !workdays_to_complete_list
                .get()
                .iter()
                .all(|deadline_options| {
                    deadline_options
                        .iter()
                        .all(|deadline_option| deadline_option.get() > 0)
                })
            || !prices_options_list.get().iter().all(|price_options| {
                price_options
                    .iter()
                    .all(|price_option| price_option.get().is_some())
            })
    });

    view! {
        <div class="flex flex-col" ref=parts_table_ref>
            <For
                each=move || parts.get().into_iter().enumerate()
                key=|(_, part)| part.id.clone()
                children=move |(_, part)| {
                    let price_options = vec![
                        create_rw_signal(None::<Money>),
                        create_rw_signal(None::<Money>),
                        create_rw_signal(None::<Money>),
                    ];
                    let workdays_to_complete_options = vec![
                        create_rw_signal(0_u64),
                        create_rw_signal(0_u64),
                        create_rw_signal(0_u64),
                    ];
                    prices_options_list.update(|prices| prices.push(price_options.clone()));
                    workdays_to_complete_list
                        .update(|workdays_to_complete| {
                            workdays_to_complete.push(workdays_to_complete_options.clone())
                        });
                    view! {
                        <PartQuotesTableRow
                            part=part.clone()
                            price_options
                            workdays_to_complete_options
                        />
                    }
                }
            />

            <Button
                class="mt-4 self-end"
                size=ButtonSize::Large
                disabled=submit_is_disabled
                on_click=move |_| create_part_quotes.dispatch(())
            >

                "Submit"
            </Button>
        </div>
    }
}
