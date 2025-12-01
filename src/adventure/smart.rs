use std::fmt::Debug;

use wasm_bindgen::prelude::*;

#[wasm_bindgen(inline_js = "export function get_smart() { return smart; }")]
extern {
    pub type Smart;

    pub fn get_smart() -> Smart;

    #[wasm_bindgen(method, getter)]
    pub fn moving(this: &Smart) -> bool;
}

impl Debug for Smart {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f
            .debug_struct("Smart")
            .field("moving", &self.moving())
            .finish()
    }
}
