use leptos::*;
use leptos_router::*;

use ::clients::auth::AuthClient;
use clients::orders::OrdersClient;
use ory_kratos_client::models::Session;

use crate::models::users::User;
use crate::pages::dashboard::Dashboard;
use crate::pages::home::Home;

mod components;
mod models;
mod pages;

pub const API_URL: &'static str = env!("API_URL");
pub const ORY_URL: &'static str = env!("ORY_URL");

#[component]
pub fn App() -> impl IntoView {
    // -- clients -- //
    let auth_client = AuthClient::new(ORY_URL);
    let orders_client = OrdersClient::new(API_URL);

    provide_context(auth_client);
    provide_context(orders_client);

    // -- signals -- //

    let session = create_rw_signal(None::<Session>);
    let user = create_rw_signal(None::<User>);

    // -- actions -- //
    let login = create_action(move |_: &()| async move {
        let result = auth_client.to_session().await;

        match result {
            Ok(sess) => {
                let local_user = User::from(*sess.identity.clone());
                user.update(move |u| *u = Some(local_user));
                session.update(move |s| *s = Some(sess));
            }
            Err(_) => {
                let return_to_url = window().location().href().unwrap();
                if let Ok(response) = auth_client.create_browser_login_flow().await {
                    auth_client
                        .redirect_to_login_url(response.id, return_to_url)
                        .await;
                }
            }
        }
    });

    login.dispatch(());
    view! {
        <Router>
            <nav></nav>
            <main>
                <Routes>
                    <Route
                        path="/"
                        view=move || {
                            view! {
                                <Show
                                    when=move || session.get().is_some()
                                    fallback=move || {
                                        view! {
                                            <div class="flex items-center justify-center h-screen">
                                                <div class="relative">
                                                    <div class="h-24 w-24 rounded-full border-t-8 border-b-8 border-gray-200"></div>
                                                    <div class="absolute top-0 left-0 h-24 w-24 rounded-full border-t-8 border-b-8 border-blue-500 animate-spin"></div>
                                                </div>
                                            </div>
                                        }
                                    }
                                >

                                    <Home user=user.get_untracked().unwrap()/>
                                </Show>
                            }
                        }
                    >

                        <Route path="/" view=move || view! { <Dashboard/> }/>
                    </Route>
                </Routes>
            </main>
        </Router>
    }
}
