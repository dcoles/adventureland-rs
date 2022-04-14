//! Low-level bindings.

use wasm_bindgen::prelude::*;
use js_sys::Promise;

#[wasm_bindgen]
extern {
    /// Print message to in-game console.
    pub fn safe_log(msg: &str);

    /// Set status message.
    pub fn set_message(msg: &str);

    /// Move to position.
    #[wasm_bindgen(js_name = move)]
    pub fn move_(x: i32, y: i32) -> Promise;

    /// Move to location using pathfinding.
    #[wasm_bindgen(js_name = smart_move)]
    pub fn smart_move(location: &str) -> Promise;
}
