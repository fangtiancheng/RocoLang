use super::{StdlibFunctionDetails, StdlibParamDoc};

pub fn docs() -> Vec<StdlibFunctionDetails> {
    let mut docs = Vec::new();

    docs.extend(enum_docs(
        "combat_result",
        "战斗结果枚举辅助。",
        "result",
        "战斗结果 ID。",
        "combat_result::WIN",
    ));
    docs.extend(enum_docs(
        "combat_status",
        "战斗异常状态枚举辅助。",
        "status",
        "异常状态 ID。",
        "combat_status::SLEEP",
    ));
    docs.extend(enum_docs(
        "personality",
        "精灵性格枚举辅助。",
        "personality",
        "性格 ID。",
        "personality::ADAMANT",
    ));
    docs.extend(enum_docs(
        "weather",
        "战斗天气/环境枚举辅助。",
        "weather",
        "天气或环境 ID。",
        "weather::RAIN",
    ));

    docs
}

fn enum_docs(
    module: &'static str,
    module_description: &'static str,
    param_name: &'static str,
    param_description: &'static str,
    example_const: &'static str,
) -> Vec<StdlibFunctionDetails> {
    vec![
        StdlibFunctionDetails {
            key: super::StdlibFunctionKey::new(module, "name"),
            return_type: "string",
            description: format!(
                "将 {module_description} 的数字 ID 转换为中文名称；未知 ID 返回空字符串。"
            ),
            params: vec![StdlibParamDoc {
                name: param_name.to_string(),
                description: param_description.to_string(),
            }],
            returns: "对应的中文名称，未知时为空字符串。".to_string(),
            examples: vec![format!("let name = {module}::name({example_const});")],
        },
        StdlibFunctionDetails {
            key: super::StdlibFunctionKey::new(module, "names"),
            return_type: "Array",
            description: format!("列出 {module_description} 的所有已知枚举项。"),
            params: Vec::new(),
            returns: "枚举项数组；不同模块的元素结构跟运行时模块保持一致。".to_string(),
            examples: vec![format!("let names = {module}::names();")],
        },
    ]
}
