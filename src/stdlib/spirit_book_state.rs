use rhai::Module;

pub const UNKNOWN: i64 = 0;
pub const FOUND: i64 = 1;
pub const CAUGHT: i64 = 2;
pub const RELEASED: i64 = 3;

pub fn register(module: &mut Module) {
    module.set_var("UNKNOWN", UNKNOWN);
    module.set_var("FOUND", FOUND);
    module.set_var("CAUGHT", CAUGHT);
    module.set_var("RELEASED", RELEASED);
}
