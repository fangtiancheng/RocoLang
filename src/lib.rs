//! RocoLang - 基于 Rhai 的洛克王国测试脚本语言
//!
//! 提供统一的脚本执行引擎和标准库接口定义

pub mod engine;
pub mod error;
pub mod stdlib;
pub mod types;

// 重导出核心类型
pub use engine::RocoEngine;
pub use error::{Result, RocoError};
pub use stdlib::RocoStdLib;
pub use types::*;
