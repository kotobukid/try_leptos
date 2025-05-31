use leptos::prelude::*;
use std::time::Duration;

/// Timer example, demonstrating the use of `use_interval`.
#[component]
pub fn TimerDemo() -> impl IntoView {
    // count_a updates with a fixed interval of 1000 ms, whereas count_b has a dynamic
    // update interval.
    let count_a = RwSignal::new(0_i32);
    let count_b = RwSignal::new(0_i32);

    let interval = RwSignal::new(1200);

    use_interval(1000, move || {
        count_a.update(|c| *c += 1);
    });
    use_interval(interval, move || {
        count_b.update(|c| *c += 1);
    });

    view! {
        <div class="block w-64 bg-pink-200 p-4 border border-gray-400 rounded-lg shadow-md">
            <div class="font-semibold mb-2">"Count A (fixed interval of 1000 ms)"</div>
            <div class="text-2xl font-bold text-blue-600 mb-3">{count_a}</div>
            <div class="font-semibold mb-2">"Count B (dynamic interval, currently " {interval} " ms)"</div>
            <div class="text-2xl font-bold text-purple-600 mb-3">{count_b}</div>
            <input 
                class="w-full p-2 border border-gray-300 rounded focus:outline-none focus:ring-2 focus:ring-pink-500"
                prop:value=interval 
                on:input:target=move |ev| {
                    if let Ok(value) = ev.target().value().parse::<u64>() {
                        interval.set(value);
                    }
                }
            />
        </div>
    }
}

/// Hook to wrap the underlying `setInterval` call and make it reactive w.r.t.
/// possible changes of the timer interval.
pub fn use_interval<T, F>(interval_millis: T, f: F)
where
    F: Fn() + Clone + 'static,
    T: Into<Signal<u64>> + 'static,
{
    let interval_millis = interval_millis.into();
    Effect::new(move |prev_handle: Option<IntervalHandle>| {
        // effects get their previous return value as an argument
        // each time the effect runs, it will return the interval handle
        // so if we have a previous one, we cancel it
        if let Some(prev_handle) = prev_handle {
            prev_handle.clear();
        };

        // here, we return the handle
        set_interval_with_handle(
            f.clone(),
            // this is the only reactive access, so this effect will only
            // re-run when the interval changes
            Duration::from_millis(interval_millis.get()),
        )
            .expect("could not create interval")
    });
}
