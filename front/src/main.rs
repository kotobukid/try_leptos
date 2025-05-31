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

    set_timeout!(
        {
            let text = "start ajax calling.".to_string();
            set_response_text.set(text);
            spawn_local(async move {
                match Request::get("/api/hello").send().await {
                    Ok(response) => {
                        if response.ok() {
                            if let Ok(text) = response.text().await {
                                log!("成功! レスポンス: {}", &text);

                                let text = text.clone(); // Clone text to ensure 'static lifetime
                                set_timeout!(
                                    {
                                        set_response_text.set(text);
                                    },
                                    2000
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
        },
        2000
    );

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
            <div class="container mx-auto p-6 bg-green-100 rounded-lg shadow-md">
                <div class="mb-4 text-lg font-semibold">"APIレスポンス: " <span class="text-blue-600">{response_text}</span></div>
                <button
                    class="bg-blue-500 hover:bg-blue-700 text-white font-bold py-2 px-4 rounded mb-4"
                    on:click=move |_| {
                        let mut current = counters.get();
                        current.push(current.len() as i32 * 100);
                        set_counters.set(current);
                    }
                >"Add Counter"</button>
                <div class="my-4">
                    <For each=move || counters.get()
                        key=|count| *count
                        children=move |initial_value| {
                            view! {
                                <SimpleCounter initial_value=initial_value step=10/>
                            }
                        }
                    />
                </div>
                <div class="my-4">
                    <ButtonCustom num={1000} label=label_text />
                </div>
                <div class="my-4">
                    <TimerDemo />
                </div>
                <div class="my-4">
                    <TodoItemsAmount />
                </div>
                <div class="my-4">
                    <TodoList />
                </div>
                <hr class="my-6 border-gray-300" />
                <div class="bg-white p-4 rounded-lg shadow">
                    <h2 class="text-xl font-bold mb-2">Card List</h2>
                    <ul class="list-disc pl-6">
                        {card_list()}
                    </ul>
                </div>
            </div>
        }
    })
}
