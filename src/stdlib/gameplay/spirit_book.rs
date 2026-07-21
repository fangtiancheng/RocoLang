use std::sync::{Arc, Mutex};

use rhai::Module;

use crate::stdlib::util::{lock_stdlib, to_rhai_error};
use crate::stdlib::RocoStdLib;

pub fn register<T: RocoStdLib + 'static>(module: &mut Module, stdlib: Arc<Mutex<T>>) {
    {
        let stdlib = stdlib.clone();
        module.set_native_fn("get_my_states", move || {
            let mut lib = lock_stdlib(&stdlib)?;
            lib.get_my_spirit_book_states().map_err(to_rhai_error)
        });
    }
    {
        let stdlib = stdlib.clone();
        module.set_native_fn("get_role_states", move |uin: i64| {
            let mut lib = lock_stdlib(&stdlib)?;
            lib.get_role_spirit_book_states(uin).map_err(to_rhai_error)
        });
    }
    {
        let stdlib = stdlib.clone();
        module.set_native_fn("get_my_spirit_state", move |spirit_id: i64| {
            let mut lib = lock_stdlib(&stdlib)?;
            lib.get_my_spirit_book_spirit_state(spirit_id)
                .map_err(to_rhai_error)
        });
    }
    {
        let stdlib = stdlib.clone();
        module.set_native_fn("get_role_spirit_state", move |uin: i64, spirit_id: i64| {
            let mut lib = lock_stdlib(&stdlib)?;
            lib.get_role_spirit_book_spirit_state(uin, spirit_id)
                .map_err(to_rhai_error)
        });
    }
}
