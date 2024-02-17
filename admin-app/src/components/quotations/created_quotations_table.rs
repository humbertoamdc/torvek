use leptos::*;
use thaw::{Card, Collapse, CollapseItem};

use api_boundary::quotations::models::Quotation;

use crate::components::quotations::created_quotations_table_row::CreatedQuotationsTableRow;

#[component]
pub fn CreatedQuotationsTable(#[prop(into)] quotations: RwSignal<Vec<Quotation>>) -> impl IntoView {
    view! {
        <Card class="inline-block min-w-full shadow rounded-lg overflow-hidden">
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
                                <CreatedQuotationsTableRow quotation/>
                            </CollapseItem>
                        }
                    }
                />

            </Collapse>
        </Card>
    }
}
