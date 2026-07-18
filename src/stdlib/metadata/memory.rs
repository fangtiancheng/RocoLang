use super::StdlibFunctionDoc;

pub fn docs() -> Vec<StdlibFunctionDoc> {
    vec![
        super::stdlib_doc!(
            "memory", "today", "memory::today() -> string",
            "返回持久化每日记忆当前使用的服务器日期。",
            params: [], returns: "YYYY-MM-DD 格式的服务器日期。",
            examples: ["print(memory::today());"]
        ),
        super::stdlib_doc!(
            "memory", "daily_get_int", "memory::daily_get_int(key: string, default_value: int) -> int",
            "读取当前账号和服务器日期下的整数值。",
            params: ["key" => "键名。", "default_value" => "键不存在时的默认值。"],
            returns: "已保存的整数或默认值。",
            examples: ["let count = memory::daily_get_int(\"task.count\", 0);"]
        ),
        super::stdlib_doc!(
            "memory", "daily_set_int", "memory::daily_set_int(key: string, value: int) -> bool",
            "写入当前账号和服务器日期下的整数值。",
            params: ["key" => "键名。", "value" => "整数值。"], returns: "写入成功返回 true。",
            examples: ["memory::daily_set_int(\"task.count\", 1);"]
        ),
        super::stdlib_doc!(
            "memory", "daily_increment_int", "memory::daily_increment_int(key: string, delta: int) -> int",
            "以可跨进程合并的增量事件更新每日整数值。",
            params: ["key" => "键名。", "delta" => "增量。"], returns: "更新后的整数值。",
            examples: ["let count = memory::daily_increment_int(\"task.count\", 1);"]
        ),
        super::stdlib_doc!(
            "memory", "daily_get_string", "memory::daily_get_string(key: string, default_value: string) -> string",
            "读取当前账号和服务器日期下的字符串值。",
            params: ["key" => "键名。", "default_value" => "键不存在时的默认值。"],
            returns: "已保存的字符串或默认值。",
            examples: ["let phase = memory::daily_get_string(\"task.phase\", \"new\");"]
        ),
        super::stdlib_doc!(
            "memory", "daily_set_string", "memory::daily_set_string(key: string, value: string) -> bool",
            "写入当前账号和服务器日期下的字符串值。",
            params: ["key" => "键名。", "value" => "字符串值。"], returns: "写入成功返回 true。",
            examples: ["memory::daily_set_string(\"task.phase\", \"done\");"]
        ),
        super::stdlib_doc!(
            "memory", "daily_get_bool", "memory::daily_get_bool(key: string, default_value: bool) -> bool",
            "读取当前账号和服务器日期下的布尔值。",
            params: ["key" => "键名。", "default_value" => "键不存在时的默认值。"],
            returns: "已保存的布尔值或默认值。",
            examples: ["let done = memory::daily_get_bool(\"task.done\", false);"]
        ),
        super::stdlib_doc!(
            "memory", "daily_set_bool", "memory::daily_set_bool(key: string, value: bool) -> bool",
            "写入当前账号和服务器日期下的布尔值。",
            params: ["key" => "键名。", "value" => "布尔值。"], returns: "写入成功返回 true。",
            examples: ["memory::daily_set_bool(\"task.done\", true);"]
        ),
        super::stdlib_doc!(
            "memory", "daily_delete", "memory::daily_delete(key: string) -> bool",
            "删除当前账号和服务器日期下的自定义键。",
            params: ["key" => "键名。"], returns: "键存在并被删除时返回 true。",
            examples: ["memory::daily_delete(\"task.phase\");"]
        ),
        super::stdlib_doc!(
            "memory", "daily_clear", "memory::daily_clear() -> bool",
            "清空当前账号和服务器日期下的自定义键，不影响自动战斗统计。",
            params: [], returns: "清空成功返回 true。", examples: ["memory::daily_clear();"]
        ),
        super::stdlib_doc!(
            "memory", "daily_list_keys", "memory::daily_list_keys() -> map",
            "列出当前账号和服务器日期下的自定义键及值类型。",
            params: [], returns: "键名到值类型的映射。", examples: ["print(memory::daily_list_keys());"]
        ),
        super::stdlib_doc!(
            "memory", "daily_battle_observed_started", "memory::daily_battle_observed_started() -> int",
            "返回本地今日观察到的服务器已接受战斗开始次数。",
            params: [], returns: "观察到的开始次数。", examples: ["print(memory::daily_battle_observed_started());"]
        ),
        super::stdlib_doc!(
            "memory", "daily_battle_observed_completed", "memory::daily_battle_observed_completed() -> int",
            "返回本地今日观察到的完整战斗结算次数。",
            params: [], returns: "观察到的结算次数。", examples: ["print(memory::daily_battle_observed_completed());"]
        ),
        super::stdlib_doc!(
            "memory", "daily_battle_tracking_since", "memory::daily_battle_tracking_since() -> int",
            "返回今日首个本地战斗观察事件的 Unix 毫秒时间戳。",
            params: [], returns: "Unix 毫秒时间戳；尚未观察到事件时为 0。", examples: ["print(memory::daily_battle_tracking_since());"]
        ),
        super::stdlib_doc!(
            "memory", "daily_battle_limit_reached", "memory::daily_battle_limit_reached() -> bool",
            "返回服务器是否已明确拒绝今日继续战斗。",
            params: [], returns: "已达到服务器每日战斗上限时返回 true。", examples: ["if memory::daily_battle_limit_reached() { print(\"limit reached\"); }"]
        ),
        super::stdlib_doc!(
            "memory", "daily_battle_limit", "memory::daily_battle_limit() -> int",
            "返回服务器明确触发的每日战斗上限。",
            params: [], returns: "每日战斗上限；未触发时为 0。", examples: ["print(memory::daily_battle_limit());"]
        ),
        super::stdlib_doc!(
            "memory", "daily_battle_limit_return_code", "memory::daily_battle_limit_return_code() -> int",
            "返回服务器每日战斗上限拒绝码。",
            params: [], returns: "拒绝码；未触发时为 0。", examples: ["print(memory::daily_battle_limit_return_code());"]
        ),
        super::stdlib_doc!(
            "memory", "daily_battle_limit_message", "memory::daily_battle_limit_message() -> string",
            "返回服务器每日战斗上限拒绝消息。",
            params: [], returns: "拒绝消息；未触发时为空字符串。", examples: ["print(memory::daily_battle_limit_message());"]
        ),
    ]
}
