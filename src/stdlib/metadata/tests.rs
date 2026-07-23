use super::docs::fallback_stdlib_function_doc;
use super::*;
use std::collections::BTreeSet;

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
fn modules_with_detailed_docs_are_fully_documented() {
    let details = super::docs::detailed_stdlib_function_details_by_key();
    let detailed_modules = details
        .keys()
        .map(|key| key.module)
        .collect::<BTreeSet<_>>();
    let missing = registered_stdlib_function_registrations()
        .iter()
        .filter(|registration| detailed_modules.contains(registration.module))
        .filter(|registration| !details.contains_key(&registration.key()))
        .map(|registration| format!("{}::{}", registration.module, registration.name))
        .collect::<Vec<_>>();

    assert!(
        missing.is_empty(),
        "modules with detailed docs may not fall back to generic docs: {missing:?}"
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
    let registration =
        StdlibFunctionRegistration::new("demo", "call", "demo::call(first_id: int, enabled: bool)");
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
fn pet_egg_functions_expose_result_struct_docs() {
    let docs = stdlib_function_docs();
    let expected = [
        ("query_info", "PetEggInfo"),
        ("vip_speed_up", "PetEggSpeedUpResult"),
        ("begin", "PetEggBeginResult"),
        ("cancel", "PetEggCancelResult"),
        ("preview", "PetEggPreviewResult"),
    ];

    for (name, return_type) in expected {
        let doc = docs
            .iter()
            .find(|doc| doc.module == "pet_egg" && doc.name == name)
            .unwrap_or_else(|| panic!("pet_egg::{name} should be documented"));
        let return_doc = doc
            .return_doc
            .as_ref()
            .unwrap_or_else(|| panic!("pet_egg::{name} should expose return docs"));
        assert_eq!(return_doc.type_name, return_type);
        assert!(!return_doc.fields.is_empty());
    }
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
    value.contains("????") || value.contains('�') || value.contains('鍙') || value.contains('杩')
}
