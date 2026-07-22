//! Error types for RocoLang.

use serde::{Deserialize, Serialize};
use std::{borrow::Cow, fmt};

use crate::types::RocoRequestContext;

mod classification;
mod invalid_param;
mod network;
mod rejection;
mod rhai;
mod script;
mod stdlib;
mod top_level;

pub use classification::*;
pub use invalid_param::*;
pub use network::*;
pub use rejection::*;
pub use script::*;
pub use stdlib::*;
pub use top_level::*;

pub(crate) use rhai::register_rhai_types;

pub type Result<T> = std::result::Result<T, RocoError>;
