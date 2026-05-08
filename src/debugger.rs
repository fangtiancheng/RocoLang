use rhai::Dynamic;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RocoDebugBreakpoint {
    pub source: Option<String>,
    pub line: usize,
    pub column: Option<usize>,
    pub enabled: bool,
}

#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct RocoDebugConfig {
    pub source: Option<String>,
    pub breakpoints: Vec<RocoDebugBreakpoint>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum RocoDebugCommand {
    SetBreakpoints(Vec<RocoDebugBreakpoint>),
    Continue,
    StepInto,
    StepOver,
    Next,
    StepOut,
    Stop,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RocoDebugStackFrame {
    pub function_name: String,
    pub source: Option<String>,
    pub line: Option<usize>,
    pub column: Option<usize>,
    pub args_preview: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RocoDebugLocalVariable {
    pub name: String,
    pub type_name: String,
    pub value_preview: String,
    pub is_constant: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum RocoDebugEvent {
    Started,
    Paused {
        reason: String,
        source: Option<String>,
        line: Option<usize>,
        column: Option<usize>,
        stack: Vec<RocoDebugStackFrame>,
        locals: Vec<RocoDebugLocalVariable>,
    },
    Continued,
    Ended,
}

pub struct RocoDebugHooks {
    pub on_event: Box<dyn FnMut(RocoDebugEvent) + Send>,
    pub wait_command: Box<dyn FnMut() -> RocoDebugCommand + Send>,
}

impl RocoDebugHooks {
    pub fn new(
        on_event: impl FnMut(RocoDebugEvent) + Send + 'static,
        wait_command: impl FnMut() -> RocoDebugCommand + Send + 'static,
    ) -> Self {
        Self {
            on_event: Box::new(on_event),
            wait_command: Box::new(wait_command),
        }
    }
}

pub(crate) fn dynamic_preview(value: &Dynamic) -> String {
    format!("{value:?}")
}
