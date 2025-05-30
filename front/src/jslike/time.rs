use gloo_timers::callback::Timeout;
use std::cell::RefCell;
use std::rc::Rc;

/// タイマーの管理構造体
#[derive(Clone)]
pub struct Timer {
    timeout: Rc<RefCell<Option<Timeout>>>,
}

impl Timer {
    /// タイマーを新規作成
    #[allow(dead_code)]
    pub fn new<F>(f: F, timeout_ms: u32) -> Self
    where
        F: FnOnce() + 'static,
    {
        let timeout = Timeout::new(timeout_ms, f);
        Timer {
            timeout: Rc::new(RefCell::new(Some(timeout))),
        }
    }

    /// タイマーをキャンセル
    pub fn cancel(&self) {
        if let Some(timeout) = self.timeout.borrow_mut().take() {
            timeout.cancel();
        }
    }
}

impl Drop for Timer {
    fn drop(&mut self) {
        self.cancel();
    }
}

/// JavaScriptのsetTimeoutのような非同期タイマー関数（キャンセル不要な場合）
pub fn set_timeout_inner<F>(f: F, timeout_ms: u32)
where
    F: FnOnce() + 'static,
{
    let timeout = Timeout::new(timeout_ms, f);
    // タイマーを明示的に保持し、コールバックが実行されるまで生存させる
    timeout.forget();
}

/// JavaScriptのsetTimeoutのような非同期タイマー関数（キャンセル可能な場合）
/// let timer_id = set_timeout_cancellable(f, timeout_ms);
/// timer_id.cancel(); // タイマーをキャンセルする
#[allow(dead_code)]
pub fn set_timeout_cancellable_inner<F>(f: F, timeout_ms: u32) -> Timeout
where
    F: FnOnce() + 'static,
{
    Timeout::new(timeout_ms, f)
}


/// JavaScriptのsetTimeoutのようなマクロ
///
/// # 使用例
/// ```rust
/// set_timeout!(|| {
///     log!("2秒経過しました！");
/// }, 2000);
/// ```
#[macro_export]
macro_rules! set_timeout {
    // ブロックと時間のみの場合（||なし）
    ({$($body:tt)*}, $ms:expr) => {
        $crate::jslike::set_timeout_inner(move || { $($body)* }, $ms)
    };
    (|| $body:block, $ms:expr) => {
        $crate::jslike::set_timeout_inner(move || $body, $ms)
    };
    (|$($arg:ident),*| $body:block, $ms:expr) => {
        $crate::jslike::set_timeout_inner(move |$($arg),*| $body, $ms)
    };
}

/// キャンセル可能なタイマーを作成するマクロ
#[macro_export]
macro_rules! set_timeout_cancellable {
    ({$body:block}, $ms:expr) => {
        $crate::jslike::set_timeout_cancellable_inner(move || $body, $ms)
    };
    (|| $body:block, $ms:expr) => {
        $crate::jslike::set_timeout_cancellable_inner(move || $body, $ms)
    };
    (|$($arg:ident),*| $body:block, $ms:expr) => {
        $crate::jslike::set_timeout_cancellable_inner(move |$($arg),*| $body, $ms)
    };
}
