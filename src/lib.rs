mod adventure;
mod util;

use std::collections::HashMap;
use std::future::Future;
use std::sync::{Arc, Mutex};
use std::time::Duration;

use js_sys::Array;
use serde_json::{Value, json};
use wasm_bindgen::{prelude::*, throw_str};
use wasm_bindgen_futures::spawn_local;
use web_sys::console;
use bonsai_mdsl::{BehaviorTree, NodeResult, TreeContext};
use async_channel::{Receiver, bounded};

use crate::adventure::skills::get_skill;
use crate::adventure::sys::{get_chests, is_in_range};
use crate::adventure::target::{get_nearest_monster, get_target};
use crate::adventure::*;
use crate::util::*;

const MDSL: &str = r#"
root {
    while [CharacterIsAlive] {
        selector {
            sequence {
                condition [HealthIsLow, 0.75]
                condition [SkillIsReady, "use_hp"]
                action [console_log, "Sequence: Restore Health"]
                action [use_skill, "use_hp"]
            }
            sequence {
                condition [ManaIsLow, 0.75]
                condition [SkillIsReady, "use_mp"]
                action [console_log, "Sequence: Restore Mana"]
                action [use_skill, "use_mp"]
            }
            sequence {
                condition [HasTarget]
                action [console_log, "Sequence: Attack"]
                selector {
                    sequence {
                        condition [TargetInRange]
                        action [wait_for_skill, "attack"]
                        action [use_skill, "attack"]
                        action [loot]
                    }
                    sequence {
                        action [smart_move, "@target"]
                    }
                }
            }
            sequence {
                action [console_log, "Sequence: Find Target"]
                selector {
                    action [target_nearest_monster]
                    lotto {
                        action [smart_move, "goo"]
                        action [smart_move, "bee"]
                    }
                }
            }
        }
    }
}
"#;

macro_rules! console_log {
    ($($t:tt)*) => (console::log_1(&format_args!($($t)*).to_string().into()))
}

type Task = Receiver<bool>;

struct Actor {
    context: TreeContext,
    tree: BehaviorTree,
    current_tasks: Arc<Mutex<HashMap<String, Task>>>,
}

impl Actor {
    pub fn new() -> Self {
        Actor {
            context: TreeContext::new(),
            tree: BehaviorTree::from_mdsl(MDSL).unwrap(),
            current_tasks: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    pub fn register_condition<F>(&mut self, name: &str, func: F)
    where F: Fn(&TreeContext, &[Value]) -> bool + Send + Sync + 'static
    {
        self.context.register_condition(name, {
            let name = name.to_owned();

            move |ctx, args| {
                console_log!("* CONDITION {name} {args:?}");
                let status = func(ctx, args);
                console_log!("  -> {:?}", if status { NodeResult::Success } else { NodeResult::Failure });
                return status;
            }
        });
    }

    pub fn register_action<F>(&mut self, name: &str, func: F)
    where F: Fn(&TreeContext, &[Value]) -> NodeResult + Send + Sync + 'static
    {
        self.context.register_action(name, {
            let name = name.to_owned();

            move |ctx, args| {
                console_log!("* ACTION {name} {args:?}");
                let status = func(ctx, args);
                console_log!("  -> {status:?}");
                return status;
            }
        });
    }

    fn register_async_action<F>(&mut self, name: &str, spawn: impl Fn(&TreeContext, &[Value]) -> F + Send + Sync + 'static)
    where F: Future<Output = bool> + 'static
    {
        let current_task = Arc::clone(&self.current_tasks);

        self.register_action(name, {
            let name = name.to_owned();
            move |ctx, args| {
                //let key = format!("{name}@{args:?}");
                let key = format!("{name}: {}", json!(args));
                let mut current_task = current_task.lock().unwrap();
                if let Some(task) = current_task.get(&key) {
                    if let Ok(result) = task.try_recv() {
                        current_task.remove(&key);
                        return if result { NodeResult::Success } else { NodeResult::Failure };
                    } else {
                        return NodeResult::Running;
                    }
                }

                let (send, recv) = bounded(1);
                let task = spawn(ctx, args);
                spawn_local({
                    let key = key.clone();

                    async move {
                        let result = task.await;
                        console_log!("  - task {key}: {result}");
                        send.send(result).await.unwrap();
                    }
                });
                current_task.insert(key.clone(), recv);

                NodeResult::Running
            }
        });
    }

    pub fn tick(&mut self) -> Result<NodeResult, bonsai_mdsl::BonsaiError> {
        {
            let current_tasks = self.current_tasks.lock().unwrap();
            console_log!("Current tasks: {:?}", current_tasks.keys());
        }
        self.tree.tick(&self.context)
    }
}


#[wasm_bindgen(start)]
pub async fn main() -> Result<(), JsValue> {
    console_error_panic_hook::set_once();

    adventure::set_message("🦀");
    adventure::safe_log("Hello, world!");
    console_log!("character: {:?}", get_character());

    let mut actor = Actor::new();

    actor.register_async_action("move", |_ctx, args| {
        if args.len() != 2 {
            throw_str("move expects 2 arguments");
        }

        let x = args[0].as_f64().expect_throw("x should be valid number");
        let y = args[1].as_f64().expect_throw("y should be valid number");
        console_log!("move {x},{y}");

        async move {
            move_(x, y).await;
            true
        }
    });

    actor.register_async_action("smart_move", |_ctx, args| {
        if args.len() != 1 {
            throw_str("smart_move expects 1 arguments");
        }

        let place = args[0].as_str().expect_throw("place should be valid string").to_owned();
        console_log!("smart_move {place}");

        async move {
            match place.as_str() {
                "@target" => {
                    if let Some(target) = get_target() {
                        console_log!("@target: {}", target.id());
                        smart_move(target.into()).await.is_ok()
                    } else {
                        console_log!("no target!");
                        false
                    }
                },
                _ => smart_move(place.into()).await.is_ok(),
            }
        }
    });

    actor.register_async_action("use_skill", |_ctx, args| {
        if args.len() != 1 {
            throw_str("use_skill expects 1 argument");
        }

        let name = args[0].as_str().expect_throw("skill name should be valid string").to_owned();
        console_log!("use_skill: {name} {args:?}");

        async move {
            let result = use_skill(name.clone()).await;
            console_log!("  {name} => {result:?}");

            result.is_ok()
        }
    });

    /*
    actor.register_action("use_skill", |_ctx, args| {
        if args.len() != 1 {
            throw_str("use_skill expects 1 argument");
        }

        let name = args[0].as_str().expect_throw("skill name should be valid string").to_owned();
        console_log!("use_skill: {name} {args:?}");
    });
    */

    actor.register_action("target_nearest_monster", |_ctx, _args| {
        if let Some(target) = get_nearest_monster() {
            console_log!("target: {target:?}");
            change_target(target.into());
            NodeResult::Success
        } else {
            change_target(JsValue::null());
            NodeResult::Failure
        }
    });

    actor.register_action("console_log", |_ctx, args| {
        let args: Array = args.iter().map(|arg| serde_wasm_bindgen::to_value(arg).unwrap()).collect();

        console::log(&args);
        NodeResult::Success
    });

    actor.register_action("loot", |_ctx, _args| {
        for id in js_sys::Object::keys(&get_chests()) {
            let id = id.as_string().unwrap();
            console_log!("loot: {id}");
            loot(&id);
        }

        NodeResult::Success
    });

    actor.register_action("wait_for_skill", |_ctx, args| {
        let skill = args[0].as_str().expect_throw("skill name should be valid string");

        if is_on_cooldown(skill) { NodeResult::Running } else { NodeResult::Success }
    });

    actor.register_condition("CharacterIsAlive", |_ctx, _args| {
        !get_character().rip()
    });

    actor.register_condition("HealthIsLow", |_ctx, args| {
        let threshold = args.get(0)
            .map(|v| v.as_f64().expect_throw("HealthIsLow threshold should be float"))
            .unwrap_or(1.0);
        let c = get_character();

        (c.hp() as f64) < threshold * (c.max_hp() as f64)
    });

    actor.register_condition("ManaIsLow", |_ctx, args| {
        let threshold = args.get(0)
            .map(|v| v.as_f64().expect_throw("ManaIsLow threshold should be float"))
            .unwrap_or(1.0);
        let c = get_character();

        (c.mp() as f64) < threshold * (c.max_mp() as f64)
    });

    actor.register_condition("SkillIsReady", |_ctx, args| {
        if args.len() != 1 {
            throw_str("SkillIsReady expects 1 argument");
        }

        let name = args[0].as_str().expect_throw("skill name should be valid string");
        let skill = get_skill(name).expect_throw("skill name should be a valid skill");
        let required_mp = skill.mp().unwrap_or(0);

        !is_on_cooldown(name) && get_character().mp() >= required_mp
    });

    actor.register_condition("HasTarget", |_ctx, _args| {
        let target = get_target();
        console_log!("target: {target:?}");

        target.is_some()
    });

    actor.register_condition("TargetInRange", |_ctx, args| {
        let skill = args.get(0)
            .map(|value| serde_wasm_bindgen::to_value(&value).unwrap())
            .unwrap_or(JsValue::NULL);

        get_target().map(|target| is_in_range(&target, &skill.into())).unwrap_or_default()
    });

    loop {
        let status = actor.tick().expect("tick should not fail");
        let now = web_time::SystemTime::now();
        console_log!("Tick @ {now:?}: {status:?}");

        sleep(Duration::from_millis(250)).await;
    }
}
