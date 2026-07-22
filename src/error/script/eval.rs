use serde::{Deserialize, Serialize};

use super::{RocoScriptParseErrorSource, RocoScriptRuntimeErrorValue};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum RocoScriptEvalErrorSource {
    System {
        message: String,
    },
    Parsing {
        source: RocoScriptParseErrorSource,
    },
    VariableExists {
        variable: String,
    },
    ForbiddenVariable {
        variable: String,
    },
    VariableNotFound {
        variable: String,
    },
    PropertyNotFound {
        property: String,
    },
    IndexNotFound,
    FunctionNotFound {
        signature: String,
    },
    ModuleNotFound {
        module: String,
    },
    FunctionCall {
        function: String,
        source: String,
        cause: Box<RocoScriptEvalErrorSource>,
    },
    Module {
        module: String,
        cause: Box<RocoScriptEvalErrorSource>,
    },
    UnboundThis,
    MismatchDataType {
        expected: String,
        actual: String,
    },
    MismatchOutputType {
        expected: String,
        actual: String,
    },
    IndexingType {
        type_name: String,
    },
    ArrayBounds {
        len: usize,
        index: i64,
    },
    StringBounds {
        len: usize,
        index: i64,
    },
    BitFieldBounds {
        len: usize,
        index: i64,
    },
    ForLoopNonIterable,
    DataRace {
        variable: String,
    },
    NonPureMethodCallOnConstant {
        method: String,
    },
    AssignmentToConstant {
        variable: String,
    },
    DotExpr {
        expression: String,
    },
    Arithmetic {
        message: String,
    },
    TooManyOperations,
    TooManyVariables,
    TooManyModules,
    StackOverflow,
    DataTooLarge {
        type_name: String,
    },
    Terminated,
    CustomSyntax {
        message: String,
        symbols: Vec<String>,
    },
    Runtime {
        value: RocoScriptRuntimeErrorValue,
    },
    LoopBreak {
        is_break: bool,
    },
    Return,
    Exit,
    UnclassifiedEval,
}

impl RocoScriptEvalErrorSource {
    pub const fn code(&self) -> &'static str {
        match self {
            Self::System { .. } => "eval.system",
            Self::Parsing { .. } => "eval.parsing",
            Self::VariableExists { .. } => "eval.variable_exists",
            Self::ForbiddenVariable { .. } => "eval.forbidden_variable",
            Self::VariableNotFound { .. } => "eval.variable_not_found",
            Self::PropertyNotFound { .. } => "eval.property_not_found",
            Self::IndexNotFound => "eval.index_not_found",
            Self::FunctionNotFound { .. } => "eval.function_not_found",
            Self::ModuleNotFound { .. } => "eval.module_not_found",
            Self::FunctionCall { .. } => "eval.function_call",
            Self::Module { .. } => "eval.module",
            Self::UnboundThis => "eval.unbound_this",
            Self::MismatchDataType { .. } => "eval.mismatch_data_type",
            Self::MismatchOutputType { .. } => "eval.mismatch_output_type",
            Self::IndexingType { .. } => "eval.indexing_type",
            Self::ArrayBounds { .. } => "eval.array_bounds",
            Self::StringBounds { .. } => "eval.string_bounds",
            Self::BitFieldBounds { .. } => "eval.bit_field_bounds",
            Self::ForLoopNonIterable => "eval.for_loop_non_iterable",
            Self::DataRace { .. } => "eval.data_race",
            Self::NonPureMethodCallOnConstant { .. } => "eval.non_pure_method_call_on_constant",
            Self::AssignmentToConstant { .. } => "eval.assignment_to_constant",
            Self::DotExpr { .. } => "eval.dot_expr",
            Self::Arithmetic { .. } => "eval.arithmetic",
            Self::TooManyOperations => "eval.too_many_operations",
            Self::TooManyVariables => "eval.too_many_variables",
            Self::TooManyModules => "eval.too_many_modules",
            Self::StackOverflow => "eval.stack_overflow",
            Self::DataTooLarge { .. } => "eval.data_too_large",
            Self::Terminated => "eval.terminated",
            Self::CustomSyntax { .. } => "eval.custom_syntax",
            Self::Runtime { value } => match value {
                RocoScriptRuntimeErrorValue::Message { .. } => "eval.runtime.message",
                RocoScriptRuntimeErrorValue::RocoTypeJsonSerialize { .. } => {
                    "eval.runtime.roco_type_json_serialize"
                }
            },
            Self::LoopBreak { .. } => "eval.loop_break",
            Self::Return => "eval.return",
            Self::Exit => "eval.exit",
            Self::UnclassifiedEval => "eval.unclassified",
        }
    }

    pub fn message(&self) -> String {
        match self {
            Self::System { message } if message.is_empty() => "System error".to_string(),
            Self::System { message } => format!("System error: {message}"),
            Self::Parsing { source } => format!("Syntax error: {}", source.message()),
            Self::VariableExists { variable } => format!("Variable already defined: {variable}"),
            Self::ForbiddenVariable { variable } => format!("Forbidden variable name: {variable}"),
            Self::VariableNotFound { variable } => format!("Variable not found: {variable}"),
            Self::PropertyNotFound { property } => format!("Property not found: {property}"),
            Self::IndexNotFound => "Invalid index".to_string(),
            Self::FunctionNotFound { signature } => format!("Function not found: {signature}"),
            Self::ModuleNotFound { module } => format!("Module not found: {module}"),
            Self::FunctionCall {
                function,
                source,
                cause,
            } => {
                let suffix = if source.is_empty() {
                    String::new()
                } else {
                    format!(" (from '{source}')")
                };
                format!(
                    "{}\nin call to function '{function}'{suffix}",
                    cause.message()
                )
            }
            Self::Module { module, cause } if module.is_empty() => {
                format!("{}\nin module", cause.message())
            }
            Self::Module { module, cause } => format!("{}\nin module '{module}'", cause.message()),
            Self::UnboundThis => "'this' not bound".to_string(),
            Self::MismatchDataType { expected, actual } => {
                match (expected.as_str(), actual.as_str()) {
                    ("", actual) => format!("Data type incorrect: {actual}"),
                    (expected, "") => format!("Data type incorrect, expecting {expected}"),
                    (expected, actual) => {
                        format!("Data type incorrect: {actual} (expecting {expected})")
                    }
                }
            }
            Self::MismatchOutputType { expected, actual } => {
                match (expected.as_str(), actual.as_str()) {
                    ("", actual) => format!("Output type incorrect: {actual}"),
                    (expected, "") => format!("Output type incorrect, expecting {expected}"),
                    (expected, actual) => {
                        format!("Output type incorrect: {actual} (expecting {expected})")
                    }
                }
            }
            Self::IndexingType { type_name } => format!("Indexer unavailable: {type_name}"),
            Self::ArrayBounds { len, index } => {
                format!("Array index {index} out of bounds: len {len}")
            }
            Self::StringBounds { len, index } => {
                format!("String index {index} out of bounds: len {len}")
            }
            Self::BitFieldBounds { len, index } => {
                format!("Bit-field index {index} out of bounds: len {len}")
            }
            Self::ForLoopNonIterable => "For loop expects iterable type".to_string(),
            Self::DataRace { variable } if variable.is_empty() => "Data race detected".to_string(),
            Self::DataRace { variable } => format!("Data race detected on variable '{variable}'"),
            Self::NonPureMethodCallOnConstant { method } => {
                format!("Non-pure method '{method}' cannot be called on constant")
            }
            Self::AssignmentToConstant { variable } => format!("Cannot modify constant {variable}"),
            Self::DotExpr { expression } if expression.is_empty() => {
                "Malformed dot expression".to_string()
            }
            Self::DotExpr { expression } => expression.clone(),
            Self::Arithmetic { message } if message.is_empty() => "Arithmetic error".to_string(),
            Self::Arithmetic { message } => message.clone(),
            Self::TooManyOperations => "Too many operations".to_string(),
            Self::TooManyVariables => "Too many variables defined".to_string(),
            Self::TooManyModules => "Too many modules imported".to_string(),
            Self::StackOverflow => "Stack overflow".to_string(),
            Self::DataTooLarge { type_name } => {
                format!("Data value over maximum size limit: {type_name}")
            }
            Self::Terminated => "Script terminated".to_string(),
            Self::CustomSyntax { message, .. } => format!("Custom syntax error: {message}"),
            Self::Runtime { value } => {
                let message = value.message_text();
                if message.is_empty() {
                    "Runtime error".to_string()
                } else {
                    format!("Runtime error: {message}")
                }
            }
            Self::LoopBreak { is_break: true } => "'break' must be within a loop".to_string(),
            Self::LoopBreak { is_break: false } => "'continue' must be within a loop".to_string(),
            Self::Return => "NOT AN ERROR - function returns value".to_string(),
            Self::Exit => "NOT AN ERROR - exit value".to_string(),
            Self::UnclassifiedEval => "Unclassified eval error".to_string(),
        }
    }
}
