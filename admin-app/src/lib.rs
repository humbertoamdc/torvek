use leptos::*;
use leptos_router::*;

use api::auth::AuthorizedApi;
use api::models::auth::UserInfo;
use clients::orders::OrdersClient;

use crate::api::auth::UnauthorizedApi;

use self::components::loading::Loading;
use self::pages::*;

mod api;
mod components;
mod env;
mod models;
mod pages;

static API_URL: &'static str = env!("API_URL");

#[component]
pub fn App() -> impl IntoView {
    // -- clients -- //
    provide_context(OrdersClient::new(API_URL));

    // -- signals -- //

    let authorized_api_signal: RwSignal<Option<AuthorizedApi>> =
        create_rw_signal(None::<AuthorizedApi>);
    let user_info_signal: RwSignal<Option<UserInfo>> = create_rw_signal(None::<UserInfo>);
    let is_logged_in_signal = Signal::derive(move || authorized_api_signal.get().is_some());

    // -- callbacks -- //

    let on_signin = move |api| {
        authorized_api_signal.update(|v| *v = Some(api));
    };

    let on_logout = move |_| {
        authorized_api_signal.update(|a| *a = None);
        user_info_signal.update(|u| *u = None);
    };

    // -- init API -- //
    let unauthorized_api = UnauthorizedApi::new();
    let logging_in = create_action(move |_| async move {
        // Try to login. If there is a session id in the cookies we can skip the login page.
        if let Ok((authorized_api, user_info)) =
            unauthorized_api.try_login_with_session_cookie().await
        {
            let _ = authorized_api_signal.update(|a| *a = Some(authorized_api));
            let _ = user_info_signal.update(|u| *u = Some(user_info));
        }
    });
    logging_in.dispatch(());

    view! {
        <Router>
            <nav></nav>
            <main>
                <Routes>
                    <Route
                        path=Page::Home.path()
                        view=move || {
                            view! {
                                <Show
                                    when=move || is_logged_in_signal.get()
                                    fallback=move || {
                                        view! {
                                            <Show
                                                when=move || !logging_in.pending().get()
                                                fallback=move || {
                                                    view! { <Loading/> }
                                                }
                                            >

                                                <Login api=unauthorized_api on_success=on_signin/>

                                            </Show>
                                        }
                                    }
                                >

                                    <Dashboard
                                        auth_client=authorized_api_signal.get().unwrap()
                                        on_logout=on_logout
                                    />
                                </Show>
                            }
                        }
                    />

                </Routes>
            </main>
        </Router>
    }
}
