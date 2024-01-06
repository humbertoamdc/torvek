use crate::api::auth::AuthorizedApi;
use crate::api::models::auth::UserInfo;
use crate::components::loading::Loading;
use crate::components::sidebar::Sidebar;
use leptos::*;
use leptos_router::*;

#[component]
pub fn Home(auth_client: AuthorizedApi, #[prop(into)] on_logout: Callback<()>) -> impl IntoView {
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
        <div class="flex h-screen bg-gray-100">
            // Sidebar
            <Sidebar auth_client=auth_client on_logout=on_logout/>

            // Main content
            <div class="flex-1 px-10 py-6">
                <Show
                    when=move || !fetch_user_info.pending().get()
                    fallback=move || {
                        view! { <Loading/> }
                    }
                >
                    <Outlet/>
                </Show>
            </div>
        </div>

    }
}
