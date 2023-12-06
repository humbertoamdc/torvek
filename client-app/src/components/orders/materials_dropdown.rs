use leptos::*;
use std::collections::HashMap;

#[component]
pub fn MaterialsDropdown(
    #[prop(into)] material: RwSignal<String>,
    #[prop(into)] on_material_change: Callback<String>,
) -> impl IntoView {
    // -- data -- //

    let menu_items = vec![
        ("Aluminum 6061-T6", "text-blue-600", "bg-blue-600"),
        ("Aluminum 2024-T3", "text-blue-600", "bg-blue-600"),
        ("Aluminum 7075-T6", "text-blue-600", "bg-blue-600"),
        ("A2 Steel", "text-red-600", "bg-red-600"),
        ("1018 Mild Steel", "text-red-600", "bg-red-600"),
        ("1045 Carbon Steel", "text-red-600", "bg-red-600"),
        ("Cast Iron", "text-red-600", "bg-red-600"),
        ("Stainless Steel 304", "text-teal-600", "bg-teal-600"),
        ("Stainless Steel 321", "text-teal-600", "bg-teal-600"),
        ("260 Brass", "text-yellow-300", "bg-yellow-300"),
        ("360 Brass", "text-yellow-300", "bg-yellow-300"),
        ("ABS", "text-gray-300", "bg-gray-300"),
        ("Delrin", "text-gray-300", "bg-gray-300"),
        ("110 Copper", "text-fuchsia-600", "bg-fuchsia-600"),
        ("Grade 5 Titanium", "text-green-600", "bg-green-600"),
        ("Other", "text-orange-600", "bg-orange-600"),
    ];

    let text_colors = menu_items
        .iter()
        .map(|item| (item.0, item.1))
        .collect::<HashMap<&str, &str>>();
    let bg_colors = menu_items
        .iter()
        .map(|item| (item.0, item.2))
        .collect::<HashMap<&str, &str>>();

    // -- signals -- //

    let text_color = create_rw_signal(text_colors[material.get_untracked().as_str()]);
    let bg_color = create_rw_signal(bg_colors[material.get_untracked().as_str()]);
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

                class="w-full inline-flex items-center justify-between px-3 py-2 text-sm text-gray-600 bg-white border rounded-lg shadow-sm outline-none"
            >
                <div class="flex items-center gap-x-3">
                    <Show when=move || !change_action.pending().get() fallback=move || view! {}>
                        <span class=format!("w-2 h-2 rounded-full {}", bg_color.get())></span>
                        <span>{material}</span>
                    </Show>

                </div>
            </button>
            // Dropdown menu
            <Show when=move || is_focused.get() fallback=move || view! {}>

                <div class="fixed w-52 h-60 mt-2 origin-top-right bg-white divide-y divide-gray-100 rounded-md shadow-lg ring-1 ring-black ring-opacity-5 focus:outline-none overflow-y-auto">

                    {menu_items
                        .iter()
                        .map(|item| {
                            let item = item.clone();
                            view! {
                                <div
                                    class="flex items-center justify-between px-3 cursor-default py-2 duration-150 text-gray-600 hover:bg-indigo-50"
                                    on:mousedown=move |_| {
                                        text_color.update(move |t| *t = item.1);
                                        bg_color.update(|b| *b = item.2);
                                        on_material_change.call(item.0.to_owned());
                                        is_focused.update(|f| *f = false);
                                    }
                                >

                                    <span class="pr-4 line-clamp-1 flex items-center gap-2">
                                        <span class=format!(
                                            "w-2 h-2 rounded-full {}",
                                            item.2,
                                        )></span>
                                        {item.0}
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
