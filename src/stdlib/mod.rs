//! Standard library trait and namespace registration modules.

use crate::error::{Result, ScriptFunctionName, ScriptUnsupportedError};

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
pub mod evolution_edge_kind;
pub mod four_seasons;
pub mod game;
pub mod gemini;
pub mod ice_crystal;
pub mod jump_machine;
pub mod ladder;
pub mod leo;
pub mod libra;
pub mod lookup;
pub mod magic_pioneer;
pub mod manor;
pub mod metadata;
pub mod mountain_sea;
pub mod multi_evolution;
pub mod mystery_fusion;
pub mod news;
pub mod news_times;
pub mod personality;
pub mod pet_training;
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
pub mod spirit_book;
pub mod spirit_book_state;
pub mod star_tower;
pub mod summon;
pub mod system;
pub mod task;
pub mod taurus;
pub mod three_starters;
pub mod treasure_realm;
pub mod type_ladder;
pub mod unicorn;
pub mod util;
pub mod virgo;
pub mod weather;

pub mod traits;

pub use metadata::{
    documented_stdlib_function_keys, find_stdlib_function_doc, registered_stdlib_function_keys,
    registered_stdlib_function_registrations, stdlib_function_docs, StdlibFieldDoc,
    StdlibFunctionDoc, StdlibFunctionKey, StdlibFunctionRegistration, StdlibParamDoc,
    StdlibReturnDoc,
};
pub use traits::*;

fn unsupported<T>(name: &str) -> Result<T> {
    Err(ScriptUnsupportedError::Function {
        name: ScriptFunctionName::parse(name),
    }
    .into())
}
