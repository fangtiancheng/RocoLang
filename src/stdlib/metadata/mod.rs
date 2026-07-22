use std::collections::BTreeMap;

use serde::Serialize;

mod combat;
mod enum_helpers;
mod game;
mod home;
mod lookup;
mod manor;
mod memory;
mod news;
mod pet_training;
mod profile;
mod registered;
mod remote_state;
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

#[derive(Debug, Clone)]
struct StdlibFunctionDetails {
    key: StdlibFunctionKey,
    return_type: &'static str,
    description: String,
    params: Vec<StdlibParamDoc>,
    returns: String,
    examples: Vec<String>,
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
pub(crate) struct StdlibFunctionKey {
    pub module: &'static str,
    pub name: &'static str,
}

impl StdlibFunctionKey {
    pub(crate) const fn new(module: &'static str, name: &'static str) -> Self {
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

    const fn key(self) -> StdlibFunctionKey {
        StdlibFunctionKey::new(self.module, self.name)
    }

    pub fn parameter_names(self) -> Vec<String> {
        let Some(open) = self.signature.find('(') else {
            return Vec::new();
        };
        let Some(close) = self.signature.rfind(')') else {
            return Vec::new();
        };
        if close <= open {
            return Vec::new();
        }

        let params = self.signature[open + 1..close].trim();
        if params.is_empty() || params == "..." {
            return Vec::new();
        }

        params
            .split(',')
            .filter_map(|param| {
                let name = param.split(':').next()?.trim();
                (!name.is_empty()).then(|| name.to_string())
            })
            .collect()
    }
}

macro_rules! stdlib_doc {
    (
        $module:literal,
        $name:literal,
        return_type: $return_type:literal,
        $description:literal,
        params: [$($param_name:literal => $param_desc:literal),* $(,)?],
        returns: $returns:literal,
        examples: [$($example:literal),* $(,)?]
    ) => {
        StdlibFunctionDetails {
            key: $crate::stdlib::metadata::StdlibFunctionKey::new($module, $name),
            return_type: $return_type,
            description: $description.to_string(),
            params: vec![$($crate::stdlib::metadata::StdlibParamDoc {
                name: $param_name.to_string(),
                description: $param_desc.to_string(),
            }),*],
            returns: $returns.to_string(),
            examples: vec![$($example.to_string()),*],
        }
    };
}

pub(crate) use stdlib_doc;

pub fn stdlib_function_docs() -> Vec<StdlibFunctionDoc> {
    let mut details = detailed_stdlib_function_details_by_key();
    let mut docs = Vec::with_capacity(registered_stdlib_function_registrations().len());
    for registration in registered_stdlib_function_registrations() {
        docs.push(match details.remove(&registration.key()) {
            Some(details) => detailed_stdlib_function_doc(*registration, details),
            None => fallback_stdlib_function_doc(*registration),
        });
    }
    assert!(
        details.is_empty(),
        "stdlib details reference unregistered functions: {:?}",
        details.keys().collect::<Vec<_>>()
    );
    enrich_return_docs(&mut docs);
    docs
}

pub fn stdlib_type_docs() -> Vec<StdlibReturnDoc> {
    types::reachable_type_docs(&stdlib_function_docs())
}

pub fn find_stdlib_function_doc(module: &str, name: &str) -> Option<StdlibFunctionDoc> {
    stdlib_function_docs()
        .into_iter()
        .find(|doc| doc.module == module && doc.name == name)
}

pub fn registered_stdlib_function_registrations() -> &'static [StdlibFunctionRegistration] {
    registered::FUNCTIONS
}

fn detailed_stdlib_function_details() -> Vec<StdlibFunctionDetails> {
    let mut details = Vec::new();
    details.extend(system::docs());
    details.extend(profile::docs());
    details.extend(scene::docs());
    details.extend(remote_state::docs());
    details.extend(game::docs());
    details.extend(role::docs());
    details.extend(home::docs());
    details.extend(manor::docs());
    details.extend(memory::docs());
    details.extend(pet_training::docs());
    details.extend(news::docs());
    details.extend(spirit::docs());
    details.extend(combat::docs());
    details.extend(lookup::docs());
    details.extend(spirit_book::docs());
    details.extend(session::docs());
    details.extend(enum_helpers::docs());
    details
}

fn detailed_stdlib_function_details_by_key() -> BTreeMap<StdlibFunctionKey, StdlibFunctionDetails> {
    let mut by_key = BTreeMap::new();
    for details in detailed_stdlib_function_details() {
        let key = details.key;
        assert!(
            by_key.insert(key, details).is_none(),
            "duplicate stdlib details for {}::{}",
            key.module,
            key.name
        );
    }
    by_key
}

fn detailed_stdlib_function_doc(
    registration: StdlibFunctionRegistration,
    details: StdlibFunctionDetails,
) -> StdlibFunctionDoc {
    let expected_params = registration.parameter_names();
    let documented_params = details
        .params
        .iter()
        .map(|param| param.name.clone())
        .collect::<Vec<_>>();
    assert_eq!(
        documented_params, expected_params,
        "stdlib parameter docs do not match the registration for {}::{}",
        registration.module, registration.name
    );
    StdlibFunctionDoc {
        module: registration.module.to_string(),
        name: registration.name.to_string(),
        signature: format!("{} -> {}", registration.signature, details.return_type),
        description: details.description,
        params: details.params,
        returns: details.returns,
        return_doc: None,
        examples: details.examples,
    }
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
        params: registration
            .parameter_names()
            .into_iter()
            .map(|name| StdlibParamDoc {
                name,
                description: "参数语义请参考接口名和调用场景；详细文档待补充。".to_string(),
            })
            .collect(),
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
            registered_stdlib_function_registrations()
                .iter()
                .map(|registration| {
                    (
                        registration.module.to_string(),
                        registration.name.to_string(),
                    )
                })
                .collect(),
        );
        let documented = sorted_unique_keys(
            stdlib_function_docs()
                .into_iter()
                .map(|doc| (doc.module, doc.name))
                .collect(),
        );

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
        let docs = stdlib_function_docs()
            .into_iter()
            .map(|doc| (doc.module, doc.name))
            .collect::<Vec<_>>();
        let unique_docs = sorted_unique_keys(docs.clone());

        assert_eq!(docs.len(), unique_docs.len(), "duplicate stdlib docs found");
    }

    #[test]
    fn fallback_docs_parse_signature_params() {
        let registration = StdlibFunctionRegistration::new(
            "demo",
            "call",
            "demo::call(first_id: int, enabled: bool)",
        );
        let doc = fallback_stdlib_function_doc(registration);

        let names: Vec<_> = doc.params.iter().map(|param| param.name.as_str()).collect();
        assert_eq!(names, ["first_id", "enabled"]);
    }

    #[test]
    fn registration_parameter_names_handle_empty_and_variadic_signatures() {
        assert_eq!(
            StdlibFunctionRegistration::new("demo", "empty", "demo::empty()").parameter_names(),
            Vec::<String>::new()
        );
        assert_eq!(
            StdlibFunctionRegistration::new("demo", "variadic", "demo::variadic(...)")
                .parameter_names(),
            Vec::<String>::new()
        );
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
    fn type_docs_include_nested_types_without_direct_function_returns() {
        let docs = stdlib_type_docs();
        assert!(docs
            .iter()
            .any(|doc| doc.type_name == "StaticSpiritEvolutionEdge"));
    }

    #[test]
    fn reachable_struct_fields_have_type_docs() {
        let docs = stdlib_type_docs();
        let known = docs
            .iter()
            .map(|doc| doc.type_name.as_str())
            .collect::<std::collections::HashSet<_>>();
        let primitives = [
            "()", "bool", "int", "float", "char", "string", "dynamic", "map", "Map", "blob",
        ];
        let missing = docs
            .iter()
            .flat_map(|doc| &doc.fields)
            .filter_map(|field| {
                let type_name = field
                    .type_name
                    .split('|')
                    .next()
                    .unwrap_or_default()
                    .trim()
                    .trim_end_matches('?')
                    .trim_end_matches("[]");
                (!type_name.is_empty()
                    && !primitives.contains(&type_name)
                    && !known.contains(type_name))
                .then_some(type_name)
            })
            .collect::<std::collections::BTreeSet<_>>();

        assert!(
            missing.is_empty(),
            "missing reachable type docs: {missing:?}"
        );
    }

    #[test]
    fn documented_struct_returns_have_return_docs() {
        let primitive_returns = ["()", "bool", "int", "string", "map", "Array", "blob"];
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
