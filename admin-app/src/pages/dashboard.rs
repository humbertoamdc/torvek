use crate::api::auth::AuthorizedApi;
use crate::api::models::auth::UserInfo;
use crate::api::models::orders::Order;
use crate::api::orders::OrdersApi;
use crate::api::parts::PartsClient;
use crate::components::orders::table::OrdersTable;
use crate::components::parts::table::PartsTable;
use crate::components::sidebar::Sidebar;
use api_boundary::parts::models::{Part, PartStatus};
use leptos::*;

#[component]
pub fn Dashboard(
    auth_client: AuthorizedApi,
    #[prop(into)] on_logout: Callback<()>,
) -> impl IntoView {
    let user_info_signal = create_rw_signal(UserInfo::default());
    provide_context(user_info_signal);

    // -- api clients -- //
    let orders_client = OrdersApi::new();
    let parts_client = PartsClient::new();

    provide_context(orders_client);
    provide_context(parts_client);

    let client_orders = create_rw_signal(Vec::<Order>::default());
    let query_client_orders = create_action(move |_| {
        async move {
            let orders_client = OrdersApi::new();
            let result = orders_client
                .query_orders_by_status(String::from("pending_quotation"))
                .await;
            match result {
                Ok(response) => client_orders.update(|o| *o = response.orders),
                Err(_) => (), // TODO: Handle error.
            }
        }
    });

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

    // Fetch user data
    create_action(move |_| async move {
        let result = auth_client.user_info().await;
        match result {
            Ok(user_info) => {
                user_info_signal.update(|u| {
                    query_client_orders.dispatch(user_info.id.clone());
                    query_parts.dispatch(());
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
                <PartsTable parts=parts/>
            </div>
        </div>
    }
}
