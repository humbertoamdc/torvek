use leptos::*;
use thaw::{Card, Collapse, CollapseItem};

use api_boundary::quotations::models::Quotation;

use crate::components::parts::part_quotes_table::PartQuotesTable;

#[component]
pub fn CreatedQuotationsCollapsible(
    #[prop(into)] quotations: RwSignal<Vec<Quotation>>,
) -> impl IntoView {
    view! {
        <Card class="min-w-full border-0 bg-inherit">
            <Collapse accordion=true>
                <For
                    each=move || quotations.get().into_iter().enumerate()
                    key=|(_, quotation)| quotation.id.clone()
                    children=move |(_, quotation)| {
                        view! {
                            <CollapseItem
                                title=format!("Quotation with ID {}", quotation.id.clone())
                                key=quotation.id.clone()
                            >
                                <PartQuotesTable quotation/>
                            </CollapseItem>
                        }
                    }
                />

            </Collapse>
        </Card>
    }
}
