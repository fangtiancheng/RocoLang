use std::collections::HashMap;
use std::sync::OnceLock;

use rhai::{Engine, FnAccess};

use crate::language_service::RocoBuiltinModuleFunctionDoc;

#[derive(Debug, Clone)]
pub(super) struct Callable {
    pub(super) parameter_count: usize,
    pub(super) params: Vec<String>,
    pub(super) signature: String,
}

pub(super) type FunctionCatalog = HashMap<String, Vec<Callable>>;

#[derive(Debug)]
pub(super) struct SemanticCatalog {
    pub(super) builtin_imports: HashMap<&'static str, FunctionCatalog>,
    pub(super) stdlib_modules: HashMap<String, FunctionCatalog>,
}

static CATALOG: OnceLock<SemanticCatalog> = OnceLock::new();

pub(super) fn get() -> &'static SemanticCatalog {
    CATALOG.get_or_init(build)
}

pub(super) fn builtin_module_function_docs() -> Vec<RocoBuiltinModuleFunctionDoc> {
    let mut docs = get()
        .builtin_imports
        .iter()
        .flat_map(|(module_path, functions)| {
            functions.iter().flat_map(move |(name, overloads)| {
                overloads
                    .iter()
                    .map(move |callable| RocoBuiltinModuleFunctionDoc {
                        module_path: (*module_path).to_string(),
                        name: name.clone(),
                        signature: callable.signature.clone(),
                        params: callable.params.clone(),
                    })
            })
        })
        .collect::<Vec<_>>();
    docs.sort_by(|left, right| {
        (&left.module_path, &left.name, left.params.len()).cmp(&(
            &right.module_path,
            &right.name,
            right.params.len(),
        ))
    });
    docs
}

fn build() -> SemanticCatalog {
    let mut stdlib_modules: HashMap<String, FunctionCatalog> = HashMap::new();
    for registration in crate::registered_stdlib_function_registrations() {
        let params = registration.parameter_names();
        stdlib_modules
            .entry(registration.module.to_string())
            .or_default()
            .entry(registration.name.to_string())
            .or_default()
            .push(Callable {
                parameter_count: params.len(),
                params,
                signature: registration.signature.to_string(),
            });
    }

    let mut builtin_imports = HashMap::new();
    let mut engine = Engine::new();
    engine.set_max_expr_depths(0, 0);
    for path in crate::builtin_sources::source_paths() {
        let source = crate::builtin_sources::source_by_path(path)
            .unwrap_or_else(|| panic!("builtin source path `{path}` is not registered"));
        let ast = engine
            .compile(source)
            .unwrap_or_else(|error| panic!("builtin source `{path}` does not compile: {error}"));
        let mut functions: FunctionCatalog = HashMap::new();
        for function in ast.iter_functions() {
            if function.access != FnAccess::Public {
                continue;
            }
            let signature = format!("{}({})", function.name, function.params.join(", "));
            functions
                .entry(function.name.to_string())
                .or_default()
                .push(Callable {
                    parameter_count: function.params.len(),
                    params: function
                        .params
                        .iter()
                        .map(|param| (*param).to_string())
                        .collect(),
                    signature,
                });
        }
        builtin_imports.insert(*path, functions);
    }

    SemanticCatalog {
        builtin_imports,
        stdlib_modules,
    }
}
