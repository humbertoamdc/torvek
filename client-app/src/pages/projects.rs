use leptos::*;
use leptos_router::*;
use thaw::Button;

use api_boundary::projects::models::Project;
use api_boundary::projects::requests::CreateProjectRequest;
use clients::projects::ProjectsClient;

use crate::api::models::auth::UserInfo;
use crate::components::projects::project_button::ProjectButton;

#[component]
pub fn ProjectsContainer() -> impl IntoView {
    view! { <Outlet/> }
}

#[component]
pub fn Projects() -> impl IntoView {
    // -- context -- //

    let user_info = use_context::<RwSignal<UserInfo>>().expect("user info to be provided");

    // -- clients -- //

    let projects_client = use_context::<ProjectsClient>().unwrap();

    // -- signals -- //

    let projects = create_rw_signal(Vec::<Project>::default());

    // -- actions -- //

    let query_projects = create_action(move |_| {
        async move {
            let result = projects_client
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
            let result = projects_client.create_project(request).await;

            match result {
                Ok(_) => query_projects_callback.call(()),
                Err(_) => (), // TODO: Handle error.
            }
        }
    });

    // -- init -- //

    query_projects.dispatch(());

    view! {
        <header class="flex justify-between items-center py-4">
            <h1 class="text-3xl font-bold text-gray-900">Projects</h1>
        </header>

        <Button loading=create_project.pending() on_click=move |_| create_project.dispatch(())>
            "New Project"
        </Button>

        <div class="mt-8 flex flex-wrap gap-4">
            <For
                each=move || projects.get().into_iter()
                key=|project| project.id.clone()
                children=move |project| {
                    view! { <ProjectButton project=project/> }
                }
            />

        </div>
    }
}
