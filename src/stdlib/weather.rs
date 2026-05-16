use rhai::{Array, EvalAltResult, Module};

use crate::stdlib::util;

const WEATHER_NAMES: &[(i64, &str)] = &[
    (1, "暴晒天气"),
    (2, "阴雨天气"),
    (3, "冰雹天气"),
    (4, "雷暴天气"),
    (5, "暗黑城环境"),
    (6, "疾风天气"),
    (7, "梦境环境"),
    (8, "瘴气环境"),
    (9, "沃土环境"),
    (10, "龙阵环境"),
    (11, "化境环境"),
    (12, "圣光天气"),
    (13, "血月天气"),
    (14, "迷宫环境"),
    (15, "铁壁环境"),
    (16, "芬芳环境"),
    (17, "迷雾天气"),
    (18, "陨石天气"),
    (19, "海市蜃楼环境"),
    (20, "星云天气"),
    (21, "锁定"),
    (22, "曙光天气"),
    (23, "风林火山环境"),
    (24, "乐园环境"),
    (25, "未知环境25"),
    (100, "无天气环境"),
];

pub fn register(module: &mut Module) {
    module.set_var("SCORCHING_SUN", 1_i64);
    module.set_var("RAIN", 2_i64);
    module.set_var("HAIL", 3_i64);
    module.set_var("THUNDERSTORM", 4_i64);
    module.set_var("DARK_CASTLE", 5_i64);
    module.set_var("GALE", 6_i64);
    module.set_var("DREAMLAND", 7_i64);
    module.set_var("MIASMA", 8_i64);
    module.set_var("FERTILE_SOIL", 9_i64);
    module.set_var("DRAGON_FORMATION", 10_i64);
    module.set_var("MARTIAL_REALM", 11_i64);
    module.set_var("HOLY_LIGHT", 12_i64);
    module.set_var("BLOOD_MOON", 13_i64);
    module.set_var("LABYRINTH", 14_i64);
    module.set_var("IRON_WALL", 15_i64);
    module.set_var("FRAGRANCE", 16_i64);
    module.set_var("FOG", 17_i64);
    module.set_var("METEOR", 18_i64);
    module.set_var("MIRAGE", 19_i64);
    module.set_var("NEBULA", 20_i64);
    module.set_var("LOCKED", 21_i64);
    module.set_var("DAWN", 22_i64);
    module.set_var("WIND_FOREST_FIRE_MOUNTAIN", 23_i64);
    module.set_var("PARADISE", 24_i64);
    module.set_var("UNKNOWN_25", 25_i64);
    module.set_var("NONE", 100_i64);

    module.set_native_fn(
        "name",
        |weather: i64| -> Result<String, Box<EvalAltResult>> {
            Ok(name(weather).unwrap_or("").to_string())
        },
    );
    module.set_native_fn("names", || -> Result<Array, Box<EvalAltResult>> {
        Ok(util::named_id_array(WEATHER_NAMES))
    });
}

fn name(weather: i64) -> Option<&'static str> {
    WEATHER_NAMES
        .iter()
        .find_map(|(id, label)| (*id == weather).then_some(*label))
}
