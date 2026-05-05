use rhai::module_resolvers::StaticModuleResolver;
use rhai::{Engine, Module, Scope};

const COMBAT: &str = include_str!("../rocolib/combat.rhai");
const SPIRIT: &str = include_str!("../rocolib/spirit.rhai");

pub fn register(engine: &mut Engine) {
    let mut resolver = StaticModuleResolver::new();
    register_script_module(engine, &mut resolver, "roco/combat", COMBAT);
    register_script_module(engine, &mut resolver, "roco/spirit", SPIRIT);
    engine.set_module_resolver(resolver);
}

fn register_script_module(
    engine: &Engine,
    resolver: &mut StaticModuleResolver,
    path: &str,
    script: &str,
) {
    let ast = engine
        .compile_with_scope(&Scope::new(), script)
        .unwrap_or_else(|err| panic!("failed to compile built-in RocoLang module {path}: {err}"));
    let module = Module::eval_ast_as_new(Scope::new(), &ast, engine)
        .unwrap_or_else(|err| panic!("failed to evaluate built-in RocoLang module {path}: {err}"));
    resolver.insert(path, module);
}
