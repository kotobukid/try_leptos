use gloo_timers::callback::Timeout;
// use std::time::Duration;
// use leptos::leptos_dom::helpers::set_timeout as set_timeout_leptos;
/// JavaScriptのsetTimeoutのような非同期タイマー関数
///
/// # 引数
/// * `f` - タイムアウト時に実行されるクロージャ
/// * `duration` - タイムアウトまでの時間
///
/// # 使用例
/// ```rust
/// set_timeout(|| {
///     log!("2秒経過しました！");
/// }, 2000);
/// ```
pub fn set_timeout<F>(f: F, timeout_ms: u32)
where
    F: FnOnce() + 'static,
{
    let _timeout = Timeout::new(timeout_ms, f);
    // Intentionally forget the timeout to prevent it from being cancelled when it goes out of scope
    _timeout.forget();
}
