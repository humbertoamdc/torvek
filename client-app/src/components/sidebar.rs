use crate::api::auth::AuthorizedClient;
use crate::pages::Page;
use leptos::*;

#[component]
pub fn Sidebar(
    auth_client: AuthorizedClient,
    #[prop(into)] on_logout: Callback<()>,
) -> impl IntoView {
    let (_, set_wait_for_response) = create_signal(false);

    let logout_action = create_action(move |_| async move {
        set_wait_for_response.update(|waiting| *waiting = true);
        let result = auth_client.logout().await;
        set_wait_for_response.update(|waiting| *waiting = false);

        match result {
            Ok(_) => on_logout.call(()),
            Err(_) => (), // TODO: Handle error.
        };
    });

    view! {
        <nav class="top-0 left-0 h-full border-r bg-white space-y-8 sm:w-72 min-w-max">
            <div class="flex flex-col h-full">
                <div class="h-20 flex items-center px-8">
                    <a href="/" class="flex-none">
                        <img
                            src="https://dewey.tailorbrands.com/production/brand_version_mockup_image/896/8709307896_af7a3de3-d175-49da-9dc6-951a99a3b27f.png?cb=1700542599"
                            width=180
                            class="mx-auto"
                        />
                    </a>
                </div>
                <div class="flex-1 flex flex-col h-full overflow-auto">
                    <ul class="px-4 text-md font-medium flex-1">
                        <li key="home-page">
                            <a
                                href=Page::Home.path()
                                class="flex items-center gap-x-2 text-gray-600 p-2 rounded-lg  hover:bg-gray-50 active:bg-gray-100 duration-150"
                            >
                                <div class="text-gray-500">
                                    <svg
                                        xmlns="http://www.w3.org/2000/svg"
                                        fill="none"
                                        viewBox="0 0 24 24"
                                        strokeWidth=1.5
                                        stroke="currentColor"
                                        class="w-5 h-5"
                                    >
                                        <path
                                            strokeLinecap="round"
                                            strokeLinejoin="round"
                                            d="M6 6.878V6a2.25 2.25 0 012.25-2.25h7.5A2.25 2.25 0 0118 6v.878m-12 0c.235-.083.487-.128.75-.128h10.5c.263 0 .515.045.75.128m-12 0A2.25 2.25 0 004.5 9v.878m13.5-3A2.25 2.25 0 0119.5 9v.878m0 0a2.246 2.246 0 00-.75-.128H5.25c-.263 0-.515.045-.75.128m15 0A2.25 2.25 0 0121 12v6a2.25 2.25 0 01-2.25 2.25H5.25A2.25 2.25 0 013 18v-6c0-.98.626-1.813 1.5-2.122"
                                        ></path>
                                    </svg>
                                </div>
                                Dashboard
                            </a>
                        </li>
                        <li key="projects-page">
                            <a
                                href=Page::Projects.path()
                                class="flex items-center gap-x-2 text-gray-600 p-2 rounded-lg  hover:bg-gray-50 active:bg-gray-100 duration-150"
                            >
                                <div class="text-gray-500">
                                    <svg
                                        xmlns="http://www.w3.org/2000/svg"
                                        fill="none"
                                        viewBox="0 0 24 24"
                                        strokeWidth=1.5
                                        stroke="currentColor"
                                        class="w-5 h-5"
                                    >
                                        <path
                                            strokeLinecap="round"
                                            strokeLinejoin="round"
                                            d="M6 6.878V6a2.25 2.25 0 012.25-2.25h7.5A2.25 2.25 0 0118 6v.878m-12 0c.235-.083.487-.128.75-.128h10.5c.263 0 .515.045.75.128m-12 0A2.25 2.25 0 004.5 9v.878m13.5-3A2.25 2.25 0 0119.5 9v.878m0 0a2.246 2.246 0 00-.75-.128H5.25c-.263 0-.515.045-.75.128m15 0A2.25 2.25 0 0121 12v6a2.25 2.25 0 01-2.25 2.25H5.25A2.25 2.25 0 013 18v-6c0-.98.626-1.813 1.5-2.122"
                                        ></path>
                                    </svg>
                                </div>
                                Projects
                            </a>
                        </li>
                    </ul>
                    <div>
                        <ul class="px-4 pb-4 text-sm font-medium text-red-600">
                            <li key="logout">
                                <a
                                    class="flex items-center gap-x-2  p-2 rounded-lg cursor-pointer hover:bg-gray-50 active:bg-gray-100 duration-150"
                                    on:click=move |_| logout_action.dispatch(())
                                >
                                    <svg
                                        xmlns="http://www.w3.org/2000/svg"
                                        fill="none"
                                        viewBox="0 0 24 24"
                                        strokeWidth=1.5
                                        stroke="currentColor"
                                        class="w-5 h-5"
                                    >
                                        <path
                                            strokeLinecap="round"
                                            strokeLinejoin="round"
                                            d="M15.75 9V5.25A2.25 2.25 0 0013.5 3h-6a2.25 2.25 0 00-2.25 2.25v13.5A2.25 2.25 0 007.5 21h6a2.25 2.25 0 002.25-2.25V15m3 0l3-3m0 0l-3-3m3 3H9"
                                        ></path>
                                    </svg>

                                    Logout
                                </a>
                            </li>
                        </ul>
                        <div class="py-4 px-4 border-t">
                            <div class="flex items-center gap-x-4">
                                <img
                                    src="https://cdn1.iconfinder.com/data/icons/monsters-avatars/512/50_Monsters_Avatar_Icons_49-512.png"
                                    class="w-12 h-12 rounded-full"
                                />
                                <div>
                                    <span class="block text-gray-700 text-sm font-semibold">
                                        John Doe
                                    </span>
                                    <a
                                        href="some other ref"
                                        class="block mt-px text-gray-600 hover:text-indigo-600 text-xs"
                                    >
                                        View profile
                                    </a>
                                </div>
                            </div>
                        </div>
                    </div>
                </div>
            </div>
        </nav>
    }
}
