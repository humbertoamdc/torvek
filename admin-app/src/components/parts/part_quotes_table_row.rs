use chrono::NaiveDate;
use leptos::*;

use api_boundary::common::money::Money;
use api_boundary::parts::models::Part;

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
            <div class="flex flex-row items-center">
                <div class="flex-shrink-0 w-80">
                    <img
                        class="object-scale-down"
                        src="https://cdn.dribbble.com/userupload/11259598/file/original-70a5fe9cc326f004bb78e36ee5e9d8a7.png?resize=300x0"
                        alt="User Image"
                    />
                </div>
                <div class="ml-3">
                    <p class="text-gray-900 whitespace-no-wrap">{part.model_file.name}</p>
                </div>
                <div></div>
                <div class="flex-col grow">
                    <div class="flex items-baseline">
                        <p class="font-bold text-base pr-2">"Process:"</p>
                        <p class="text-md text-gray-900">{part.process}</p>
                    </div>
                    <div class="flex items-baseline">
                        <p class="font-bold text-base pr-2">"Material:"</p>
                        <p class="text-md text-gray-900">{part.material}</p>
                    </div>
                    <div class="flex items-baseline">
                        <p class="font-bold text-base pr-2">"Tolerance:"</p>
                        <p class="text-md text-gray-900">{part.tolerance}</p>
                    </div>
                    <div class="flex items-baseline">
                        <p class="font-bold text-base pr-2">"Quantity:"</p>
                        <p class="text-md text-gray-900">{part.quantity}</p>
                    </div>
                </div>
                <div class="flex flex-col w-96 space-y-2">
                    <PartQuoteCard
                        price_option=price_options[0]
                        deadline_option=deadline_options[0]
                    />
                    <PartQuoteCard
                        price_option=price_options[1]
                        deadline_option=deadline_options[1]
                    />
                    <PartQuoteCard
                        price_option=price_options[2]
                        deadline_option=deadline_options[2]
                    />
                </div>
            </div>
        </div>
    }
}
