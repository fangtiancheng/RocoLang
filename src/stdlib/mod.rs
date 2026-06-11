//! Standard library trait and namespace registration modules.

use crate::error::{Result, RocoError};

pub mod alchemy_furnace;
pub mod aquarius;
pub mod aries;
pub mod cancer;
pub mod capricorn;
pub mod combat;
pub mod combat_result;
pub mod combat_status;
pub mod dark_city;
pub mod diamond_tear;
pub mod four_seasons;
pub mod game;
pub mod gemini;
pub mod ice_crystal;
pub mod ladder;
pub mod leo;
pub mod libra;
pub mod lookup;
pub mod magic_pioneer;
pub mod manor;
pub mod mountain_sea;
pub mod multi_evolution;
pub mod mystery_fusion;
pub mod news;
pub mod news_times;
pub mod personality;
pub mod pisces;
pub mod play_guide;
pub mod profile;
pub mod role;
pub mod sagittarius;
pub mod scene;
pub mod scorpio;
pub mod sentinel_intelligence;
pub mod session;
pub mod spirit;
pub mod star_tower;
pub mod summon;
pub mod system;
pub mod taurus;
pub mod three_starters;
pub mod treasure_realm;
pub mod type_ladder;
pub mod unicorn;
pub mod util;
pub mod virgo;
pub mod weather;

pub mod traits;

pub use traits::*;

fn unsupported<T>(name: &str) -> Result<T> {
    Err(RocoError::StdLibError(format!(
        "{name} unsupported by this runtime"
    )))
}
