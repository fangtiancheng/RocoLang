use std::sync::{Arc, Mutex};

use rhai::{Array, Dynamic, Module};

use crate::stdlib::util::{lock_stdlib, to_array, to_rhai_error};
use crate::stdlib::RocoStdLib;

pub fn register<T: RocoStdLib + 'static>(module: &mut Module, stdlib: Arc<Mutex<T>>) {
    {
        let stdlib = stdlib.clone();
        module.set_native_fn("query_reports", move || {
            let mut lib = lock_stdlib(&stdlib)?;
            lib.news_query_reports().map_err(to_rhai_error)
        });
    }
    {
        let stdlib = stdlib.clone();
        module.set_native_fn("query_active_ids", move || {
            let mut lib = lock_stdlib(&stdlib)?;
            lib.news_query_active_ids()
                .map(|ids| ids.into_iter().map(Dynamic::from).collect::<Array>())
                .map_err(to_rhai_error)
        });
    }
    {
        let stdlib = stdlib.clone();
        module.set_native_fn("query_active_items", move || {
            let mut lib = lock_stdlib(&stdlib)?;
            lib.news_query_active_items()
                .map(|items| to_array(&items))
                .map_err(to_rhai_error)
        });
    }
    {
        let stdlib = stdlib.clone();
        module.set_native_fn("list_active_config_items", move || {
            let mut lib = lock_stdlib(&stdlib)?;
            lib.news_list_active_config_items()
                .map(|items| to_array(&items))
                .map_err(to_rhai_error)
        });
    }
}
