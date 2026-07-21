use std::collections::{BTreeSet, HashMap};
use std::sync::OnceLock;

use rhai::{ASTNode, Engine, Expr, FnAccess, FnCallExpr, Position, Stmt};

use super::{diagnostic_at, RocoBuiltinModuleFunctionDoc, RocoScriptDiagnostic};

#[derive(Debug, Clone)]
struct Callable {
    parameter_count: usize,
    params: Vec<String>,
    signature: String,
}

type FunctionCatalog = HashMap<String, Vec<Callable>>;

#[derive(Debug)]
struct SemanticCatalog {
    builtin_imports: HashMap<&'static str, FunctionCatalog>,
    stdlib_modules: HashMap<String, FunctionCatalog>,
}

static SEMANTIC_CATALOG: OnceLock<SemanticCatalog> = OnceLock::new();

pub(super) fn analyze(ast: &rhai::AST) -> Vec<RocoScriptDiagnostic> {
    let catalog = SEMANTIC_CATALOG.get_or_init(build_semantic_catalog);
    let imports = collect_imports(ast);
    let mut diagnostics = Vec::new();
    let mut seen = BTreeSet::new();

    ast.walk(&mut |path| {
        let Some(node) = path.last() else {
            return true;
        };
        let call = match node {
            ASTNode::Stmt(Stmt::FnCall(call, position)) => Some((&**call, *position)),
            ASTNode::Expr(Expr::FnCall(call, position)) => Some((&**call, *position)),
            _ => None,
        };
        if let Some((call, position)) = call {
            analyze_qualified_call(
                call,
                position,
                catalog,
                &imports,
                &mut diagnostics,
                &mut seen,
            );
        }
        true
    });

    diagnostics
}

pub(super) fn builtin_module_function_docs() -> Vec<RocoBuiltinModuleFunctionDoc> {
    let catalog = SEMANTIC_CATALOG.get_or_init(build_semantic_catalog);
    let mut docs = catalog
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

fn analyze_qualified_call(
    call: &FnCallExpr,
    function_position: Position,
    catalog: &SemanticCatalog,
    imports: &HashMap<String, Option<&'static str>>,
    diagnostics: &mut Vec<RocoScriptDiagnostic>,
    seen: &mut BTreeSet<(usize, usize, String)>,
) {
    if call.namespace.is_empty() || call.is_operator_call() {
        return;
    }

    let module_name = call.namespace.root();
    let functions = if let Some(functions) = catalog.stdlib_modules.get(module_name) {
        Some(functions)
    } else if let Some(import_path) = imports.get(module_name) {
        match import_path {
            Some(path) => catalog.builtin_imports.get(path),
            None => return,
        }
    } else {
        push_unique_diagnostic(
            diagnostics,
            seen,
            diagnostic_at(
                call.namespace.position(),
                module_name.len(),
                format!("unknown Roco module `{module_name}`"),
            ),
        );
        return;
    };

    let Some(functions) = functions else {
        return;
    };
    let Some(overloads) = functions.get(call.name.as_str()) else {
        push_unique_diagnostic(
            diagnostics,
            seen,
            diagnostic_at(
                function_position,
                call.name.len(),
                format!("unknown function `{module_name}::{}`", call.name),
            ),
        );
        return;
    };

    let actual_count = call.args.len();
    if overloads
        .iter()
        .any(|callable| callable.parameter_count == actual_count)
    {
        return;
    }

    let expected_counts = overloads
        .iter()
        .map(|callable| callable.parameter_count)
        .collect::<BTreeSet<_>>()
        .into_iter()
        .collect::<Vec<_>>();
    let expected = expected_counts
        .iter()
        .map(|count| count.to_string())
        .collect::<Vec<_>>()
        .join(" or ");
    let expected_noun = if expected_counts.as_slice() == [1] {
        "argument"
    } else {
        "arguments"
    };
    let signatures = overloads
        .iter()
        .map(|callable| callable.signature.as_str())
        .collect::<Vec<_>>()
        .join("; ");
    let noun = if actual_count == 1 {
        "argument"
    } else {
        "arguments"
    };
    push_unique_diagnostic(
        diagnostics,
        seen,
        diagnostic_at(
            function_position,
            call.name.len(),
            format!(
                "`{module_name}::{}` expects {expected} {expected_noun}, got {actual_count} {noun}; {signatures}",
                call.name
            ),
        ),
    );
}

fn collect_imports(ast: &rhai::AST) -> HashMap<String, Option<&'static str>> {
    let mut imports = HashMap::new();
    ast.walk(&mut |path| {
        if let Some(ASTNode::Stmt(Stmt::Import(import, _))) = path.last() {
            let (path, alias) = &**import;
            let import_path = match path {
                Expr::StringConstant(path, _) => crate::builtin_sources::source_paths()
                    .iter()
                    .copied()
                    .find(|candidate| *candidate == path.as_str()),
                _ => None,
            };
            imports.insert(alias.name.to_string(), import_path);
        }
        true
    });
    imports
}

fn build_semantic_catalog() -> SemanticCatalog {
    let mut stdlib_modules: HashMap<String, FunctionCatalog> = HashMap::new();
    for doc in crate::stdlib_function_docs() {
        stdlib_modules
            .entry(doc.module.clone())
            .or_default()
            .entry(doc.name.clone())
            .or_default()
            .push(Callable {
                parameter_count: doc.params.len(),
                params: doc.params.into_iter().map(|param| param.name).collect(),
                signature: doc.signature,
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

fn push_unique_diagnostic(
    diagnostics: &mut Vec<RocoScriptDiagnostic>,
    seen: &mut BTreeSet<(usize, usize, String)>,
    diagnostic: RocoScriptDiagnostic,
) {
    let key = (
        diagnostic.start_line,
        diagnostic.start_column,
        diagnostic.message.clone(),
    );
    if seen.insert(key) {
        diagnostics.push(diagnostic);
    }
}

#[cfg(test)]
mod tests {
    use crate::language_service::{analyze_script, builtin_module_function_docs};

    #[test]
    fn unknown_stdlib_function_is_reported() {
        let diagnostics = analyze_script("combat::not_a_function();");
        assert_eq!(diagnostics.len(), 1);
        assert!(diagnostics[0]
            .message
            .contains("unknown function `combat::not_a_function`"));
    }

    #[test]
    fn stdlib_argument_count_is_checked() {
        let diagnostics = analyze_script("combat::try_use_skill();");
        assert_eq!(diagnostics.len(), 1);
        assert!(diagnostics[0]
            .message
            .contains("expects 1 argument, got 0 arguments"));
    }

    #[test]
    fn builtin_import_alias_is_checked() {
        let diagnostics = analyze_script(
            r#"
                import "roco/combat" as helper;
                helper::challenge_boss();
            "#,
        );
        assert_eq!(diagnostics.len(), 1);
        assert!(diagnostics[0]
            .message
            .contains("expects 1 argument, got 0 arguments"));
    }

    #[test]
    fn unknown_external_import_is_not_guessed() {
        assert!(analyze_script(
            r#"
                import "custom/module" as custom;
                custom::dynamic_function(1, 2, 3);
            "#,
        )
        .is_empty());
    }

    #[test]
    fn builtin_module_docs_expose_public_combat_helpers() {
        let docs = builtin_module_function_docs();
        let challenge_boss = docs
            .iter()
            .find(|doc| doc.module_path == "roco/combat" && doc.name == "challenge_boss")
            .expect("roco/combat::challenge_boss should be documented");
        assert_eq!(challenge_boss.params, ["boss_id"]);
    }
}
