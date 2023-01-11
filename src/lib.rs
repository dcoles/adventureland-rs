mod adventure;

use wasm_bindgen::prelude::*;
use web_sys::console;

#[wasm_bindgen(start)]
pub async fn main() -> Result<(), JsValue> {
    console_error_panic_hook::set_once();

    console::log_1(&JsValue::from("Hello, world!"));
    adventure::set_message("🦀");
    adventure::safe_log("Hello, world!");

    loop {
        adventure::move_(-50, -50).await;
        adventure::move_(50, -50).await;
        adventure::move_(50, 50).await;
        adventure::move_(-50, 50).await;
    }

    Ok(())
}
