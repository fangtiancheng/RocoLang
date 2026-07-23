use std::collections::BTreeMap;

use super::*;

pub fn stdlib_function_docs() -> Vec<StdlibFunctionDoc> {
    let mut details = detailed_stdlib_function_details_by_key();
    let mut docs = Vec::with_capacity(registered_stdlib_function_registrations().len());
    for registration in registered_stdlib_function_registrations() {
        docs.push(match details.remove(&registration.key()) {
            Some(details) => detailed_stdlib_function_doc(*registration, details),
            None => inferred_stdlib_function_doc(*registration),
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
    details.extend(adventure::docs());
    details.extend(alchemy_furnace::docs());
    details.extend(aquarius::docs());
    details.extend(aries::docs());
    details.extend(dark_city::docs());
    details.extend(diamond_tear::docs());
    details.extend(system::docs());
    details.extend(profile::docs());
    details.extend(scene::docs());
    details.extend(remote_state::docs());
    details.extend(game::docs());
    details.extend(role::docs());
    details.extend(home::docs());
    details.extend(ice_crystal::docs());
    details.extend(incubative_machine::docs());
    details.extend(jump_machine::docs());
    details.extend(ladder::docs());
    details.extend(friend::docs());
    details.extend(four_seasons::docs());
    details.extend(manor::docs());
    details.extend(memory::docs());
    details.extend(multi_evolution::docs());
    details.extend(mountain_sea::docs());
    details.extend(mystery_fusion::docs());
    details.extend(pet_egg::docs());
    details.extend(pet_training::docs());
    details.extend(play_guide::docs());
    details.extend(news::docs());
    details.extend(news_times::docs());
    details.extend(spirit::docs());
    details.extend(combat::docs());
    details.extend(lookup::docs());
    details.extend(spirit_book::docs());
    details.extend(star_tower::docs());
    details.extend(session::docs());
    details.extend(summon::docs());
    details.extend(task::docs());
    details.extend(three_starters::docs());
    details.extend(treasure_realm::docs());
    details.extend(type_ladder::docs());
    details.extend(unicorn::docs());
    details.extend(sentinel_intelligence::docs());
    details.extend(enum_helpers::docs());
    details
}

pub(super) fn detailed_stdlib_function_details_by_key(
) -> BTreeMap<StdlibFunctionKey, StdlibFunctionDetails> {
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
    // A few adapters intentionally reshape a Rust return value before exposing it
    // to Rhai (for example, fixed arrays become script arrays). Keep those explicit
    // script-facing types; direct registrations are checked against the generated type.
    let return_type = match generated_stdlib_return_type(registration.module, registration.name) {
        Some(generated) if generated == details.return_type => generated.to_string(),
        _ => details.return_type.to_string(),
    };
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
        signature: format!("{} -> {}", registration.signature, return_type),
        description: details.description,
        params: details.params,
        returns: details.returns,
        return_doc: None,
        examples: details.examples,
    }
}

pub(in crate::stdlib::metadata) fn inferred_stdlib_function_doc(
    registration: StdlibFunctionRegistration,
) -> StdlibFunctionDoc {
    let call_signature = registration.signature.to_string();
    let generated_return_type =
        generated_stdlib_return_type(registration.module, registration.name)
            .filter(|return_type| types::has_complete_return_doc(return_type));
    let signature = generated_return_type
        .map(|return_type| format!("{} -> {}", registration.signature, return_type))
        .unwrap_or_else(|| call_signature.clone());
    StdlibFunctionDoc {
        module: registration.module.to_string(),
        name: registration.name.to_string(),
        signature: signature.clone(),
        description: semantic::function_description(registration),
        params: registration
            .parameter_names()
            .into_iter()
            .map(|name| StdlibParamDoc {
                description: semantic::parameter_description(&name),
                name,
            })
            .collect(),
        returns: semantic::return_description(registration.name, generated_return_type),
        return_doc: None,
        examples: vec![format!("let result = {};", call_signature)],
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
