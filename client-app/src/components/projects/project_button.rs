use api_boundary::projects::models::Project;
use leptos::*;
use leptos_router::use_navigate;

#[component]
pub fn ProjectButton(project: Project) -> impl IntoView {
    let navigate = use_navigate();
    let quotations_for_project_url = format!("/projects/{}/quotations", project.id);

    view! {
        <button
            class="rounded-md bg-neutral-50 hover:bg-neutral-200 w-48 h-48"
            on:click=move |_| navigate(&quotations_for_project_url, Default::default())
        >
            <p class="font-bold">{project.name}</p>
        </button>
    }
}
