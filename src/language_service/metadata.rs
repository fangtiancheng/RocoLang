use std::collections::BTreeMap;

use rhai::Engine;
use serde::{Deserialize, Serialize};

use crate::StdlibReturnDoc;

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RocoLanguageFunctionDoc {
    pub name: String,
    pub signature: String,
    pub params: Vec<String>,
    pub parameter_types: Vec<String>,
    pub return_type: String,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RocoLanguageKeywordDoc {
    pub name: String,
    pub kind: String,
    pub type_name: Option<String>,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RocoLanguageModuleConstantDoc {
    pub module: String,
    pub name: String,
    pub type_name: String,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RocoLanguageMetadata {
    pub functions: Vec<RocoLanguageFunctionDoc>,
    pub keywords: Vec<RocoLanguageKeywordDoc>,
    pub module_constants: Vec<RocoLanguageModuleConstantDoc>,
    pub types: Vec<StdlibReturnDoc>,
}

pub(super) fn rhai_language_metadata() -> RocoLanguageMetadata {
    let raw_metadata = Engine::new()
        .definitions()
        .json()
        .expect("Rhai built-in metadata should serialize");
    let metadata: RhaiModuleMetadata =
        serde_json::from_str(&raw_metadata).expect("Rhai built-in metadata should deserialize");
    RocoLanguageMetadata {
        functions: language_function_docs(metadata.functions),
        keywords: keyword_docs(),
        module_constants: module_constant_docs(),
        types: crate::stdlib_type_docs(),
    }
}

fn module_constant_docs() -> Vec<RocoLanguageModuleConstantDoc> {
    let mut docs = crate::stdlib::registered_value_modules()
        .into_iter()
        .flat_map(|(module_name, module)| {
            module
                .iter_var()
                .map(move |(name, value)| RocoLanguageModuleConstantDoc {
                    module: module_name.to_string(),
                    name: name.to_string(),
                    type_name: normalize_type_name(value.type_name()),
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();
    docs.sort_by(|left, right| (&left.module, &left.name).cmp(&(&right.module, &right.name)));
    docs
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct RhaiModuleMetadata {
    #[serde(default)]
    functions: Vec<RhaiFunctionMetadata>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct RhaiFunctionMetadata {
    name: String,
    #[serde(default)]
    params: Vec<RhaiFunctionParam>,
    #[serde(default)]
    return_type: String,
    signature: String,
}

#[derive(Deserialize)]
struct RhaiFunctionParam {
    name: Option<String>,
    #[serde(rename = "type")]
    type_name: Option<String>,
}

fn language_function_docs(functions: Vec<RhaiFunctionMetadata>) -> Vec<RocoLanguageFunctionDoc> {
    let mut docs = BTreeMap::new();
    for function in functions.into_iter().chain(special_function_docs()) {
        if !is_identifier(&function.name) {
            continue;
        }
        let doc = RocoLanguageFunctionDoc {
            name: function.name,
            signature: function.signature,
            params: function
                .params
                .iter()
                .enumerate()
                .map(|(index, param)| {
                    param
                        .name
                        .clone()
                        .unwrap_or_else(|| format!("arg{}", index + 1))
                })
                .collect(),
            parameter_types: function
                .params
                .iter()
                .map(|param| normalize_type_name(param.type_name.as_deref().unwrap_or("dynamic")))
                .collect(),
            return_type: normalize_type_name(&function.return_type),
        };
        docs.insert((doc.name.clone(), doc.signature.clone()), doc);
    }
    docs.into_values().collect()
}

fn special_function_docs() -> impl Iterator<Item = RhaiFunctionMetadata> {
    [
        special_function("print", &["data: dynamic"], "()"),
        special_function("debug", &["data: dynamic"], "()"),
        special_function("type_of", &["data: dynamic"], "string"),
        special_function("Fn", &["fn_name: string"], "FnPtr"),
        special_function("call", &["fn_ptr: FnPtr", "...args: dynamic"], "dynamic"),
        special_function("curry", &["fn_ptr: FnPtr", "...args: dynamic"], "FnPtr"),
        special_function("is_def_fn", &["fn_name: string", "num_params: int"], "bool"),
        special_function("is_def_var", &["var_name: string"], "bool"),
        special_function("is_shared", &["value: dynamic"], "bool"),
        special_function("eval", &["script: string"], "dynamic"),
    ]
    .into_iter()
}

fn special_function(name: &str, params: &[&str], return_type: &str) -> RhaiFunctionMetadata {
    let params = params
        .iter()
        .map(|param| {
            let (name, type_name) = param.split_once(':').expect("special function parameter");
            RhaiFunctionParam {
                name: Some(name.trim().to_string()),
                type_name: Some(type_name.trim().to_string()),
            }
        })
        .collect::<Vec<_>>();
    RhaiFunctionMetadata {
        name: name.to_string(),
        signature: format!(
            "{name}({}) -> {return_type}",
            params
                .iter()
                .map(|param| format!(
                    "{}: {}",
                    param.name.as_deref().unwrap_or("arg"),
                    param.type_name.as_deref().unwrap_or("dynamic")
                ))
                .collect::<Vec<_>>()
                .join(", ")
        ),
        params,
        return_type: return_type.to_string(),
    }
}

fn normalize_type_name(type_name: &str) -> String {
    let type_name = script_value_type(type_name.trim());
    match type_name {
        "INT" | "i64" => "int".to_string(),
        "FLOAT" | "f64" => "float".to_string(),
        "str" | "String" | "ImmutableString" => "string".to_string(),
        "Array" => "dynamic[]".to_string(),
        "Map" => "map".to_string(),
        "Dynamic" | "?" | "" => "dynamic".to_string(),
        other => other.to_string(),
    }
}

fn script_value_type(type_name: &str) -> &str {
    let without_reference = type_name
        .strip_prefix("&mut ")
        .or_else(|| type_name.strip_prefix('&'))
        .unwrap_or(type_name)
        .trim();
    let Some(result_body) = without_reference
        .strip_prefix("Result<")
        .and_then(|body| body.strip_suffix('>'))
    else {
        return without_reference;
    };

    first_generic_argument(result_body).trim()
}

fn first_generic_argument(arguments: &str) -> &str {
    let mut depth = 0usize;
    for (index, character) in arguments.char_indices() {
        match character {
            '<' => depth += 1,
            '>' => depth = depth.saturating_sub(1),
            ',' if depth == 0 => return &arguments[..index],
            _ => {}
        }
    }
    arguments
}

fn is_identifier(value: &str) -> bool {
    let mut chars = value.chars();
    chars
        .next()
        .is_some_and(|first| first.is_ascii_alphabetic() || first == '_')
        && chars.all(|char| char.is_ascii_alphanumeric() || char == '_')
}

fn keyword_docs() -> Vec<RocoLanguageKeywordDoc> {
    const ACTIVE: &[&str] = &[
        "let", "const", "if", "else", "switch", "do", "while", "until", "loop", "for", "in",
        "continue", "break", "fn", "private", "return", "throw", "try", "catch", "import",
        "export", "as", "global", "this",
    ];
    let mut docs = ACTIVE
        .iter()
        .map(|name| RocoLanguageKeywordDoc {
            name: (*name).to_string(),
            kind: "keyword".to_string(),
            type_name: None,
        })
        .collect::<Vec<_>>();
    docs.extend(
        [("true", "bool"), ("false", "bool")].map(|(name, type_name)| RocoLanguageKeywordDoc {
            name: name.to_string(),
            kind: "constant".to_string(),
            type_name: Some(type_name.to_string()),
        }),
    );
    docs
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn language_metadata_contains_constants_and_len_overloads() {
        let metadata = rhai_language_metadata();
        assert!(metadata
            .keywords
            .iter()
            .any(|doc| doc.name == "true" && doc.type_name.as_deref() == Some("bool")));
        assert!(metadata
            .functions
            .iter()
            .any(|doc| doc.name == "len" && doc.return_type == "int"));
        assert!(metadata.functions.iter().any(|doc| {
            doc.name == "push"
                && doc
                    .parameter_types
                    .first()
                    .is_some_and(|ty| ty == "dynamic[]")
        }));
        assert!(metadata
            .functions
            .iter()
            .any(|doc| { doc.parameter_types.first().is_some_and(|ty| ty == "map") }));
        assert!(metadata.functions.iter().any(|doc| doc.name == "print"));
        assert!(metadata.module_constants.iter().any(|doc| {
            doc.module == "evolution_edge_kind" && doc.name == "ORDINARY" && doc.type_name == "int"
        }));
    }
}
