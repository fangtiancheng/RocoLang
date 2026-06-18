use rhai::Module;

pub const ORDINARY: i64 = crate::types::StaticSpiritEvolutionEdge::KIND_ORDINARY;
pub const DISPLAY: i64 = crate::types::StaticSpiritEvolutionEdge::KIND_DISPLAY;

pub fn register(module: &mut Module) {
    module.set_var("ORDINARY", ORDINARY);
    module.set_var("DISPLAY", DISPLAY);
}
