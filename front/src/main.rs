mod button_custom;
mod counter;
mod timer_label;

use crate::button_custom::ButtonCustom;
use crate::counter::SimpleCounter;
use crate::timer_label::TimerDemo;
// use leptos::dom::*;
use leptos::prelude::*;

fn main() {
    _ = console_log::init_with_level(log::Level::Debug);
    console_error_panic_hook::set_once();

    // カウンターのリストを管理するSignalVecを作成
    let (counters, set_counters) = signal(vec![0, 100]);
    let (label_text, _) = signal(String::from("Custom Button"));

    mount_to_body(move || {
        view! {
            <div>
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
            </div>
        }
    })
}
