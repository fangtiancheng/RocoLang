use ::rhai::{Array, Dynamic, Engine};

use crate::types::*;

macro_rules! register_getters {
    ($engine:expr, $type:ty, $($field:ident),+ $(,)?) => {
        $(
            $engine.register_get(stringify!($field), |value: &mut $type| {
                value.$field.clone()
            });
        )+
    };
}

macro_rules! register_error_info_getters {
    ($engine:expr, $type:ty) => {
        $engine.register_get("error_kind_code", |value: &mut $type| {
            value
                .error
                .as_ref()
                .map(crate::RocoErrorInfo::kind_code)
                .unwrap_or_default()
        });
        $engine.register_get("error_detail_kind_code", |value: &mut $type| {
            value
                .error
                .as_ref()
                .map(crate::RocoErrorInfo::detail_kind_code)
                .unwrap_or_default()
        });
        $engine.register_get("error_network_kind_code", |value: &mut $type| {
            value
                .error
                .as_ref()
                .map(crate::RocoErrorInfo::network_kind_code)
                .unwrap_or_default()
        });
        $engine.register_get("error_code", |value: &mut $type| {
            value
                .error
                .as_ref()
                .map(|error| error.code.clone())
                .unwrap_or_default()
        });
        $engine.register_get("error_message", |value: &mut $type| {
            value
                .error
                .as_ref()
                .map(|error| error.message.clone())
                .unwrap_or_default()
        });
        $engine.register_get("error_detail", |value: &mut $type| {
            value
                .error
                .as_ref()
                .map(|error| error.detail.clone())
                .unwrap_or(crate::RocoErrorDetail::None)
        });
    };
}

macro_rules! register_optional_getters {
    ($engine:expr, $type:ty) => {
        $engine.register_get("present", |value: &mut $type| value.is_present());
        $engine.register_get("value", |value: &mut $type| {
            value
                .value()
                .map(::rhai::Dynamic::from)
                .unwrap_or(::rhai::Dynamic::UNIT)
        });
    };
}

mod action;
mod activities;
mod combat;
mod common;
mod game;
mod home;
mod jump_machine;
mod ladder;
mod manor;
mod news;
mod pet_training;
mod play_guide;
mod profile;
mod scene;
mod spirit;
mod spirit_book;
mod star_tower;
mod static_data;
mod task;
mod zodiac;

pub(crate) fn register_rhai_getters(engine: &mut Engine) {
    register_optional_getters!(engine, RocoOptionalIceCrystalBattleInfo);
    register_optional_getters!(engine, RocoOptionalCapricornSecondTask);
    register_optional_getters!(engine, RocoOptionalCapricornTeamSnapshot);
    action::register_rhai_getters(engine);
    activities::register_rhai_getters(engine);
    combat::register_rhai_getters(engine);
    common::register_rhai_getters(engine);
    game::register_rhai_getters(engine);
    home::register_rhai_getters(engine);
    jump_machine::register_rhai_getters(engine);
    ladder::register_rhai_getters(engine);
    manor::register_rhai_getters(engine);
    news::register_rhai_getters(engine);
    pet_training::register_rhai_getters(engine);
    play_guide::register_rhai_getters(engine);
    profile::register_rhai_getters(engine);
    scene::register_rhai_getters(engine);
    spirit_book::register_rhai_getters(engine);
    spirit::register_rhai_getters(engine);
    star_tower::register_rhai_getters(engine);
    static_data::register_rhai_getters(engine);
    task::register_rhai_getters(engine);
    zodiac::register_rhai_getters(engine);
}

fn to_array<T: Clone + Send + Sync + 'static>(items: &[T]) -> Array {
    items.iter().cloned().map(Dynamic::from).collect()
}
