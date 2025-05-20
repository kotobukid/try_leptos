use leptos::prelude::ElementChild;
use leptos::prelude::ReadSignal;
use leptos::{component, view, IntoView};

#[component]
pub fn ButtonCustom(num: i64, label: ReadSignal<String>) -> impl IntoView {
    view! {
      <button>
        { label }
        <br />
        { num }
      </button>
    }
}