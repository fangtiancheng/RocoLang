use super::StdlibFunctionDoc;

pub fn docs() -> Vec<StdlibFunctionDoc> {
    vec![
        super::stdlib_doc!(
            "profile",
            "get_user_info",
            "profile::get_user_info() -> UserInfo",
            "查询当前登录用户信息。",
            params: [],
            returns: "当前用户信息。",
            examples: ["let user = profile::get_user_info();"]
        ),
        super::stdlib_doc!(
            "profile",
            "is_in_combat",
            "profile::is_in_combat() -> bool",
            "判断当前是否处于战斗中。",
            params: [],
            returns: "处于战斗中返回 true。",
            examples: ["if profile::is_in_combat() { combat::wait_next_action(30000); }"]
        ),
        super::stdlib_doc!(
            "profile",
            "query_server_time",
            "profile::query_server_time() -> ServerTimeInfo",
            "查询服务器时间，返回结构化日期、时间和时间戳。",
            params: [],
            returns: "服务器当前时间。",
            examples: ["let server_time = profile::query_server_time(); system::log(server_time.stamp);"]
        ),
        super::stdlib_doc!(
            "profile",
            "try_query_server_time",
            "profile::try_query_server_time() -> ServerTimeResult",
            "尝试查询服务器时间，失败时返回结构化结果而不是中止脚本。",
            params: [],
            returns: "服务器时间查询结果。",
            examples: ["let result = profile::try_query_server_time(); if result.ok { system::log(result.result.stamp); }"]
        ),
    ]
}
