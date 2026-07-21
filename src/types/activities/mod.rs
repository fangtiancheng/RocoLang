mod alchemy_furnace;
mod dark_city;
mod diamond_tear;
mod four_seasons;
mod ice_crystal;
mod magic_pioneer;
mod mountain_sea;
mod multi_evolution;
mod mystery_fusion;
mod sentinel_intelligence;
mod summon;
mod three_starters;
mod treasure_realm;
mod unicorn;

use super::Engine;
pub use alchemy_furnace::*;
pub use dark_city::*;
pub use diamond_tear::*;
pub use four_seasons::*;
pub use ice_crystal::*;
pub use magic_pioneer::*;
pub use mountain_sea::*;
pub use multi_evolution::*;
pub use mystery_fusion::*;
pub use sentinel_intelligence::*;
pub use summon::*;
pub use three_starters::*;
pub use treasure_realm::*;
pub use unicorn::*;

pub(super) fn register_rhai_getters(engine: &mut Engine) {
    alchemy_furnace::register_rhai_getters(engine);
    dark_city::register_rhai_getters(engine);
    diamond_tear::register_rhai_getters(engine);
    four_seasons::register_rhai_getters(engine);
    ice_crystal::register_rhai_getters(engine);
    magic_pioneer::register_rhai_getters(engine);
    mountain_sea::register_rhai_getters(engine);
    multi_evolution::register_rhai_getters(engine);
    mystery_fusion::register_rhai_getters(engine);
    sentinel_intelligence::register_rhai_getters(engine);
    summon::register_rhai_getters(engine);
    three_starters::register_rhai_getters(engine);
    treasure_realm::register_rhai_getters(engine);
    unicorn::register_rhai_getters(engine);
}
