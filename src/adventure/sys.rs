//! Low-level bindings.

use wasm_bindgen::prelude::*;
use js_sys::{Object, Promise};

#[wasm_bindgen]
extern {
    /// Print message to in-game console.
    pub fn safe_log(msg: &str);

    /// Set status message.
    pub fn set_message(msg: &str);

    /// Move to position.
    #[wasm_bindgen(js_name = move)]
    pub fn move_(x: f64, y: f64) -> Promise;

    /// Move to location using pathfinding.
    pub fn smart_move(destination: JsValue) -> Promise;

    /// Use skill.
    pub fn use_skill(name: String) -> Promise;

    /// Check if skill is on cooldown
    pub fn is_on_cooldown(name: &str) -> bool;

    /// Change target.
    pub fn change_target(target: JsValue) -> JsValue;

    // Check if target is in range.
    pub fn is_in_range(target: &JsValue, skill: &JsValue) -> bool;

    /// Get nearby chests.
    pub fn get_chests() -> Object;

    /// Loot chest.
    pub fn loot(id: &str);
}
