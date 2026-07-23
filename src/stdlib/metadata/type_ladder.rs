use super::StdlibFunctionDetails;

pub fn docs() -> Vec<StdlibFunctionDetails> {
    vec![
        super::stdlib_doc!(
            "type_ladder",
            "query_info",
            return_type: "TypeLadderInfo",
            "查询系别天梯当前赛季信息。",
            params: [],
            returns: "返回系别限制、参战宠物、段位和战斗记录。",
            examples: ["let info = type_ladder::query_info();"]
        ),
        super::stdlib_doc!(
            "type_ladder",
            "query_rank",
            return_type: "TypeLadderRankInfo",
            "查询系别天梯排行榜。",
            params: [],
            returns: "返回排行榜信息。",
            examples: ["let rank = type_ladder::query_rank();"]
        ),
        super::stdlib_doc!(
            "type_ladder",
            "recover_spirits",
            return_type: "bool",
            "恢复系别天梯相关宠物状态。",
            params: [],
            returns: "返回服务端是否接受恢复操作。",
            examples: ["let ok = type_ladder::recover_spirits();"]
        ),
        super::stdlib_doc!(
            "type_ladder",
            "try_recover_spirits",
            return_type: "ActionResult",
            "尝试恢复系别天梯相关宠物状态，并将业务失败转换为结构化结果。",
            params: [],
            returns: "返回 ok、错误码、消息和结构化错误信息。",
            examples: ["let result = type_ladder::try_recover_spirits();"]
        ),
    ]
}
