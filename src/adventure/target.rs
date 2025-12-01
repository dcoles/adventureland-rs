use std::fmt::Debug;

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern {
    pub type Target;

    pub fn get_nearest_monster() -> Option<Target>;
    pub fn get_target() -> Option<Target>;

    #[wasm_bindgen(method, getter)]
    pub fn name(this: &Target) -> String;

    #[wasm_bindgen(method, getter, js_name = "type")]
    pub fn type_(this: &Target) -> String;

    #[wasm_bindgen(method, getter)]
    pub fn id(this: &Target) -> String;
}

impl Debug for Target {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f
            .debug_struct("Target")
            .field("name", &self.name())
            .field("type", &self.type_())
            .field("id", &self.id())
            .finish()
    }
}
