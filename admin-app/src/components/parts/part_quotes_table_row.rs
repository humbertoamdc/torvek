use crate::clients::parts::{GeneratePresignedUrlRequest, PartsClient};
use crate::components::parts::part_quote_card::PartQuoteCard;
use crate::models::file::File;
use crate::models::money::Money;
use crate::models::part::{Part, PartAttributes};
use leptos::*;
use thaw::Button;
use thaw::ButtonColor::Error;

#[component]
pub fn PartQuotesTableRow(
    #[prop(into)] part: Part,
    #[prop(into)] price_options: Vec<RwSignal<Option<Money>>>,
    #[prop(into)] workdays_to_complete_options: Vec<RwSignal<u64>>,
) -> impl IntoView {
    // -- variables -- //

    let model_file = part.model_file.clone();
    let drawing_file = part.drawing_file.clone();

    // -- clients -- //

    let parts_client = use_context::<PartsClient>().unwrap();

    // actions

    let get_file = create_action(move |file: &File| {
        let file_key = file.key.clone();
        let request = GeneratePresignedUrlRequest {
            key: file_key,
            operation: "Get".to_string(),
        };

        async move {
            let result = parts_client.admin_generate_presigned_url(request).await;

            match result {
                Ok(response) => {
                    web_sys::window()
                        .unwrap()
                        .open_with_url(&response.presigned_url)
                        .expect("Unable to open the file");
                }
                Err(_) => (),
            }
        }
    });

    view! {
        <div class="flex shadow bg-white text-sm my-2 h-80 rounded-xl overflow-hidden p-4 space-x-2">
            <div class="flex flex-col items-center">
                <div class="flex-shrink-0 w-80">
                    <img
                        class="object-scale-down"
                        src="https://cdn.dribbble.com/userupload/11259598/file/original-70a5fe9cc326f004bb78e36ee5e9d8a7.png?resize=300x0"
                        alt="User Image"
                    />
                </div>
            </div>
            <div class="flex-col grow">
                <div class="flex space-x-2">
                    <label for=format!("model-file-{}", part.model_file.name.clone())>
                        <Button
                            class="inline-flex items-center rounded-xl bg-gray-100 hover:bg-red-100 px-3 py-1 my-1 text-xs font-medium text-gray-600 ring-inset hover:ring-gray-600 cursor-pointer"
                            round=true
                            on_click=move |_| { get_file.dispatch(model_file.clone()) }
                        >

                            <img
                                style="width: 18px; height: 18px;"
                                src="https://icons.veryicon.com/png/o/construction-tools/cloud-device/spare-part-type-01.png"
                                alt="User Image"
                            />
                            <div>{part.model_file.name}</div>
                        </Button>
                    </label>
                    {match drawing_file {
                        Some(file) => {
                            let file_clone = file.clone();
                            view! {
                                <div>
                                    <Button
                                        class="inline-flex items-center rounded-xl bg-red-50 hover:bg-red-100 px-3 py-1 my-1 text-xs font-medium text-red-600 ring-inset hover:ring-red-600 cursor-pointer"
                                        round=true
                                        color=Error
                                        on_click=move |_| { get_file.dispatch(file_clone.clone()) }
                                    >

                                        <img
                                            style="width: 18px; height: 18px;"
                                            src="https://icons.veryicon.com/png/o/transport/traffic-2/pdf-34.png"
                                            alt="User Image"
                                        />
                                        <div>{file.name}</div>
                                    </Button>
                                </div>
                            }
                        }
                        None => view! { <div></div> },
                    }}

                </div>
                <div class="flex items-baseline">
                    <p class="font-bold text-base pr-2">"Process:"</p>
                    <p class="text-md text-gray-900">{part.process.to_string()}</p>
                </div>
                <div class="flex items-baseline">
                    <p class="font-bold text-base pr-2">"Material:"</p>
                    <p class="text-md text-gray-900">
                        {match part.attributes.clone() {
                            PartAttributes::CNC(attributes) => attributes.material,
                        }}

                    </p>
                </div>
                <div class="flex items-baseline">
                    <p class="font-bold text-base pr-2">"Tolerance:"</p>
                    <p class="text-md text-gray-900">
                        {match part.attributes.clone() {
                            PartAttributes::CNC(attributes) => attributes.tolerance,
                        }}

                    </p>
                </div>
                <div class="flex items-baseline">
                    <p class="font-bold text-base pr-2">"Quantity:"</p>
                    <p class="text-md text-gray-900">{part.quantity}</p>
                </div>
                <p class="font-bold text-base pr-2">"Additional Notes:"</p>
                <div class="flex w-96">{part.additional_notes}</div>
            </div>
            <div class="flex flex-col w-96 space-y-2">
                <PartQuoteCard
                    price_option=price_options[0]
                    workdays_to_complete_option=workdays_to_complete_options[0]
                />
                <PartQuoteCard
                    price_option=price_options[1]
                    workdays_to_complete_option=workdays_to_complete_options[1]
                />
                <PartQuoteCard
                    price_option=price_options[2]
                    workdays_to_complete_option=workdays_to_complete_options[2]
                />
            </div>
        </div>
    }
}
