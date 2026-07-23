use super::StdlibFunctionDetails;

pub fn docs() -> Vec<StdlibFunctionDetails> {
    vec![super::stdlib_doc!(
        "news_times",
        "query_reports",
        return_type: "NewsTimesReportsResult",
        "查询新闻次数和可领取状态。",
        params: [],
        returns: "返回各新闻活动的次数信息。",
        examples: ["let reports = news_times::query_reports();"]
    )]
}
