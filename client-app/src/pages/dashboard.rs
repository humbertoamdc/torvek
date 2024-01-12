use crate::api::models::auth::UserInfo;
use crate::api::models::orders::Order;
use crate::api::orders::OrdersClient;
use crate::components::dropzone_upload::DropzoneUpload;
use crate::components::orders::table::OrdersTable;
use leptos::*;

#[component]
pub fn Dashboard() -> impl IntoView {
    // -- context -- //

    let user_info = use_context::<RwSignal<UserInfo>>().expect("user info to be provided");

    // -- signals -- //
    let client_orders = create_rw_signal(Vec::<Order>::default());

    // -- action -- //

    let query_client_orders_action = create_action(move |client_id: &String| {
        let client_id = client_id.clone();
        async move {
            let orders_client = OrdersClient::new();
            let result = orders_client.query_orders_for_client(client_id).await;
            match result {
                Ok(response) => client_orders.update(|o| *o = response.orders),
                Err(_) => (), // TODO: Handle error.
            }
        }
    });

    let on_upload = move |_| {
        query_client_orders_action.dispatch(user_info.get_untracked().id);
    };

    query_client_orders_action.dispatch(user_info.get_untracked().id.clone());

    view! {
        <header class="flex justify-between items-center py-4">
            <h1 class="text-3xl font-bold text-gray-900">Dashboard</h1>
        </header>

        <OrdersTable client_orders=client_orders/>

        <div class="mt-8">
            <DropzoneUpload on_upload=on_upload/>
        </div>
    }
}
