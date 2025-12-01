//! AdventureLand bindings.

pub mod sys;
pub mod character;
pub mod skills;
pub mod target;
pub mod smart;

use serde_json::Value;
use wasm_bindgen::{JsValue, UnwrapThrowExt};
use wasm_bindgen_futures::JsFuture;

pub use sys::{
    safe_log,
    set_message,
    is_on_cooldown,
    change_target,
    loot
};

pub use character::{Character, get_character};

/// Move to position.
pub async fn move_(x: f64, y: f64) {
    JsFuture::from(sys::move_(x, y)).await.unwrap();
}

/// Move to location using pathfinding.
pub async fn smart_move(destination: JsValue) -> Result<JsValue, JsValue> {
    JsFuture::from(sys::smart_move(destination)).await
}

/// Use skill.
pub async fn use_skill(name: String) -> Result<Value, Value> {
    js_to_json_result(
        JsFuture::from(sys::use_skill(name)).await
    )
}

fn js_to_json_result(result: Result<JsValue, JsValue>) -> Result<Value, Value> {
    result.map(|ok| serde_wasm_bindgen::from_value(ok).expect_throw("non-JSON Ok result"))
        .map_err(|err| serde_wasm_bindgen::from_value(err).expect_throw("non-JSON Err result"))
}
