use leptos::prelude::ElementChild;
use leptos::prelude::ReadSignal;
use leptos::prelude::*;
use leptos::{component, view, IntoView};

#[component]
pub fn ButtonCustom(num: i64, label: ReadSignal<String>) -> impl IntoView {
    view! {
      <button class="bg-purple-500 hover:bg-purple-700 text-white font-bold py-2 px-4 rounded transition duration-300 ease-in-out transform hover:scale-105">
        <span class="block text-lg">{ label }</span>
        <span class="block text-sm text-purple-200 mt-1">{ num }</span>
      </button>
    }
}
