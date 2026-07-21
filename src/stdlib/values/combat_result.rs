use rhai::{Array, EvalAltResult, Module};

use crate::stdlib::util;

const RESULT_NAMES: &[(i64, &str)] = &[
    (0, "战斗未结束"),
    (1, "战斗失败"),
    (2, "战斗胜利"),
    (3, "战斗逃跑"),
    (255, "战斗结束"),
];

pub fn register(module: &mut Module) {
    module.set_var("UNCLOSED", 0_i64);
    module.set_var("LOSE", 1_i64);
    module.set_var("WIN", 2_i64);
    module.set_var("RUN_AWAY", 3_i64);
    module.set_var("UNKNOWN", 255_i64);

    module.set_native_fn(
        "name",
        |result: i64| -> Result<String, Box<EvalAltResult>> {
            Ok(name(result).unwrap_or("").to_string())
        },
    );
    module.set_native_fn("names", || -> Result<Array, Box<EvalAltResult>> {
        Ok(util::named_id_array(RESULT_NAMES))
    });
}

fn name(result: i64) -> Option<&'static str> {
    RESULT_NAMES
        .iter()
        .find_map(|(id, label)| (*id == result).then_some(*label))
}
