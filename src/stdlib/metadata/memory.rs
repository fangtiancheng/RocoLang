use super::StdlibFunctionDoc;

pub fn docs() -> Vec<StdlibFunctionDoc> {
    vec![
        super::stdlib_doc!(
            "memory",
            "today",
            "memory::today() -> string",
            "返回当前账号的服务器日期（YYYY-MM-DD）；绝不使用本机日期降级。",
            params: [],
            returns: "服务器日期字符串。",
            examples: ["print(memory::today());"]
        ),
        super::stdlib_doc!(
            "memory",
            "daily_get_int",
            "memory::daily_get_int(key: string, default_value: int) -> int",
            "读取按账号和服务器日期隔离的持久化整数。",
            params: ["key" => "带功能前缀的键名。", "default_value" => "今日尚无该键时的默认值。"],
            returns: "今日保存的整数或默认值。",
            examples: ["let count = memory::daily_get_int(\"farm.count\", 0);"]
        ),
        super::stdlib_doc!(
            "memory",
            "daily_set_int",
            "memory::daily_set_int(key: string, value: int) -> bool",
            "写入按账号和服务器日期隔离的持久化整数。",
            params: ["key" => "带功能前缀的键名。", "value" => "要保存的整数。"],
            returns: "落盘成功返回 true。",
            examples: ["memory::daily_set_int(\"farm.count\", 3);"]
        ),
        super::stdlib_doc!(
            "memory",
            "daily_increment_int",
            "memory::daily_increment_int(key: string, delta: int) -> int",
            "原子追加一个日整数增量；多程序实例同时运行也不会互相覆盖。",
            params: ["key" => "带功能前缀的键名。", "delta" => "正数或负数增量。"],
            returns: "合并所有实例后当前可见的值。",
            examples: ["let count = memory::daily_increment_int(\"farm.count\", 1);"]
        ),
        super::stdlib_doc!(
            "memory",
            "daily_get_string",
            "memory::daily_get_string(key: string, default_value: string) -> string",
            "读取按账号和服务器日期隔离的持久化字符串。",
            params: ["key" => "键名。", "default_value" => "今日尚无该键时的默认值。"],
            returns: "今日保存的字符串或默认值。",
            examples: ["let value = memory::daily_get_string(\"task.phase\", \"new\");"]
        ),
        super::stdlib_doc!(
            "memory",
            "daily_set_string",
            "memory::daily_set_string(key: string, value: string) -> bool",
            "写入按账号和服务器日期隔离的持久化字符串。",
            params: ["key" => "键名。", "value" => "要保存的字符串。"],
            returns: "落盘成功返回 true。",
            examples: ["memory::daily_set_string(\"task.phase\", \"done\");"]
        ),
        super::stdlib_doc!(
            "memory",
            "daily_get_bool",
            "memory::daily_get_bool(key: string, default_value: bool) -> bool",
            "读取按账号和服务器日期隔离的持久化布尔值。",
            params: ["key" => "键名。", "default_value" => "今日尚无该键时的默认值。"],
            returns: "今日保存的布尔值或默认值。",
            examples: ["let claimed = memory::daily_get_bool(\"reward.claimed\", false);"]
        ),
        super::stdlib_doc!(
            "memory",
            "daily_set_bool",
            "memory::daily_set_bool(key: string, value: bool) -> bool",
            "写入按账号和服务器日期隔离的持久化布尔值。",
            params: ["key" => "键名。", "value" => "要保存的布尔值。"],
            returns: "落盘成功返回 true。",
            examples: ["memory::daily_set_bool(\"reward.claimed\", true);"]
        ),
        super::stdlib_doc!(
            "memory",
            "daily_delete",
            "memory::daily_delete(key: string) -> bool",
            "删除今日持久化键，不影响其他日期。",
            params: ["key" => "键名。"],
            returns: "键原先存在且删除事件已落盘时返回 true。",
            examples: ["memory::daily_delete(\"task.phase\");"]
        ),
        super::stdlib_doc!(
            "memory",
            "daily_clear",
            "memory::daily_clear() -> bool",
            "清空当前账号今日的脚本自定义键；不会清除自动战斗统计。",
            params: [],
            returns: "清空事件落盘成功返回 true。",
            examples: ["memory::daily_clear();"]
        ),
        super::stdlib_doc!(
            "memory",
            "daily_list_keys",
            "memory::daily_list_keys() -> map",
            "列出当前账号今日的脚本自定义键及类型。",
            params: [],
            returns: "键名到类型（integer/string/bool）的映射。",
            examples: ["print(memory::daily_list_keys());"]
        ),
        super::stdlib_doc!(
            "memory",
            "daily_battle_observed_started",
            "memory::daily_battle_observed_started() -> int",
            "返回本持久化层今日观察到的服务器已接受战斗开始次数；这是可靠下界，不冒充服务器隐藏总数。",
            params: [],
            returns: "本地观察到的已接受开始次数。",
            examples: ["print(memory::daily_battle_observed_started());"]
        ),
        super::stdlib_doc!(
            "memory",
            "daily_battle_observed_completed",
            "memory::daily_battle_observed_completed() -> int",
            "返回今日收到胜、负或逃跑结算的战斗次数。",
            params: [],
            returns: "本地观察到的完成次数。",
            examples: ["print(memory::daily_battle_observed_completed());"]
        ),
        super::stdlib_doc!(
            "memory",
            "daily_battle_tracking_since",
            "memory::daily_battle_tracking_since() -> int",
            "返回今日首个本地战斗观察事件的 Unix 毫秒时间戳；0 表示今日尚未观察到。",
            params: [],
            returns: "Unix 毫秒时间戳或 0。",
            examples: ["print(memory::daily_battle_tracking_since());"]
        ),
        super::stdlib_doc!(
            "memory",
            "daily_battle_limit_reached",
            "memory::daily_battle_limit_reached() -> bool",
            "返回服务器今日是否已用拒绝码 41 明确报告战斗疲劳上限。",
            params: [],
            returns: "已收到每日上限拒绝时返回 true。",
            examples: ["if memory::daily_battle_limit_reached() { print(\"今日已封顶\"); }"]
        ),
        super::stdlib_doc!(
            "memory",
            "daily_battle_limit",
            "memory::daily_battle_limit() -> int",
            "返回已确认的每日战斗上限；尚未收到上限拒绝时返回 0。",
            params: [],
            returns: "当前规则下为 2000，未知时为 0。",
            examples: ["print(memory::daily_battle_limit());"]
        ),
        super::stdlib_doc!(
            "memory",
            "daily_battle_limit_return_code",
            "memory::daily_battle_limit_return_code() -> int",
            "返回今日上限拒绝的服务器返回码；尚未收到时返回 0。",
            params: [],
            returns: "服务器返回码。",
            examples: ["print(memory::daily_battle_limit_return_code());"]
        ),
        super::stdlib_doc!(
            "memory",
            "daily_battle_limit_message",
            "memory::daily_battle_limit_message() -> string",
            "返回今日上限拒绝的服务器原始消息；尚未收到时为空字符串。",
            params: [],
            returns: "服务器原始消息。",
            examples: ["print(memory::daily_battle_limit_message());"]
        ),
    ]
}
