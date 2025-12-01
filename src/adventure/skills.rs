use std::fmt::Debug;

use wasm_bindgen::prelude::*;

#[wasm_bindgen(inline_js = "export function get_skill(id) { return G.skills[id]; }")]
extern {
    pub type Skill;

    pub fn get_skill(id: &str) -> Option<Skill>;

    #[wasm_bindgen(method, getter)]
    pub fn name(this: &Skill) -> String;

    #[wasm_bindgen(method, getter, js_name = "type")]
    pub fn type_(this: &Skill) -> String;

    #[wasm_bindgen(method, getter)]
    pub fn mp(this: &Skill) -> Option<i32>;
}

impl Debug for Skill {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f
            .debug_struct("Skill")
            .field("type", &self.type_())
            .field("name", &self.name())
            .finish()
    }
}
