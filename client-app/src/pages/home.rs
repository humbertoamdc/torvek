use leptos::*;
use leptos_router::*;
use thaw::{Layout, LayoutPosition, LayoutSider};

use crate::api::auth::AuthorizedClient;
use crate::api::models::auth::UserInfo;
use crate::components::loading::Loading;
use crate::components::sidebar::Sidebar;

#[component]
pub fn Home(auth_client: AuthorizedClient, #[prop(into)] on_logout: Callback<()>) -> impl IntoView {
    // -- signals -- //

    let user_info_signal = create_rw_signal(UserInfo::default());

    // -- actions -- //

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
        <Show
            when=move || !fetch_user_info.pending().get()
            fallback=move || {
                view! { <Loading/> }
            }
        >

            <Layout position=LayoutPosition::Static has_sider=true>

                <LayoutSider>
                    <Sidebar auth_client=auth_client on_logout/>
                </LayoutSider>
                <Layout class="h-screen px-8 py-6">
                    <Layout>
                        <Outlet/>
                    </Layout>
                </Layout>
            </Layout>
        </Show>
    }
}
