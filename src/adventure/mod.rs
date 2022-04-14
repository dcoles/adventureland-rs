//! AdventureLand bindings.

mod sys;

use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::JsFuture;

pub use sys::{
    safe_log,
    set_message,
};

/// Move to position.
pub async fn move_(x: i32, y: i32) {
    JsFuture::from(sys::move_(x, y)).await.unwrap();
}

/// Move to location using pathfinding.
pub async fn smart_move(location: &str) {
    JsFuture::from(sys::smart_move(location)).await.unwrap();
}
