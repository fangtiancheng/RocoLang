//! 共享类型定义

use rhai::{Array, Dynamic, Engine};

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
                .map(rhai::Dynamic::from)
                .unwrap_or(rhai::Dynamic::UNIT)
        });
    };
}

mod action;
mod activities;
mod combat;
mod common;
mod game;
mod home;
mod incubative_machine;
mod jump_machine;
mod ladder;
mod manor;
mod news;
mod pet_egg;
mod pet_training;
mod play_guide;
mod profile;
mod remote_state;
mod scene;
mod spirit;
mod spirit_book;
mod star_tower;
mod static_data;
mod task;
mod zodiac;

pub use action::*;
pub use activities::*;
pub use combat::*;
pub use common::*;
pub use game::*;
pub use home::*;
pub use incubative_machine::*;
pub use jump_machine::*;
pub use ladder::*;
pub use manor::*;
pub use news::*;
pub use pet_egg::*;
pub use pet_training::*;
pub use play_guide::*;
pub use profile::*;
pub use remote_state::*;
pub use scene::*;
pub use spirit::*;
pub use spirit_book::*;
pub use star_tower::*;
pub use static_data::*;
pub use task::*;
pub use zodiac::*;

pub(crate) fn register_rhai_getters(engine: &mut Engine) {
    register_shared_rhai_getters(engine);
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
    spirit::register_extra_rhai_getters(engine);
    ladder::register_config_rhai_getters(engine);
    static_data::register_rhai_getters(engine);
    star_tower::register_rhai_getters(engine);
    task::register_rhai_getters(engine);
    zodiac::register_rhai_getters(engine);
}

fn register_shared_rhai_getters(engine: &mut Engine) {
    register_optional_getters!(engine, RocoOptionalIceCrystalBattleInfo);
    register_optional_getters!(engine, RocoOptionalCapricornSecondTask);
    register_optional_getters!(engine, RocoOptionalCapricornTeamSnapshot);
}

fn to_array<T: Clone + Send + Sync + 'static>(items: &[T]) -> Array {
    items.iter().cloned().map(Dynamic::from).collect()
}
