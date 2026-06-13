use std::sync::{Arc, Mutex};

use rhai::{Array, Dynamic, EvalAltResult, NativeCallContext, Position};

use crate::error::{RocoError, RocoGeneralError};

pub fn to_rhai_error(err: RocoError) -> Box<EvalAltResult> {
    EvalAltResult::ErrorRuntime(err.to_string().into(), rhai::Position::NONE).into()
}

pub fn to_rhai_error_at(err: RocoError, position: Position) -> Box<EvalAltResult> {
    EvalAltResult::ErrorRuntime(err.to_string().into(), position).into()
}

pub fn to_rhai_error_in_context(
    err: RocoError,
    context: &NativeCallContext<'_>,
) -> Box<EvalAltResult> {
    to_rhai_error_at(err, context.call_position())
}

pub fn lock_stdlib<T>(
    stdlib: &Arc<Mutex<T>>,
) -> Result<std::sync::MutexGuard<'_, T>, Box<EvalAltResult>> {
    stdlib.lock().map_err(|error| {
        to_rhai_error(RocoError::Other(RocoGeneralError::LockPoisoned {
            message: error.to_string(),
        }))
    })
}

pub fn to_array<T: Clone + Send + Sync + 'static>(items: &[T]) -> rhai::Array {
    items.iter().cloned().map(Dynamic::from).collect()
}

pub fn named_id_array(items: &[(i64, &str)]) -> Array {
    items
        .iter()
        .map(|(id, name)| {
            let mut item = rhai::Map::new();
            item.insert("id".into(), Dynamic::from(*id));
            item.insert("name".into(), Dynamic::from((*name).to_string()));
            Dynamic::from_map(item)
        })
        .collect()
}

macro_rules! register_stdlib_fn_0 {
    ($module:expr, $stdlib:expr, $name:literal, $method:ident) => {{
        let stdlib = $stdlib.clone();
        $module.set_native_fn($name, move |context: rhai::NativeCallContext| {
            let mut lib = $crate::stdlib::util::lock_stdlib(&stdlib)?;
            lib.$method()
                .map_err(|error| $crate::stdlib::util::to_rhai_error_in_context(error, &context))
        });
    }};
}

macro_rules! register_stdlib_fn_1 {
    ($module:expr, $stdlib:expr, $name:literal, $method:ident, $arg:ident : $ty:ty) => {{
        let stdlib = $stdlib.clone();
        $module.set_native_fn($name, move |context: rhai::NativeCallContext, $arg: $ty| {
            let mut lib = $crate::stdlib::util::lock_stdlib(&stdlib)?;
            lib.$method($arg)
                .map_err(|error| $crate::stdlib::util::to_rhai_error_in_context(error, &context))
        });
    }};
}

macro_rules! register_stdlib_fn_2 {
    ($module:expr, $stdlib:expr, $name:literal, $method:ident, $arg1:ident : $ty1:ty, $arg2:ident : $ty2:ty) => {{
        let stdlib = $stdlib.clone();
        $module.set_native_fn(
            $name,
            move |context: rhai::NativeCallContext, $arg1: $ty1, $arg2: $ty2| {
                let mut lib = $crate::stdlib::util::lock_stdlib(&stdlib)?;
                lib.$method($arg1, $arg2).map_err(|error| {
                    $crate::stdlib::util::to_rhai_error_in_context(error, &context)
                })
            },
        );
    }};
}

macro_rules! register_stdlib_fn_3 {
    ($module:expr, $stdlib:expr, $name:literal, $method:ident, $a:ident : $ta:ty, $b:ident : $tb:ty, $c:ident : $tc:ty) => {{
        let stdlib = $stdlib.clone();
        $module.set_native_fn(
            $name,
            move |context: rhai::NativeCallContext, $a: $ta, $b: $tb, $c: $tc| {
                let mut lib = $crate::stdlib::util::lock_stdlib(&stdlib)?;
                lib.$method($a, $b, $c).map_err(|error| {
                    $crate::stdlib::util::to_rhai_error_in_context(error, &context)
                })
            },
        );
    }};
}

macro_rules! register_stdlib_fn_4 {
    ($module:expr, $stdlib:expr, $name:literal, $method:ident, $a:ident : $ta:ty, $b:ident : $tb:ty, $c:ident : $tc:ty, $d:ident : $td:ty) => {{
        let stdlib = $stdlib.clone();
        $module.set_native_fn(
            $name,
            move |context: rhai::NativeCallContext, $a: $ta, $b: $tb, $c: $tc, $d: $td| {
                let mut lib = $crate::stdlib::util::lock_stdlib(&stdlib)?;
                lib.$method($a, $b, $c, $d).map_err(|error| {
                    $crate::stdlib::util::to_rhai_error_in_context(error, &context)
                })
            },
        );
    }};
}

macro_rules! register_stdlib_fn_5 {
    ($module:expr, $stdlib:expr, $name:literal, $method:ident, $a:ident : $ta:ty, $b:ident : $tb:ty, $c:ident : $tc:ty, $d:ident : $td:ty, $e:ident : $te:ty) => {{
        let stdlib = $stdlib.clone();
        $module.set_native_fn(
            $name,
            move |context: rhai::NativeCallContext, $a: $ta, $b: $tb, $c: $tc, $d: $td, $e: $te| {
                let mut lib = $crate::stdlib::util::lock_stdlib(&stdlib)?;
                lib.$method($a, $b, $c, $d, $e).map_err(|error| {
                    $crate::stdlib::util::to_rhai_error_in_context(error, &context)
                })
            },
        );
    }};
}

macro_rules! register_stdlib_fn_7 {
    ($module:expr, $stdlib:expr, $name:literal, $method:ident, $a:ident : $ta:ty, $b:ident : $tb:ty, $c:ident : $tc:ty, $d:ident : $td:ty, $e:ident : $te:ty, $f:ident : $tf:ty, $g:ident : $tg:ty) => {{
        let stdlib = $stdlib.clone();
        $module.set_native_fn(
            $name,
            move |context: rhai::NativeCallContext,
                  $a: $ta,
                  $b: $tb,
                  $c: $tc,
                  $d: $td,
                  $e: $te,
                  $f: $tf,
                  $g: $tg| {
                let mut lib = $crate::stdlib::util::lock_stdlib(&stdlib)?;
                lib.$method($a, $b, $c, $d, $e, $f, $g).map_err(|error| {
                    $crate::stdlib::util::to_rhai_error_in_context(error, &context)
                })
            },
        );
    }};
}

pub(crate) use register_stdlib_fn_0;
pub(crate) use register_stdlib_fn_1;
pub(crate) use register_stdlib_fn_2;
pub(crate) use register_stdlib_fn_3;
pub(crate) use register_stdlib_fn_4;
pub(crate) use register_stdlib_fn_5;
pub(crate) use register_stdlib_fn_7;
