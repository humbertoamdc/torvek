use leptos::*;
use supplier_app::*;

fn main() {
    _ = console_log::init_with_level(log::Level::Debug);
    mount_to_body(move || view! { <App/> })
}
