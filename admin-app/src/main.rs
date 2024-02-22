use leptos::*;

use admin_app::*;

fn main() {
    _ = console_log::init_with_level(log::Level::Debug);
    mount_to_body(|| view! { <App/> })
}
