use super::{stdlib_doc, StdlibFunctionDetails};

pub(super) fn docs() -> Vec<StdlibFunctionDetails> {
    vec![
        stdlib_doc!(
            "adventure",
            "query_status",
            return_type: "AdventureStatus",
            "查询冒险系统当前状态。",
            params: [],
            returns: "返回当前关卡、道具数量、自动挑战和每日奖励状态。",
            examples: ["let status = adventure::query_status();"]
        ),
        stdlib_doc!(
            "adventure",
            "begin",
            return_type: "()",
            "请求挑战指定冒险关卡；战斗需要由脚本另行发起。",
            params: ["point" => "关卡编号。"],
            returns: "服务器接受请求时正常返回，否则抛出结构化错误。",
            examples: ["adventure::begin(1);"]
        ),
        stdlib_doc!(
            "adventure",
            "claim_reward",
            return_type: "AdventureRewards",
            "领取指定冒险关卡的胜利奖励。",
            params: ["point" => "关卡编号。"],
            returns: "返回本次领取的奖励列表。",
            examples: ["let rewards = adventure::claim_reward(1);"]
        ),
        stdlib_doc!(
            "adventure",
            "begin_auto",
            return_type: "()",
            "开始自动挑战指定冒险关卡。",
            params: ["point" => "关卡编号。", "count" => "自动挑战次数。"],
            returns: "服务器接受请求时正常返回，否则抛出结构化错误。",
            examples: ["adventure::begin_auto(1, 10);"]
        ),
        stdlib_doc!(
            "adventure",
            "end_auto",
            return_type: "()",
            "停止指定关卡的自动挑战。",
            params: ["point" => "关卡编号。"],
            returns: "服务器接受请求时正常返回，否则抛出结构化错误。",
            examples: ["adventure::end_auto(1);"]
        ),
        stdlib_doc!(
            "adventure",
            "claim_daily",
            return_type: "AdventureRewards",
            "领取每日冒险道具。",
            params: [],
            returns: "返回本次领取的奖励列表。",
            examples: ["let rewards = adventure::claim_daily();"]
        ),
        stdlib_doc!(
            "adventure",
            "end_auto_vip",
            return_type: "()",
            "使用 VIP 能力立即完成指定关卡的自动挑战。",
            params: ["point" => "关卡编号。"],
            returns: "服务器接受请求时正常返回，否则抛出结构化错误。",
            examples: ["adventure::end_auto_vip(1);"]
        ),
    ]
}
