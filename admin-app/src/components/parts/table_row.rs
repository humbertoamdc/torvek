use crate::api::parts::PartsClient;
use crate::models::ReactivePart;
use api_boundary::parts::requests::AdminUpdatePartRequest;
use leptos::*;
use rusty_money::{iso, Money};

#[component]
pub fn PartsRow(#[prop(into)] reactive_part: ReactivePart) -> impl IntoView {
    let parts_client = use_context::<PartsClient>().unwrap_or(PartsClient::new());

    let update_part = create_action(move |_| {
        let update_part_request = AdminUpdatePartRequest {
            id: reactive_part.id.clone(),
            client_id: reactive_part.client_id.clone(),
            project_id: reactive_part.project_id.clone(),
            quotation_id: reactive_part.quotation_id.clone(),
            unit_price: reactive_part.unit_price.get_untracked().unwrap(),
            sub_total: reactive_part.sub_total.get_untracked().unwrap(),
        };

        async move {
            let response = parts_client.update_part(update_part_request).await;

            match response {
                Ok(_) => (),
                Err(_) => (),
            }
        }
    });

    view! {
        <tr>
            <td class="px-2 py-5 border-b border-gray-200 bg-white text-sm">
                <div class="flex items-center">
                    <div class="flex-shrink-0 w-10 h-10">
                        <img
                            class="w-full h-full rounded-full"
                            src="https://cdn.dribbble.com/userupload/11259598/file/original-70a5fe9cc326f004bb78e36ee5e9d8a7.png?resize=100x0"
                            alt="User Image"
                        />
                    </div>
                    <div class="ml-3">
                        <p class="text-gray-900 whitespace-no-wrap">{reactive_part.model_file.name}</p>
                    </div>
                </div>
            </td>
            <td class="px-2 py-5 border-b border-gray-200 bg-white text-sm">
                <div class="flex justify-center">
                    <p class="text-gray-900 whitespace-no-wrap">
                        {reactive_part.process}
                    </p>
                </div>
            </td>
            <td class="px-2 py-5 border-b border-gray-200 bg-white text-sm">
                <div class="flex justify-center">{reactive_part.material}</div>

            </td>
            <td class="px-2 py-5 border-b border-gray-200 bg-white text-sm">
                <div class="flex justify-center">
                    <p class="text-gray-900 whitespace-no-wrap">{reactive_part.tolerance}</p>
                </div>
            </td>
            <td class="px-2 py-5 border-b border-gray-200 bg-white text-sm">
                <div class="flex justify-center">
                    <p class="text-gray-900 whitespace-no-wrap">{reactive_part.quantity}</p>
                </div>
            </td>
            <td class="px-2 py-5 border-b border-gray-200 bg-white text-sm">
                <div class="flex justify-center">
                    <p class="text-gray-900 whitespace-no-wrap">
                        <input
                            type="number"
                            id="quantity"
                            name="quantity"
                            min=1
                            class="w-32 px-3 py-2 text-center bg-white border border-gray-300 rounded-md text-sm focus:outline-none focus:ring-indigo-500 focus:border-indigo-500"
                            placeholder="N/A"
                            value=reactive_part.unit_price.get_untracked()
                            on:change=move |ev| {
                                let unit_price = (event_target_value(&ev).parse::<f64>().unwrap() * 100.0) as u64;
                                reactive_part.unit_price.update(|u| *u = Some(unit_price));
                                reactive_part
                                    .sub_total
                                    .update(|s| {
                                        *s = Some(
                                            unit_price * reactive_part.quantity
                                        );
                                    });
                            }
                        />

                    </p>
                </div>
            </td>
            <td class="px-2 py-5 border-b border-gray-200 bg-white text-sm">
                <div class="flex justify-center">
                    <p class="text-gray-900 whitespace-no-wrap">
                        {move || {
                            match reactive_part.sub_total.get() {
                                Some(sub_total) => Money::from_minor(sub_total as i64, iso::MXN).to_string(),
                                None => String::from("N/A"),
                            }
                        }}

                    </p>
                </div>
            </td>
            <td class="px-2 py-3 border-b border-gray-200 bg-white text-sm">
                <div class="flex justify-center">
                    <p class="text-gray-900 whitespace-no-wrap">
                        <button
                            class="px-5 py-3 text-white bg-indigo-600 rounded-lg duration-150 hover:bg-indigo-700 active:shadow-lg"
                            on:click=move |_| update_part.dispatch(())
                        >
                            Submit
                        </button>
                    </p>
                </div>
            </td>
        </tr>
    }
}
