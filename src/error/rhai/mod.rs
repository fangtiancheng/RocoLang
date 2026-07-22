use ::rhai::Engine;

macro_rules! register_getters {
    ($engine:expr, $type:ty, $($field:ident),+ $(,)?) => {
        $(
            $engine.register_get(stringify!($field), |value: &mut $type| {
                value.$field.clone()
            });
        )+
    };
}

mod activity;
mod bridge;
mod classification;
mod combat_action;
mod combat_runtime;
mod detail_view;
mod identity;
mod invalid_param;
mod network;
mod request;
mod script;
mod spirit;
mod stdlib_context;
mod type_names;

pub(crate) fn register_rhai_types(engine: &mut Engine) {
    script::register(engine);
    type_names::register(engine);
    stdlib_context::register(engine);
    activity::register(engine);
    combat_action::register(engine);
    request::register(engine);
    spirit::register(engine);
    identity::register(engine);
    classification::register(engine);
    invalid_param::register(engine);
    network::register(engine);
    bridge::register(engine);
    combat_runtime::register(engine);
}
