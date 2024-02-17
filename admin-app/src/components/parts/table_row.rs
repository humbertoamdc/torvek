use leptos::*;
use rusty_money::{iso, Money};

use api_boundary::parts::models::Part;

#[component]
pub fn PartsRow<F>(#[prop(into)] part: Part, on_change: F) -> impl IntoView
where
    F: Fn(u64) + 'static,
{
    // -- signals -- //

    let sub_total = create_rw_signal(None::<u64>);

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
                        <p class="text-gray-900 whitespace-no-wrap">{part.model_file.name}</p>
                    </div>
                </div>
            </td>
            <td class="px-2 py-5 border-b border-gray-200 bg-white text-sm">
                <div class="flex justify-center">
                    <p class="text-gray-900 whitespace-no-wrap">{part.process}</p>
                </div>
            </td>
            <td class="px-2 py-5 border-b border-gray-200 bg-white text-sm">
                <div class="flex justify-center">{part.material}</div>

            </td>
            <td class="px-2 py-5 border-b border-gray-200 bg-white text-sm">
                <div class="flex justify-center">
                    <p class="text-gray-900 whitespace-no-wrap">{part.tolerance}</p>
                </div>
            </td>
            <td class="px-2 py-5 border-b border-gray-200 bg-white text-sm">
                <div class="flex justify-center">
                    <p class="text-gray-900 whitespace-no-wrap">{part.quantity}</p>
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
                            value=part.unit_price
                            on:change=move |ev| {
                                let unit_price = (event_target_value(&ev)
                                    .parse::<f64>()
                                    .unwrap() * 100.0) as u64;
                                sub_total.update(|s| *s = Some(unit_price * part.quantity));
                                on_change(unit_price);
                            }
                        />

                    </p>
                </div>
            </td>
            <td class="px-2 py-5 border-b border-gray-200 bg-white text-sm">
                <div class="flex justify-center">
                    <p class="text-gray-900 whitespace-no-wrap">
                        {move || {
                            match sub_total.get() {
                                Some(sub_total) => {
                                    Money::from_minor(sub_total as i64, iso::MXN).to_string()
                                }
                                None => String::from("N/A"),
                            }
                        }}

                    </p>
                </div>
            </td>
        </tr>
    }
}
