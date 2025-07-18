use crate::api::auth::AuthorizedApi;
use crate::api::models::auth::UserInfo;
use crate::api::quotations::QuotationsClient;
use crate::clients::orders::OrdersClient;
use crate::clients::parts::PartsClient;
use crate::components::orders::add_order_payouts_table::AddOrderPayoutsTable;
use crate::components::quotations::created_quotations_collapsible::CreatedQuotationsCollapsible;
use crate::components::sidebar::Sidebar;
use crate::models::order::Order;
use crate::models::quotation::{Quotation, QuotationStatus};
use leptos::*;

pub const API_URL: &'static str = env!("API_URL");

#[component]
pub fn Dashboard(
    auth_client: AuthorizedApi,
    #[prop(into)] on_logout: Callback<()>,
) -> impl IntoView {
    let user_info_signal = create_rw_signal(UserInfo::default());
    provide_context(user_info_signal);

    // -- api clients -- //

    let parts_client = PartsClient::new(API_URL);
    let quotations_client = QuotationsClient::new(API_URL);
    let orders_client = OrdersClient::new(API_URL);

    provide_context(parts_client);
    provide_context(quotations_client);
    provide_context(orders_client);

    // -- signals -- //

    let created_quotations = create_rw_signal(Vec::<Quotation>::default());
    let orders = create_rw_signal(Vec::<Order>::default());

    // -- actions -- //

    let query_quotations_pending_review = create_action(move |_| async move {
        let result = quotations_client
            .query_quotations_by_status(QuotationStatus::PendingReview)
            .await;

        match result {
            Ok(response) => created_quotations.update(|q| *q = response.quotations),
            Err(_) => (), // TODO: Handle error.
        }
    });

    // Fetch user data
    create_action(move |_| async move {
        let result = auth_client.user_info().await;
        match result {
            Ok(user_info) => {
                user_info_signal.update(|u| {
                    query_quotations_pending_review.dispatch(());
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

                <h2 class="text-xl font-bold text-gray-900 mt-6 mb-4">Created Quotations</h2>
                <CreatedQuotationsCollapsible quotations=created_quotations/>

                <h2 class="text-xl font-bold text-gray-900 mt-6 mb-4">Orders Pending Pricing</h2>
                <AddOrderPayoutsTable orders/>
            </div>
        </div>
    }
}
