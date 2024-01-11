use crate::api::models::auth::UserInfo;
use crate::api::models::orders::Order;
use crate::api::projects::ProjectsClient;
use crate::components::projects::project_button::ProjectButton;
use api_boundary::projects::models::Project;
use api_boundary::projects::requests::CreateProjectRequest;
use leptos::*;

#[component]
pub fn Projects() -> impl IntoView {
    // -- context -- //

    let user_info = use_context::<RwSignal<UserInfo>>().expect("user info to be provided");

    // -- signals -- //

    let projects = create_rw_signal(Vec::<Project>::default());

    // -- actions -- //

    let query_projects = create_action(move |_| {
        async move {
            let project_client = ProjectsClient::new();
            let result = project_client
                .query_projects_for_client(user_info.get_untracked().id)
                .await;

            match result {
                Ok(response) => projects.update(|p| *p = response.projects),
                Err(_) => (), // TODO: Handle error.
            }
        }
    });

    let query_projects_callback = Callback::<()>::new(move |_| {
        query_projects.dispatch(());
    });

    let create_project = create_action(move |_| {
        let request = CreateProjectRequest::new(user_info.get_untracked().id);
        async move {
            let projects_client = ProjectsClient::new();
            let result = projects_client.create_project(request).await;

            match result {
                Ok(_) => query_projects_callback.call(()),
                Err(_) => (), // TODO: Handle error.
            }
        }
    });

    query_projects.dispatch(());

    view! {
        <header class="flex justify-between items-center py-4">
            <h1 class="text-3xl font-bold text-gray-900">Projects</h1>
        </header>

        <button
            type="submit"
            class="flex justify-center rounded-md bg-indigo-600 px-3 py-1.5 text-sm font-semibold text-white shadow-sm hover:bg-indigo-500 focus-visible:outline focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-indigo-600"
            on:click=move |_| {
                create_project.dispatch(());
            }
        >

            "New Project"
        </button>

        <div class="mt-8 flex flex-wrap gap-4">
            <For
                each=move || projects.get().into_iter()
                key=|project| project.id.clone()
                children=move |project| {
                    view! { <ProjectButton/> }
                }
            />
        </div>
    }
}
