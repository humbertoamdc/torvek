use crate::api;
use leptos::*;
use reqwest::header::{ACCEPT, ACCESS_CONTROL_ALLOW_CREDENTIALS, ACCESS_CONTROL_ALLOW_ORIGIN, HeaderMap, HeaderValue, ORIGIN};
use ory_kratos_client::apis::configuration::Configuration;
use ory_kratos_client::apis::frontend_api::create_browser_login_flow;
use ory_kratos_client::models::UiNodeAttributes;

use crate::api::auth::{AuthorizedApi, UnauthorizedApi};
use crate::api::models::auth::Credentials;
use crate::env::ORY_ADMIN_URL;

#[component]
pub fn Login(
    api: UnauthorizedApi,
    #[prop(into)] on_success: Callback<AuthorizedApi>,
) -> impl IntoView {
    // -- signals -- //

    let (email, set_email) = create_signal(String::new());
    let (password, set_password) = create_signal(String::new());
    let (login_error, set_login_error) = create_signal(None::<String>);
    let (wait_for_response, set_wait_for_response) = create_signal(false);
    let disabled = Signal::derive(move || wait_for_response.get());
    let button_is_disabled = Signal::derive(move || {
        disabled.get() || password.get().is_empty() || email.get().is_empty()
    });
    let (flow_id, set_flow_id) = create_signal(String::new());
    let (csrf_token, set_csrf_token) = create_signal(String::new());

    // -- actions -- //

    let init_login_flow = create_action(move |_: &()| {
        let mut headers = HeaderMap::new();
        headers.insert(ACCEPT, HeaderValue::from_static("application/json"));
        headers.insert(ACCESS_CONTROL_ALLOW_CREDENTIALS, HeaderValue::from_static("true"));
        headers.insert(ORIGIN, HeaderValue::from_static("http://127.0.0.1:8081"));
        let client = reqwest::Client::builder().default_headers(headers).build().unwrap();
        let config = Configuration {
            base_path: ORY_ADMIN_URL.to_string(),
            user_agent: None,
            client,
            basic_auth: None,
            oauth_access_token: None,
            bearer_access_token: None,
            api_key: None,
        };
        async move {
            let response = create_browser_login_flow(&config, None, None, None, None, None).await;
            match response {
                Ok(login_flow) => {
                    set_flow_id(login_flow.id);
                    let _csrf_token = login_flow.ui.nodes.iter().find(|node| {
                        match *node.attributes.clone() {
                            UiNodeAttributes::UiNodeInputAttributes { name,.. } => name == String::from("csrf_token"),
                            _ => false

                        }
                    }).map(|node| {
                        match *node.attributes.clone() {
                            UiNodeAttributes::UiNodeInputAttributes { value, .. } => serde_json::from_str::<String>(&value.unwrap().to_string()).unwrap(),
                            _ => String::default(),
                        }
                    }).unwrap_or_default();
                    set_csrf_token(_csrf_token);
                },
                Err(_) => log::error!("error creating login flow"),
            }
        }
    });
    init_login_flow.dispatch(());

    let login_action = create_action(move |(email, password, flow_id, csrf_token): &(String, String, String, String)| {
        let email = email.to_string();
        let password = password.to_string();
        let flow_id = flow_id.to_string();
        let csrf_token = csrf_token.to_string();
        let credentials = Credentials { email, password, flow_id, csrf_token };


        async move {
            set_wait_for_response.update(|waiting| *waiting = true);
            let result = api.admin_login(&credentials).await;
            set_wait_for_response.update(|waiting| *waiting = false);
            match result {
                Ok(res) => {
                    set_login_error.update(|error| *error = None);
                    on_success(res);
                }
                Err(err) => {
                    let msg = match err {
                        api::common::Error::Fetch(js_err) => {
                            format!("{js_err:?}")
                        }
                        api::common::Error::Api(err) => err.message,
                        api::common::Error::UnknownError => String::from("unknown error"),
                    };
                    log::error!("Unable to login with {}: {msg}", credentials.email);
                    set_login_error.update(|e| *e = Some(msg));
                    // TODO: Display error message below textbox.
                }
            }
        }
    });

    let dispatch_action = move || login_action.dispatch((email.get(), password.get(), flow_id.get(), csrf_token.get()));

    view! {
        <div class="grid grid-cols-1 h-screen place-content-center bg-stone-50">
            <div class="w-1/2 m-auto max-w-md mx-auto mt-10 overflow-hidden bg-white rounded-lg shadow-lg">
                <div class="flex min-h-full flex-col justify-center px-6 py-12 lg:px-8">
                    <div class="sm:mx-auto sm:w-full sm:max-w-sm">
                        <img
                            class="mx-auto h-10 w-auto"
                            src="https://tailwindui.com/img/logos/mark.svg?color=indigo&shade=600"
                            alt="Your Company"
                        />
                        <h2 class="mt-10 text-center text-2xl font-bold leading-9 tracking-tight text-gray-900">
                            "Sign in to your account"
                        </h2>
                    </div>

                    <div class="mt-10 sm:mx-auto sm:w-full sm:max-w-sm">
                        <form class="space-y-3" on:submit=move |ev| ev.prevent_default()>

                            <div>
                                <label
                                    for="email"
                                    class="block text-sm font-medium leading-6 text-gray-900"
                                >
                                    "Email address"
                                </label>
                                <div class="mt-2">
                                    <input
                                        id="email"
                                        name="email"
                                        type="email"
                                        autocomplete="email"
                                        placeholder="Email"
                                        required
                                        class="p-4 block w-full rounded-md border-0 py-2.5 text-gray-900 shadow-sm ring-1 ring-inset ring-gray-300 placeholder:text-gray-400 focus:ring-2 focus:ring-inset focus:ring-indigo-600 sm:text-sm sm:leading-6"
                                        prop:disabled=move || disabled.get()
                                        on:change=move |ev| {
                                            set_email.update(|v| *v = event_target_value(&ev));
                                        }

                                        on:keyup=move |ev: ev::KeyboardEvent| {
                                            let val = event_target_value(&ev);
                                            set_email.update(|v| *v = val);
                                        }
                                    />

                                </div>
                            </div>

                            <div>
                                <label
                                    for="password"
                                    class="block text-sm font-medium leading-6 text-gray-900"
                                >
                                    "Password"
                                </label>

                                <div class="mt-2">
                                    <input
                                        id="password"
                                        name="password"
                                        type="password"
                                        autocomplete="password"
                                        placeholder="Password"
                                        required
                                        class="p-4 block w-full rounded-md border-0 py-2.5 text-gray-900 shadow-sm ring-1 ring-inset ring-gray-300 placeholder:text-gray-400 focus:ring-2 focus:ring-inset focus:ring-indigo-600 sm:text-sm sm:leading-6"
                                        prop:disabled=move || disabled.get()
                                        on:change=move |ev| {
                                            set_password.update(|v| *v = event_target_value(&ev));
                                        }

                                        on:keyup=move |ev: ev::KeyboardEvent| {
                                            match &*ev.key() {
                                                "Enter" => {
                                                    dispatch_action();
                                                }
                                                _ => {
                                                    let val = event_target_value(&ev);
                                                    set_password.update(|p| *p = val);
                                                }
                                            }
                                        }
                                    />

                                </div>
                            </div>

                            <div class="pt-4">
                                <button
                                    type="submit"
                                    class="flex w-full justify-center rounded-md bg-indigo-600 px-3 py-3.5 text-sm font-semibold leading-6 text-white shadow-sm hover:bg-indigo-500 focus-visible:outline focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-indigo-600"
                                    prop:disabled=move || button_is_disabled.get()
                                    on:click=move |_| dispatch_action()
                                >

                                    "Sign in"
                                </button>
                            </div>
                        </form>

                    </div>

                </div>
            </div>
        </div>
    }
}
