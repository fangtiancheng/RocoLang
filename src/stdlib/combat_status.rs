use rhai::{Array, EvalAltResult, Module};

use crate::stdlib::util;

const STATUS_NAMES: &[(i64, &str)] = &[
    (1, "睡眠"),
    (2, "麻醉"),
    (3, "烧伤"),
    (4, "冰冻"),
    (5, "中毒"),
    (6, "剧毒"),
    (7, "混乱"),
    (8, "恐惧"),
    (9, "寄生"),
    (10, "诅咒"),
    (11, "迷惑"),
    (12, "噩梦"),
    (13, "束缚"),
];

pub fn register(module: &mut Module) {
    module.set_var("SLEEP", 1_i64);
    module.set_var("NUMB", 2_i64);
    module.set_var("BURN", 3_i64);
    module.set_var("FREEZE", 4_i64);
    module.set_var("POISON", 5_i64);
    module.set_var("TOXIC_POISON", 6_i64);
    module.set_var("CONFUSION", 7_i64);
    module.set_var("FEAR", 8_i64);
    module.set_var("PARASITE", 9_i64);
    module.set_var("CURSE", 10_i64);
    module.set_var("BEWILDER", 11_i64);
    module.set_var("NIGHTMARE", 12_i64);
    module.set_var("BIND", 13_i64);

    module.set_native_fn(
        "name",
        |status: i64| -> Result<String, Box<EvalAltResult>> {
            Ok(name(status).unwrap_or("").to_string())
        },
    );
    module.set_native_fn("names", || -> Result<Array, Box<EvalAltResult>> {
        Ok(util::named_id_array(STATUS_NAMES))
    });
}

fn name(status: i64) -> Option<&'static str> {
    STATUS_NAMES
        .iter()
        .find_map(|(id, label)| (*id == status).then_some(*label))
}
