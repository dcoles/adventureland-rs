mod adventure;
mod bt;
mod util;

use std::time::Duration;

use serde_json::json;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::spawn_local;
use web_sys::console;

use crate::adventure::smart::get_smart;
use crate::adventure::sys::{get_chests, is_in_range};
use crate::adventure::target::{get_nearest_monster, get_target};
use crate::adventure::*;
use crate::util::*;
use crate::bt::{BehaviorTree, Node, NodeResult};

macro_rules! console_log {
    ($($t:tt)*) => (console::log_1(&format_args!($($t)*).to_string().into()))
}

struct Actor {
    tree: BehaviorTree,
}

impl Actor {
    pub fn new() -> Self {
        Actor {
            tree: BehaviorTree::new(
                Node::root(
                    Node::selector(vec![
                        Node::sequence(vec![
                            Node::condition(|_node| {
                                console_log!("CONDITION: is_low_hp?");
                                let character = get_character();

                                (character.hp() as f64) < 0.5 * (character.max_hp() as f64)
                            }),
                            Node::action(|_node| {
                                console_log!("ACTION: use_hp");
                                if is_on_cooldown("use_hp") {
                                    console_log!("> is_on_cooldown(\"use_hp\")");
                                    return NodeResult::Running;
                                }

                                console_log!("> use_skill(\"use_hp\")");
                                spawn_local(async { use_skill("use_hp".into()).await.ok(); });

                                NodeResult::Success
                            }),
                        ]),
                        Node::sequence(vec![
                            Node::condition(|_node| {
                                console_log!("CONDITION: is_low_mp?");
                                let character = get_character();

                                (character.mp() as f64) < 0.5 * (character.max_mp() as f64)
                            }),
                            Node::action(|_node| {
                                console_log!("ACTION: use_hp");
                                if is_on_cooldown("use_mp") {
                                    console_log!("> is_on_cooldown(\"use_mp\")");
                                    return NodeResult::Running;
                                }

                                console_log!("> use_skill(\"use_mp\")");
                                spawn_local(async { use_skill("use_mp".into()).await.ok(); });

                                NodeResult::Success
                            }),
                        ]),
                        Node::sequence(vec![
                            Node::condition(|_node| {
                                console_log!("CONDITION: is_chests?");
                                js_sys::Object::keys(&get_chests()).into_iter().count() > 0
                            }),
                            Node::action(|_node| {
                                console_log!("ACTION: loot");
                                for id in js_sys::Object::keys(&get_chests()) {
                                    let id = id.as_string().unwrap();
                                    loot(&id);
                                }

                                NodeResult::Success
                            }),
                        ]),
                        Node::sequence(vec![
                            Node::condition(|_node| {
                                console_log!("CONDITION: has_target?");
                                get_target().is_some()
                            }),
                            Node::selector(vec![
                                Node::sequence(vec![
                                    Node::condition(|_node| {
                                        console_log!("CONDITION: target_in_range?");

                                        get_target()
                                            .map(|target| is_in_range(&target, &"attack".into()))
                                            .unwrap_or_default()
                                    }),
                                    Node::action(|_node| {
                                        console_log!("ACTION: attack");
                                        if is_on_cooldown("attack") {
                                            console_log!("> is_on_cooldown(\"attack\")");
                                            return NodeResult::Running;
                                        }

                                        console_log!("> use_skill(\"attack\")");
                                        spawn_local(async { use_skill("attack".into()).await.ok(); });

                                        NodeResult::Success
                                    }),
                                ]),
                                Node::sequence(vec![
                                    Node::action(|_node| {
                                        console_log!("ACTION: move_to_target");
                                        if let Some(target) = get_target() {
                                            console_log!("@target: {}", target.id());
                                            if get_smart().moving() {
                                                return NodeResult::Running;
                                            }
                                            spawn_local(async { smart_move(target.into()).await.ok(); });
                                            NodeResult::Running
                                        } else {
                                            console_log!("no target!");
                                            NodeResult::Failure
                                        }
                                    }),
                                ]),
                            ])
                        ]),
                        Node::sequence(vec![
                            Node::selector(vec![
                                Node::action(|_node| {
                                    console_log!("ACTION: target_nearest_monster");
                                    let args = serde_wasm_bindgen::to_value(&json!({
                                        "no_target": true,
                                    })).unwrap();
                                    if let Some(target) = get_nearest_monster(&args) {
                                        console_log!("target: {target:?}");
                                        change_target(target.into());
                                        NodeResult::Success
                                    } else {
                                        change_target(JsValue::null());
                                        NodeResult::Failure
                                    }
                                }),
                                Node::action(|_node| {
                                    console_log!("ACTION: move_to_mob");
                                    if get_smart().moving() {
                                        return NodeResult::Running;
                                    }
                                    spawn_local(async { smart_move("bee".into()).await.ok(); });
                                    NodeResult::Running
                                }),
                            ])
                        ]),
                    ]),
                )
            ),
        }
    }

    pub fn tick(&mut self) -> NodeResult {
        self.tree.tick()
    }
}


#[wasm_bindgen(start)]
pub async fn main() -> Result<(), JsValue> {
    console_error_panic_hook::set_once();

    adventure::set_message("🦀");
    adventure::safe_log("Hello, world!");
    console_log!("character: {:?}", get_character());

    let mut actor = Actor::new();

    loop {
        let status = actor.tick();
        let now = web_time::SystemTime::now();
        console_log!("Tick @ {now:?}: {status:?}");

        sleep(Duration::from_millis(100)).await;
    }
}
