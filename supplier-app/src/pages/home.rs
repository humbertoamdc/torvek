use crate::components::sidebar::Sidebar;
use crate::models::users::User;
use leptos::*;
use leptos_router::*;

#[component]
pub fn Home(user: User) -> impl IntoView {
    view! {
        <div class="flex h-screen bg-gray-100">
            // Sidebar
            <Sidebar user=user/>

            // Main content
            <div class="flex-1 px-10 py-6">
                <Outlet/>
            </div>
        </div>
    }
}
