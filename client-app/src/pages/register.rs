use crate::api;
use crate::api::auth::{AuthorizedApi, UnauthorizedApi};
use crate::api::models::auth::Credentials;
use crate::pages::Page;

use leptos::*;
use leptos_router::*;

#[component]
pub fn Register(
    api: UnauthorizedApi,
    #[prop(into)] on_success: Callback<AuthorizedApi>,
) -> impl IntoView {
    // -- signals -- //

    let (email, set_email) = create_signal(String::new());
    let (password, set_password) = create_signal(String::new());
    let (register_error, set_register_error) = create_signal(None::<String>);
    let (wait_for_response, set_wait_for_response) = create_signal(false);
    let disabled = Signal::derive(move || wait_for_response.get());
    let button_is_disabled = Signal::derive(move || {
        disabled.get() || password.get().is_empty() || email.get().is_empty()
    });

    // -- actions -- //

    let register_action = create_action(move |(email, password): &(String, String)| {
        let email = email.to_string();
        let password = password.to_string();
        let credentials = Credentials { email, password };

        async move {
            set_wait_for_response.update(|waiting| *waiting = true);
            let result = api.register(&credentials).await;
            set_wait_for_response.update(|waiting| *waiting = false);
            match result {
                Ok(res) => {
                    set_register_error.update(|e| *e = None);
                    on_success.call(res);
                }
                Err(err) => {
                    let msg = match err {
                        api::common::Error::Fetch(js_err) => {
                            format!("{js_err:?}")
                        }
                        api::common::Error::Api(err) => err.message,
                        api::common::Error::UnknownError => String::from("unknown error"),
                    };
                    log::error!("Unable to register with {}: {msg}", credentials.email);
                    set_register_error.update(|e| *e = Some(msg));
                    // TODO: Display error message below textbox.
                }
            }
        }
    });

    let dispatch_action = move || register_action.dispatch((email.get(), password.get()));

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
                            "Create your account"
                        </h2>
                    </div>

                    <div class="mt-10 sm:mx-auto sm:w-full sm:max-w-sm">
                        <form class="space-y-3" on:submit=|ev| ev.prevent_default()>
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
                                        on:keyup=move |ev: ev::KeyboardEvent| {
                                            let val = event_target_value(&ev);
                                            set_email.update(|v| *v = val);
                                        }

                                        on:change=move |ev| {
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
                                        autocomplete="current-password"
                                        placeholder="Password"
                                        required
                                        class="p-4 block w-full rounded-md border-0 py-2.5 text-gray-900 shadow-sm ring-1 ring-inset ring-gray-300 placeholder:text-gray-400 focus:ring-2 focus:ring-inset focus:ring-indigo-600 sm:text-sm sm:leading-6"
                                        prop:disabled=move || disabled.get()
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

                                        on:change=move |ev| {
                                            let val = event_target_value(&ev);
                                            set_password.update(|p| *p = val);
                                        }
                                    />

                                </div>
                            </div>

                            <div>
                                <div class="flex items-center justify-between">
                                    <label
                                        for="name"
                                        class="block text-sm font-medium leading-6 text-gray-900"
                                    >
                                        "Name"
                                    </label>
                                </div>
                                <div class="mt-2">
                                    <input
                                        id="name"
                                        name="name"
                                        type="name"
                                        autocomplete="name"
                                        placeholder="Name"
                                        required
                                        class="p-4 block w-full rounded-md border-0 py-2.5 text-gray-900 shadow-sm ring-1 ring-inset ring-gray-300 placeholder:text-gray-400 focus:ring-2 focus:ring-inset focus:ring-indigo-600 sm:text-sm sm:leading-6"
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
                                    "Sign up"
                                </button>
                            </div>
                        </form>

                    </div>

                </div>
            </div>
            <p class="mt-10 text-center text-sm text-gray-500">
                "Already have an account? "
                <A
                    href=Page::Login.path()
                    class="font-semibold leading-6 text-indigo-600 hover:text-indigo-500"
                >
                    "Sign in"
                </A>
            </p>
        </div>
    }
}
