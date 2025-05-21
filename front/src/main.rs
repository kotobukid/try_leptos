mod components;
mod store;

use crate::components::{ButtonCustom, SimpleCounter, TimerDemo, TodoItemsAmount, TodoList};
use crate::store::data;
use leptos::prelude::*;
use gloo_net::http::Request;
use leptos::logging::log;
use reactive_stores::Store;
use wasm_bindgen_futures::spawn_local;

fn main() {
    _ = console_log::init_with_level(log::Level::Debug);
    console_error_panic_hook::set_once();

    spawn_local(async move {
        match Request::get("http://127.0.0.1:3000/api/hello").send().await {
            Ok(response) => {
                if response.ok() {
                    if let Ok(text) = response.text().await {
                        log!("成功! レスポンス: {}", text);
                    } else {
                        log!("レスポンスのテキストを取得できませんでした");
                    }
                } else {
                    log!("サーバーでエラー: {}", response.status());
                }
            }
            Err(err) => {
                log!("リクエストエラー: {:?}", err);
            }
        }
    });

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
