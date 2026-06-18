use super::StdlibFunctionDoc;

pub fn docs() -> Vec<StdlibFunctionDoc> {
    vec![
        super::stdlib_doc!(
            "session",
            "session_get_int",
            "session::session_get_int(key: string, default_value: int) -> int",
            "读取脚本会话级整数值。",
            params: ["key" => "键名。", "default_value" => "不存在时返回的默认值。"],
            returns: "整数值。",
            examples: ["let total = session::session_get_int(\"total\", 0);"]
        ),
        super::stdlib_doc!(
            "session",
            "session_set_int",
            "session::session_set_int(key: string, value: int) -> bool",
            "写入脚本会话级整数值。",
            params: ["key" => "键名。", "value" => "要写入的整数值。"],
            returns: "写入成功返回 true。",
            examples: ["session::session_set_int(\"total\", total + 1);"]
        ),
        super::stdlib_doc!(
            "session",
            "session_get_string",
            "session::session_get_string(key: string, default_value: string) -> string",
            "读取脚本会话级字符串值。",
            params: ["key" => "键名。", "default_value" => "不存在时返回的默认值。"],
            returns: "字符串值。",
            examples: ["let name = session::session_get_string(\"name\", \"\");"]
        ),
        super::stdlib_doc!(
            "session",
            "session_set_string",
            "session::session_set_string(key: string, value: string) -> bool",
            "写入脚本会话级字符串值。",
            params: ["key" => "键名。", "value" => "要写入的字符串值。"],
            returns: "写入成功返回 true。",
            examples: ["session::session_set_string(\"name\", \"demo\");"]
        ),
        super::stdlib_doc!(
            "session",
            "session_get_bool",
            "session::session_get_bool(key: string, default_value: bool) -> bool",
            "读取脚本会话级布尔值。",
            params: ["key" => "键名。", "default_value" => "不存在时返回的默认值。"],
            returns: "布尔值。",
            examples: ["let enabled = session::session_get_bool(\"enabled\", false);"]
        ),
        super::stdlib_doc!(
            "session",
            "session_set_bool",
            "session::session_set_bool(key: string, value: bool) -> bool",
            "写入脚本会话级布尔值。",
            params: ["key" => "键名。", "value" => "要写入的布尔值。"],
            returns: "写入成功返回 true。",
            examples: ["session::session_set_bool(\"enabled\", true);"]
        ),
        super::stdlib_doc!(
            "session",
            "session_delete",
            "session::session_delete(key: string) -> bool",
            "删除脚本会话级键值。",
            params: ["key" => "键名。"],
            returns: "删除成功返回 true。",
            examples: ["session::session_delete(\"total\");"]
        ),
        super::stdlib_doc!(
            "session",
            "session_clear",
            "session::session_clear() -> bool",
            "清空当前脚本会话级存储。",
            params: [],
            returns: "清空成功返回 true。",
            examples: ["session::session_clear();"]
        ),
        super::stdlib_doc!(
            "session",
            "session_list_keys",
            "session::session_list_keys() -> map",
            "列出当前脚本会话级存储中的键及其值类型。",
            params: [],
            returns: "键名到值类型的映射。",
            examples: ["let keys = session::session_list_keys();"]
        ),
    ]
}
