use std::sync::{Arc, Mutex};

use rhai::{Array, Module};

use crate::stdlib::util::{
    lock_stdlib, parse_i64_array_at, register_stdlib_fn_0, register_stdlib_fn_1,
    register_stdlib_fn_2, to_array, to_rhai_error_in_context,
};
use crate::stdlib::RocoStdLib;

pub fn register<T: RocoStdLib + 'static>(module: &mut Module, stdlib: Arc<Mutex<T>>) {
    register_stdlib_fn_0!(
        module,
        stdlib,
        "query_initial_state",
        friend_query_initial_state
    );
    register_stdlib_fn_1!(
        module,
        stdlib,
        "send_application",
        friend_send_application,
        friend_uin: i64
    );
    register_stdlib_fn_2!(
        module,
        stdlib,
        "handle_application",
        friend_handle_application,
        sender_uin: i64,
        accept: bool
    );
    register_stdlib_fn_1!(
        module,
        stdlib,
        "delete",
        friend_delete,
        friend_uin: i64
    );
    register_stdlib_fn_2!(
        module,
        stdlib,
        "send_chat",
        friend_send_chat,
        friend_uin: i64,
        message: &str
    );
    {
        let stdlib = stdlib.clone();
        module.set_native_fn("query_online", move |context: rhai::NativeCallContext| {
            let mut lib = lock_stdlib(&stdlib)?;
            lib.friend_query_online()
                .map(|friends| to_array(&friends))
                .map_err(|error| to_rhai_error_in_context(error, &context))
        });
    }
    {
        let stdlib = stdlib.clone();
        module.set_native_fn(
            "query_nicknames",
            move |context: rhai::NativeCallContext, friend_uins: Array| {
                let friend_uins =
                    parse_i64_array_at("friend_uins[]", friend_uins, context.call_position())?;
                let mut lib = lock_stdlib(&stdlib)?;
                lib.friend_query_nicknames(friend_uins)
                    .map(|friends| to_array(&friends))
                    .map_err(|error| to_rhai_error_in_context(error, &context))
            },
        );
    }
}
