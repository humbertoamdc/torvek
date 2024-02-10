use crate::components::parts::part_to_order_table_row::PartToOrderRow;
use crate::components::quotations::table_row::PartToOrderData;
use api_boundary::orders::requests::{AdminCreateOrdersRequest, AdminCreateOrdersRequestData};
use api_boundary::quotations::models::Quotation;
use chrono::NaiveDate;
use clients::admin_orders::AdminOrdersClient;
use leptos::*;

#[component]
pub fn PartToOrderTable(
    #[prop(into)] quotation: Quotation,
    #[prop(into)] parts_to_orders_data: RwSignal<Vec<PartToOrderData>>,
    #[prop(into)] remove_quotation: Callback<()>,
) -> impl IntoView {
    // -- clients -- //

    let orders_client = use_context::<AdminOrdersClient>().unwrap();

    // -- signals -- //

    let is_ready = move || {
        parts_to_orders_data
            .get()
            .iter()
            .all(|p| p.payment.get().is_some() && p.deadline.get().is_some())
    };

    // -- actions -- //

    let create_orders_from_parts = create_action(move |_| {
        let data = parts_to_orders_data
            .get()
            .into_iter()
            .map(|part_to_order_data| {
                let naive_date = NaiveDate::from(
                    part_to_order_data
                        .deadline
                        .get()
                        .expect("deadline should be set"),
                );

                AdminCreateOrdersRequestData {
                    part_id: part_to_order_data.part.id,
                    model_file: part_to_order_data.part.model_file,
                    payment: part_to_order_data
                        .payment
                        .get()
                        .expect("money should have a value"),
                    deadline: naive_date,
                }
            })
            .collect();

        let request = AdminCreateOrdersRequest {
            client_id: quotation.client_id.clone(),
            project_id: quotation.project_id.clone(),
            quotation_id: quotation.id.clone(),
            data,
        };

        async move {
            match orders_client.create_order(request).await {
                Ok(_) => remove_quotation.call(()),
                Err(_) => (),
            }
        }
    });

    view! {
        <div class="inline-block min-w-full shadow rounded-lg overflow-hidden">
            <table class="min-w-full leading-normal">
                <thead>
                    <tr>
                        <For
                            each=move || {
                                [
                                    "Part",
                                    "Process",
                                    "Material",
                                    "Tolerance",
                                    "Quantity",
                                    "Unit Price",
                                    "Subtotal",
                                    "Payment",
                                    "Deadline",
                                ]
                                    .into_iter()
                                    .enumerate()
                            }

                            key=|(_, column_name)| column_name.to_string()
                            children=move |(_, column_name)| {
                                view! {
                                    <th class="px-2 py-3 border-b-2 border-gray-200 bg-gray-100 text-left text-xs font-semibold text-gray-600 uppercase tracking-wider">
                                        <div class="flex justify-center">{column_name}</div>
                                    </th>
                                }
                            }
                        />

                    </tr>
                </thead>
                <tbody>
                    <For
                        each=move || parts_to_orders_data.get()
                        key=move |part_to_order_data| part_to_order_data.part.id.clone()
                        children=move |part_to_order_data| {
                            view! {
                                <PartToOrderRow
                                    part=part_to_order_data.part.clone()
                                    payment=part_to_order_data.payment
                                    deadline=part_to_order_data.deadline
                                />
                            }
                        }
                    />

                </tbody>
            </table>

            <div class="flex flex-col p-4 items-end">
                <button
                    type="submit"
                    class="rounded-md bg-indigo-600 px-6 py-3 text-sm font-semibold text-white shadow-sm hover:bg-indigo-500 focus-visible:outline focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-indigo-600"
                    hidden=move || !is_ready()
                    on:click=move |_| create_orders_from_parts.dispatch(())
                >

                    Submit
                </button>
            </div>
        </div>
    }
}
