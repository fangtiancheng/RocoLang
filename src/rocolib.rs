use rhai::module_resolvers::StaticModuleResolver;
use rhai::{Engine, Module, Scope};

pub fn register(engine: &mut Engine) {
    let mut resolver = StaticModuleResolver::new();
    for path in crate::builtin_sources::source_paths() {
        let script = crate::builtin_sources::source_by_path(path)
            .unwrap_or_else(|| panic!("missing built-in RocoLang source: {path}"));
        register_script_module(engine, &mut resolver, path, script);
    }
    engine.set_module_resolver(resolver);
}

fn register_script_module(
    engine: &Engine,
    resolver: &mut StaticModuleResolver,
    path: &str,
    script: &str,
) {
    let mut ast = engine
        .compile_with_scope(&Scope::new(), script)
        .unwrap_or_else(|err| panic!("failed to compile built-in RocoLang module {path}: {err}"));
    ast.set_source(path);
    let module = Module::eval_ast_as_new(Scope::new(), &ast, engine)
        .unwrap_or_else(|err| panic!("failed to evaluate built-in RocoLang module {path}: {err}"));
    resolver.insert(path, module);
}
