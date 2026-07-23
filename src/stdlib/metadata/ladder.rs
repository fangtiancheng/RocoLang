use super::StdlibFunctionDetails;

pub fn docs() -> Vec<StdlibFunctionDetails> {
    vec![
        super::stdlib_doc!(
            "ladder",
            "query_info",
            return_type: "LadderInfo",
            "查询普通天梯当前赛季和阵容信息。",
            params: [],
            returns: "返回胜场、积分、参战宠物、任务和战斗记录。",
            examples: ["let info = ladder::query_info();"]
        ),
        super::stdlib_doc!(
            "ladder",
            "query_rank",
            return_type: "LadderRankInfo",
            "查询普通天梯排行榜。",
            params: [],
            returns: "返回排行榜玩家、段位和排名变化。",
            examples: ["let rank = ladder::query_rank();"]
        ),
        super::stdlib_doc!(
            "ladder",
            "recover_spirits",
            return_type: "bool",
            "恢复普通天梯相关宠物状态。",
            params: [],
            returns: "返回服务端是否接受恢复操作。",
            examples: ["let ok = ladder::recover_spirits();"]
        ),
        super::stdlib_doc!(
            "ladder",
            "try_recover_spirits",
            return_type: "ActionResult",
            "尝试恢复普通天梯相关宠物状态，并将业务失败转换为结构化结果。",
            params: [],
            returns: "返回 ok、错误码、消息和结构化错误信息。",
            examples: ["let result = ladder::try_recover_spirits();"]
        ),
    ]
}
