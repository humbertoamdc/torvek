use leptos::*;
use leptos_router::*;
use thaw::{Layout, LayoutPosition, LayoutSider};

use crate::api::auth::AuthorizedClient;
use crate::api::models::auth::UserInfo;
use crate::components::loading::Loading;
use crate::components::sidebar::Sidebar;

#[component]
pub fn Home(auth_client: AuthorizedClient, #[prop(into)] on_logout: Callback<()>) -> impl IntoView {
    let user_info_signal = create_rw_signal(UserInfo::default());

    // Fetch user data
    let fetch_user_info = create_action(move |_| async move {
        let result = auth_client.user_info().await;
        match result {
            Ok(user_info) => {
                user_info_signal.update(|u| *u = user_info);
                provide_context(user_info_signal);
            }
            Err(_) => log::error!("Unable to fetch user information"),
        }
    });

    fetch_user_info.dispatch(());

    view! {
        <Layout position=LayoutPosition::Static has_sider=true>
            <LayoutSider>
                <Sidebar auth_client=auth_client on_logout=on_logout/>
            </LayoutSider>
            <Layout class="h-screen bg-gray-100 px-8 py-6 h-screen bg-gray-100">
                <Layout>
                    // Main content
                    <Show
                        when=move || !fetch_user_info.pending().get()
                        fallback=move || {
                            view! { <Loading/> }
                        }
                    >

                        <Outlet/>
                    </Show>
                </Layout>
            </Layout>
        </Layout>
    }
}
