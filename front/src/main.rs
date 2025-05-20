mod button_custom;
mod counter;

use crate::button_custom::ButtonCustom;
use crate::counter::SimpleCounter;
use leptos::prelude::*;

fn main() {
    _ = console_log::init_with_level(log::Level::Debug);
    console_error_panic_hook::set_once();
    let (label_text, _) = signal(String::from("Custom Button"));

    mount_to_body(move || {
        view! {
            <div>
                <SimpleCounter initial_value=0 step=10/>
                <br />
                <ButtonCustom num={1000} label=label_text />
            </div>
        }
    })
}
