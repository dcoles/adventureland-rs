use std::fmt::Debug;

use wasm_bindgen::prelude::*;

#[wasm_bindgen(inline_js = "export function get_character() { return parent.character; }")]
extern {
    pub type Character;

    pub fn get_character() -> Character;

    #[wasm_bindgen(method, getter)]
    pub fn name(this: &Character) -> String;

    #[wasm_bindgen(method, getter)]
    pub fn hp(this: &Character) -> i32;

    #[wasm_bindgen(method, getter)]
    pub fn max_hp(this: &Character) -> i32;

    #[wasm_bindgen(method, getter)]
    pub fn mp(this: &Character) -> i32;

    #[wasm_bindgen(method, getter)]
    pub fn max_mp(this: &Character) -> i32;

    #[wasm_bindgen(method, getter)]
    pub fn rip(this: &Character) -> bool;
}

impl Debug for Character {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f
            .debug_struct("Character")
            .field("name", &self.name())
            .finish()
    }
}
