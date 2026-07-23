mod adventure;
mod alchemy_furnace;
mod aquarius;
mod aries;
mod cancer;
mod combat;
mod dark_city;
mod diamond_tear;
mod docs;
mod enum_helpers;
mod four_seasons;
mod friend;
mod game;
mod gemini;
mod home;
mod ice_crystal;
mod incubative_machine;
mod jump_machine;
mod ladder;
mod libra;
mod lookup;
mod manor;
mod memory;
mod model;
mod mountain_sea;
mod multi_evolution;
mod mystery_fusion;
mod news;
mod news_times;
mod pet_egg;
mod pet_training;
mod pisces;
mod play_guide;
mod profile;
mod registered;
mod remote_state;
mod role;
mod scene;
mod scorpio;
mod semantic;
mod sentinel_intelligence;
mod session;
mod spirit;
mod spirit_book;
mod star_tower;
mod summon;
mod system;
mod task;
mod taurus;
mod three_starters;
mod treasure_realm;
mod type_ladder;
mod types;
mod unicorn;

pub use docs::{
    find_stdlib_function_doc, registered_stdlib_function_registrations, stdlib_function_docs,
    stdlib_type_docs,
};
pub use model::{
    StdlibFieldDoc, StdlibFunctionDoc, StdlibFunctionRegistration, StdlibParamDoc, StdlibReturnDoc,
};

include!(concat!(env!("OUT_DIR"), "/roco_stdlib_return_types.rs"));
use model::{StdlibFunctionDetails, StdlibFunctionKey};

macro_rules! stdlib_doc {
    (
        $module:literal,
        $name:literal,
        return_type: $return_type:literal,
        $description:literal,
        params: [$($param_name:literal => $param_desc:literal),* $(,)?],
        returns: $returns:literal,
        examples: [$($example:literal),* $(,)?]
    ) => {
        StdlibFunctionDetails {
            key: $crate::stdlib::metadata::StdlibFunctionKey::new($module, $name),
            return_type: $return_type,
            description: $description.to_string(),
            params: vec![$($crate::stdlib::metadata::StdlibParamDoc {
                name: $param_name.to_string(),
                description: $param_desc.to_string(),
            }),*],
            returns: $returns.to_string(),
            examples: vec![$($example.to_string()),*],
        }
    };
}

pub(crate) use stdlib_doc;

#[cfg(test)]
mod tests;
