mod components;
mod store;

use crate::components::{ButtonCustom, SimpleCounter, TimerDemo, TodoItemsAmount, TodoList};
use crate::store::data;
use leptos::prelude::*;
use reactive_stores::Store;

fn main() {
    _ = console_log::init_with_level(log::Level::Debug);
    console_error_panic_hook::set_once();

    let todo_store = Store::new(data());

    // カウンターのリストを管理するSignalVecを作成
    let (counters, set_counters) = signal(vec![0, 100]);
    let (label_text, _) = signal(String::from("Custom Button"));

    mount_to_body(move || {
        // グローバルコンテキストとしてStoreを提供すると、他のコンポーネントでも使用可能に
        provide_context(todo_store);

        view! {
            <div style="background-color: lightgreen;">
                <button
                    on:click=move |_| {
                        let mut current = counters.get();
                        current.push(current.len() as i32 * 100);
                        set_counters.set(current);
                    }
                >"Add Counter"</button>
                <br />
                <For each=move || counters.get()
                    key=|count| *count
                    children=move |initial_value| {
                        view! {
                            <SimpleCounter initial_value=initial_value step=10/>
                        }
                    }
                />
                <br />
                <ButtonCustom num={1000} label=label_text />
                <TimerDemo />
                <TodoItemsAmount />
                <TodoList />
            </div>
        }
    })
}
