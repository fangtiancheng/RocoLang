use std::sync::{Arc, Mutex};

use rhai::Module;

use crate::stdlib::util::{lock_stdlib, to_rhai_error};
use crate::stdlib::RocoStdLib;

pub fn register<T: RocoStdLib + 'static>(module: &mut Module, stdlib: Arc<Mutex<T>>) {
    {
        let stdlib = stdlib.clone();
        module.set_native_fn("query_reports", move || {
            let mut lib = lock_stdlib(&stdlib)?;
            lib.news_times_query_reports().map_err(to_rhai_error)
        });
    }
}
