//! Standard library trait and namespace registration modules.

use crate::error::{Result, ScriptFunctionName, ScriptUnsupportedError};

mod activities;
mod gameplay;
mod homestead;
pub mod metadata;
mod registration;
mod services;
pub mod traits;
pub mod util;
mod values;
mod zodiac;

pub use activities::*;
pub use gameplay::*;
pub use homestead::*;
pub use services::*;
pub use values::*;
pub use zodiac::*;

pub use metadata::{
    documented_stdlib_function_keys, find_stdlib_function_doc, registered_stdlib_function_keys,
    registered_stdlib_function_registrations, stdlib_function_docs, stdlib_type_docs,
    StdlibFieldDoc, StdlibFunctionDoc, StdlibFunctionKey, StdlibFunctionRegistration,
    StdlibParamDoc, StdlibReturnDoc,
};
pub(crate) use registration::{register_modules, registered_value_modules};
pub use traits::*;

fn unsupported<T>(name: &str) -> Result<T> {
    Err(ScriptUnsupportedError::Function {
        name: ScriptFunctionName::parse(name),
    }
    .into())
}
