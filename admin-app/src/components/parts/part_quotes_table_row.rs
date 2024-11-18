use chrono::NaiveDate;
use leptos::*;

use api_boundary::common::money::Money;
use api_boundary::parts::models::{Part, PartAttributes};

use crate::components::parts::part_quote_card::PartQuoteCard;

#[component]
pub fn PartQuotesTableRow(
    #[prop(into)] part: Part,
    #[prop(into)] price_options: Vec<RwSignal<Option<Money>>>,
    #[prop(into)] deadline_options: Vec<RwSignal<Option<NaiveDate>>>,
) -> impl IntoView {
    // -- signals -- //
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
                        <input
                            id=format!("model-file-{}", part.model_file.name.clone())
                            type="file"
                            class="hidden"
                            accept=".stp,.step"
                            on:change=move |_| {}
                        />

                        <span class="inline-flex items-center rounded-xl bg-gray-100 hover:bg-red-100 px-3 py-1 my-1 text-xs font-medium text-gray-600 ring-1 ring-inset hover:ring-gray-600 cursor-pointer ">
                            <img
                                style="width: 18px; height: 18px;"
                                src="https://icons.veryicon.com/png/o/construction-tools/cloud-device/spare-part-type-01.png"
                                alt="User Image"
                            />
                            <div class="ml-1">{part.model_file.name}</div>

                        </span>
                    </label>
                    {match part.drawing_file {
                        Some(drawing_file) => {
                            view! {
                                <label for=format!("drawing-file-{}", drawing_file.name)>
                                    <input
                                        id=format!("drawing-file-{}", drawing_file.name)

                                        type="file"
                                        class="hidden"
                                        accept=".pdf"
                                    />

                                    <span class="inline-flex items-center rounded-xl bg-red-50 hover:bg-red-100 px-3 py-1 my-1 text-xs font-medium text-red-600 ring-1 ring-inset hover:ring-red-600 cursor-pointer ">
                                        <img
                                            style="width: 18px; height: 18px;"
                                            src="https://icons.veryicon.com/png/o/transport/traffic-2/pdf-34.png"
                                            alt="User Image"
                                        />
                                        <div class="ml-1">{drawing_file.name}</div>

                                    </span>
                                </label>
                            }
                        }
                        None => view! { <label></label> },
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
            </div>
            <div class="flex flex-col w-96 space-y-2">
                <PartQuoteCard price_option=price_options[0] deadline_option=deadline_options[0]/>
                <PartQuoteCard price_option=price_options[1] deadline_option=deadline_options[1]/>
                <PartQuoteCard price_option=price_options[2] deadline_option=deadline_options[2]/>
            </div>
        </div>
    }
}
