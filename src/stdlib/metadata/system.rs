use super::StdlibFunctionDoc;

pub fn docs() -> Vec<StdlibFunctionDoc> {
    vec![
        super::stdlib_doc!(
            "system",
            "log",
            "system::log(message: string) -> ()",
            "向脚本输出面板追加一行日志。",
            params: ["message" => "要输出的文本。"],
            returns: "无返回值。",
            examples: ["system::log(\"hello\");"]
        ),
        super::stdlib_doc!(
            "system",
            "status",
            "system::status(message: string) -> ()",
            "更新脚本运行状态栏，用于显示当前正在执行的阶段。",
            params: ["message" => "状态文本。"],
            returns: "无返回值。",
            examples: ["system::status(\"查询背包精灵...\");"]
        ),
        super::stdlib_doc!(
            "system",
            "sleep",
            "system::sleep(ms: int) -> ()",
            "暂停脚本指定毫秒数。",
            params: ["ms" => "等待毫秒数。"],
            returns: "无返回值。",
            examples: ["system::sleep(1000);"]
        ),
        super::stdlib_doc!(
            "system",
            "now_ms",
            "system::now_ms() -> int",
            "返回当前 Unix 时间戳，单位毫秒。",
            params: [],
            returns: "当前时间戳毫秒数。",
            examples: ["let start = system::now_ms();"]
        ),
        super::stdlib_doc!(
            "system",
            "sleep_until_ms",
            "system::sleep_until_ms(target_ms: int) -> ()",
            "暂停脚本直到指定 Unix 毫秒时间戳。",
            params: ["target_ms" => "目标 Unix 毫秒时间戳。"],
            returns: "无返回值。",
            examples: ["system::sleep_until_ms(system::now_ms() + 1000);"]
        ),
        super::stdlib_doc!(
            "system",
            "format_time",
            "system::format_time(timestamp: int) -> string",
            "将 Unix 秒级时间戳格式化为北京时间字符串。",
            params: ["timestamp" => "Unix 秒级时间戳。"],
            returns: "格式化后的时间文本。",
            examples: ["system::log(system::format_time(1710000000));"]
        ),
        super::stdlib_doc!(
            "system",
            "assert",
            "system::assert(condition: bool, message: string) -> ()",
            "断言条件为真，否则中止脚本并抛出错误。",
            params: ["condition" => "需要满足的条件。", "message" => "断言失败时显示的错误信息。"],
            returns: "无返回值。",
            examples: ["system::assert(total > 0, \"total must be positive\");"]
        ),
    ]
}
