use super::StdlibFunctionDoc;

pub fn docs() -> Vec<StdlibFunctionDoc> {
    vec![
        super::stdlib_doc!(
            "news",
            "query_reports",
            "news::query_reports() -> NewsTimesReportsResult",
            "查询新闻报告信息。",
            params: [],
            returns: "新闻报告信息。",
            examples: ["let reports = news::query_reports();"]
        ),
        super::stdlib_doc!(
            "news",
            "query_active_ids",
            "news::query_active_ids() -> int[]",
            "查询当前活跃活动 ID 列表。",
            params: [],
            returns: "活动 ID 数组。",
            examples: ["let ids = news::query_active_ids();"]
        ),
        super::stdlib_doc!(
            "news",
            "query_active_items",
            "news::query_active_items() -> NewsActiveItem[]",
            "查询当前活跃活动条目。",
            params: [],
            returns: "活动条目数组。",
            examples: ["let items = news::query_active_items();"]
        ),
        super::stdlib_doc!(
            "news",
            "list_active_config_items",
            "news::list_active_config_items() -> NewsActiveItem[]",
            "列出本地活动配置条目。",
            params: [],
            returns: "活动配置条目数组。",
            examples: ["let configs = news::list_active_config_items();"]
        ),
    ]
}
