use rhai::{Array, Dynamic, EvalAltResult, Module};

pub const PERSONALITY_NAMES: [&str; 25] = [
    "孤僻", "固执", "调皮", "勇敢", "大胆", "淘气", "无虑", "悠闲", "保守", "稳重", "马虎", "冷静",
    "沉着", "温顺", "慎重", "狂妄", "胆小", "急躁", "开朗", "天真", "坦率", "害羞", "认真", "实干",
    "浮躁",
];

pub fn register(module: &mut Module) {
    module.set_var("LONELY", 1_i64);
    module.set_var("ADAMANT", 2_i64);
    module.set_var("NAUGHTY", 3_i64);
    module.set_var("BRAVE", 4_i64);
    module.set_var("BOLD", 5_i64);
    module.set_var("IMPISH", 6_i64);
    module.set_var("LAX", 7_i64);
    module.set_var("RELAXED", 8_i64);
    module.set_var("MODEST", 9_i64);
    module.set_var("MILD", 10_i64);
    module.set_var("RASH", 11_i64);
    module.set_var("QUIET", 12_i64);
    module.set_var("CALM", 13_i64);
    module.set_var("GENTLE", 14_i64);
    module.set_var("CAREFUL", 15_i64);
    module.set_var("SASSY", 16_i64);
    module.set_var("TIMID", 17_i64);
    module.set_var("HASTY", 18_i64);
    module.set_var("JOLLY", 19_i64);
    module.set_var("NAIVE", 20_i64);
    module.set_var("HARDY", 21_i64);
    module.set_var("BASHFUL", 22_i64);
    module.set_var("SERIOUS", 23_i64);
    module.set_var("DOCILE", 24_i64);
    module.set_var("QUIRKY", 25_i64);

    module.set_native_fn(
        "name",
        |personality: i64| -> Result<String, Box<EvalAltResult>> {
            Ok(personality_name(personality).unwrap_or("").to_string())
        },
    );
    module.set_native_fn("names", || -> Result<Array, Box<EvalAltResult>> {
        Ok(PERSONALITY_NAMES
            .iter()
            .map(|name| Dynamic::from((*name).to_string()))
            .collect())
    });
}

fn personality_name(personality: i64) -> Option<&'static str> {
    if personality <= 0 {
        return None;
    }
    PERSONALITY_NAMES
        .get(usize::try_from(personality - 1).ok()?)
        .copied()
}
