use crate::api::models::auth::UserInfo;
use crate::api::projects::ProjectsClient;
use api_boundary::projects::requests::CreateProjectRequest;
use leptos::*;

#[component]
pub fn Projects() -> impl IntoView {
    // -- context -- //

    let user_info = use_context::<RwSignal<UserInfo>>().expect("user info to be provided");

    // -- actions -- //

    let create_project = create_action(move |_| {
        let request = CreateProjectRequest::new(user_info.get_untracked().id);
        async {
            let projects_client = ProjectsClient::new();
            let result = projects_client.create_project(request).await;
            match result {
                Ok(_) => (),
                Err(_) => (), // TODO: Handle error.
            }
        }
    });

    view! {
        <header class="flex justify-between items-center py-4">
            <h1 class="text-3xl font-bold text-gray-900">Projects</h1>
        </header>

        <button
            type="submit"
            class="flex justify-center rounded-md bg-indigo-600 px-3 py-1.5 text-sm font-semibold text-white shadow-sm hover:bg-indigo-500 focus-visible:outline focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-indigo-600"
            on:click=move |_| create_project.dispatch(())
        >
            "New Project"
        </button>
    }
}
