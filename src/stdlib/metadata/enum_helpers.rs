use super::StdlibFunctionDetails;

pub fn docs() -> Vec<StdlibFunctionDetails> {
    vec![
        super::stdlib_doc!(
            "combat_result",
            "name",
            return_type: "string",
            "将战斗结果 ID 转换为中文名称；未知 ID 返回空字符串。",
            params: ["result" => "战斗结果 ID。"],
            returns: "对应的中文名称，未知时为空字符串。",
            examples: ["let name = combat_result::name(combat_result::WIN);"]
        ),
        super::stdlib_doc!(
            "combat_result",
            "names",
            return_type: "Array",
            "列出所有已知战斗结果枚举项。",
            params: [],
            returns: "战斗结果枚举项数组。",
            examples: ["let names = combat_result::names();"]
        ),
        super::stdlib_doc!(
            "combat_status",
            "name",
            return_type: "string",
            "将异常状态 ID 转换为中文名称；未知 ID 返回空字符串。",
            params: ["status" => "异常状态 ID。"],
            returns: "对应的异常状态名称，未知时为空字符串。",
            examples: ["let name = combat_status::name(combat_status::SLEEP);"]
        ),
        super::stdlib_doc!(
            "combat_status",
            "names",
            return_type: "Array",
            "列出所有已知异常状态枚举项。",
            params: [],
            returns: "异常状态枚举项数组。",
            examples: ["let names = combat_status::names();"]
        ),
        super::stdlib_doc!(
            "personality",
            "name",
            return_type: "string",
            "将宠物性格 ID 转换为中文名称；未知 ID 返回空字符串。",
            params: ["personality" => "性格 ID。"],
            returns: "对应的宠物性格名称，未知时为空字符串。",
            examples: ["let name = personality::name(personality::ADAMANT);"]
        ),
        super::stdlib_doc!(
            "personality",
            "names",
            return_type: "Array",
            "列出所有已知宠物性格枚举项。",
            params: [],
            returns: "宠物性格枚举项数组。",
            examples: ["let names = personality::names();"]
        ),
        super::stdlib_doc!(
            "weather",
            "name",
            return_type: "string",
            "将天气或环境 ID 转换为中文名称；未知 ID 返回空字符串。",
            params: ["weather" => "天气或环境 ID。"],
            returns: "对应的天气或环境名称，未知时为空字符串。",
            examples: ["let name = weather::name(weather::RAIN);"]
        ),
        super::stdlib_doc!(
            "weather",
            "names",
            return_type: "Array",
            "列出所有已知天气和环境枚举项。",
            params: [],
            returns: "天气和环境枚举项数组。",
            examples: ["let names = weather::names();"]
        ),
    ]
}
