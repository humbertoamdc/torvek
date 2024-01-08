use crate::api::models::auth::UserInfo;
use crate::api::models::orders::CreateOrdersRequest;
use crate::api::orders::OrdersClient;
use leptos::*;
use web_sys::HtmlInputElement;

#[component]
pub fn DropzoneUpload(#[prop(into)] on_upload: Callback<()>) -> impl IntoView {
    // -- context -- //
    let user_info = use_context::<RwSignal<UserInfo>>().expect("user info to be provided");

    // -- action -- //
    let create_orders_action = create_action(move |input_element: &HtmlInputElement| {
        let file_list = input_element.clone().files().unwrap();
        let mut file_names: Vec<String> = Vec::with_capacity(file_list.length() as usize);
        for i in 0..file_list.length() {
            if let Some(file) = file_list.item(i) {
                file_names.push(file.name());
            }
        }

        let input_element = input_element.clone();
        let request =
            CreateOrdersRequest::new(String::from(user_info.get().id), file_names.to_owned());
        async move {
            let orders_client = OrdersClient::new();
            match orders_client.create_orders(request).await {
                Ok(response) => {
                    for i in 0..file_list.length() {
                        if let Some(file) = file_list.item(i) {
                            orders_client
                                .upload_file_with_presigned_url(
                                    file,
                                    response[i as usize].upload_url.clone(),
                                )
                                .await
                                .expect("error while uploading file with presigned url");
                        }
                    }
                }
                Err(err) => log::error!("{err:?}"),
            }
            input_element.set_value("");
            on_upload.call(());
        }
    });

    view! {
        <link rel="stylesheet" href="https://unpkg.com/flowbite@1.4.4/dist/flowbite.min.css"/>

        <div class="flex items-center justify-center w-full">
            <label
                for="dropzone-file"
                class="flex flex-col items-center justify-center w-full h-64 border-2 border-gray-300 border-dashed rounded-lg cursor-pointer bg-gray-50 dark:hover:bg-bray-800 dark:bg-gray-700 hover:bg-gray-100 dark:border-gray-600 dark:hover:border-gray-500 dark:hover:bg-gray-600"
            >
                <div class="flex flex-col items-center justify-center pt-5 pb-6">
                    // SVG code here (use raw SVG code or a component)
                    <p class="mb-2 text-sm text-gray-500 dark:text-gray-400">
                        <span class="font-semibold">Click to upload</span>
                        or drag and drop
                    </p>
                    <p class="text-xs text-gray-500 dark:text-gray-400">STEP or STP</p>
                </div>
                <input
                    id="dropzone-file"
                    type="file"
                    class="hidden"
                    accept=".stp,.step"
                    multiple
                    on:change=move |ev| {
                        let input_element = event_target::<HtmlInputElement>(&ev);
                        create_orders_action.dispatch(input_element);
                    }
                />

            </label>
        </div>

        <script src="https://unpkg.com/flowbite@1.4.0/dist/flowbite.js"></script>
    }
}
