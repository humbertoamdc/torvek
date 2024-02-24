use leptos::*;

#[component]
pub fn TolerancesDropdown(
    #[prop(into)] tolerance: RwSignal<String>,
    #[prop(into)] on_tolerance_change: Callback<String>,
) -> impl IntoView {
    // -- data -- //

    let items = vec!["ISO 2768 Medium", "ISO 2768 Fine", "Other"];

    // -- signals -- //

    let is_focused = create_rw_signal(false);

    // -- actions -- //

    let change_action = create_action(move |()| async move {});

    view! {
        <div class="w-52 max-w-full">
            <button
                on:click=move |_| {
                    is_focused.update(|f| *f = !*f);
                }

                on:blur=move |_| {
                    is_focused.update(|f| *f = false);
                }

                class="w-full inline-flex items-center justify-between px-2 py-1.5 text-sm text-gray-600 bg-white border rounded-lg shadow-sm outline-none"
            >
                <div class="flex items-center gap-x-3">
                    <Show when=move || !change_action.pending().get() fallback=move || view! {}>
                        <span>{tolerance}</span>
                    </Show>

                </div>
            </button>
            // Dropdown menu
            <Show when=move || is_focused.get() fallback=move || view! {}>

                <div class="fixed w-52 mt-1.5 origin-top-right bg-white divide-y divide-gray-100 rounded-md shadow-lg ring-1 ring-black ring-opacity-5 focus:outline-none overflow-y-auto">

                    {items
                        .iter()
                        .map(|item| {
                            let item = *item;
                            view! {
                                <div
                                    class="flex items-center justify-between px-3 cursor-default py-2 duration-150 text-gray-600 hover:bg-indigo-50"
                                    on:mousedown=move |_| {
                                        on_tolerance_change.call(item.to_string());
                                        is_focused.update(|f| *f = false);
                                    }
                                >

                                    <span class="pr-4 line-clamp-1 text-sm flex items-center gap-2">
                                        {item}
                                    </span>
                                </div>
                            }
                        })
                        .collect::<Vec<_>>()}
                </div>
            </Show>
        </div>
    }
}
