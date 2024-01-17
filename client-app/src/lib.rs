use crate::api::auth::UnauthorizedClient;
use crate::pages::home::Home;
use crate::pages::parts::{Parts, PartsContainer};
use crate::pages::projects::{Projects, ProjectsContainer};
use crate::pages::quotations::{Quotations, QuotationsContainer};
use api::auth::AuthorizedClient;
use api::models::auth::UserInfo;
use leptos::*;
use leptos_router::*;
use std::default::Default;

use self::components::loading::Loading;
use self::pages::*;

mod api;
mod components;
mod env;
mod pages;

mod models;

#[component]
pub fn App() -> impl IntoView {
    // -- signals -- //

    let authorized_api_signal: RwSignal<Option<AuthorizedClient>> =
        create_rw_signal(None::<AuthorizedClient>);
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
    let unauthorized_api = UnauthorizedClient::new();
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
                        path="/"
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

                                    <Home
                                        auth_client=authorized_api_signal.get().unwrap()
                                        on_logout=on_logout
                                    />
                                </Show>
                            }
                        }
                    >

                        <Route path="projects" view=ProjectsContainer>
                            <Route path="" view=Projects/>
                            <Route path=":project_id/quotations" view=QuotationsContainer>
                                <Route path="" view=Quotations/>
                                <Route path=":quotation_id/parts" view=PartsContainer>
                                    <Route path="" view=Parts/>
                                </Route>
                            </Route>
                        </Route>
                        <Route
                            path=Page::Home.path()
                            view=move || {
                                view! { <Dashboard/> }
                            }
                        />

                    </Route>

                    <Route
                        path=Page::Register.path()
                        view=move || {
                            view! {
                                <Show
                                    when=move || !is_logged_in_signal.get()
                                    fallback=move || {
                                        let navigate = use_navigate();
                                        navigate(Page::Home.path(), Default::default());
                                    }
                                >

                                    <Show
                                        when=move || !logging_in.pending().get()
                                        fallback=move || {
                                            view! { <Loading/> }
                                        }
                                    >

                                        <Register api=unauthorized_api on_success=on_signin/>

                                    </Show>
                                </Show>
                            }
                        }
                    />

                </Routes>
            </main>
        </Router>
    }
}
