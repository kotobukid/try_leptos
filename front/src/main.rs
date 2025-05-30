mod components;
mod jslike;
mod store;

use crate::components::{ButtonCustom, SimpleCounter, TimerDemo, TodoItemsAmount, TodoList};
use crate::store::data;
use datapack::export_features;
use gloo_net::http::Request;
use leptos::logging::log;
use leptos::prelude::*;
use reactive_stores::Store;
use wasm_bindgen_futures::spawn_local;

fn main() {
    _ = console_log::init_with_level(log::Level::Debug);
    console_error_panic_hook::set_once();

    let (response_text, set_response_text) = signal(String::from("initial string"));

    jslike::set_timeout(move || {
        let text = "start ajax calling.".to_string();
        set_response_text.set(text);
        spawn_local(async move {
            match Request::get("/api/hello").send().await {
                Ok(response) => {
                    if response.ok() {
                        if let Ok(text) = response.text().await {
                            log!("成功! レスポンス: {}", &text);
                            let set_response_text = set_response_text.clone();
                            let text = text.clone(); // Clone text to ensure 'static lifetime
                            jslike::set_timeout(
                                move || {
                                    set_response_text.set(text);
                                },
                                2000,
                            );
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
    }, 2000);

    let todo_store = Store::new(data());

    // カウンターのリストを管理するSignalVecを作成
    let (counters, set_counters) = signal(vec![0, 100]);
    let (label_text, _) = signal(String::from("Custom Button"));

    let cards = export_features();
    log!("Cards {:?}", cards);

    // Convert the HashMap to a list of li elements
    let card_list = move || {
        let mut list = Vec::new();
        for (feature_name, cards_vec) in cards.iter() {
            let feature_item = view! {
                <li>
                    {feature_name.clone()}
                    <ul>
                        {cards_vec.iter().map(|card| {
                            view! {
                                <li>"ID: " {card.id} ", Name: " {card.name.clone()}</li>
                            }
                        }).collect_view()}
                    </ul>
                </li>
            };
            list.push(feature_item);
        }
        list.into_iter().collect_view()
    };

    mount_to_body(move || {
        // グローバルコンテキストとしてStoreを提供すると、他のコンポーネントでも使用可能に
        provide_context(todo_store);

        view! {
            <div style="background-color: lightgreen;">
                <div>"APIレスポンス: " {response_text}</div>
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
                <hr />
                <ul>
                    {card_list()}
                </ul>
            </div>
        }
    })
}
