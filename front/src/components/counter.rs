use leptos::prelude::*;

/// A simple counter component.
///
/// You can use doc comments like this to document your component.
#[component]
pub fn SimpleCounter(
    /// The starting value for the counter
    initial_value: i32,
    /// The change that should be applied each time the button is clicked.
    step: i32,
) -> impl IntoView {
    let (value, set_value) = signal(initial_value);

    view! {
        <div class="my-4">
            <table class="border-collapse table-fixed w-full max-w-md mx-auto border border-gray-300 rounded">
                <tbody>
                    <tr>
                        <td class="border border-gray-300 text-center p-2">
                            <button 
                                class="bg-red-500 hover:bg-red-700 text-white font-bold py-1 px-3 rounded"
                                on:click=move |_| set_value.set(0)
                            >
                                "Clear"
                            </button>
                        </td>
                        <td class="border border-gray-300 text-center p-2">
                            <button 
                                class="bg-yellow-500 hover:bg-yellow-700 text-white font-bold py-1 px-3 rounded"
                                on:click=move |_| *set_value.write() -= step
                            >
                                "-1"
                            </button>
                        </td>
                        <td class="border border-gray-300 text-center p-2 text-brown-600 font-medium">
                            <span>"Value: " {value} "!"</span>
                        </td>
                        <td class="border border-gray-300 text-center p-2">
                            <button 
                                class="bg-green-500 hover:bg-green-700 text-white font-bold py-1 px-3 rounded"
                                on:click=move |_| set_value.update(|value| *value += step)
                            >
                                "+1"
                            </button>
                        </td>
                    </tr>
                </tbody>
            </table>
        </div>
    }
}
