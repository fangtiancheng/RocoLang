use std::sync::{Arc, Mutex};

use rhai::Module;

use crate::stdlib::util::{register_stdlib_fn_0, to_rhai_error};
use crate::stdlib::RocoStdLib;

pub fn register<T: RocoStdLib + 'static>(module: &mut Module, stdlib: Arc<Mutex<T>>) {
    register_stdlib_fn_0!(module, stdlib, "query_info", type_ladder_query_info);
    register_stdlib_fn_0!(module, stdlib, "query_rank", type_ladder_query_rank);
    register_stdlib_fn_0!(
        module,
        stdlib,
        "recover_spirits",
        type_ladder_recover_spirits
    );
    {
        let stdlib = stdlib.clone();
        module.set_native_fn("try_recover_spirits", move || {
            let mut lib = crate::stdlib::util::lock_stdlib(&stdlib)?;
            lib.type_ladder_try_recover_spirits().map_err(to_rhai_error)
        });
    }
}
