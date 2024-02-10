use crate::api::auth::AuthorizedApi;
use crate::api::models::auth::UserInfo;
use crate::api::parts::PartsClient;
use crate::api::quotations::QuotationsClient;
use crate::components::parts::table::PartsTable;
use crate::components::quotations::table::QuotationsTable;
use crate::components::sidebar::Sidebar;
use api_boundary::parts::models::{Part, PartStatus};
use api_boundary::quotations::models::{Quotation, QuotationStatus};
use leptos::*;

#[component]
pub fn Dashboard(
    auth_client: AuthorizedApi,
    #[prop(into)] on_logout: Callback<()>,
) -> impl IntoView {
    let user_info_signal = create_rw_signal(UserInfo::default());
    provide_context(user_info_signal);

    // -- api clients -- //
    let parts_client = PartsClient::new();
    let quotations_client = QuotationsClient::new();

    provide_context(parts_client);
    provide_context(quotations_client);

    let parts = create_rw_signal(Vec::<Part>::default());
    let query_parts = create_action(move |_| async move {
        let result = parts_client
            .query_parts_by_status(PartStatus::AwaitingPricing.to_string())
            .await;

        match result {
            Ok(response) => parts.update(|p| *p = response.parts),
            Err(_) => (), // TODO: Handle error.
        }
    });

    let quotations = create_rw_signal(Vec::<Quotation>::default());
    let query_quotations = create_action(move |_| async move {
        let result = quotations_client
            .query_quotations_by_status(QuotationStatus::Payed)
            .await;

        match result {
            Ok(response) => quotations.update(|q| *q = response.quotations),
            Err(_) => (), // TODO: Handle error.
        }
    });

    // Fetch user data
    create_action(move |_| async move {
        let result = auth_client.user_info().await;
        match result {
            Ok(user_info) => {
                user_info_signal.update(|u| {
                    query_parts.dispatch(());
                    query_quotations.dispatch(());
                    *u = user_info;
                });
            }
            Err(_) => (), // TODO: Handle error.
        }
    })
    .dispatch(());

    view! {
        <div class="flex h-screen bg-gray-100">
            // Sidebar
            <Sidebar auth_client=auth_client on_logout=on_logout/>

            // Main content
            <div class="flex-1 px-10 py-6">
                <header class="flex justify-between items-center py-4">
                    <h1 class="text-3xl font-bold text-gray-900">Dashboard</h1>
                </header>

                // <OrdersTable client_orders=client_orders/>
                <h2 class="text-xl font-bold text-gray-900 mb-4">Parts Awaiting Pricing</h2>
                <PartsTable parts=parts/>

                <h2 class="text-xl font-bold text-gray-900 mt-6 mb-4">Payed Quotations</h2>
                <QuotationsTable quotations=quotations/>
            </div>
        </div>
    }
}
