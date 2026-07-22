use super::StdlibFunctionDetails;

pub fn docs() -> Vec<StdlibFunctionDetails> {
    vec![
        super::stdlib_doc!(
            "session",
            "session_get_int",
            return_type: "int",
            "读取脚本会话级整数值。",
            params: ["key" => "键名。", "default_value" => "不存在时返回的默认值。"],
            returns: "整数值。",
            examples: ["let total = session::session_get_int(\"total\", 0);"]
        ),
        super::stdlib_doc!(
            "session",
            "session_set_int",
            return_type: "bool",
            "写入脚本会话级整数值。",
            params: ["key" => "键名。", "value" => "要写入的整数值。"],
            returns: "写入成功返回 true。",
            examples: ["session::session_set_int(\"total\", total + 1);"]
        ),
        super::stdlib_doc!(
            "session",
            "session_get_string",
            return_type: "string",
            "读取脚本会话级字符串值。",
            params: ["key" => "键名。", "default_value" => "不存在时返回的默认值。"],
            returns: "字符串值。",
            examples: ["let name = session::session_get_string(\"name\", \"\");"]
        ),
        super::stdlib_doc!(
            "session",
            "session_set_string",
            return_type: "bool",
            "写入脚本会话级字符串值。",
            params: ["key" => "键名。", "value" => "要写入的字符串值。"],
            returns: "写入成功返回 true。",
            examples: ["session::session_set_string(\"name\", \"demo\");"]
        ),
        super::stdlib_doc!(
            "session",
            "session_get_bool",
            return_type: "bool",
            "读取脚本会话级布尔值。",
            params: ["key" => "键名。", "default_value" => "不存在时返回的默认值。"],
            returns: "布尔值。",
            examples: ["let enabled = session::session_get_bool(\"enabled\", false);"]
        ),
        super::stdlib_doc!(
            "session",
            "session_set_bool",
            return_type: "bool",
            "写入脚本会话级布尔值。",
            params: ["key" => "键名。", "value" => "要写入的布尔值。"],
            returns: "写入成功返回 true。",
            examples: ["session::session_set_bool(\"enabled\", true);"]
        ),
        super::stdlib_doc!(
            "session",
            "session_delete",
            return_type: "bool",
            "删除脚本会话级键值。",
            params: ["key" => "键名。"],
            returns: "删除成功返回 true。",
            examples: ["session::session_delete(\"total\");"]
        ),
        super::stdlib_doc!(
            "session",
            "session_clear",
            return_type: "bool",
            "清空当前脚本会话级存储。",
            params: [],
            returns: "清空成功返回 true。",
            examples: ["session::session_clear();"]
        ),
        super::stdlib_doc!(
            "session",
            "session_list_keys",
            return_type: "map",
            "列出当前脚本会话级存储中的键及其值类型。",
            params: [],
            returns: "键名到值类型的映射。",
            examples: ["let keys = session::session_list_keys();"]
        ),
    ]
}
