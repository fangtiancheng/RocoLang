mod adventure;
mod combat;
mod docs;
mod enum_helpers;
mod friend;
mod game;
mod home;
mod lookup;
mod manor;
mod memory;
mod model;
mod news;
mod pet_training;
mod profile;
mod registered;
mod remote_state;
mod role;
mod scene;
mod session;
mod spirit;
mod spirit_book;
mod system;
mod types;

pub use docs::{
    find_stdlib_function_doc, registered_stdlib_function_registrations, stdlib_function_docs,
    stdlib_type_docs,
};
pub use model::{
    StdlibFieldDoc, StdlibFunctionDoc, StdlibFunctionRegistration, StdlibParamDoc, StdlibReturnDoc,
};
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
