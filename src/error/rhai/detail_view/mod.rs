use super::super::*;

mod combat;
mod core;
mod domain;
mod network;

pub(super) struct RocoErrorDetailScriptView<'a>(pub(super) &'a RocoErrorDetail);

macro_rules! register_error_detail_getters {
    ($engine:expr; $($name:ident),+ $(,)?) => {
        $(
            $engine.register_get(stringify!($name), |value: &mut RocoErrorDetail| {
                RocoErrorDetailScriptView(value).$name()
            });
        )+
    };
}

pub(super) use register_error_detail_getters;
