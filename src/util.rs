use std::time::Duration;

use wasm_bindgen_futures::JsFuture;

#[derive(Debug)]
pub struct TimeoutID(pub i32);

pub fn window() -> web_sys::Window {
    web_sys::window().expect("global `window` should exist")
}

pub fn set_timeout(handler: &js_sys::Function, duration: Duration) -> TimeoutID {
    let timeout = duration.as_millis().try_into().unwrap_or(i32::MAX);

    TimeoutID(
        window().set_timeout_with_callback_and_timeout_and_arguments_0(handler, timeout).unwrap()
    )
}

pub fn clear_timeout(id: TimeoutID) {
    window().clear_timeout_with_handle(id.0);
}

pub async fn sleep(duration: Duration) {
    let promise = js_sys::Promise::new(&mut |resolve, _reject| {
        set_timeout(&resolve, duration);
    });

    JsFuture::from(promise).await.ok();
}
