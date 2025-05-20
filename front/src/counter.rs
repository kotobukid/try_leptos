use leptos::prelude::*;
use leptos_style::Style;

/// A simple counter component.
///
/// You can use doc comments like this to document your component.
#[component]
pub fn SimpleCounter(
    #[prop(into, optional)] style: Style,
    /// The starting value for the counter
    initial_value: i32,
    /// The change that should be applied each time the button is clicked.
    step: i32,
) -> impl IntoView {
    let (value, set_value) = signal(initial_value);

    let bordered_style = style.clone().with_defaults([
        ("border", "1px solid black"),
        ("text-align", "center"),
        ("padding", "5px")
    ]);

    let colored_style = bordered_style.clone().with_defaults([
        ("color", "brown")
    ]);

    view! {
        <table style={style.with_defaults([
            ("border-collapse", "collapse"),
            ("table-layout", "fixed"),
            ("width", "410px"),
            ("margin", "2px 0"),
        ])}>
            <colgroup>
                <col style="width: 100px" />
                <col style="width: 100px" />
                <col style="width: 100px" />
                <col style="width: 100px" />
            </colgroup>
            <tbody>
                <td style={bordered_style.clone()}>
                    <button on:click=move |_| set_value.set(0)>"Clear"</button>
                </td>
                <td style={bordered_style.clone()}>
                    <button on:click=move |_| *set_value.write() -= step>"-1"</button>
                </td>
                <td style={colored_style}>
                    <span>"Value: " {value} "!"</span>
                </td>
                <td style={bordered_style.clone()}>
                    <button on:click=move |_| set_value.update(|value| *value += step)>"+1"</button>
                </td>
            </tbody>
        </table>
    }
}
