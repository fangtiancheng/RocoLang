use serde::Serialize;

mod combat;
mod enum_helpers;
mod game;
mod lookup;
mod manor;
mod news;
mod pet_training;
mod profile;
mod registered;
mod role;
mod scene;
mod session;
mod spirit;
mod spirit_book;
mod system;
mod types;

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct StdlibFunctionDoc {
    pub module: String,
    pub name: String,
    pub signature: String,
    pub description: String,
    pub params: Vec<StdlibParamDoc>,
    pub returns: String,
    pub return_doc: Option<StdlibReturnDoc>,
    pub examples: Vec<String>,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct StdlibParamDoc {
    pub name: String,
    pub description: String,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct StdlibReturnDoc {
    pub type_name: String,
    pub description: String,
    pub fields: Vec<StdlibFieldDoc>,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct StdlibFieldDoc {
    pub name: String,
    pub type_name: String,
    pub description: String,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
pub struct StdlibFunctionKey {
    pub module: &'static str,
    pub name: &'static str,
}

impl StdlibFunctionKey {
    pub const fn new(module: &'static str, name: &'static str) -> Self {
        Self { module, name }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct StdlibFunctionRegistration {
    pub module: &'static str,
    pub name: &'static str,
    pub signature: &'static str,
}

impl StdlibFunctionRegistration {
    pub const fn new(module: &'static str, name: &'static str, signature: &'static str) -> Self {
        Self {
            module,
            name,
            signature,
        }
    }

    pub const fn key(self) -> StdlibFunctionKey {
        StdlibFunctionKey::new(self.module, self.name)
    }
}

macro_rules! stdlib_doc {
    (
        $module:literal,
        $name:literal,
        $signature:literal,
        $description:literal,
        params: [$($param_name:literal => $param_desc:literal),* $(,)?],
        returns: $returns:literal,
        examples: [$($example:literal),* $(,)?]
    ) => {
        StdlibFunctionDoc {
            module: $module.to_string(),
            name: $name.to_string(),
            signature: $signature.to_string(),
            description: $description.to_string(),
            params: vec![$($crate::stdlib::metadata::StdlibParamDoc {
                name: $param_name.to_string(),
                description: $param_desc.to_string(),
            }),*],
            returns: $returns.to_string(),
            return_doc: None,
            examples: vec![$($example.to_string()),*],
        }
    };
}

pub(crate) use stdlib_doc;

pub fn stdlib_function_docs() -> Vec<StdlibFunctionDoc> {
    let mut docs = detailed_stdlib_function_docs();
    for registration in registered_stdlib_function_registrations() {
        if docs
            .iter()
            .any(|doc| doc.module == registration.module && doc.name == registration.name)
        {
            continue;
        }
        docs.push(fallback_stdlib_function_doc(*registration));
    }
    enrich_return_docs(&mut docs);
    docs
}

pub fn find_stdlib_function_doc(module: &str, name: &str) -> Option<StdlibFunctionDoc> {
    stdlib_function_docs()
        .into_iter()
        .find(|doc| doc.module == module && doc.name == name)
}

pub fn documented_stdlib_function_keys() -> Vec<(String, String)> {
    stdlib_function_docs()
        .into_iter()
        .map(|doc| (doc.module, doc.name))
        .collect()
}

pub fn registered_stdlib_function_keys() -> &'static [StdlibFunctionKey] {
    registered::FUNCTION_KEYS
}

pub fn registered_stdlib_function_registrations() -> &'static [StdlibFunctionRegistration] {
    registered::FUNCTIONS
}

fn detailed_stdlib_function_docs() -> Vec<StdlibFunctionDoc> {
    let mut docs = Vec::new();
    docs.extend(system::docs());
    docs.extend(profile::docs());
    docs.extend(scene::docs());
    docs.extend(game::docs());
    docs.extend(role::docs());
    docs.extend(manor::docs());
    docs.extend(pet_training::docs());
    docs.extend(news::docs());
    docs.extend(spirit::docs());
    docs.extend(combat::docs());
    docs.extend(lookup::docs());
    docs.extend(spirit_book::docs());
    docs.extend(session::docs());
    docs.extend(enum_helpers::docs());
    docs
}

fn fallback_stdlib_function_doc(registration: StdlibFunctionRegistration) -> StdlibFunctionDoc {
    let signature = registration.signature.to_string();
    StdlibFunctionDoc {
        module: registration.module.to_string(),
        name: registration.name.to_string(),
        signature: signature.clone(),
        description: format!(
            "`{}` 模块的脚本接口。该接口已注册到 RocoLang；详细参数语义后续应补充为专门文档。",
            registration.module
        ),
        params: fallback_param_docs(&signature),
        returns: "返回值取决于具体接口；请结合脚本示例或调用结果使用。".to_string(),
        return_doc: None,
        examples: vec![format!("let result = {};", signature)],
    }
}

fn enrich_return_docs(docs: &mut [StdlibFunctionDoc]) {
    for doc in docs {
        if doc.return_doc.is_some() {
            continue;
        }

        let Some(return_type) = types::infer_return_type(&doc.signature) else {
            continue;
        };
        doc.return_doc = types::return_doc_for(&return_type);
    }
}

fn fallback_param_docs(signature: &str) -> Vec<StdlibParamDoc> {
    let Some(open) = signature.find('(') else {
        return Vec::new();
    };
    let Some(close) = signature.rfind(')') else {
        return Vec::new();
    };
    if close <= open {
        return Vec::new();
    }

    let params = signature[open + 1..close].trim();
    if params.is_empty() || params == "..." {
        return Vec::new();
    }

    params
        .split(',')
        .filter_map(|param| {
            let name = param.split(':').next()?.trim();
            if name.is_empty() {
                return None;
            }

            Some(StdlibParamDoc {
                name: name.to_string(),
                description: "参数语义请参考接口名和调用场景；详细文档待补充。".to_string(),
            })
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sorted_unique_keys(mut keys: Vec<(String, String)>) -> Vec<(String, String)> {
        keys.sort();
        keys.dedup();
        keys
    }

    #[test]
    fn registered_stdlib_functions_have_exactly_one_doc() {
        let registered = sorted_unique_keys(
            registered_stdlib_function_keys()
                .iter()
                .map(|key| (key.module.to_string(), key.name.to_string()))
                .collect(),
        );
        let documented = sorted_unique_keys(documented_stdlib_function_keys());

        let missing_docs: Vec<_> = registered
            .iter()
            .filter(|key| !documented.contains(key))
            .collect();
        let stale_docs: Vec<_> = documented
            .iter()
            .filter(|key| !registered.contains(key))
            .collect();

        assert!(
            missing_docs.is_empty() && stale_docs.is_empty(),
            "stdlib doc mismatch: missing_docs={missing_docs:?}, stale_docs={stale_docs:?}"
        );
    }

    #[test]
    fn stdlib_function_docs_do_not_contain_duplicates() {
        let docs = documented_stdlib_function_keys();
        let unique_docs = sorted_unique_keys(docs.clone());

        assert_eq!(docs.len(), unique_docs.len(), "duplicate stdlib docs found");
    }

    #[test]
    fn fallback_docs_parse_signature_params() {
        let doc = fallback_stdlib_function_doc(StdlibFunctionRegistration::new(
            "demo",
            "call",
            "demo::call(first_id: int, enabled: bool)",
        ));

        let names: Vec<_> = doc.params.iter().map(|param| param.name.as_str()).collect();
        assert_eq!(names, ["first_id", "enabled"]);
    }

    #[test]
    fn return_docs_are_inferred_from_registered_types() {
        let docs = stdlib_function_docs();
        let scene_spirits = docs
            .iter()
            .find(|doc| doc.module == "scene" && doc.name == "get_scene_spirits")
            .expect("scene::get_scene_spirits should be documented");

        let return_doc = scene_spirits
            .return_doc
            .as_ref()
            .expect("SceneSpiritInfo[] should infer return doc");
        assert_eq!(return_doc.type_name, "SceneSpiritInfo[]");
        assert!(return_doc
            .fields
            .iter()
            .any(|field| field.name == "spirit_id"));
    }

    #[test]
    fn documented_struct_returns_have_return_docs() {
        let primitive_returns = ["()", "bool", "int", "string", "map", "Array"];
        let docs = stdlib_function_docs();
        let missing: Vec<_> = docs
            .iter()
            .filter_map(|doc| {
                let return_type = types::infer_return_type(&doc.signature)?;
                let normalized = return_type.trim_end_matches("[]");
                if primitive_returns.contains(&normalized) {
                    return None;
                }
                doc.return_doc
                    .is_none()
                    .then_some(format!("{}::{} -> {}", doc.module, doc.name, return_type))
            })
            .collect();

        assert!(
            missing.is_empty(),
            "missing return docs for struct returns: {missing:?}"
        );
    }

    #[test]
    fn stdlib_docs_do_not_contain_mojibake_or_replacement_text() {
        let docs = stdlib_function_docs();
        let mut bad = Vec::new();
        for doc in docs {
            for value in [
                doc.description.as_str(),
                doc.returns.as_str(),
                doc.signature.as_str(),
            ] {
                if looks_corrupted(value) {
                    bad.push(format!("{}::{}: {}", doc.module, doc.name, value));
                }
            }
            for param in &doc.params {
                if looks_corrupted(&param.description) {
                    bad.push(format!(
                        "{}::{} param {}: {}",
                        doc.module, doc.name, param.name, param.description
                    ));
                }
            }
            if let Some(return_doc) = &doc.return_doc {
                if looks_corrupted(&return_doc.description) {
                    bad.push(format!(
                        "{}::{} return: {}",
                        doc.module, doc.name, return_doc.description
                    ));
                }
                for field in &return_doc.fields {
                    if looks_corrupted(&field.description) {
                        bad.push(format!(
                            "{}::{} field {}: {}",
                            doc.module, doc.name, field.name, field.description
                        ));
                    }
                }
            }
        }

        assert!(bad.is_empty(), "corrupted stdlib docs found: {bad:?}");
    }

    fn looks_corrupted(value: &str) -> bool {
        value.contains("????")
            || value.contains('�')
            || value.contains('鍙')
            || value.contains('杩')
    }
}
