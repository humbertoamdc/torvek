use api_boundary::projects::models::Project;
use api_boundary::ErrorCode;
use leptos::*;
use leptos_router::*;
use thaw::{Breadcrumb, BreadcrumbItem, Button};

use api_boundary::quotations::models::Quotation;
use api_boundary::quotations::requests::CreateQuotationRequest;
use clients::projects::ProjectsClient;
use clients::quotations::QuotationsClient;

use crate::api::models::auth::UserInfo;
use crate::components::quotations::quotation_button::QuotationButton;
use crate::pages::Page;

#[derive(Params, PartialEq)]
struct QuotationsParams {
    project_id: Option<String>,
}

#[component]
pub fn QuotationsContainer() -> impl IntoView {
    view! { <Outlet/> }
}

#[component]
pub fn Quotations() -> impl IntoView {
    let navigate = use_navigate();

    // -- clients -- //

    let projects_client = use_context::<ProjectsClient>().unwrap();
    let quotations_client = use_context::<QuotationsClient>().unwrap();

    // -- context -- //

    let user_info = use_context::<RwSignal<UserInfo>>().expect("user info to be provided");

    // -- signals -- //

    let project = create_rw_signal(None::<Project>);
    let quotations = create_rw_signal(Vec::<Quotation>::default());

    // -- params -- //

    let params = use_params::<QuotationsParams>();
    let project_id = move || {
        params.with_untracked(|params| {
            params
                .as_ref()
                .map(|params| params.project_id.clone())
                .unwrap_or_default()
        })
    };

    // -- actions -- //

    let _query_project = create_action(move |_| {
        let client_id = user_info.get_untracked().id;
        let project_id = project_id().unwrap();
        async move {
            match projects_client
                .get_project_by_id(client_id, project_id)
                .await
            {
                Ok(project_response) => project.update(|project| *project = Some(project_response)),
                Err(err) => {
                    if err.code == ErrorCode::ItemNotFound {
                        let navigate = use_navigate();
                        navigate(Page::Home.path(), Default::default())
                    }
                }
            }
        }
    })
    .dispatch(());

    let query_quotations = create_action(move |_| {
        async move {
            let result = quotations_client
                .query_quotations_for_project(
                    user_info.get_untracked().id,
                    project_id().unwrap_or_default(),
                )
                .await;

            match result {
                Ok(response) => quotations.update(|p| *p = response.quotations),
                Err(_) => (), // TODO: Handle error.
            }
        }
    });

    let query_quotations_callback = Callback::<()>::new(move |_| {
        query_quotations.dispatch(());
    });

    let create_quotation = create_action(move |_| {
        let request = CreateQuotationRequest::new(
            user_info.get_untracked().id,
            project_id().unwrap_or_default(),
        );
        async move {
            let result = quotations_client.create_quotation(request).await;

            match result {
                Ok(_) => query_quotations_callback.call(()),
                Err(_) => (), // TODO: Handle error.
            }
        }
    });

    // -- derived signals -- //

    let is_creating_quotation = Signal::derive(move || create_quotation.pending().get());

    query_quotations.dispatch(());

    view! {
        <Breadcrumb>
            <BreadcrumbItem>
                <button on:click=move |_| navigate(
                    "/projects",
                    Default::default(),
                )>"Projects"</button>
            </BreadcrumbItem>
            <BreadcrumbItem>"Quotations"</BreadcrumbItem>
        </Breadcrumb>

        <header class="flex justify-between items-center py-4">
            <h1 class="text-4xl font-bold text-gray-900">
                {
                    move || {
                        match project.get() {
                            Some(project) => project.name,
                            None => String::default(),
                        }
                    }
                }
            </h1>
        </header>

        <header class="flex justify-between items-center py-4">
            <h1 class="text-2xl font-bold text-gray-900">Quotations</h1>
        </header>

        <Button loading=is_creating_quotation on_click=move |_| create_quotation.dispatch(())>
            "New Quotation"
        </Button>

        <div class="mt-8 flex flex-wrap gap-4">
            <For
                each=move || quotations.get().into_iter()
                key=|project| project.id.clone()
                children=move |quotation| {
                    view! { <QuotationButton quotation=quotation/> }
                }
            />

        </div>
    }
}
