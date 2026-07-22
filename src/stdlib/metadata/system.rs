use super::StdlibFunctionDetails;

pub fn docs() -> Vec<StdlibFunctionDetails> {
    vec![
        super::stdlib_doc!(
            "system",
            "log",
            return_type: "()",
            "向脚本输出面板追加一行日志。",
            params: ["message" => "要输出的文本。"],
            returns: "无返回值。",
            examples: ["system::log(\"hello\");"]
        ),
        super::stdlib_doc!(
            "system",
            "status",
            return_type: "()",
            "更新脚本运行状态栏，用于显示当前正在执行的阶段。",
            params: ["message" => "状态文本。"],
            returns: "无返回值。",
            examples: ["system::status(\"查询背包精灵...\");"]
        ),
        super::stdlib_doc!(
            "system",
            "sleep",
            return_type: "()",
            "暂停脚本指定毫秒数。",
            params: ["ms" => "等待毫秒数。"],
            returns: "无返回值。",
            examples: ["system::sleep(1000);"]
        ),
        super::stdlib_doc!(
            "system",
            "now_ms",
            return_type: "int",
            "返回当前 Unix 时间戳，单位毫秒。",
            params: [],
            returns: "当前时间戳毫秒数。",
            examples: ["let start = system::now_ms();"]
        ),
        super::stdlib_doc!(
            "system",
            "random_int",
            return_type: "int",
            "在闭区间内均匀生成一个随机整数。",
            params: [
                "min_inclusive" => "允许返回的最小整数。",
                "max_inclusive" => "允许返回的最大整数，必须不小于 min_inclusive。"
            ],
            returns: "闭区间 [min_inclusive, max_inclusive] 内的随机整数。",
            examples: ["let die = system::random_int(1, 6);"]
        ),
        super::stdlib_doc!(
            "system",
            "sleep_until_ms",
            return_type: "()",
            "暂停脚本直到指定 Unix 毫秒时间戳。",
            params: ["target_ms" => "目标 Unix 毫秒时间戳。"],
            returns: "无返回值。",
            examples: ["system::sleep_until_ms(system::now_ms() + 1000);"]
        ),
        super::stdlib_doc!(
            "system",
            "format_time",
            return_type: "string",
            "将 Unix 秒级时间戳格式化为北京时间字符串。",
            params: ["timestamp" => "Unix 秒级时间戳。"],
            returns: "格式化后的时间文本。",
            examples: ["system::log(system::format_time(1710000000));"]
        ),
        super::stdlib_doc!(
            "system",
            "assert",
            return_type: "()",
            "断言条件为真，否则中止脚本并抛出错误。",
            params: ["condition" => "需要满足的条件。", "message" => "断言失败时显示的错误信息。"],
            returns: "无返回值。",
            examples: ["system::assert(total > 0, \"total must be positive\");"]
        ),
    ]
}
