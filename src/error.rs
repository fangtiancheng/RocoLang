//! Error types for RocoLang.

use serde::{Deserialize, Serialize};
use std::{borrow::Cow, fmt};

use crate::types::RocoRequestContext;

pub type Result<T> = std::result::Result<T, RocoError>;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum RocoScriptErrorKind {
    Parse,
    Runtime,
    FunctionCall,
    Module,
    Terminated,
    Eval,
    Other,
}

impl RocoScriptErrorKind {
    pub const fn as_str(&self) -> &'static str {
        match self {
            Self::Parse => "parse",
            Self::Runtime => "runtime",
            Self::FunctionCall => "function_call",
            Self::Module => "module",
            Self::Terminated => "terminated",
            Self::Eval => "eval",
            Self::Other => "other",
        }
    }
}

impl fmt::Display for RocoScriptErrorKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum RocoScriptPosition {
    Line { line: usize },
    LineColumn { line: usize, column: usize },
}

impl RocoScriptPosition {
    pub const fn line(&self) -> usize {
        match self {
            Self::Line { line } | Self::LineColumn { line, .. } => *line,
        }
    }

    pub const fn column(&self) -> Option<usize> {
        match self {
            Self::Line { .. } => None,
            Self::LineColumn { column, .. } => Some(*column),
        }
    }
}

impl fmt::Display for RocoScriptPosition {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Line { line } => write!(f, "{line}"),
            Self::LineColumn { line, column } => write!(f, "{line}:{column}"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum RocoScriptLocation {
    Unknown,
    Anonymous {
        position: RocoScriptPosition,
    },
    Source {
        source: String,
        position: RocoScriptPosition,
    },
}

impl RocoScriptLocation {
    pub const fn position(&self) -> Option<&RocoScriptPosition> {
        match self {
            Self::Unknown => None,
            Self::Anonymous { position } | Self::Source { position, .. } => Some(position),
        }
    }

    pub fn source(&self) -> Option<&str> {
        match self {
            Self::Source { source, .. } => Some(source),
            Self::Unknown | Self::Anonymous { .. } => None,
        }
    }

    pub fn parts(&self) -> (Option<&str>, Option<usize>, Option<usize>) {
        match self {
            Self::Unknown => (None, None, None),
            Self::Anonymous { position } => (None, Some(position.line()), position.column()),
            Self::Source { source, position } => (
                Some(source.as_str()),
                Some(position.line()),
                position.column(),
            ),
        }
    }
}

impl fmt::Display for RocoScriptLocation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Unknown => f.write_str("unknown"),
            Self::Anonymous { position } => write!(f, "{position}"),
            Self::Source { source, position } => write!(f, "{source}:{position}"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RocoScriptError {
    pub kind: RocoScriptErrorKind,
    pub message: String,
    pub location: RocoScriptLocation,
    pub source: RocoScriptErrorSource,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum RocoScriptErrorSource {
    Parse(RocoScriptParseErrorSource),
    Eval(RocoScriptEvalErrorSource),
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum RocoScriptParseErrorSource {
    UnexpectedEof,
    BadInput,
    UnknownOperator { operator: String },
    MissingToken { token: String, description: String },
    MissingSymbol { description: String },
    MalformedIndexExpr { description: String },
    MalformedCapture { description: String },
    DuplicatedProperty { property: String },
    DuplicatedVariable { variable: String },
    VariableExists { variable: String },
    VariableUndefined { variable: String },
    ModuleUndefined { module: String },
    MismatchedType { expected: String, actual: String },
    ExprExpected { expression: String },
    FunctionDuplicatedDefinition { function: String, params: usize },
    FunctionMissingName,
    FunctionMissingParams { function: String },
    FunctionDuplicatedParam { function: String, parameter: String },
    FunctionMissingBody { function: String },
    AssignmentToConstant { variable: String },
    AssignmentToInvalidLhs { description: String },
    LiteralTooLarge { type_name: String, max_size: usize },
    Reserved { symbol: String },
    ForbiddenVariable { variable: String },
    WrongSwitchIntegerCase,
    WrongSwitchDefaultCase,
    WrongSwitchCaseCondition,
    PropertyExpected,
    VariableExpected,
    WrongDocComment,
    WrongFunctionDefinition,
    WrongExport,
    ExprTooDeep,
    TooManyFunctions,
    LoopBreak,
    DeprecatedMalformedCallExpr { description: String },
    DeprecatedMalformedInExpr { description: String },
    DeprecatedDuplicatedSwitchCase,
    UnclassifiedParse,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum RocoScriptRuntimeErrorValue {
    Message {
        message: String,
    },
    RocoTypeJsonSerialize {
        type_name: String,
        source: RocoJsonErrorInfo,
    },
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RocoJsonErrorInfo {
    pub category: RocoJsonErrorCategory,
    pub line: usize,
    pub column: usize,
    pub message: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum RocoJsonErrorCategory {
    Io,
    Syntax,
    Data,
    Eof,
}

impl RocoScriptRuntimeErrorValue {
    pub fn message(message: impl Into<String>) -> Self {
        Self::Message {
            message: message.into(),
        }
    }

    pub fn roco_type_json_serialize<T>(error: &serde_json::Error) -> Self {
        Self::RocoTypeJsonSerialize {
            type_name: std::any::type_name::<T>().to_string(),
            source: RocoJsonErrorInfo::from_serde_json_error(error),
        }
    }

    pub fn code(&self) -> &'static str {
        match self {
            Self::Message { .. } => "message",
            Self::RocoTypeJsonSerialize { .. } => "roco_type_json_serialize",
        }
    }

    pub fn message_text(&self) -> String {
        match self {
            Self::Message { message } => message.clone(),
            Self::RocoTypeJsonSerialize { type_name, source } => {
                format!(
                    "failed to serialize roco type {type_name} as JSON [{} at {}:{}]: {}",
                    source.category.code(),
                    source.line,
                    source.column,
                    source.message
                )
            }
        }
    }

    pub fn json_error_category_code(&self) -> String {
        match self {
            Self::RocoTypeJsonSerialize { source, .. } => source.category.code().to_string(),
            Self::Message { .. } => String::new(),
        }
    }

    pub fn roco_type_name(&self) -> String {
        match self {
            Self::RocoTypeJsonSerialize { type_name, .. } => type_name.clone(),
            Self::Message { .. } => String::new(),
        }
    }
}

impl RocoJsonErrorInfo {
    fn from_serde_json_error(error: &serde_json::Error) -> Self {
        Self {
            category: RocoJsonErrorCategory::from_serde_json_category(error.classify()),
            line: error.line(),
            column: error.column(),
            message: error.to_string(),
        }
    }
}

impl RocoJsonErrorCategory {
    fn from_serde_json_category(category: serde_json::error::Category) -> Self {
        match category {
            serde_json::error::Category::Io => Self::Io,
            serde_json::error::Category::Syntax => Self::Syntax,
            serde_json::error::Category::Data => Self::Data,
            serde_json::error::Category::Eof => Self::Eof,
        }
    }

    pub const fn code(self) -> &'static str {
        match self {
            Self::Io => "io",
            Self::Syntax => "syntax",
            Self::Data => "data",
            Self::Eof => "eof",
        }
    }
}

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

impl RocoScriptErrorSource {
    pub const fn code(&self) -> &'static str {
        match self {
            Self::Parse(source) => source.code(),
            Self::Eval(source) => source.code(),
        }
    }

    pub fn message(&self) -> String {
        match self {
            Self::Parse(source) => source.message(),
            Self::Eval(source) => source.message(),
        }
    }
}

impl RocoScriptParseErrorSource {
    pub const fn code(&self) -> &'static str {
        match self {
            Self::UnexpectedEof => "parse.unexpected_eof",
            Self::BadInput => "parse.bad_input",
            Self::UnknownOperator { .. } => "parse.unknown_operator",
            Self::MissingToken { .. } => "parse.missing_token",
            Self::MissingSymbol { .. } => "parse.missing_symbol",
            Self::MalformedIndexExpr { .. } => "parse.malformed_index_expr",
            Self::MalformedCapture { .. } => "parse.malformed_capture",
            Self::DuplicatedProperty { .. } => "parse.duplicated_property",
            Self::DuplicatedVariable { .. } => "parse.duplicated_variable",
            Self::VariableExists { .. } => "parse.variable_exists",
            Self::VariableUndefined { .. } => "parse.variable_undefined",
            Self::ModuleUndefined { .. } => "parse.module_undefined",
            Self::MismatchedType { .. } => "parse.mismatched_type",
            Self::ExprExpected { .. } => "parse.expr_expected",
            Self::FunctionDuplicatedDefinition { .. } => "parse.function_duplicated_definition",
            Self::FunctionMissingName => "parse.function_missing_name",
            Self::FunctionMissingParams { .. } => "parse.function_missing_params",
            Self::FunctionDuplicatedParam { .. } => "parse.function_duplicated_param",
            Self::FunctionMissingBody { .. } => "parse.function_missing_body",
            Self::AssignmentToConstant { .. } => "parse.assignment_to_constant",
            Self::AssignmentToInvalidLhs { .. } => "parse.assignment_to_invalid_lhs",
            Self::LiteralTooLarge { .. } => "parse.literal_too_large",
            Self::Reserved { .. } => "parse.reserved",
            Self::ForbiddenVariable { .. } => "parse.forbidden_variable",
            Self::WrongSwitchIntegerCase => "parse.wrong_switch_integer_case",
            Self::WrongSwitchDefaultCase => "parse.wrong_switch_default_case",
            Self::WrongSwitchCaseCondition => "parse.wrong_switch_case_condition",
            Self::PropertyExpected => "parse.property_expected",
            Self::VariableExpected => "parse.variable_expected",
            Self::WrongDocComment => "parse.wrong_doc_comment",
            Self::WrongFunctionDefinition => "parse.wrong_function_definition",
            Self::WrongExport => "parse.wrong_export",
            Self::ExprTooDeep => "parse.expr_too_deep",
            Self::TooManyFunctions => "parse.too_many_functions",
            Self::LoopBreak => "parse.loop_break",
            Self::DeprecatedMalformedCallExpr { .. } => "parse.deprecated_malformed_call_expr",
            Self::DeprecatedMalformedInExpr { .. } => "parse.deprecated_malformed_in_expr",
            Self::DeprecatedDuplicatedSwitchCase => "parse.deprecated_duplicated_switch_case",
            Self::UnclassifiedParse => "parse.unclassified",
        }
    }

    pub fn message(&self) -> String {
        match self {
            Self::UnexpectedEof => "Script is incomplete".to_string(),
            Self::BadInput => "Bad input".to_string(),
            Self::UnknownOperator { operator } => format!("Unknown operator: '{operator}'"),
            Self::MissingToken { token, description } => format!("Expecting '{token}' {description}"),
            Self::MissingSymbol { description } if description.is_empty() => {
                "Expecting a symbol".to_string()
            }
            Self::MissingSymbol { description } => description.clone(),
            Self::MalformedIndexExpr { description } if description.is_empty() => {
                "Invalid index in indexing expression".to_string()
            }
            Self::MalformedIndexExpr { description } => description.clone(),
            Self::MalformedCapture { description } if description.is_empty() => {
                "Invalid capturing".to_string()
            }
            Self::MalformedCapture { description } => description.clone(),
            Self::DuplicatedProperty { property } => {
                format!("Duplicated property for object map literal: {property}")
            }
            Self::DuplicatedVariable { variable } => format!("Duplicated variable name: {variable}"),
            Self::VariableExists { variable } => format!("Variable already defined: {variable}"),
            Self::VariableUndefined { variable } => format!("Undefined variable: {variable}"),
            Self::ModuleUndefined { module } => format!("Undefined module: {module}"),
            Self::MismatchedType { expected, actual } => format!("Expecting {expected}, not {actual}"),
            Self::ExprExpected { expression } => format!("Expecting {expression} expression"),
            Self::FunctionDuplicatedDefinition { function, params } => {
                format!("Function {function} with {params} parameters already exists")
            }
            Self::FunctionMissingName => "Expecting function name in function declaration".to_string(),
            Self::FunctionMissingParams { function } => {
                format!("Expecting parameters for function {function}")
            }
            Self::FunctionDuplicatedParam { function, parameter } => {
                format!("Duplicated parameter {parameter} for function {function}")
            }
            Self::FunctionMissingBody { function } if function.is_empty() => {
                "Expecting body statement block for anonymous function".to_string()
            }
            Self::FunctionMissingBody { function } => {
                format!("Expecting body statement block for function {function}")
            }
            Self::AssignmentToConstant { variable } if variable.is_empty() => {
                "Cannot assign to a constant value".to_string()
            }
            Self::AssignmentToConstant { variable } => format!("Cannot assign to constant {variable}"),
            Self::AssignmentToInvalidLhs { description } if description.is_empty() => {
                "Expression cannot be assigned to".to_string()
            }
            Self::AssignmentToInvalidLhs { description } => description.clone(),
            Self::LiteralTooLarge { type_name, max_size } => {
                format!("{type_name} exceeds the maximum limit ({max_size})")
            }
            Self::Reserved { symbol } => format!("'{symbol}' is reserved"),
            Self::ForbiddenVariable { variable } => format!("Forbidden variable name: {variable}"),
            Self::WrongSwitchIntegerCase => "Numeric switch case cannot follow a range case".to_string(),
            Self::WrongSwitchDefaultCase => "Default switch case must be the last".to_string(),
            Self::WrongSwitchCaseCondition => "This switch case cannot have a condition".to_string(),
            Self::PropertyExpected => "Expecting name of a property".to_string(),
            Self::VariableExpected => "Expecting name of a variable".to_string(),
            Self::WrongDocComment => {
                "Doc-comment must be followed immediately by a function definition".to_string()
            }
            Self::WrongFunctionDefinition => {
                "Function definitions must be at global level and cannot be inside a block or another function"
                    .to_string()
            }
            Self::WrongExport => "Export statement can only appear at global level".to_string(),
            Self::ExprTooDeep => "Expression exceeds maximum complexity".to_string(),
            Self::TooManyFunctions => "Number of functions defined exceeds maximum limit".to_string(),
            Self::LoopBreak => "Break statement should only be used inside a loop".to_string(),
            Self::DeprecatedMalformedCallExpr { description }
            | Self::DeprecatedMalformedInExpr { description } => description.clone(),
            Self::DeprecatedDuplicatedSwitchCase => "Duplicated switch case".to_string(),
            Self::UnclassifiedParse => "Unclassified parse error".to_string(),
        }
    }
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

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum RocoStdLibError {
    FunctionContext(ScriptFunctionContextError),
    Query(ScriptQueryError),
    CombatAction(ScriptCombatActionError),
    CombatRuntime(ScriptCombatRuntimeError),
    CombatWait(ScriptCombatWaitError),
    PendingResponse(ScriptPendingResponseError),
    Lookup(ScriptLookupError),
    SessionMemory(ScriptSessionMemoryError),
    StaticData(ScriptStaticDataError),
    SpiritOperation(ScriptSpiritOperationError),
    System(ScriptSystemError),
    ActivityOperation(ScriptActivityOperationError),
    Bridge(ScriptBridgeError),
    Response(ScriptResponseError),
    Request(ScriptRequestError),
    Unsupported(ScriptUnsupportedError),
}

impl fmt::Display for RocoStdLibError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::FunctionContext(error) => write!(f, "{error}"),
            Self::Query(error) => write!(f, "{error}"),
            Self::CombatAction(error) => write!(f, "{error}"),
            Self::CombatRuntime(error) => write!(f, "{error}"),
            Self::CombatWait(error) => write!(f, "{error}"),
            Self::PendingResponse(error) => write!(f, "{error}"),
            Self::Lookup(error) => write!(f, "{error}"),
            Self::SessionMemory(error) => write!(f, "{error}"),
            Self::StaticData(error) => write!(f, "{error}"),
            Self::SpiritOperation(error) => write!(f, "{error}"),
            Self::System(error) => write!(f, "{error}"),
            Self::ActivityOperation(error) => write!(f, "{error}"),
            Self::Bridge(error) => write!(f, "{error}"),
            Self::Response(error) => write!(f, "{error}"),
            Self::Request(error) => write!(f, "{error}"),
            Self::Unsupported(error) => write!(f, "{error}"),
        }
    }
}

impl RocoStdLibError {
    pub const fn code(&self) -> &'static str {
        match self {
            Self::FunctionContext(_) => "stdlib.function_context",
            Self::Query(_) => "stdlib.query",
            Self::CombatAction(_) => "stdlib.combat_action",
            Self::CombatRuntime(_) => "stdlib.combat_runtime",
            Self::CombatWait(_) => "stdlib.combat_wait",
            Self::PendingResponse(_) => "stdlib.pending_response",
            Self::Lookup(_) => "stdlib.lookup",
            Self::SessionMemory(_) => "stdlib.session_memory",
            Self::StaticData(_) => "stdlib.static_data",
            Self::SpiritOperation(_) => "stdlib.spirit_operation",
            Self::System(_) => "stdlib.system",
            Self::ActivityOperation(_) => "stdlib.activity_operation",
            Self::Bridge(_) => "stdlib.bridge",
            Self::Response(_) => "stdlib.response",
            Self::Request(_) => "stdlib.request",
            Self::Unsupported(error) => error.code(),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ScriptFunctionContextError {
    RequiresActiveCombat,
    NoCombatAvailable,
    CannotWaitForCombat { phase: ScriptCombatPhase },
    RequiresOutOfCombat,
}

impl ScriptFunctionContextError {
    pub const fn kind_code(&self) -> &'static str {
        match self {
            Self::RequiresActiveCombat => "requires_active_combat",
            Self::NoCombatAvailable => "no_combat_available",
            Self::CannotWaitForCombat { .. } => "cannot_wait_for_combat",
            Self::RequiresOutOfCombat => "requires_out_of_combat",
        }
    }

    pub fn combat_phase_code(&self) -> String {
        match self {
            Self::CannotWaitForCombat { phase } => phase.code().to_string(),
            _ => String::new(),
        }
    }
}

impl fmt::Display for ScriptFunctionContextError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::RequiresActiveCombat => f.write_str("This function can only be used in combat"),
            Self::NoCombatAvailable => f.write_str("No combat is available to wait for"),
            Self::CannotWaitForCombat { phase } => {
                write!(f, "Cannot wait for combat in phase {}", phase.as_str())
            }
            Self::RequiresOutOfCombat => f.write_str("This function cannot be used during combat"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ScriptCombatPhase {
    Idle,
    WaitingStartReply,
    WaitingLocalReady,
    WaitingPeerReady,
    PlayingOpening,
    WaitingPlayerAction,
    WaitingRoundResult,
    PlayingRoundResult,
    WaitingRoundRelease,
    WaitingMyExtraSwitch,
    WaitingOpponentExtraSwitch,
    Finished,
    Aborted,
    Unknown,
}

impl ScriptCombatPhase {
    pub const fn code(self) -> &'static str {
        match self {
            Self::Idle => "idle",
            Self::WaitingStartReply => "waiting_start_reply",
            Self::WaitingLocalReady => "waiting_local_ready",
            Self::WaitingPeerReady => "waiting_peer_ready",
            Self::PlayingOpening => "playing_opening",
            Self::WaitingPlayerAction => "waiting_player_action",
            Self::WaitingRoundResult => "waiting_round_result",
            Self::PlayingRoundResult => "playing_round_result",
            Self::WaitingRoundRelease => "waiting_round_release",
            Self::WaitingMyExtraSwitch => "waiting_my_extra_switch",
            Self::WaitingOpponentExtraSwitch => "waiting_opponent_extra_switch",
            Self::Finished => "finished",
            Self::Aborted => "aborted",
            Self::Unknown => "unknown",
        }
    }

    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Idle => "Idle",
            Self::WaitingStartReply => "WaitingStartReply",
            Self::WaitingLocalReady => "WaitingLocalReady",
            Self::WaitingPeerReady => "WaitingPeerReady",
            Self::PlayingOpening => "PlayingOpening",
            Self::WaitingPlayerAction => "WaitingPlayerAction",
            Self::WaitingRoundResult => "WaitingRoundResult",
            Self::PlayingRoundResult => "PlayingRoundResult",
            Self::WaitingRoundRelease => "WaitingRoundRelease",
            Self::WaitingMyExtraSwitch => "WaitingMyExtraSwitch",
            Self::WaitingOpponentExtraSwitch => "WaitingOpponentExtraSwitch",
            Self::Finished => "Finished",
            Self::Aborted => "Aborted",
            Self::Unknown => "Unknown",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ScriptQueryError {
    NotInAnyScene,
    SceneSpiritSnapshotUnavailable,
    CurrentRoleSnapshotUnavailable,
    BattleResultBeforeCombatStarts,
    BattleResultBeforeCombatFinishes,
    NotInCombat,
    InvalidActiveSpiritIndex,
    InvalidActiveRivalSpiritIndex,
    NoActiveSpirit,
    NoActiveRivalSpirit,
    NoSkillAtIndex { index: u32 },
}

impl ScriptQueryError {
    pub const fn kind_code(&self) -> &'static str {
        match self {
            Self::NotInAnyScene => "not_in_any_scene",
            Self::SceneSpiritSnapshotUnavailable => "scene_spirit_snapshot_unavailable",
            Self::CurrentRoleSnapshotUnavailable => "current_role_snapshot_unavailable",
            Self::BattleResultBeforeCombatStarts => "battle_result_before_combat_starts",
            Self::BattleResultBeforeCombatFinishes => "battle_result_before_combat_finishes",
            Self::NotInCombat => "not_in_combat",
            Self::InvalidActiveSpiritIndex => "invalid_active_spirit_index",
            Self::InvalidActiveRivalSpiritIndex => "invalid_active_rival_spirit_index",
            Self::NoActiveSpirit => "no_active_spirit",
            Self::NoActiveRivalSpirit => "no_active_rival_spirit",
            Self::NoSkillAtIndex { .. } => "no_skill_at_index",
        }
    }

    pub const fn skill_index(&self) -> i64 {
        match self {
            Self::NoSkillAtIndex { index } => *index as i64,
            _ => -1,
        }
    }
}

impl fmt::Display for ScriptQueryError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::NotInAnyScene => f.write_str("Not in any scene"),
            Self::SceneSpiritSnapshotUnavailable => {
                f.write_str("scene spirit snapshot unavailable")
            }
            Self::CurrentRoleSnapshotUnavailable => {
                f.write_str("current role snapshot unavailable")
            }
            Self::BattleResultBeforeCombatStarts => {
                f.write_str("battle result is unavailable before combat starts")
            }
            Self::BattleResultBeforeCombatFinishes => {
                f.write_str("battle result is unavailable before combat finishes")
            }
            Self::NotInCombat => f.write_str("Not in combat"),
            Self::InvalidActiveSpiritIndex => f.write_str("Invalid active spirit index"),
            Self::InvalidActiveRivalSpiritIndex => f.write_str("Invalid active rival spirit index"),
            Self::NoActiveSpirit => f.write_str("No active spirit"),
            Self::NoActiveRivalSpirit => f.write_str("No active rival spirit"),
            Self::NoSkillAtIndex { index } => write!(f, "No skill at index {index}"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ScriptCombatActionError {
    NotInCombat,
    InvalidActiveSpiritIndex { position: u8 },
    CannotUseSkill,
    SkillUnavailableOrNoPp { skill_id: u32 },
    CannotUseItem,
    CombatItemUnavailable { item_id: u32 },
    CannotChangeSpirit,
    CannotEscape,
    SpiritIndexOutOfRange { position: u32 },
    InvalidTargetSpiritIndex { position: u8 },
    TargetSpiritFainted { position: u8 },
    TargetSpiritNotFound { position: u8 },
}

impl ScriptCombatActionError {
    pub const fn kind_code(&self) -> &'static str {
        match self {
            Self::NotInCombat => "not_in_combat",
            Self::InvalidActiveSpiritIndex { .. } => "invalid_active_spirit_index",
            Self::CannotUseSkill => "cannot_use_skill",
            Self::SkillUnavailableOrNoPp { .. } => "skill_unavailable_or_no_pp",
            Self::CannotUseItem => "cannot_use_item",
            Self::CombatItemUnavailable { .. } => "combat_item_unavailable",
            Self::CannotChangeSpirit => "cannot_change_spirit",
            Self::CannotEscape => "cannot_escape",
            Self::SpiritIndexOutOfRange { .. } => "spirit_index_out_of_range",
            Self::InvalidTargetSpiritIndex { .. } => "invalid_target_spirit_index",
            Self::TargetSpiritFainted { .. } => "target_spirit_fainted",
            Self::TargetSpiritNotFound { .. } => "target_spirit_not_found",
        }
    }

    pub const fn position(&self) -> i64 {
        match self {
            Self::InvalidActiveSpiritIndex { position }
            | Self::InvalidTargetSpiritIndex { position }
            | Self::TargetSpiritFainted { position }
            | Self::TargetSpiritNotFound { position } => *position as i64,
            Self::SpiritIndexOutOfRange { position } => *position as i64,
            _ => -1,
        }
    }

    pub const fn skill_id(&self) -> i64 {
        match self {
            Self::SkillUnavailableOrNoPp { skill_id } => *skill_id as i64,
            _ => -1,
        }
    }

    pub const fn item_id(&self) -> i64 {
        match self {
            Self::CombatItemUnavailable { item_id } => *item_id as i64,
            _ => -1,
        }
    }
}

impl fmt::Display for ScriptCombatActionError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::NotInCombat => f.write_str("Not in combat"),
            Self::InvalidActiveSpiritIndex { position } => {
                write!(f, "Invalid active spirit index: {position}")
            }
            Self::CannotUseSkill => f.write_str("cannot use skill in current combat state"),
            Self::SkillUnavailableOrNoPp { skill_id } => {
                write!(f, "skill unavailable or no PP: skill_id={skill_id}")
            }
            Self::CannotUseItem => f.write_str("cannot use item in current combat state"),
            Self::CombatItemUnavailable { item_id } => {
                write!(f, "combat item unavailable: item_id={item_id}")
            }
            Self::CannotChangeSpirit => f.write_str("cannot change spirit in current combat state"),
            Self::CannotEscape => f.write_str("cannot escape in current combat state"),
            Self::SpiritIndexOutOfRange { position } => {
                write!(f, "spirit_index out of range: {position}")
            }
            Self::InvalidTargetSpiritIndex { position } => {
                write!(f, "invalid target spirit index: {position}")
            }
            Self::TargetSpiritFainted { position } => {
                write!(f, "target spirit is fainted: {position}")
            }
            Self::TargetSpiritNotFound { position } => {
                write!(f, "no spirit at target index: {position}")
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ScriptCombatRuntimeError {
    AutoFinishPresentationFailed {
        kind: ScriptCombatCommandFailureKind,
    },
    MarkFrontendLoadedFailed {
        kind: ScriptCombatCommandFailureKind,
    },
    Backend {
        kind: ScriptBackendCombatRuntimeErrorKind,
    },
}

impl fmt::Display for ScriptCombatRuntimeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.message())
    }
}

impl ScriptCombatRuntimeError {
    pub fn message(&self) -> String {
        match self {
            Self::AutoFinishPresentationFailed { kind } => {
                format!(
                    "failed to auto-finish script combat presentation after {}",
                    kind.as_str()
                )
            }
            Self::MarkFrontendLoadedFailed { kind } => {
                format!(
                    "failed to mark script combat frontend loaded after {}",
                    kind.as_str()
                )
            }
            Self::Backend { kind } => {
                format!("combat runtime {}", kind.as_str())
            }
        }
    }

    pub fn command_failure_kind(&self) -> Option<ScriptCombatCommandFailureKind> {
        match self {
            Self::AutoFinishPresentationFailed { kind }
            | Self::MarkFrontendLoadedFailed { kind } => Some(*kind),
            Self::Backend { .. } => None,
        }
    }

    pub fn command_failure_kind_code(&self) -> String {
        self.command_failure_kind()
            .map(|kind| kind.code().to_string())
            .unwrap_or_default()
    }

    pub fn backend_kind(&self) -> Option<ScriptBackendCombatRuntimeErrorKind> {
        match self {
            Self::Backend { kind } => Some(*kind),
            Self::AutoFinishPresentationFailed { .. } | Self::MarkFrontendLoadedFailed { .. } => {
                None
            }
        }
    }

    pub fn backend_kind_code(&self) -> String {
        self.backend_kind()
            .map(|kind| kind.code().to_string())
            .unwrap_or_default()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ScriptCombatCommandFailureKind {
    SessionNotStarted,
    OfflineUnsupported,
    CombatRuntime,
    CombatActionValidation,
    CombatEffects,
    ScriptSystem,
    Workflow,
    Wpe,
    LoginContext,
    LoginTarget,
    NetCommand,
    LineupSkill,
    InvalidUin,
    SystemClock,
    NetBridge,
    HttpBridge,
    SnapshotUnavailable,
    RuntimeStopped,
    ReplyDropped,
}

impl ScriptCombatCommandFailureKind {
    pub const fn code(self) -> &'static str {
        match self {
            Self::SessionNotStarted => "session_not_started",
            Self::OfflineUnsupported => "offline_unsupported",
            Self::CombatRuntime => "combat_runtime",
            Self::CombatActionValidation => "combat_action_validation",
            Self::CombatEffects => "combat_effects",
            Self::ScriptSystem => "script_system",
            Self::Workflow => "workflow",
            Self::Wpe => "wpe",
            Self::LoginContext => "login_context",
            Self::LoginTarget => "login_target",
            Self::NetCommand => "net_command",
            Self::LineupSkill => "lineup_skill",
            Self::InvalidUin => "invalid_uin",
            Self::SystemClock => "system_clock",
            Self::NetBridge => "net_bridge",
            Self::HttpBridge => "http_bridge",
            Self::SnapshotUnavailable => "snapshot_unavailable",
            Self::RuntimeStopped => "runtime_stopped",
            Self::ReplyDropped => "reply_dropped",
        }
    }

    pub const fn as_str(self) -> &'static str {
        match self {
            Self::SessionNotStarted => "session not started",
            Self::OfflineUnsupported => "offline unsupported",
            Self::CombatRuntime => "combat runtime",
            Self::CombatActionValidation => "combat action validation",
            Self::CombatEffects => "combat effects",
            Self::ScriptSystem => "script system",
            Self::Workflow => "workflow",
            Self::Wpe => "WPE",
            Self::LoginContext => "login context",
            Self::LoginTarget => "login target",
            Self::NetCommand => "net command",
            Self::LineupSkill => "lineup skill",
            Self::InvalidUin => "invalid uin",
            Self::SystemClock => "system clock",
            Self::NetBridge => "net bridge",
            Self::HttpBridge => "http bridge",
            Self::SnapshotUnavailable => "snapshot unavailable",
            Self::RuntimeStopped => "runtime stopped",
            Self::ReplyDropped => "reply dropped",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ScriptBackendCombatRuntimeErrorKind {
    BattleFacts,
    SideResolution,
    InvalidReadyPhasePayload,
    MissingStartContext,
    InvalidPhase,
    MissingStartSummary,
    MissingSideRegistry,
    MissingHistoryRecorder,
    MissingBattleFactsForHistorySnapshot,
    MissingObservedInitialStateForRoundHistory,
    MissingRoundBarrierForPresentation,
    HistoryRecorder,
    HistoryProjection,
    ChangeSpiritOwnerWithoutBattleFacts,
}

impl ScriptBackendCombatRuntimeErrorKind {
    pub const fn code(self) -> &'static str {
        match self {
            Self::BattleFacts => "battle_facts",
            Self::SideResolution => "side_resolution",
            Self::InvalidReadyPhasePayload => "invalid_ready_phase_payload",
            Self::MissingStartContext => "missing_start_context",
            Self::InvalidPhase => "invalid_phase",
            Self::MissingStartSummary => "missing_start_summary",
            Self::MissingSideRegistry => "missing_side_registry",
            Self::MissingHistoryRecorder => "missing_history_recorder",
            Self::MissingBattleFactsForHistorySnapshot => {
                "missing_battle_facts_for_history_snapshot"
            }
            Self::MissingObservedInitialStateForRoundHistory => {
                "missing_observed_initial_state_for_round_history"
            }
            Self::MissingRoundBarrierForPresentation => "missing_round_barrier_for_presentation",
            Self::HistoryRecorder => "history_recorder",
            Self::HistoryProjection => "history_projection",
            Self::ChangeSpiritOwnerWithoutBattleFacts => "change_spirit_owner_without_battle_facts",
        }
    }

    pub const fn as_str(self) -> &'static str {
        match self {
            Self::BattleFacts => "battle facts",
            Self::SideResolution => "side resolution",
            Self::InvalidReadyPhasePayload => "invalid ready phase payload",
            Self::MissingStartContext => "missing start context",
            Self::InvalidPhase => "invalid phase",
            Self::MissingStartSummary => "missing start summary",
            Self::MissingSideRegistry => "missing side registry",
            Self::MissingHistoryRecorder => "missing history recorder",
            Self::MissingBattleFactsForHistorySnapshot => {
                "missing battle facts for history snapshot"
            }
            Self::MissingObservedInitialStateForRoundHistory => {
                "missing observed initial state for round history"
            }
            Self::MissingRoundBarrierForPresentation => "missing round barrier for presentation",
            Self::HistoryRecorder => "history recorder",
            Self::HistoryProjection => "history projection",
            Self::ChangeSpiritOwnerWithoutBattleFacts => "change spirit owner without battle facts",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ScriptCombatWaitError {
    TimeoutWaitingCombatAction {
        phase: ScriptCombatPhase,
        elapsed_ms: u128,
    },
}

impl ScriptCombatWaitError {
    pub const fn kind_code(&self) -> &'static str {
        match self {
            Self::TimeoutWaitingCombatAction { .. } => "timeout_waiting_combat_action",
        }
    }

    pub fn combat_phase_code(&self) -> String {
        match self {
            Self::TimeoutWaitingCombatAction { phase, .. } => phase.code().to_string(),
        }
    }

    pub const fn elapsed_ms(&self) -> i64 {
        match self {
            Self::TimeoutWaitingCombatAction { elapsed_ms, .. } => {
                if *elapsed_ms > i64::MAX as u128 {
                    i64::MAX
                } else {
                    *elapsed_ms as i64
                }
            }
        }
    }
}

impl fmt::Display for ScriptCombatWaitError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::TimeoutWaitingCombatAction { phase, elapsed_ms } => write!(
                f,
                "timeout waiting for combat action, phase {}, elapsed_ms={elapsed_ms}",
                phase.as_str()
            ),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ScriptSessionMemoryError {
    TypeMismatch {
        key: String,
        expected: ScriptSessionValueKind,
        actual: ScriptSessionValueKind,
    },
}

impl fmt::Display for ScriptSessionMemoryError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::TypeMismatch {
                key,
                expected,
                actual,
            } => write!(
                f,
                "session key '{key}' has different type: expected {}, got {}",
                expected.as_str(),
                actual.as_str()
            ),
        }
    }
}

impl ScriptSessionMemoryError {
    pub const fn kind_code(&self) -> &'static str {
        match self {
            Self::TypeMismatch { .. } => "type_mismatch",
        }
    }

    pub fn key(&self) -> String {
        match self {
            Self::TypeMismatch { key, .. } => key.clone(),
        }
    }

    pub fn expected_kind_code(&self) -> String {
        match self {
            Self::TypeMismatch { expected, .. } => expected.as_str().to_string(),
        }
    }

    pub fn actual_kind_code(&self) -> String {
        match self {
            Self::TypeMismatch { actual, .. } => actual.as_str().to_string(),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ScriptSessionValueKind {
    Integer,
    String,
    Bool,
}

impl ScriptSessionValueKind {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Integer => "integer",
            Self::String => "string",
            Self::Bool => "bool",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ScriptLookupError {
    NotFound {
        entity: ScriptLookupEntity,
        key: String,
    },
}

impl ScriptLookupError {
    pub const fn kind_code(&self) -> &'static str {
        match self {
            Self::NotFound { .. } => "not_found",
        }
    }

    pub fn entity_code(&self) -> String {
        match self {
            Self::NotFound { entity, .. } => entity.code().to_string(),
        }
    }

    pub fn key(&self) -> String {
        match self {
            Self::NotFound { key, .. } => key.clone(),
        }
    }
}

impl fmt::Display for ScriptLookupError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::NotFound { entity, key } => {
                write!(f, "{} not found: {key}", entity.label())
            }
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ScriptLookupEntity {
    ItemInfo,
    StriveItemInfo,
    GuardianPetPropertyInfo,
    TitleInfo,
    MagicInfo,
    PluginInfo,
    SpiritBook,
    TalentInfo,
    SkillInfo,
    SpiritInfo,
}

impl ScriptLookupEntity {
    pub const fn code(self) -> &'static str {
        match self {
            Self::ItemInfo => "item_info",
            Self::StriveItemInfo => "strive_item_info",
            Self::GuardianPetPropertyInfo => "guardian_pet_property_info",
            Self::TitleInfo => "title_info",
            Self::MagicInfo => "magic_info",
            Self::PluginInfo => "plugin_info",
            Self::SpiritBook => "spirit_book",
            Self::TalentInfo => "talent_info",
            Self::SkillInfo => "skill_info",
            Self::SpiritInfo => "spirit_info",
        }
    }

    pub const fn label(self) -> &'static str {
        match self {
            Self::ItemInfo => "item info",
            Self::StriveItemInfo => "strive item info",
            Self::GuardianPetPropertyInfo => "guardian pet property info",
            Self::TitleInfo => "title info",
            Self::MagicInfo => "magic info",
            Self::PluginInfo => "plugin info",
            Self::SpiritBook => "spirit book",
            Self::TalentInfo => "talent info",
            Self::SkillInfo => "skill info",
            Self::SpiritInfo => "spirit info",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ScriptStaticDataError {
    BagSpiritNotFound {
        position: u8,
    },
    StaticGameDataNotInitialized,
    ActiveConfigNotAvailable {
        source: ScriptActiveConfigUnavailableSource,
        message: String,
    },
    NotImplemented {
        function_name: String,
    },
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ScriptActiveConfigUnavailableSource {
    Download,
    OptionalResourceParser,
    Other { code: String },
}

impl ScriptActiveConfigUnavailableSource {
    pub fn from_error_code(code: impl Into<String>) -> Self {
        match code.into().as_str() {
            "static_data.download" => Self::Download,
            "static_data.optional_resource_parser" => Self::OptionalResourceParser,
            code => Self::Other {
                code: code.to_string(),
            },
        }
    }

    pub fn code(&self) -> &str {
        match self {
            Self::Download => "static_data.download",
            Self::OptionalResourceParser => "static_data.optional_resource_parser",
            Self::Other { code } => code.as_str(),
        }
    }
}

impl ScriptStaticDataError {
    pub const fn kind_code(&self) -> &'static str {
        match self {
            Self::BagSpiritNotFound { .. } => "bag_spirit_not_found",
            Self::StaticGameDataNotInitialized => "static_game_data_not_initialized",
            Self::ActiveConfigNotAvailable { .. } => "active_config_not_available",
            Self::NotImplemented { .. } => "not_implemented",
        }
    }

    pub const fn position(&self) -> i64 {
        match self {
            Self::BagSpiritNotFound { position } => *position as i64,
            _ => -1,
        }
    }

    pub fn function_name(&self) -> String {
        match self {
            Self::NotImplemented { function_name } => function_name.clone(),
            _ => String::new(),
        }
    }

    pub fn message(&self) -> String {
        match self {
            Self::ActiveConfigNotAvailable { message, .. } => message.clone(),
            _ => String::new(),
        }
    }

    pub fn active_config_source_code(&self) -> String {
        match self {
            Self::ActiveConfigNotAvailable { source, .. } => source.code().to_string(),
            _ => String::new(),
        }
    }
}

impl fmt::Display for ScriptStaticDataError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::BagSpiritNotFound { position } => {
                write!(f, "bag spirit not found at position {position}")
            }
            Self::StaticGameDataNotInitialized => f.write_str("static game data not initialized"),
            Self::ActiveConfigNotAvailable { message, .. } => {
                write!(f, "active config not available: {message}")
            }
            Self::NotImplemented { function_name } => {
                write!(f, "{function_name} not yet implemented")
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ScriptSpiritOperationError {
    RecoverAllRequiresVip,
    StorageSpiritNotFound { spirit_id: u32, catch_time: u32 },
    MultipleStorageSpiritsMatch { spirit_id: u32 },
    StorageSpiritCatchTimeOutOfRange { catch_time: i64 },
}

impl ScriptSpiritOperationError {
    pub const fn kind_code(&self) -> &'static str {
        match self {
            Self::RecoverAllRequiresVip => "recover_all_requires_vip",
            Self::StorageSpiritNotFound { .. } => "storage_spirit_not_found",
            Self::MultipleStorageSpiritsMatch { .. } => "multiple_storage_spirits_match",
            Self::StorageSpiritCatchTimeOutOfRange { .. } => {
                "storage_spirit_catch_time_out_of_range"
            }
        }
    }

    pub const fn spirit_id(&self) -> i64 {
        match self {
            Self::StorageSpiritNotFound { spirit_id, .. }
            | Self::MultipleStorageSpiritsMatch { spirit_id } => *spirit_id as i64,
            _ => -1,
        }
    }

    pub const fn catch_time(&self) -> i64 {
        match self {
            Self::StorageSpiritNotFound { catch_time, .. } => *catch_time as i64,
            Self::StorageSpiritCatchTimeOutOfRange { catch_time } => *catch_time,
            _ => -1,
        }
    }
}

impl fmt::Display for ScriptSpiritOperationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::RecoverAllRequiresVip => f.write_str("recover_all_spirits requires VIP"),
            Self::StorageSpiritNotFound {
                spirit_id,
                catch_time,
            } => write!(
                f,
                "storage spirit not found: spirit_id={spirit_id} catch_time={catch_time}"
            ),
            Self::MultipleStorageSpiritsMatch { spirit_id } => write!(
                f,
                "multiple storage spirits match spirit_id={spirit_id}; pass catch_time to disambiguate"
            ),
            Self::StorageSpiritCatchTimeOutOfRange { catch_time } => {
                write!(f, "storage spirit catch_time out of u32 range: {catch_time}")
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ScriptSystemError {
    CurrentTimeBeforeUnixEpoch { failure: ScriptSystemFailure },
    CurrentTimestampExceedsI64,
    BuildTimeOffsetFailed { failure: ScriptSystemFailure },
    ParseTimeFormatFailed { failure: ScriptSystemFailure },
    FormatTimestampFailed { failure: ScriptSystemFailure },
    SendLogEventFailed { failure: ScriptSystemFailure },
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ScriptSystemFailure {
    pub operation: ScriptSystemOperation,
    pub source: ScriptSystemFailureSource,
}

impl ScriptSystemFailure {
    pub const fn new(operation: ScriptSystemOperation, source: ScriptSystemFailureSource) -> Self {
        Self { operation, source }
    }

    pub fn external(operation: ScriptSystemOperation, message: impl Into<String>) -> Self {
        Self::new(
            operation,
            ScriptSystemFailureSource::External {
                message: message.into(),
            },
        )
    }

    pub fn operation_code(&self) -> String {
        self.operation.code().to_string()
    }

    pub fn source_code(&self) -> String {
        self.source.code().to_string()
    }

    pub fn message(&self) -> String {
        self.source.message()
    }

    pub fn description(&self) -> String {
        format!("{} failed: {}", self.operation, self.message())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ScriptSystemFailureSource {
    SystemTimeBeforeUnixEpoch {
        seconds: u64,
        nanos: u32,
    },
    TimeComponentRange {
        component: String,
        conditional_range: bool,
        message: String,
    },
    TimeFormatDescription {
        message: String,
    },
    TimeFormat {
        message: String,
    },
    SendLogEvent {
        message: String,
    },
    External {
        message: String,
    },
}

impl ScriptSystemFailureSource {
    pub const fn code(&self) -> &'static str {
        match self {
            Self::SystemTimeBeforeUnixEpoch { .. } => "system_time_before_unix_epoch",
            Self::TimeComponentRange { .. } => "time_component_range",
            Self::TimeFormatDescription { .. } => "time_format_description",
            Self::TimeFormat { .. } => "time_format",
            Self::SendLogEvent { .. } => "send_log_event",
            Self::External { .. } => "external",
        }
    }

    pub fn message(&self) -> String {
        match self {
            Self::SystemTimeBeforeUnixEpoch { seconds, nanos } => {
                format!("system time is {seconds}s {nanos}ns before unix epoch")
            }
            Self::TimeComponentRange {
                component,
                conditional_range,
                message,
            } => format!("{component} component out of range: {message}; conditional_range={conditional_range}"),
            Self::TimeFormatDescription { message }
            | Self::TimeFormat { message }
            | Self::SendLogEvent { message }
            | Self::External { message } => message.clone(),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ScriptSystemOperation {
    CurrentTimeBeforeUnixEpoch,
    BuildTimeOffset,
    ParseTimeFormat,
    FormatTimestamp,
    SendLogEvent,
}

impl ScriptSystemOperation {
    pub const fn code(&self) -> &'static str {
        match self {
            Self::CurrentTimeBeforeUnixEpoch => "current_time_before_unix_epoch",
            Self::BuildTimeOffset => "build_time_offset",
            Self::ParseTimeFormat => "parse_time_format",
            Self::FormatTimestamp => "format_timestamp",
            Self::SendLogEvent => "send_log_event",
        }
    }
}

impl fmt::Display for ScriptSystemOperation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.code())
    }
}

impl ScriptSystemError {
    pub fn failure(&self) -> Option<ScriptSystemFailure> {
        match self {
            Self::CurrentTimeBeforeUnixEpoch { failure }
            | Self::BuildTimeOffsetFailed { failure }
            | Self::ParseTimeFormatFailed { failure }
            | Self::FormatTimestampFailed { failure }
            | Self::SendLogEventFailed { failure } => Some(failure.clone()),
            Self::CurrentTimestampExceedsI64 => None,
        }
    }
}

impl fmt::Display for ScriptSystemError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::CurrentTimeBeforeUnixEpoch { failure } => {
                write!(f, "current time before unix epoch: {}", failure.message())
            }
            Self::CurrentTimestampExceedsI64 => f.write_str("current timestamp exceeds i64 range"),
            Self::BuildTimeOffsetFailed { failure } => {
                write!(f, "build UTC+8 offset: {}", failure.message())
            }
            Self::ParseTimeFormatFailed { failure } => {
                write!(f, "parse time format: {}", failure.message())
            }
            Self::FormatTimestampFailed { failure } => {
                write!(f, "format timestamp: {}", failure.message())
            }
            Self::SendLogEventFailed { failure } => {
                write!(f, "Failed to send log event: {}", failure.message())
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ScriptActivityOperationError {
    MysteryFusionMaterialCountExceedsLimit {
        count: usize,
        limit: usize,
    },
    SummonDrawCountInvalid {
        count: i64,
    },
    InvalidOption {
        activity: ScriptActivityName,
        field: ScriptActivityOptionField,
        value: u32,
    },
}

impl ScriptActivityOperationError {
    pub const fn kind_code(&self) -> &'static str {
        match self {
            Self::MysteryFusionMaterialCountExceedsLimit { .. } => {
                "mystery_fusion_material_count_exceeds_limit"
            }
            Self::SummonDrawCountInvalid { .. } => "summon_draw_count_invalid",
            Self::InvalidOption { .. } => "invalid_option",
        }
    }

    pub const fn activity_code(&self) -> &'static str {
        match self {
            Self::MysteryFusionMaterialCountExceedsLimit { .. } => "mystery_fusion",
            Self::SummonDrawCountInvalid { .. } => "summon",
            Self::InvalidOption { activity, .. } => activity.code(),
        }
    }

    pub const fn field_code(&self) -> &'static str {
        match self {
            Self::InvalidOption { field, .. } => field.code(),
            _ => "",
        }
    }

    pub const fn count(&self) -> i64 {
        match self {
            Self::MysteryFusionMaterialCountExceedsLimit { count, .. } => *count as i64,
            Self::SummonDrawCountInvalid { count } => *count,
            _ => -1,
        }
    }

    pub const fn limit(&self) -> i64 {
        match self {
            Self::MysteryFusionMaterialCountExceedsLimit { limit, .. } => *limit as i64,
            _ => -1,
        }
    }

    pub const fn value(&self) -> i64 {
        match self {
            Self::InvalidOption { value, .. } => *value as i64,
            _ => -1,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ScriptActivityName {
    Gemini,
}

impl ScriptActivityName {
    pub const fn code(self) -> &'static str {
        match self {
            Self::Gemini => "gemini",
        }
    }
}

impl fmt::Display for ScriptActivityName {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.code())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ScriptActivityOptionField {
    Kind,
    Side,
}

impl ScriptActivityOptionField {
    pub const fn code(self) -> &'static str {
        match self {
            Self::Kind => "kind",
            Self::Side => "side",
        }
    }
}

impl fmt::Display for ScriptActivityOptionField {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.code())
    }
}

impl fmt::Display for ScriptActivityOperationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::MysteryFusionMaterialCountExceedsLimit { count, limit } => write!(
                f,
                "mystery_fusion::fuse material count {count} exceeds AS limit {limit}"
            ),
            Self::SummonDrawCountInvalid { count } => {
                write!(f, "summon::draw draw_count must be 1 or 10, got {count}")
            }
            Self::InvalidOption {
                activity,
                field,
                value,
            } => write!(f, "{activity} invalid {field}: {value}"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ScriptBridgeError {
    SendRequestFailed { failure: ScriptBridgeFailure },
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ScriptBridgeFailure {
    pub operation: ScriptBridgeOperation,
    pub reason: ScriptBridgeFailureReason,
}

impl ScriptBridgeFailure {
    pub fn send_request(reason: ScriptBridgeFailureReason) -> Self {
        Self {
            operation: ScriptBridgeOperation::SendRequest,
            reason,
        }
    }

    pub fn operation_code(&self) -> String {
        self.operation.code().to_string()
    }

    pub fn reason_code(&self) -> String {
        self.reason.code().to_string()
    }

    pub fn message(&self) -> String {
        self.reason.message().to_string()
    }

    pub fn description(&self) -> String {
        format!("{} failed: {}", self.operation, self.message())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ScriptBridgeOperation {
    SendRequest,
}

impl ScriptBridgeOperation {
    pub const fn code(&self) -> &'static str {
        match self {
            Self::SendRequest => "send_request",
        }
    }
}

impl fmt::Display for ScriptBridgeOperation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.code())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ScriptBridgeFailureReason {
    RequestChannelClosed,
}

impl ScriptBridgeFailureReason {
    pub const fn code(&self) -> &'static str {
        match self {
            Self::RequestChannelClosed => "request_channel_closed",
        }
    }

    pub const fn message(&self) -> &'static str {
        match self {
            Self::RequestChannelClosed => "request channel closed",
        }
    }
}

impl fmt::Display for ScriptBridgeFailureReason {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.code())
    }
}

impl fmt::Display for ScriptBridgeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SendRequestFailed { failure } => {
                write!(f, "Failed to send request: {}", failure.message())
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ScriptResponseName {
    pub code: String,
}

impl ScriptResponseName {
    pub fn new(code: impl Into<String>) -> Self {
        Self { code: code.into() }
    }
}

impl fmt::Display for ScriptResponseName {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.code)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ScriptResponseError {
    TypeMismatch {
        expected: ScriptResponseName,
        actual: ScriptResponseName,
    },
}

impl fmt::Display for ScriptResponseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::TypeMismatch { expected, actual } => {
                write!(f, "Expected {expected} response, got {actual}")
            }
        }
    }
}

impl ScriptResponseError {
    pub const fn kind_code(&self) -> &'static str {
        match self {
            Self::TypeMismatch { .. } => "type_mismatch",
        }
    }

    pub fn expected_response_code(&self) -> String {
        match self {
            Self::TypeMismatch { expected, .. } => expected.code.clone(),
        }
    }

    pub fn actual_response_code(&self) -> String {
        match self {
            Self::TypeMismatch { actual, .. } => actual.code.clone(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ScriptRequestError {
    NoRunningScriptForRequest,
    NoRunningScriptForCombatCommand,
    PauseStateUnknown,
    EquipItemPositionMustBeOneBased,
    ClearLineupRequiresBatchSupport,
    PendingReplyRefreshFailed {
        context: ScriptWaitContext,
        kind: ScriptRequestSystemFailureKind,
    },
    WaitStateRejected {
        context: ScriptWaitContext,
        kind: ScriptRequestSystemFailureKind,
    },
    InvalidCombatIntent {
        kind: ScriptCombatIntentKind,
        spirit_index: u8,
        value: u32,
        error: ScriptCombatProtocolError,
    },
    InvalidCombatCommand {
        kind: ScriptCombatActionValidationKind,
    },
    UnsupportedCombatServerType {
        value: u8,
    },
    UnsupportedCombatType {
        value: u8,
    },
}

impl ScriptRequestError {
    pub const fn kind_code(&self) -> &'static str {
        match self {
            Self::NoRunningScriptForRequest => "no_running_script_for_request",
            Self::NoRunningScriptForCombatCommand => "no_running_script_for_combat_command",
            Self::PauseStateUnknown => "pause_state_unknown",
            Self::EquipItemPositionMustBeOneBased => "equip_item_position_must_be_one_based",
            Self::ClearLineupRequiresBatchSupport => "clear_lineup_requires_batch_support",
            Self::PendingReplyRefreshFailed { .. } => "pending_reply_refresh_failed",
            Self::WaitStateRejected { .. } => "wait_state_rejected",
            Self::InvalidCombatIntent { .. } => "invalid_combat_intent",
            Self::InvalidCombatCommand { .. } => "invalid_combat_command",
            Self::UnsupportedCombatServerType { .. } => "unsupported_combat_server_type",
            Self::UnsupportedCombatType { .. } => "unsupported_combat_type",
        }
    }

    pub const fn wait_context_code(&self) -> &'static str {
        match self {
            Self::PendingReplyRefreshFailed { context, .. }
            | Self::WaitStateRejected { context, .. } => context.code(),
            _ => "",
        }
    }

    pub const fn system_failure_kind_code(&self) -> &'static str {
        match self {
            Self::PendingReplyRefreshFailed { kind, .. } | Self::WaitStateRejected { kind, .. } => {
                kind.code()
            }
            _ => "",
        }
    }

    pub const fn combat_intent_kind_code(&self) -> &'static str {
        match self {
            Self::InvalidCombatIntent { kind, .. } => kind.code(),
            _ => "",
        }
    }

    pub const fn combat_validation_kind_code(&self) -> &'static str {
        match self {
            Self::InvalidCombatCommand { kind } => kind.code(),
            _ => "",
        }
    }

    pub const fn spirit_index(&self) -> i64 {
        match self {
            Self::InvalidCombatIntent { spirit_index, .. } => *spirit_index as i64,
            _ => -1,
        }
    }

    pub const fn value(&self) -> i64 {
        match self {
            Self::InvalidCombatIntent { value, .. } => *value as i64,
            Self::UnsupportedCombatServerType { value } | Self::UnsupportedCombatType { value } => {
                *value as i64
            }
            _ => -1,
        }
    }

    pub const fn combat_protocol_error_kind_code(&self) -> &'static str {
        match self {
            Self::InvalidCombatIntent { error, .. } => error.kind_code(),
            _ => "",
        }
    }

    pub const fn combat_protocol_error_value(&self) -> i64 {
        match self {
            Self::InvalidCombatIntent { error, .. } => error.value(),
            _ => -1,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ScriptCombatIntentKind {
    UseSkill,
    ChangeSpirit,
    UseItem,
    Escape,
}

impl ScriptCombatIntentKind {
    pub const fn code(self) -> &'static str {
        self.as_str()
    }

    pub const fn as_str(self) -> &'static str {
        match self {
            Self::UseSkill => "use_skill",
            Self::ChangeSpirit => "change_spirit",
            Self::UseItem => "use_item",
            Self::Escape => "escape",
        }
    }
}

impl fmt::Display for ScriptCombatIntentKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ScriptCombatProtocolError {
    InvalidSpiritPosition {
        value: u8,
    },
    TargetSpiritPositionOutOfRange {
        value: u32,
    },
    UnmappedAbnormalState {
        raw_id: u32,
    },
    PropertyStageOutOfRange {
        field: ScriptCombatProtocolPropertyStage,
        value: i16,
    },
    UnknownWeatherEffect {
        raw_weather: u8,
    },
    ParticipantDisplaySideUnresolved {
        role: ScriptCombatParticipantDisplayRole,
        id: u32,
        participant_type: u8,
        position: u8,
    },
    ChangeSpiritPositionOutOfRange {
        skill_id: u32,
    },
    InvalidChangeSpiritPosition {
        position: u8,
    },
}

impl ScriptCombatProtocolError {
    pub const fn kind_code(&self) -> &'static str {
        match self {
            Self::InvalidSpiritPosition { .. } => "invalid_spirit_position",
            Self::TargetSpiritPositionOutOfRange { .. } => "target_spirit_position_out_of_range",
            Self::UnmappedAbnormalState { .. } => "unmapped_abnormal_state",
            Self::PropertyStageOutOfRange { .. } => "property_stage_out_of_range",
            Self::UnknownWeatherEffect { .. } => "unknown_weather_effect",
            Self::ParticipantDisplaySideUnresolved { .. } => "participant_display_side_unresolved",
            Self::ChangeSpiritPositionOutOfRange { .. } => "change_spirit_position_out_of_range",
            Self::InvalidChangeSpiritPosition { .. } => "invalid_change_spirit_position",
        }
    }

    pub const fn value(&self) -> i64 {
        match self {
            Self::InvalidSpiritPosition { value } => *value as i64,
            Self::TargetSpiritPositionOutOfRange { value } => *value as i64,
            Self::UnmappedAbnormalState { raw_id } => *raw_id as i64,
            Self::PropertyStageOutOfRange { value, .. } => *value as i64,
            Self::UnknownWeatherEffect { raw_weather } => *raw_weather as i64,
            Self::ParticipantDisplaySideUnresolved { id, .. } => *id as i64,
            Self::ChangeSpiritPositionOutOfRange { skill_id } => *skill_id as i64,
            Self::InvalidChangeSpiritPosition { position } => *position as i64,
        }
    }

    pub const fn property_stage_code(&self) -> &'static str {
        match self {
            Self::PropertyStageOutOfRange { field, .. } => field.code(),
            _ => "",
        }
    }

    pub const fn participant_role_code(&self) -> &'static str {
        match self {
            Self::ParticipantDisplaySideUnresolved { role, .. } => role.code(),
            _ => "",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ScriptCombatProtocolPropertyStage {
    PhysicalAttack,
    PhysicalDefense,
    MagicAttack,
    MagicDefense,
    Speed,
    SpiritPower,
    DefensePierce,
    Critical,
}

impl ScriptCombatProtocolPropertyStage {
    pub const fn code(self) -> &'static str {
        self.as_str()
    }

    pub const fn as_str(self) -> &'static str {
        match self {
            Self::PhysicalAttack => "pa",
            Self::PhysicalDefense => "pd",
            Self::MagicAttack => "ma",
            Self::MagicDefense => "md",
            Self::Speed => "ve",
            Self::SpiritPower => "sp",
            Self::DefensePierce => "dp",
            Self::Critical => "crit",
        }
    }
}

impl fmt::Display for ScriptCombatProtocolPropertyStage {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ScriptCombatParticipantDisplayRole {
    Actor,
    Target,
}

impl ScriptCombatParticipantDisplayRole {
    pub const fn code(self) -> &'static str {
        self.as_str()
    }

    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Actor => "actor",
            Self::Target => "target",
        }
    }

    pub const fn identity_str(self) -> &'static str {
        match self {
            Self::Actor => "offense",
            Self::Target => "defense",
        }
    }
}

impl fmt::Display for ScriptCombatParticipantDisplayRole {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

impl fmt::Display for ScriptCombatProtocolError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidSpiritPosition { value } => {
                write!(f, "spirit position must be in 1..=6, got {value}")
            }
            Self::TargetSpiritPositionOutOfRange { value } => {
                write!(f, "target spirit position out of range: {value}")
            }
            Self::UnmappedAbnormalState { raw_id } => {
                write!(f, "unmapped combat abnormal state id: {raw_id}")
            }
            Self::PropertyStageOutOfRange { field, value } => {
                write!(f, "combat property stage {field} out of range: {value}")
            }
            Self::UnknownWeatherEffect { raw_weather } => {
                write!(f, "unknown combat weather effect: {raw_weather}")
            }
            Self::ParticipantDisplaySideUnresolved {
                role,
                id,
                participant_type,
                position,
            } => write!(
                f,
                "combat participant display {role} side unresolved {}_id={id} {}_type={participant_type} {}_index={position}",
                role.identity_str(),
                role.identity_str(),
                role.identity_str()
            ),
            Self::ChangeSpiritPositionOutOfRange { skill_id } => {
                write!(f, "combat change spirit position out of range: {skill_id}")
            }
            Self::InvalidChangeSpiritPosition { position } => {
                write!(f, "combat change spirit position must be in 1..=6, got {position}")
            }
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ScriptWaitContext {
    WaitNextCombatAction,
    CombatActionSettlingAfterAck,
    UseItemAction,
    TalentRefresh,
    TalentRefreshResultAfterAck,
}

impl ScriptWaitContext {
    pub const fn code(self) -> &'static str {
        self.as_str()
    }

    pub const fn as_str(self) -> &'static str {
        match self {
            Self::WaitNextCombatAction => "wait_next_combat_action",
            Self::CombatActionSettlingAfterAck => "combat_action_settling_after_ack",
            Self::UseItemAction => "use_item_action",
            Self::TalentRefresh => "talent_refresh",
            Self::TalentRefreshResultAfterAck => "talent_refresh_result_after_ack",
        }
    }
}

impl fmt::Display for ScriptWaitContext {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

impl fmt::Display for ScriptRequestError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::NoRunningScriptForRequest => {
                f.write_str("script request submitted while no script is running")
            }
            Self::NoRunningScriptForCombatCommand => {
                f.write_str("script combat command submitted while no script is running")
            }
            Self::PauseStateUnknown => f.write_str(
                "pause state is unknown; call game::try_set_pause or game::set_pause first",
            ),
            Self::EquipItemPositionMustBeOneBased => {
                f.write_str("equip_item position must be 1-based")
            }
            Self::ClearLineupRequiresBatchSupport => {
                f.write_str("clear_lineup requires confirmed batch request support")
            }
            Self::PendingReplyRefreshFailed { context, kind } => {
                write!(f, "{context} failed to refresh pending script reply: {kind}")
            }
            Self::WaitStateRejected { context, kind } => {
                write!(f, "{context} failed to set script wait state: {kind}")
            }
            Self::InvalidCombatIntent {
                kind,
                spirit_index,
                value,
                error,
            } => write!(
                f,
                "invalid combat intent kind={kind} spirit_index={spirit_index} value={value}: {error}"
            ),
            Self::InvalidCombatCommand { kind } => {
                write!(f, "invalid combat command: {kind}")
            }
            Self::UnsupportedCombatServerType { value } => {
                write!(f, "unsupported combat server_type: {value}")
            }
            Self::UnsupportedCombatType { value } => {
                write!(f, "unsupported combat_type: {value}")
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ScriptCombatActionValidationKind {
    CannotSubmitAction,
    BattleFactsUnavailable,
    InvalidActivePosition {
        position: u8,
    },
    SpiritPositionMismatch {
        active: u8,
        request: u8,
    },
    ActiveSpiritNotFound {
        position: u8,
    },
    ActiveSpiritFainted {
        position: u8,
    },
    ActionAvailabilityUnavailable,
    ActionUnavailable {
        action: ScriptCombatActionAvailabilityKind,
    },
    SkillUnavailableOrNoPp {
        skill_id: u32,
    },
    TargetSpiritAlreadyActive {
        position: u8,
    },
    TargetSpiritNotFound {
        position: u8,
    },
    TargetSpiritFainted {
        position: u8,
    },
    InvalidItemId {
        item_id: u32,
    },
}

impl ScriptCombatActionValidationKind {
    pub const fn code(&self) -> &'static str {
        match self {
            Self::CannotSubmitAction => "cannot_submit_action",
            Self::BattleFactsUnavailable => "battle_facts_unavailable",
            Self::InvalidActivePosition { .. } => "invalid_active_position",
            Self::SpiritPositionMismatch { .. } => "spirit_position_mismatch",
            Self::ActiveSpiritNotFound { .. } => "active_spirit_not_found",
            Self::ActiveSpiritFainted { .. } => "active_spirit_fainted",
            Self::ActionAvailabilityUnavailable => "action_availability_unavailable",
            Self::ActionUnavailable { .. } => "action_unavailable",
            Self::SkillUnavailableOrNoPp { .. } => "skill_unavailable_or_no_pp",
            Self::TargetSpiritAlreadyActive { .. } => "target_spirit_already_active",
            Self::TargetSpiritNotFound { .. } => "target_spirit_not_found",
            Self::TargetSpiritFainted { .. } => "target_spirit_fainted",
            Self::InvalidItemId { .. } => "invalid_item_id",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ScriptCombatActionAvailabilityKind {
    UseSkill,
    ChangeSpirit,
    UseItem,
    Escape,
}

impl ScriptCombatActionAvailabilityKind {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::UseSkill => "use skill",
            Self::ChangeSpirit => "change spirit",
            Self::UseItem => "use item",
            Self::Escape => "escape",
        }
    }
}

impl fmt::Display for ScriptCombatActionAvailabilityKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

impl fmt::Display for ScriptCombatActionValidationKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::CannotSubmitAction => {
                f.write_str("cannot submit combat action in current combat runtime phase")
            }
            Self::BattleFactsUnavailable => f.write_str("combat battle facts unavailable"),
            Self::InvalidActivePosition { position } => {
                write!(f, "invalid active spirit position: {position}")
            }
            Self::SpiritPositionMismatch { active, request } => write!(
                f,
                "combat action spirit_position mismatch: active={active}, request={request}"
            ),
            Self::ActiveSpiritNotFound { position } => {
                write!(f, "active spirit not found: {position}")
            }
            Self::ActiveSpiritFainted { position } => {
                write!(f, "active spirit is fainted: {position}")
            }
            Self::ActionAvailabilityUnavailable => {
                f.write_str("combat action availability unavailable")
            }
            Self::ActionUnavailable { action } => {
                write!(f, "cannot {action} in current combat facts")
            }
            Self::SkillUnavailableOrNoPp { skill_id } => {
                write!(f, "skill unavailable or no PP: skill_id={skill_id}")
            }
            Self::TargetSpiritAlreadyActive { position } => {
                write!(f, "target spirit is already active: {position}")
            }
            Self::TargetSpiritNotFound { position } => {
                write!(f, "no spirit at target position: {position}")
            }
            Self::TargetSpiritFainted { position } => {
                write!(f, "target spirit is fainted: {position}")
            }
            Self::InvalidItemId { item_id } => write!(f, "invalid combat item id: {item_id}"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ScriptRequestSystemFailureKind {
    DraftUpdateWhileRunning,
    ScriptAlreadyRunning,
    NoRunningScript,
    NoDebugScriptRunning,
    InactiveSession {
        session_id: u64,
        current_session_id: u64,
    },
    MissingPendingReply {
        serial_num: u32,
    },
    SendResponseFailed {
        payload_returned: bool,
    },
    SendDebugCommandFailed {
        payload_returned: bool,
    },
}

impl ScriptRequestSystemFailureKind {
    pub const fn code(&self) -> &'static str {
        match self {
            Self::DraftUpdateWhileRunning => "draft_update_while_running",
            Self::ScriptAlreadyRunning => "script_already_running",
            Self::NoRunningScript => "no_running_script",
            Self::NoDebugScriptRunning => "no_debug_script_running",
            Self::InactiveSession { .. } => "inactive_session",
            Self::MissingPendingReply { .. } => "missing_pending_reply",
            Self::SendResponseFailed { .. } => "send_response_failed",
            Self::SendDebugCommandFailed { .. } => "send_debug_command_failed",
        }
    }
}

impl fmt::Display for ScriptRequestSystemFailureKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::DraftUpdateWhileRunning => {
                f.write_str("cannot update script draft while script is running")
            }
            Self::ScriptAlreadyRunning => {
                f.write_str("script is running; stop it before running a new task")
            }
            Self::NoRunningScript => f.write_str("no script running"),
            Self::NoDebugScriptRunning => f.write_str("no debug script running"),
            Self::InactiveSession {
                session_id,
                current_session_id,
            } => write!(
                f,
                "inactive script session session_id={session_id} current_session_id={current_session_id}"
            ),
            Self::MissingPendingReply { serial_num } => {
                write!(f, "no pending script reply for serial_num={serial_num}")
            }
            Self::SendResponseFailed { payload_returned } => {
                write!(
                    f,
                    "failed to send runtime response (payload_returned={payload_returned})"
                )
            }
            Self::SendDebugCommandFailed { payload_returned } => {
                write!(
                    f,
                    "failed to send debug command (payload_returned={payload_returned})"
                )
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ScriptModuleName {
    AlchemyFurnace,
    Aquarius,
    Aries,
    Cancer,
    Capricorn,
    Combat,
    DarkCity,
    DiamondTear,
    FourSeasons,
    Game,
    Gemini,
    IceCrystal,
    Ladder,
    Leo,
    Libra,
    Lookup,
    MagicPioneer,
    Manor,
    MountainSea,
    MultiEvolution,
    MysteryFusion,
    News,
    NewsTimes,
    PetTraining,
    Pisces,
    PlayGuide,
    Profile,
    Role,
    Sagittarius,
    Scene,
    Scorpio,
    SentinelIntelligence,
    Spirit,
    SpiritBook,
    StarTower,
    Summon,
    System,
    Taurus,
    ThreeStarters,
    TreasureRealm,
    TypeLadder,
    Unicorn,
    Virgo,
    Custom { name: String },
}

impl ScriptModuleName {
    pub fn parse(name: &str) -> Self {
        match name {
            "alchemy_furnace" => Self::AlchemyFurnace,
            "aquarius" => Self::Aquarius,
            "aries" => Self::Aries,
            "cancer" => Self::Cancer,
            "capricorn" => Self::Capricorn,
            "combat" => Self::Combat,
            "dark_city" => Self::DarkCity,
            "diamond_tear" => Self::DiamondTear,
            "four_seasons" => Self::FourSeasons,
            "game" => Self::Game,
            "gemini" => Self::Gemini,
            "ice_crystal" => Self::IceCrystal,
            "ladder" => Self::Ladder,
            "leo" => Self::Leo,
            "libra" => Self::Libra,
            "lookup" => Self::Lookup,
            "magic_pioneer" => Self::MagicPioneer,
            "manor" => Self::Manor,
            "mountain_sea" => Self::MountainSea,
            "multi_evolution" => Self::MultiEvolution,
            "mystery_fusion" => Self::MysteryFusion,
            "news" => Self::News,
            "news_times" => Self::NewsTimes,
            "pet_training" => Self::PetTraining,
            "pisces" => Self::Pisces,
            "play_guide" => Self::PlayGuide,
            "profile" => Self::Profile,
            "role" => Self::Role,
            "sagittarius" => Self::Sagittarius,
            "scene" => Self::Scene,
            "scorpio" => Self::Scorpio,
            "sentinel_intelligence" => Self::SentinelIntelligence,
            "spirit" => Self::Spirit,
            "spirit_book" => Self::SpiritBook,
            "star_tower" => Self::StarTower,
            "summon" => Self::Summon,
            "system" => Self::System,
            "taurus" => Self::Taurus,
            "three_starters" => Self::ThreeStarters,
            "treasure_realm" => Self::TreasureRealm,
            "type_ladder" => Self::TypeLadder,
            "unicorn" => Self::Unicorn,
            "virgo" => Self::Virgo,
            _ => Self::Custom {
                name: name.to_string(),
            },
        }
    }

    pub fn code(&self) -> &str {
        match self {
            Self::AlchemyFurnace => "alchemy_furnace",
            Self::Aquarius => "aquarius",
            Self::Aries => "aries",
            Self::Cancer => "cancer",
            Self::Capricorn => "capricorn",
            Self::Combat => "combat",
            Self::DarkCity => "dark_city",
            Self::DiamondTear => "diamond_tear",
            Self::FourSeasons => "four_seasons",
            Self::Game => "game",
            Self::Gemini => "gemini",
            Self::IceCrystal => "ice_crystal",
            Self::Ladder => "ladder",
            Self::Leo => "leo",
            Self::Libra => "libra",
            Self::Lookup => "lookup",
            Self::MagicPioneer => "magic_pioneer",
            Self::Manor => "manor",
            Self::MountainSea => "mountain_sea",
            Self::MultiEvolution => "multi_evolution",
            Self::MysteryFusion => "mystery_fusion",
            Self::News => "news",
            Self::NewsTimes => "news_times",
            Self::PetTraining => "pet_training",
            Self::Pisces => "pisces",
            Self::PlayGuide => "play_guide",
            Self::Profile => "profile",
            Self::Role => "role",
            Self::Sagittarius => "sagittarius",
            Self::Scene => "scene",
            Self::Scorpio => "scorpio",
            Self::SentinelIntelligence => "sentinel_intelligence",
            Self::Spirit => "spirit",
            Self::SpiritBook => "spirit_book",
            Self::StarTower => "star_tower",
            Self::Summon => "summon",
            Self::System => "system",
            Self::Taurus => "taurus",
            Self::ThreeStarters => "three_starters",
            Self::TreasureRealm => "treasure_realm",
            Self::TypeLadder => "type_ladder",
            Self::Unicorn => "unicorn",
            Self::Virgo => "virgo",
            Self::Custom { name } => name.as_str(),
        }
    }
}

impl fmt::Display for ScriptModuleName {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.code())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ScriptFunctionName {
    pub module: ScriptModuleName,
    pub function: String,
}

impl ScriptFunctionName {
    pub fn parse(name: &str) -> Self {
        match name.split_once("::") {
            Some((module, function)) => Self {
                module: ScriptModuleName::parse(module),
                function: function.to_string(),
            },
            None => Self {
                module: ScriptModuleName::Custom {
                    name: String::new(),
                },
                function: name.to_string(),
            },
        }
    }

    pub fn module_code(&self) -> String {
        self.module.code().to_string()
    }

    pub fn qualified_name(&self) -> String {
        let module = self.module.code();
        if module.is_empty() {
            self.function.clone()
        } else {
            format!("{}::{}", module, self.function)
        }
    }
}

impl fmt::Display for ScriptFunctionName {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.qualified_name())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ScriptUnsupportedError {
    Function { name: ScriptFunctionName },
}

impl ScriptUnsupportedError {
    pub const fn code(&self) -> &'static str {
        match self {
            Self::Function { .. } => "stdlib.unsupported.function",
        }
    }
}

impl fmt::Display for ScriptUnsupportedError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Function { name } => write!(f, "{name} unsupported by this runtime"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ScriptHttpResponseName {
    pub code: String,
}

impl ScriptHttpResponseName {
    pub fn new(code: impl Into<String>) -> Self {
        Self { code: code.into() }
    }
}

impl fmt::Display for ScriptHttpResponseName {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.code)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ScriptPendingResponseError {
    ExternalGameExtraFieldInvalidInteger {
        key: String,
        value: String,
        expected: ScriptIntegerType,
    },
    UnexpectedHttpResponse {
        pending: ScriptHttpResponseName,
        expected: ScriptHttpResponseName,
        actual: ScriptHttpResponseName,
    },
    CombatLoadedUnexpectedPhase {
        phase: ScriptCombatPhase,
    },
    CombatActionAckMismatch {
        expected_action_kind: u8,
        expected_spirit_index: u8,
        actual_action_kind: u8,
        actual_spirit_index: u8,
    },
    SkillPoolIndexExceedsI64 {
        index: usize,
    },
    MissingNetResponsePayload {
        target: RocoNetResponseParseTarget,
    },
    MissingScriptSessionForActionWait,
    StorageSpiritDetailEmpty,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ScriptIntegerType {
    I64,
}

impl fmt::Display for ScriptIntegerType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::I64 => f.write_str("i64"),
        }
    }
}

impl fmt::Display for ScriptPendingResponseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::ExternalGameExtraFieldInvalidInteger {
                key,
                value,
                expected,
            } => write!(
                f,
                "external_game extra field {key}={value:?} is not a valid {expected}"
            ),
            Self::UnexpectedHttpResponse {
                pending,
                expected,
                actual,
            } => write!(
                f,
                "unexpected script http response for {pending}: expected {expected}, got {actual}"
            ),
            Self::CombatLoadedUnexpectedPhase { phase } => write!(
                f,
                "combat did not become ready after loaded ack, current phase {}",
                phase.as_str()
            ),
            Self::CombatActionAckMismatch {
                expected_action_kind,
                expected_spirit_index,
                actual_action_kind,
                actual_spirit_index,
            } => write!(
                f,
                "combat action ack mismatch: expected action_kind={expected_action_kind} spirit_index={expected_spirit_index}, got action_kind={actual_action_kind} spirit_index={actual_spirit_index}"
            ),
            Self::SkillPoolIndexExceedsI64 { index } => {
                write!(f, "skill pool index exceeds i64 range: {index}")
            }
            Self::MissingNetResponsePayload { target } => {
                write!(f, "network response {} is missing payload", target.label())
            }
            Self::MissingScriptSessionForActionWait => {
                f.write_str("script action wait requested while no script is running")
            }
            Self::StorageSpiritDetailEmpty => f.write_str("storage spirit detail is empty"),
        }
    }
}

impl ScriptPendingResponseError {
    pub const fn kind_code(&self) -> &'static str {
        match self {
            Self::ExternalGameExtraFieldInvalidInteger { .. } => {
                "external_game_extra_field_invalid_integer"
            }
            Self::UnexpectedHttpResponse { .. } => "unexpected_http_response",
            Self::CombatLoadedUnexpectedPhase { .. } => "combat_loaded_unexpected_phase",
            Self::CombatActionAckMismatch { .. } => "combat_action_ack_mismatch",
            Self::SkillPoolIndexExceedsI64 { .. } => "skill_pool_index_exceeds_i64",
            Self::MissingNetResponsePayload { .. } => "missing_net_response_payload",
            Self::MissingScriptSessionForActionWait => "missing_script_session_for_action_wait",
            Self::StorageSpiritDetailEmpty => "storage_spirit_detail_empty",
        }
    }

    pub fn pending_http_response_code(&self) -> String {
        match self {
            Self::UnexpectedHttpResponse { pending, .. } => pending.code.clone(),
            _ => String::new(),
        }
    }

    pub fn expected_http_response_code(&self) -> String {
        match self {
            Self::UnexpectedHttpResponse { expected, .. } => expected.code.clone(),
            _ => String::new(),
        }
    }

    pub fn actual_http_response_code(&self) -> String {
        match self {
            Self::UnexpectedHttpResponse { actual, .. } => actual.code.clone(),
            _ => String::new(),
        }
    }

    pub fn combat_phase_code(&self) -> String {
        match self {
            Self::CombatLoadedUnexpectedPhase { phase } => phase.as_str().to_string(),
            _ => String::new(),
        }
    }

    pub fn net_response_parse_target_code(&self) -> String {
        match self {
            Self::MissingNetResponsePayload { target } => target.code().to_string(),
            _ => String::new(),
        }
    }

    pub fn expected_action_kind(&self) -> i64 {
        match self {
            Self::CombatActionAckMismatch {
                expected_action_kind,
                ..
            } => i64::from(*expected_action_kind),
            _ => 0,
        }
    }

    pub fn expected_spirit_index(&self) -> i64 {
        match self {
            Self::CombatActionAckMismatch {
                expected_spirit_index,
                ..
            } => i64::from(*expected_spirit_index),
            _ => 0,
        }
    }

    pub fn actual_action_kind(&self) -> i64 {
        match self {
            Self::CombatActionAckMismatch {
                actual_action_kind, ..
            } => i64::from(*actual_action_kind),
            _ => 0,
        }
    }

    pub fn actual_spirit_index(&self) -> i64 {
        match self {
            Self::CombatActionAckMismatch {
                actual_spirit_index,
                ..
            } => i64::from(*actual_spirit_index),
            _ => 0,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum RocoParamRange {
    Inclusive { min: i64, max: i64 },
    MinInclusive { min: i64 },
    TypeBounds { type_name: String },
}

impl fmt::Display for RocoParamRange {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Inclusive { min, max } => write!(f, "{min}..={max}"),
            Self::MinInclusive { min } => write!(f, ">={min}"),
            Self::TypeBounds { type_name } => write!(f, "valid {type_name} range"),
        }
    }
}

impl RocoParamRange {
    pub const fn kind_code(&self) -> &'static str {
        match self {
            Self::Inclusive { .. } => "inclusive",
            Self::MinInclusive { .. } => "min_inclusive",
            Self::TypeBounds { .. } => "type_bounds",
        }
    }

    pub fn min_value(&self) -> i64 {
        match self {
            Self::Inclusive { min, .. } | Self::MinInclusive { min } => *min,
            Self::TypeBounds { .. } => 0,
        }
    }

    pub fn max_value(&self) -> i64 {
        match self {
            Self::Inclusive { max, .. } => *max,
            _ => 0,
        }
    }

    pub fn type_name(&self) -> String {
        match self {
            Self::TypeBounds { type_name } => type_name.clone(),
            _ => String::new(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum RocoInvalidParamError {
    OutOfRange {
        name: String,
        value: i64,
        expected: RocoParamRange,
    },
    MustBePositive {
        name: String,
        value: i64,
    },
    MustBeNonZero {
        name: String,
        value: i64,
    },
    RhaiTypeMismatch {
        name: String,
        message: String,
    },
    Rejected {
        name: String,
        source_code: String,
        message: String,
    },
    ProtocolRejected {
        name: String,
        field: RocoProtocolFieldName,
        value: i64,
        expected: RocoParamRange,
    },
    InvalidTimestamp {
        value: i64,
        failure: ScriptSystemFailure,
    },
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum RocoInvalidParamKind {
    OutOfRange,
    MustBePositive,
    MustBeNonZero,
    RhaiTypeMismatch,
    Rejected,
    ProtocolRejected,
    InvalidTimestamp,
}

impl RocoInvalidParamKind {
    pub fn code(&self) -> &'static str {
        match self {
            Self::OutOfRange => "out_of_range",
            Self::MustBePositive => "must_be_positive",
            Self::MustBeNonZero => "must_be_non_zero",
            Self::RhaiTypeMismatch => "rhai_type_mismatch",
            Self::Rejected => "rejected",
            Self::ProtocolRejected => "protocol_rejected",
            Self::InvalidTimestamp => "invalid_timestamp",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RocoInvalidParamInfo {
    pub kind: RocoInvalidParamKind,
    pub name: String,
    pub value: i64,
    pub detail: RocoInvalidParamDetail,
    pub message: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum RocoInvalidParamDetail {
    None,
    Rejected {
        source_code: String,
    },
    ExpectedRange(RocoParamRange),
    ProtocolRejected {
        field: RocoProtocolFieldName,
        expected: RocoParamRange,
    },
    SystemFailure(ScriptSystemFailure),
}

impl RocoInvalidParamDetail {
    pub const fn kind_code(&self) -> &'static str {
        match self {
            Self::None => "",
            Self::Rejected { .. } => "rejected",
            Self::ExpectedRange(_) => "expected_range",
            Self::ProtocolRejected { .. } => "protocol_rejected",
            Self::SystemFailure(_) => "system_failure",
        }
    }

    pub fn expected_text(&self) -> String {
        match self {
            Self::ExpectedRange(expected) | Self::ProtocolRejected { expected, .. } => {
                expected.to_string()
            }
            _ => String::new(),
        }
    }

    pub fn protocol_field(&self) -> String {
        match self {
            Self::ProtocolRejected { field, .. } => field.code().to_string(),
            _ => String::new(),
        }
    }

    pub fn protocol_field_name(&self) -> Option<RocoProtocolFieldName> {
        match self {
            Self::ProtocolRejected { field, .. } => Some(field.clone()),
            _ => None,
        }
    }

    pub fn rejected_source_code(&self) -> String {
        match self {
            Self::Rejected { source_code } => source_code.clone(),
            _ => String::new(),
        }
    }

    pub fn system_operation_code(&self) -> String {
        match self {
            Self::SystemFailure(failure) => failure.operation_code(),
            _ => String::new(),
        }
    }

    pub fn system_source_code(&self) -> String {
        match self {
            Self::SystemFailure(failure) => failure.source_code(),
            _ => String::new(),
        }
    }

    pub fn system_message(&self) -> String {
        match self {
            Self::SystemFailure(failure) => failure.message(),
            _ => String::new(),
        }
    }
}

impl RocoInvalidParamInfo {
    pub fn kind_code(&self) -> String {
        self.kind.code().to_string()
    }

    pub fn detail_kind_code(&self) -> String {
        self.detail.kind_code().to_string()
    }

    pub fn system_operation_code(&self) -> String {
        self.detail.system_operation_code()
    }

    pub fn system_source_code(&self) -> String {
        self.detail.system_source_code()
    }

    pub fn expected_text(&self) -> String {
        self.detail.expected_text()
    }

    pub fn protocol_field(&self) -> String {
        self.detail.protocol_field()
    }

    pub fn protocol_field_name(&self) -> Option<RocoProtocolFieldName> {
        self.detail.protocol_field_name()
    }

    pub fn rejected_source_code(&self) -> String {
        self.detail.rejected_source_code()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RocoProtocolFieldName {
    code: String,
}

impl RocoProtocolFieldName {
    pub fn new(code: impl Into<String>) -> Self {
        Self { code: code.into() }
    }

    pub fn code(&self) -> &str {
        &self.code
    }
}

impl fmt::Display for RocoProtocolFieldName {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.code())
    }
}

impl RocoInvalidParamError {
    pub fn type_bounds(name: impl Into<String>, value: i64, type_name: impl Into<String>) -> Self {
        Self::OutOfRange {
            name: name.into(),
            value,
            expected: RocoParamRange::TypeBounds {
                type_name: type_name.into(),
            },
        }
    }

    pub fn inclusive(name: impl Into<String>, value: i64, min: i64, max: i64) -> Self {
        Self::OutOfRange {
            name: name.into(),
            value,
            expected: RocoParamRange::Inclusive { min, max },
        }
    }

    pub fn min_inclusive(name: impl Into<String>, value: i64, min: i64) -> Self {
        Self::OutOfRange {
            name: name.into(),
            value,
            expected: RocoParamRange::MinInclusive { min },
        }
    }

    pub fn info(&self) -> RocoInvalidParamInfo {
        match self {
            Self::OutOfRange {
                name,
                value,
                expected,
            } => RocoInvalidParamInfo {
                kind: RocoInvalidParamKind::OutOfRange,
                name: name.clone(),
                value: *value,
                detail: RocoInvalidParamDetail::ExpectedRange(expected.clone()),
                message: expected.to_string(),
            },
            Self::MustBePositive { name, value } => RocoInvalidParamInfo {
                kind: RocoInvalidParamKind::MustBePositive,
                name: name.clone(),
                value: *value,
                detail: RocoInvalidParamDetail::ExpectedRange(RocoParamRange::MinInclusive {
                    min: 1,
                }),
                message: String::new(),
            },
            Self::MustBeNonZero { name, value } => RocoInvalidParamInfo {
                kind: RocoInvalidParamKind::MustBeNonZero,
                name: name.clone(),
                value: *value,
                detail: RocoInvalidParamDetail::ExpectedRange(RocoParamRange::MinInclusive {
                    min: 1,
                }),
                message: String::new(),
            },
            Self::RhaiTypeMismatch { name, message } => RocoInvalidParamInfo {
                kind: RocoInvalidParamKind::RhaiTypeMismatch,
                name: name.clone(),
                value: 0,
                detail: RocoInvalidParamDetail::None,
                message: message.clone(),
            },
            Self::Rejected {
                name,
                source_code,
                message,
            } => RocoInvalidParamInfo {
                kind: RocoInvalidParamKind::Rejected,
                name: name.clone(),
                value: 0,
                detail: RocoInvalidParamDetail::Rejected {
                    source_code: source_code.clone(),
                },
                message: message.clone(),
            },
            Self::ProtocolRejected {
                name,
                field,
                value,
                expected,
            } => RocoInvalidParamInfo {
                kind: RocoInvalidParamKind::ProtocolRejected,
                name: name.clone(),
                value: *value,
                detail: RocoInvalidParamDetail::ProtocolRejected {
                    field: field.clone(),
                    expected: expected.clone(),
                },
                message: String::new(),
            },
            Self::InvalidTimestamp { value, failure } => RocoInvalidParamInfo {
                kind: RocoInvalidParamKind::InvalidTimestamp,
                name: "timestamp".to_string(),
                value: *value,
                detail: RocoInvalidParamDetail::SystemFailure(failure.clone()),
                message: failure.message(),
            },
        }
    }
}

impl fmt::Display for RocoInvalidParamError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::OutOfRange {
                name,
                value,
                expected,
            } => write!(f, "{name} out of range: {value}, expected {expected}"),
            Self::MustBePositive { name, value } => {
                write!(f, "{name} must be positive: {value}")
            }
            Self::MustBeNonZero { name, value } => {
                write!(f, "{name} must be non-zero: {value}")
            }
            Self::RhaiTypeMismatch { name, message } => {
                write!(f, "{name} has invalid Rhai type: {message}")
            }
            Self::Rejected {
                name,
                source_code,
                message,
            } => write!(f, "{name} rejected by {source_code}: {message}"),
            Self::ProtocolRejected {
                name,
                field,
                value,
                expected,
            } => {
                write!(
                    f,
                    "{name} rejected by protocol field {field}: {value}, expected {expected}"
                )
            }
            Self::InvalidTimestamp { value, failure } => {
                write!(f, "invalid timestamp {value}: {}", failure.message())
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum RocoNetworkError {
    ChannelClosed,
    HttpRequestFailed {
        message: String,
    },
    HttpBridgeRequestFailed {
        bridge: RocoBridgeErrorInfo,
    },
    NetBridgeRequestFailed {
        bridge: RocoBridgeErrorInfo,
    },
    NetResponseParseFailed {
        target: RocoNetResponseParseTarget,
        source: RocoNetResponseParseSource,
    },
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum RocoBridgeErrorChannel {
    Http,
    Net,
}

impl RocoBridgeErrorChannel {
    pub const fn code(&self) -> &'static str {
        match self {
            Self::Http => "http",
            Self::Net => "net",
        }
    }
}

impl fmt::Display for RocoBridgeErrorChannel {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.code())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum RocoHttpBridgeErrorKind {
    Validation,
    Timeout,
    Transport,
    Url,
    HttpStatus,
    Parse,
    Business,
    StaleSession,
    ReceiverDropped,
    RuntimeStopped,
    SessionNotStarted,
    DuplicatePendingRequest,
    PreloginSession,
    Dispatch,
    System,
}

impl RocoHttpBridgeErrorKind {
    pub const fn code(&self) -> &'static str {
        match self {
            Self::Validation => "validation",
            Self::Timeout => "timeout",
            Self::Transport => "transport",
            Self::Url => "url",
            Self::HttpStatus => "http_status",
            Self::Parse => "parse",
            Self::Business => "business",
            Self::StaleSession => "stale_session",
            Self::ReceiverDropped => "receiver_dropped",
            Self::RuntimeStopped => "runtime_stopped",
            Self::SessionNotStarted => "session_not_started",
            Self::DuplicatePendingRequest => "duplicate_pending_request",
            Self::PreloginSession => "prelogin_session",
            Self::Dispatch => "dispatch",
            Self::System => "system",
        }
    }
}

impl fmt::Display for RocoHttpBridgeErrorKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.code())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum RocoNetBridgeErrorKind {
    Session,
    Timeout,
    Transport,
    Protocol,
    Validation,
    Business,
    Dispatch,
}

impl RocoNetBridgeErrorKind {
    pub const fn code(&self) -> &'static str {
        match self {
            Self::Session => "session",
            Self::Timeout => "timeout",
            Self::Transport => "transport",
            Self::Protocol => "protocol",
            Self::Validation => "validation",
            Self::Business => "business",
            Self::Dispatch => "dispatch",
        }
    }
}

impl fmt::Display for RocoNetBridgeErrorKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.code())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum RocoBridgeErrorKind {
    Http(RocoHttpBridgeErrorKind),
    Net(RocoNetBridgeErrorKind),
}

impl RocoBridgeErrorKind {
    pub const fn channel(&self) -> RocoBridgeErrorChannel {
        match self {
            Self::Http(_) => RocoBridgeErrorChannel::Http,
            Self::Net(_) => RocoBridgeErrorChannel::Net,
        }
    }

    pub const fn code(&self) -> &'static str {
        match self {
            Self::Http(kind) => kind.code(),
            Self::Net(kind) => kind.code(),
        }
    }
}

impl fmt::Display for RocoBridgeErrorKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.code())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RocoBridgeErrorInfo {
    pub channel: RocoBridgeErrorChannel,
    pub kind: RocoBridgeErrorKind,
    pub operation_code: String,
    pub message: String,
}

impl RocoBridgeErrorInfo {
    pub fn http(
        kind: RocoHttpBridgeErrorKind,
        operation_code: impl Into<String>,
        message: impl Into<String>,
    ) -> Self {
        Self {
            channel: RocoBridgeErrorChannel::Http,
            kind: RocoBridgeErrorKind::Http(kind),
            operation_code: operation_code.into(),
            message: message.into(),
        }
    }

    pub fn net(
        kind: RocoNetBridgeErrorKind,
        operation_code: impl Into<String>,
        message: impl Into<String>,
    ) -> Self {
        Self {
            channel: RocoBridgeErrorChannel::Net,
            kind: RocoBridgeErrorKind::Net(kind),
            operation_code: operation_code.into(),
            message: message.into(),
        }
    }

    pub fn kind_code(&self) -> String {
        self.kind.code().to_string()
    }

    pub fn channel_code(&self) -> String {
        self.channel.code().to_string()
    }

    pub fn operation_code(&self) -> String {
        self.operation_code.clone()
    }

    pub fn description(&self) -> String {
        format!(
            "{} bridge error [{}]: {}",
            self.channel, self.operation_code, self.message
        )
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum RocoNetResponseParseSource {
    Protocol {
        kind: RocoProtocolParseFailureKind,
        layer: RocoProtocolParseLayer,
        error_type: RocoProtocolParseErrorType,
        reason: RocoProtocolParseReason,
    },
    UnexpectedCommand {
        cmd_id: u32,
    },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum RocoProtocolParseFailureKind {
    Decode,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum RocoProtocolParseLayer {
    Wire,
    DomainResponse,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum RocoProtocolParseReason {
    ByteArrayReadOverflow {
        offset: usize,
        bytes_needed: usize,
        bytes_available: usize,
    },
    ByteArrayWriteInvalidSourceOffset {
        src_idx: usize,
        src_len: usize,
    },
    ByteArrayWriteInvalidSourceLength {
        src_idx: usize,
        write_len: usize,
        src_len: usize,
    },
    ProtoEncode,
    ProtoDecode,
    ProtoMissingRetInfo,
    ProtoMissingRetCode,
    MissingRetInfo {
        context: RocoProtocolParseContext,
    },
    MissingRetCode,
    MissingField {
        field: RocoProtocolFieldName,
    },
    MissingIndexedField {
        context: RocoProtocolParseContext,
        index: usize,
        field: RocoProtocolFieldName,
    },
    TooManyItems {
        context: RocoProtocolParseContext,
        expected_at_most: usize,
        actual: usize,
    },
    IndexOutOfBounds {
        context: RocoProtocolParseContext,
        index: usize,
        len: usize,
    },
    IndexOverflow {
        context: RocoProtocolParseContext,
        index: usize,
    },
    InvalidSpiritSex {
        value: u8,
    },
    UnsupportedListCommand {
        cmd_id: u32,
    },
    SpiritStorageMissingField {
        field: RocoSpiritStorageProtoField,
    },
    SpiritStorageIncompleteVarint {
        context: RocoSpiritStorageProtoContext,
        offset: Option<usize>,
    },
    SpiritStorageLengthOutOfRange {
        context: RocoSpiritStorageProtoContext,
        offset: Option<usize>,
    },
    SpiritStorageTruncatedPayload {
        context: RocoSpiritStorageProtoContext,
    },
    SpiritStorageIncompleteKey {
        context: RocoSpiritStorageProtoContext,
        offset: usize,
    },
    SpiritStorageBadWireType {
        context: RocoSpiritStorageProtoContext,
        wire_type: u8,
        offset: usize,
    },
    MissingSpiritTalentTail,
    MissingSpiritSkinTail,
    CombatSpiritInvalidSex {
        value: u8,
    },
    UnmappedFightEventTag {
        tag: u8,
    },
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RocoProtocolParseContext {
    code: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum RocoSpiritStorageProtoContext {
    Push,
    RetInfoLength,
    RetInfo,
    RetCode,
    RetMsgLength,
    RetMsg,
    AlreadyAddStorageNum,
    MaxAddStorageNum,
    ResponseFlag,
    SpiritInfoLength,
    SpiritInfo,
    SpiritId,
    CatchTime,
    StorageTime,
    Sex,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum RocoSpiritStorageProtoField {
    Sex,
    SpiritId,
    CatchTime,
    StorageTime,
}

impl RocoSpiritStorageProtoField {
    pub const fn code(self) -> &'static str {
        match self {
            Self::Sex => "sex",
            Self::SpiritId => "spirit_id",
            Self::CatchTime => "catch_time",
            Self::StorageTime => "storage_time",
        }
    }

    pub const fn label(self) -> &'static str {
        match self {
            Self::Sex => "sex",
            Self::SpiritId => "spiritId",
            Self::CatchTime => "catchTime",
            Self::StorageTime => "storageTime",
        }
    }
}

impl RocoSpiritStorageProtoContext {
    pub const fn code(self) -> &'static str {
        match self {
            Self::Push => "push",
            Self::RetInfoLength => "ret_info_length",
            Self::RetInfo => "ret_info",
            Self::RetCode => "ret_code",
            Self::RetMsgLength => "ret_msg_length",
            Self::RetMsg => "ret_msg",
            Self::AlreadyAddStorageNum => "already_add_storage_num",
            Self::MaxAddStorageNum => "max_add_storage_num",
            Self::ResponseFlag => "response_flag",
            Self::SpiritInfoLength => "spirit_info_length",
            Self::SpiritInfo => "spirit_info",
            Self::SpiritId => "spirit_id",
            Self::CatchTime => "catch_time",
            Self::StorageTime => "storage_time",
            Self::Sex => "sex",
        }
    }

    pub const fn label(self) -> &'static str {
        match self {
            Self::Push => "spirit storage push",
            Self::RetInfoLength => "spirit storage retInfo length",
            Self::RetInfo => "spirit storage retInfo",
            Self::RetCode => "spirit storage retCode",
            Self::RetMsgLength => "spirit storage retMsg length",
            Self::RetMsg => "spirit storage retMsg",
            Self::AlreadyAddStorageNum => "spirit storage alreadyAddStorageNum",
            Self::MaxAddStorageNum => "spirit storage maxAddStorageNum",
            Self::ResponseFlag => "spirit storage reponseFlag",
            Self::SpiritInfoLength => "spirit storage spiritInfo length",
            Self::SpiritInfo => "spirit storage spiritInfo",
            Self::SpiritId => "spirit storage spiritId",
            Self::CatchTime => "spirit storage catchTime",
            Self::StorageTime => "spirit storage storageTime",
            Self::Sex => "spirit storage sex",
        }
    }
}

impl RocoProtocolParseContext {
    pub fn new(code: impl Into<String>) -> Self {
        Self { code: code.into() }
    }

    pub fn code(&self) -> &str {
        self.code.as_str()
    }

    pub fn label(&self) -> Cow<'_, str> {
        if self.code.contains('_') {
            Cow::Owned(self.code.replace('_', " "))
        } else {
            Cow::Borrowed(self.code.as_str())
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum RocoProtocolParseErrorType {
    ByteArray,
    ProtoBody,
    SocketProtocolBody,
    SocketProtocol,
    SpiritBag,
    SpiritStorage,
    SpiritStorageProtoRead,
    SpiritStorageProtoDecode,
    CombatStart,
    CombatFightPacket,
    CombatSpiritUpgrade,
}

impl RocoProtocolParseErrorType {
    pub fn code(&self) -> &'static str {
        match self {
            Self::ByteArray => "byte_array",
            Self::ProtoBody => "proto_body",
            Self::SocketProtocolBody => "socket_protocol_body",
            Self::SocketProtocol => "socket_protocol",
            Self::SpiritBag => "spirit_bag",
            Self::SpiritStorage => "spirit_storage",
            Self::SpiritStorageProtoRead => "spirit_storage_proto_read",
            Self::SpiritStorageProtoDecode => "spirit_storage_proto_decode",
            Self::CombatStart => "combat_start",
            Self::CombatFightPacket => "combat_fight_packet",
            Self::CombatSpiritUpgrade => "combat_spirit_upgrade",
        }
    }

    pub const fn layer(&self) -> RocoProtocolParseLayer {
        match self {
            Self::ByteArray | Self::ProtoBody | Self::SocketProtocolBody | Self::SocketProtocol => {
                RocoProtocolParseLayer::Wire
            }
            Self::SpiritBag
            | Self::SpiritStorage
            | Self::SpiritStorageProtoRead
            | Self::SpiritStorageProtoDecode
            | Self::CombatStart
            | Self::CombatFightPacket
            | Self::CombatSpiritUpgrade => RocoProtocolParseLayer::DomainResponse,
        }
    }
}

impl RocoProtocolParseFailureKind {
    pub const fn code(self) -> &'static str {
        match self {
            Self::Decode => "decode",
        }
    }
}

impl RocoProtocolParseLayer {
    pub const fn code(self) -> &'static str {
        match self {
            Self::Wire => "wire",
            Self::DomainResponse => "domain_response",
        }
    }
}

impl RocoProtocolParseReason {
    pub fn code(&self) -> &'static str {
        match self {
            Self::ByteArrayReadOverflow { .. } => "byte_array_read_overflow",
            Self::ByteArrayWriteInvalidSourceOffset { .. } => {
                "byte_array_write_invalid_source_offset"
            }
            Self::ByteArrayWriteInvalidSourceLength { .. } => {
                "byte_array_write_invalid_source_length"
            }
            Self::ProtoEncode => "proto_encode",
            Self::ProtoDecode => "proto_decode",
            Self::ProtoMissingRetInfo => "proto_missing_ret_info",
            Self::ProtoMissingRetCode => "proto_missing_ret_code",
            Self::MissingRetInfo { .. } => "missing_ret_info",
            Self::MissingRetCode => "missing_ret_code",
            Self::MissingField { .. } => "missing_field",
            Self::MissingIndexedField { .. } => "missing_indexed_field",
            Self::TooManyItems { .. } => "too_many_items",
            Self::IndexOutOfBounds { .. } => "index_out_of_bounds",
            Self::IndexOverflow { .. } => "index_overflow",
            Self::InvalidSpiritSex { .. } => "invalid_spirit_sex",
            Self::UnsupportedListCommand { .. } => "unsupported_list_command",
            Self::SpiritStorageMissingField { .. } => "spirit_storage_missing_field",
            Self::SpiritStorageIncompleteVarint { .. } => "spirit_storage_incomplete_varint",
            Self::SpiritStorageLengthOutOfRange { .. } => "spirit_storage_length_out_of_range",
            Self::SpiritStorageTruncatedPayload { .. } => "spirit_storage_truncated_payload",
            Self::SpiritStorageIncompleteKey { .. } => "spirit_storage_incomplete_key",
            Self::SpiritStorageBadWireType { .. } => "spirit_storage_bad_wire_type",
            Self::MissingSpiritTalentTail => "missing_spirit_talent_tail",
            Self::MissingSpiritSkinTail => "missing_spirit_skin_tail",
            Self::CombatSpiritInvalidSex { .. } => "combat_spirit_invalid_sex",
            Self::UnmappedFightEventTag { .. } => "unmapped_fight_event_tag",
        }
    }

    pub fn message(&self) -> String {
        match self {
            Self::ByteArrayReadOverflow {
                offset,
                bytes_needed,
                bytes_available,
            } => format!(
                "ADF read overflow at offset {offset}: need {bytes_needed} bytes, available {bytes_available}"
            ),
            Self::ByteArrayWriteInvalidSourceOffset { src_idx, src_len } => {
                format!("ADF write overflow: source offset {src_idx} exceeds source length {src_len}")
            }
            Self::ByteArrayWriteInvalidSourceLength {
                src_idx,
                write_len,
                src_len,
            } => format!(
                "ADF write overflow: source range {src_idx}..{} exceeds source length {src_len}",
                src_idx.saturating_add(*write_len)
            ),
            Self::ProtoEncode => "proto encode failed".to_string(),
            Self::ProtoDecode => "proto decode failed".to_string(),
            Self::ProtoMissingRetInfo => "proto response missing ret_info".to_string(),
            Self::ProtoMissingRetCode => "proto response ret_info missing ret_code".to_string(),
            Self::MissingRetInfo { context } => {
                format!("{} response missing ret_info", context.label())
            }
            Self::MissingRetCode => "ret_info missing ret_code field".to_string(),
            Self::MissingField { field } => format!("response missing {field} field"),
            Self::MissingIndexedField {
                context,
                index,
                field,
            } => format!("{} item {index} missing {field} field", context.label()),
            Self::TooManyItems {
                context,
                expected_at_most,
                actual,
            } => format!(
                "{} response has too many items: expected at most {expected_at_most}, got {actual}",
                context.label()
            ),
            Self::IndexOutOfBounds {
                context,
                index,
                len,
            } => format!(
                "{} item index out of bounds: index {index}, len {len}",
                context.label()
            ),
            Self::IndexOverflow { context, index } => {
                format!("{} item index overflow: {index}", context.label())
            }
            Self::InvalidSpiritSex { value } => format!("unknown spirit sex: {value}"),
            Self::UnsupportedListCommand { cmd_id } => {
                format!("unsupported spirit storage list cmd_id 0x{cmd_id:x}")
            }
            Self::SpiritStorageMissingField { field } => {
                format!("storage spirit missing {} field", field.label())
            }
            Self::SpiritStorageIncompleteVarint { context, offset } => match offset {
                Some(offset) => {
                    format!(
                        "failed to parse {} at {offset}: incomplete varint",
                        context.label()
                    )
                }
                None => format!("failed to parse {}: incomplete varint", context.label()),
            },
            Self::SpiritStorageLengthOutOfRange { context, offset } => match offset {
                Some(offset) => {
                    format!(
                        "failed to parse {} at {offset}: length out of range",
                        context.label()
                    )
                }
                None => format!("failed to parse {}: value out of range", context.label()),
            },
            Self::SpiritStorageTruncatedPayload { context } => {
                format!("failed to parse {}: truncated payload", context.label())
            }
            Self::SpiritStorageIncompleteKey { context, offset } => {
                format!(
                    "failed to parse {}: incomplete key at {offset}",
                    context.label()
                )
            }
            Self::SpiritStorageBadWireType {
                context,
                wire_type,
                offset,
            } => format!(
                "failed to parse {}: bad wire type {wire_type} at {offset}",
                context.label()
            ),
            Self::MissingSpiritTalentTail => "combat start spirit talent tail missing".to_string(),
            Self::MissingSpiritSkinTail => "combat start spirit skin tail missing".to_string(),
            Self::CombatSpiritInvalidSex { value } => {
                format!("combat spirit sex parse failed: unknown spirit sex: {value}")
            }
            Self::UnmappedFightEventTag { tag } => format!("unmapped fight result tag {tag}"),
        }
    }

    pub fn context(&self) -> Option<RocoProtocolParseContext> {
        match self {
            Self::MissingRetInfo { context }
            | Self::MissingIndexedField { context, .. }
            | Self::TooManyItems { context, .. }
            | Self::IndexOutOfBounds { context, .. }
            | Self::IndexOverflow { context, .. } => Some(context.clone()),
            _ => None,
        }
    }

    pub fn context_code(&self) -> String {
        self.context()
            .map(|context| context.code().to_string())
            .unwrap_or_default()
    }

    pub fn field_name(&self) -> Option<RocoProtocolFieldName> {
        match self {
            Self::MissingField { field } | Self::MissingIndexedField { field, .. } => {
                Some(field.clone())
            }
            _ => None,
        }
    }

    pub fn field_code(&self) -> String {
        self.field_name()
            .map(|field| field.code().to_string())
            .unwrap_or_default()
    }

    pub fn spirit_storage_context(&self) -> Option<RocoSpiritStorageProtoContext> {
        match self {
            Self::SpiritStorageIncompleteVarint { context, .. }
            | Self::SpiritStorageLengthOutOfRange { context, .. }
            | Self::SpiritStorageTruncatedPayload { context }
            | Self::SpiritStorageIncompleteKey { context, .. }
            | Self::SpiritStorageBadWireType { context, .. } => Some(*context),
            _ => None,
        }
    }

    pub fn spirit_storage_context_code(&self) -> String {
        self.spirit_storage_context()
            .map(|context| context.code().to_string())
            .unwrap_or_default()
    }

    pub fn spirit_storage_field(&self) -> Option<RocoSpiritStorageProtoField> {
        match self {
            Self::SpiritStorageMissingField { field } => Some(*field),
            _ => None,
        }
    }

    pub fn spirit_storage_field_code(&self) -> String {
        self.spirit_storage_field()
            .map(|field| field.code().to_string())
            .unwrap_or_default()
    }
}

impl fmt::Display for RocoProtocolParseReason {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.message())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RocoNetResponseParseFailure {
    pub target: RocoNetResponseParseTarget,
    pub source: RocoNetResponseParseSource,
}

impl RocoNetResponseParseFailure {
    pub fn target_code(&self) -> String {
        self.target.code().to_string()
    }

    pub fn target_label(&self) -> String {
        self.target.label().to_string()
    }

    pub fn source_kind_code(&self) -> String {
        self.source.kind_code().to_string()
    }

    pub fn protocol_message(&self) -> String {
        self.source.protocol_message()
    }

    pub fn protocol_reason(&self) -> Option<RocoProtocolParseReason> {
        self.source.protocol_reason()
    }

    pub fn protocol_reason_code(&self) -> String {
        self.source.protocol_reason_code()
    }

    pub fn protocol_error_type_code(&self) -> String {
        self.source.protocol_error_type_code()
    }

    pub fn protocol_layer(&self) -> Option<RocoProtocolParseLayer> {
        self.source.protocol_layer()
    }

    pub fn protocol_layer_code(&self) -> String {
        self.source.protocol_layer_code()
    }

    pub fn protocol_kind_code(&self) -> String {
        self.source.protocol_kind_code()
    }

    pub fn unexpected_cmd_id(&self) -> i64 {
        self.source.unexpected_cmd_id()
    }

    pub fn description(&self) -> String {
        format!(
            "failed to parse network response {}: {}",
            self.target.label(),
            self.source
        )
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum RocoNetResponseParseTarget {
    ChangeScene,
    ReturnCode,
    QueryServerTime,
    QuerySpiritBookStates,
    PauseState,
    QuerySpiritBag,
    QuerySpiritItemList,
    QuerySpiritStorageList,
    QuerySpiritStorageListPush,
    QueryAbandonedStorageList,
    QuerySpiritStorageDetail,
    WakeupSkills,
    SwitchSkill,
    UseSpiritItem,
    EquipmentBag,
    SpiritEquipmentOperation,
    ManorGroundInfo,
    ManorItemCount,
    ManorApplySow,
    ManorApplyReap,
    ManorApplyUproot,
    ManorApplyWeed,
    ManorUseFertilizer,
    NewsTimesQueryReports,
    QueryActivities,
    LadderPersonalInfo,
    LadderRank,
    TypeLadderInfo,
    TypeLadderRank,
    CapricornPalaceNotes,
    CapricornInviteList,
    VirgoBellFoxStatus,
    VirgoBellFoxExchange,
    AquariusSecondStatus,
    AquariusSecondExchange,
    AriesThirdStatus,
    AriesThirdExchange,
    LibraThirdStatus,
    LibraThirdExchange,
    LeoFirstStatus,
    LeoFirstExchange,
    CapricornTeamOperation,
    CombatFightRequest,
    CombatItems,
    CombatStartPacket,
}

impl RocoNetResponseParseTarget {
    pub const fn code(self) -> &'static str {
        match self {
            Self::ChangeScene => "change_scene",
            Self::ReturnCode => "return_code",
            Self::QueryServerTime => "query_server_time",
            Self::QuerySpiritBookStates => "query_spirit_book_states",
            Self::PauseState => "pause_state",
            Self::QuerySpiritBag => "query_spirit_bag",
            Self::QuerySpiritItemList => "query_spirit_item_list",
            Self::QuerySpiritStorageList => "query_spirit_storage_list",
            Self::QuerySpiritStorageListPush => "query_spirit_storage_list_push",
            Self::QueryAbandonedStorageList => "query_abandoned_storage_list",
            Self::QuerySpiritStorageDetail => "query_spirit_storage_detail",
            Self::WakeupSkills => "wakeup_skills",
            Self::SwitchSkill => "switch_skill",
            Self::UseSpiritItem => "use_spirit_item",
            Self::EquipmentBag => "equipment_bag",
            Self::SpiritEquipmentOperation => "spirit_equipment_operation",
            Self::ManorGroundInfo => "manor_ground_info",
            Self::ManorItemCount => "manor_item_count",
            Self::ManorApplySow => "manor_apply_sow",
            Self::ManorApplyReap => "manor_apply_reap",
            Self::ManorApplyUproot => "manor_apply_uproot",
            Self::ManorApplyWeed => "manor_apply_weed",
            Self::ManorUseFertilizer => "manor_use_fertilizer",
            Self::NewsTimesQueryReports => "news_times_query_reports",
            Self::QueryActivities => "query_activities",
            Self::LadderPersonalInfo => "ladder_personal_info",
            Self::LadderRank => "ladder_rank",
            Self::TypeLadderInfo => "type_ladder_info",
            Self::TypeLadderRank => "type_ladder_rank",
            Self::CapricornPalaceNotes => "capricorn_palace_notes",
            Self::CapricornInviteList => "capricorn_invite_list",
            Self::VirgoBellFoxStatus => "virgo_bell_fox_status",
            Self::VirgoBellFoxExchange => "virgo_bell_fox_exchange",
            Self::AquariusSecondStatus => "aquarius_second_status",
            Self::AquariusSecondExchange => "aquarius_second_exchange",
            Self::AriesThirdStatus => "aries_third_status",
            Self::AriesThirdExchange => "aries_third_exchange",
            Self::LibraThirdStatus => "libra_third_status",
            Self::LibraThirdExchange => "libra_third_exchange",
            Self::LeoFirstStatus => "leo_first_status",
            Self::LeoFirstExchange => "leo_first_exchange",
            Self::CapricornTeamOperation => "capricorn_team_operation",
            Self::CombatFightRequest => "combat_fight_request",
            Self::CombatItems => "combat_items",
            Self::CombatStartPacket => "combat_start_packet",
        }
    }

    pub const fn label(self) -> &'static str {
        match self {
            Self::ChangeScene => "ChangeSceneResponse",
            Self::ReturnCode => "ReturnCode",
            Self::QueryServerTime => "QueryServerTimeResponse",
            Self::QuerySpiritBookStates => "SpiritBookStateResponse",
            Self::PauseState => "PauseStateResponse",
            Self::QuerySpiritBag => "QuerySpiritBagResponse",
            Self::QuerySpiritItemList => "QuerySpiritItemListResponse",
            Self::QuerySpiritStorageList => "QuerySpiritStorageListResponse",
            Self::QuerySpiritStorageListPush => "QuerySpiritStorageListResponse push",
            Self::QueryAbandonedStorageList => "QuerySpiritStorageListResponse abandoned",
            Self::QuerySpiritStorageDetail => "QuerySpiritStorageDetailResponse",
            Self::WakeupSkills => "WakeupSkillsResponse",
            Self::SwitchSkill => "SwitchSkillResponse",
            Self::UseSpiritItem => "UseSpiritItemResponse",
            Self::EquipmentBag => "EquipmentBagResponse",
            Self::SpiritEquipmentOperation => "SpiritEquipmentOperationResponse",
            Self::ManorGroundInfo => "ManorGroundInfoResponse",
            Self::ManorItemCount => "ManorItemCountResponse",
            Self::ManorApplySow => "ManorApplySowResponse",
            Self::ManorApplyReap => "ManorApplyReapResponse",
            Self::ManorApplyUproot => "ManorApplyUprootResponse",
            Self::ManorApplyWeed => "ManorApplyWeedResponse",
            Self::ManorUseFertilizer => "ManorUseFertilizerResponse",
            Self::NewsTimesQueryReports => "NewsTimesQueryReportsResponse",
            Self::QueryActivities => "QueryActivitiesResponse",
            Self::LadderPersonalInfo => "LadderPersonalInfoResponse",
            Self::LadderRank => "LadderRankResponse",
            Self::TypeLadderInfo => "TypeLadderInfoResponse",
            Self::TypeLadderRank => "TypeLadderRankResponse",
            Self::CapricornPalaceNotes => "CapricornPalaceNotesResponse",
            Self::CapricornInviteList => "CapricornInviteListResponse",
            Self::VirgoBellFoxStatus => "VirgoBellFoxStatusResponse",
            Self::VirgoBellFoxExchange => "VirgoBellFoxExchangeResponse",
            Self::AquariusSecondStatus => "AquariusSecondStatusResponse",
            Self::AquariusSecondExchange => "AquariusSecondExchangeResponse",
            Self::AriesThirdStatus => "AriesThirdStatusResponse",
            Self::AriesThirdExchange => "AriesThirdExchangeResponse",
            Self::LibraThirdStatus => "LibraThirdStatusResponse",
            Self::LibraThirdExchange => "LibraThirdExchangeResponse",
            Self::LeoFirstStatus => "LeoFirstStatusResponse",
            Self::LeoFirstExchange => "LeoFirstExchangeResponse",
            Self::CapricornTeamOperation => "CapricornTeamOperationResponse",
            Self::CombatFightRequest => "CombatFightRequestResponse",
            Self::CombatItems => "CombatItemsResponse",
            Self::CombatStartPacket => "CombatStartPacketResponse",
        }
    }
}

impl RocoNetResponseParseSource {
    pub const fn kind_code(&self) -> &'static str {
        match self {
            Self::Protocol { .. } => "protocol",
            Self::UnexpectedCommand { .. } => "unexpected_command",
        }
    }

    pub fn protocol_message(&self) -> String {
        match self {
            Self::Protocol { reason, .. } => reason.message(),
            _ => String::new(),
        }
    }

    pub fn protocol_reason(&self) -> Option<RocoProtocolParseReason> {
        match self {
            Self::Protocol { reason, .. } => Some(reason.clone()),
            _ => None,
        }
    }

    pub fn protocol_reason_code(&self) -> String {
        match self {
            Self::Protocol { reason, .. } => reason.code().to_string(),
            _ => String::new(),
        }
    }

    pub fn protocol_error_type_code(&self) -> String {
        match self {
            Self::Protocol { error_type, .. } => error_type.code().to_string(),
            _ => String::new(),
        }
    }

    pub fn protocol_layer(&self) -> Option<RocoProtocolParseLayer> {
        match self {
            Self::Protocol { layer, .. } => Some(*layer),
            _ => None,
        }
    }

    pub fn protocol_layer_code(&self) -> String {
        match self {
            Self::Protocol { layer, .. } => layer.code().to_string(),
            _ => String::new(),
        }
    }

    pub fn protocol_kind_code(&self) -> String {
        match self {
            Self::Protocol { kind, .. } => kind.code().to_string(),
            _ => String::new(),
        }
    }

    pub fn unexpected_cmd_id(&self) -> i64 {
        match self {
            Self::UnexpectedCommand { cmd_id } => i64::from(*cmd_id),
            _ => 0,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum RocoNetworkErrorKind {
    ChannelClosed,
    HttpRequestFailed,
    HttpBridgeRequestFailed,
    NetBridgeRequestFailed,
    NetResponseParseFailed,
}

impl RocoNetworkErrorKind {
    pub const fn code(&self) -> &'static str {
        match self {
            Self::ChannelClosed => "channel_closed",
            Self::HttpRequestFailed => "http_request_failed",
            Self::HttpBridgeRequestFailed => "http_bridge_request_failed",
            Self::NetBridgeRequestFailed => "net_bridge_request_failed",
            Self::NetResponseParseFailed => "net_response_parse_failed",
        }
    }
}

impl fmt::Display for RocoNetworkErrorKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.code())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum RocoErrorKind {
    ScriptError,
    StdLib,
    InvalidParam,
    Network { kind: RocoNetworkErrorKind },
    Timeout,
    ServerRejected,
    Assertion,
    Other,
}

impl RocoErrorKind {
    pub const fn family_code(&self) -> &'static str {
        match self {
            Self::ScriptError => "script_error",
            Self::StdLib => "stdlib",
            Self::InvalidParam => "invalid_param",
            Self::Network { .. } => "network",
            Self::Timeout => "timeout",
            Self::ServerRejected => "server_rejected",
            Self::Assertion => "assertion",
            Self::Other => "other",
        }
    }

    pub fn kind_code(&self) -> String {
        self.to_string()
    }
}

impl fmt::Display for RocoErrorKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Network { kind } => write!(f, "network.{kind}"),
            _ => f.write_str(self.family_code()),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RocoErrorInfo {
    pub kind: RocoErrorKind,
    pub code: String,
    pub message: String,
    pub detail: RocoErrorDetail,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum RocoErrorDetail {
    None,
    General(RocoGeneralError),
    InvalidParam(RocoInvalidParamInfo),
    UnsupportedFunction(ScriptFunctionName),
    FunctionContext(ScriptFunctionContextError),
    Query(ScriptQueryError),
    StaticData(ScriptStaticDataError),
    SystemFailure(ScriptSystemFailure),
    StdlibBridge(ScriptBridgeFailure),
    ActivityOperation(ScriptActivityOperationError),
    CombatAction(ScriptCombatActionError),
    CombatWait(ScriptCombatWaitError),
    CombatRuntime(ScriptCombatRuntimeError),
    PendingResponse(ScriptPendingResponseError),
    Response(ScriptResponseError),
    Request(ScriptRequestError),
    SessionMemory(ScriptSessionMemoryError),
    Lookup(ScriptLookupError),
    SpiritOperation(ScriptSpiritOperationError),
    Bridge(RocoBridgeErrorInfo),
    NetResponseParse(RocoNetResponseParseFailure),
    ReturnCode(RocoReturnCodeRejection),
    HttpBusiness(RocoHttpBusinessRejection),
}

impl RocoErrorDetail {
    pub const fn kind_code(&self) -> &'static str {
        match self {
            Self::None => "",
            Self::General(_) => "general",
            Self::InvalidParam(_) => "invalid_param",
            Self::UnsupportedFunction(_) => "unsupported_function",
            Self::FunctionContext(_) => "function_context",
            Self::Query(_) => "query",
            Self::StaticData(_) => "static_data",
            Self::SystemFailure(_) => "system_failure",
            Self::StdlibBridge(_) => "stdlib_bridge",
            Self::ActivityOperation(_) => "activity_operation",
            Self::CombatAction(_) => "combat_action",
            Self::CombatWait(_) => "combat_wait",
            Self::CombatRuntime(_) => "combat_runtime",
            Self::PendingResponse(_) => "pending_response",
            Self::Response(_) => "response",
            Self::Request(_) => "request",
            Self::SessionMemory(_) => "session_memory",
            Self::Lookup(_) => "lookup",
            Self::SpiritOperation(_) => "spirit_operation",
            Self::Bridge(_) => "bridge",
            Self::NetResponseParse(_) => "net_response_parse",
            Self::ReturnCode(_) => "return_code",
            Self::HttpBusiness(_) => "http_business",
        }
    }

    pub fn general_lock_target(&self) -> Option<RocoGeneralLockTarget> {
        match self {
            Self::General(RocoGeneralError::LockPoisoned { target }) => Some(*target),
            _ => None,
        }
    }

    pub fn general_lock_target_code(&self) -> String {
        self.general_lock_target()
            .map(|target| target.code().to_string())
            .unwrap_or_default()
    }

    pub fn invalid_param_kind_code(&self) -> String {
        match self {
            Self::InvalidParam(info) => info.kind_code(),
            _ => String::new(),
        }
    }

    pub fn invalid_param_name(&self) -> String {
        match self {
            Self::InvalidParam(info) => info.name.clone(),
            _ => String::new(),
        }
    }

    pub fn invalid_param_value(&self) -> i64 {
        match self {
            Self::InvalidParam(info) => info.value,
            _ => 0,
        }
    }

    pub fn invalid_param_message(&self) -> String {
        match self {
            Self::InvalidParam(info) => info.message.clone(),
            _ => String::new(),
        }
    }

    pub fn invalid_param_expected(&self) -> String {
        match self {
            Self::InvalidParam(info) => info.expected_text(),
            _ => String::new(),
        }
    }

    pub fn invalid_param_protocol_field(&self) -> String {
        match self {
            Self::InvalidParam(info) => info.protocol_field(),
            _ => String::new(),
        }
    }

    pub fn invalid_param_rejected_source_code(&self) -> String {
        match self {
            Self::InvalidParam(info) => info.rejected_source_code(),
            _ => String::new(),
        }
    }

    pub fn unsupported_module(&self) -> String {
        match self {
            Self::UnsupportedFunction(function) => function.module_code(),
            _ => String::new(),
        }
    }

    pub fn unsupported_function_name(&self) -> String {
        match self {
            Self::UnsupportedFunction(function) => function.function.clone(),
            _ => String::new(),
        }
    }

    pub fn function_context_kind_code(&self) -> String {
        match self {
            Self::FunctionContext(error) => error.kind_code().to_string(),
            _ => String::new(),
        }
    }

    pub fn function_context_combat_phase_code(&self) -> String {
        match self {
            Self::FunctionContext(error) => error.combat_phase_code(),
            _ => String::new(),
        }
    }

    pub fn query_kind_code(&self) -> String {
        match self {
            Self::Query(error) => error.kind_code().to_string(),
            _ => String::new(),
        }
    }

    pub fn query_skill_index(&self) -> i64 {
        match self {
            Self::Query(error) => error.skill_index(),
            _ => -1,
        }
    }

    pub fn static_data_kind_code(&self) -> String {
        match self {
            Self::StaticData(error) => error.kind_code().to_string(),
            _ => String::new(),
        }
    }

    pub fn static_data_position(&self) -> i64 {
        match self {
            Self::StaticData(error) => error.position(),
            _ => -1,
        }
    }

    pub fn static_data_function_name(&self) -> String {
        match self {
            Self::StaticData(error) => error.function_name(),
            _ => String::new(),
        }
    }

    pub fn static_data_message(&self) -> String {
        match self {
            Self::StaticData(error) => error.message(),
            _ => String::new(),
        }
    }

    pub fn static_data_active_config_source_code(&self) -> String {
        match self {
            Self::StaticData(error) => error.active_config_source_code(),
            _ => String::new(),
        }
    }

    pub fn system_operation_code(&self) -> String {
        match self {
            Self::SystemFailure(failure) => failure.operation_code(),
            _ => String::new(),
        }
    }

    pub fn system_source_code(&self) -> String {
        match self {
            Self::SystemFailure(failure) => failure.source_code(),
            _ => String::new(),
        }
    }

    pub fn system_message(&self) -> String {
        match self {
            Self::SystemFailure(failure) => failure.message(),
            _ => String::new(),
        }
    }

    pub fn stdlib_bridge_operation_code(&self) -> String {
        match self {
            Self::StdlibBridge(failure) => failure.operation_code(),
            _ => String::new(),
        }
    }

    pub fn stdlib_bridge_message(&self) -> String {
        match self {
            Self::StdlibBridge(failure) => failure.message(),
            _ => String::new(),
        }
    }

    pub fn activity_operation_kind_code(&self) -> String {
        match self {
            Self::ActivityOperation(error) => error.kind_code().to_string(),
            _ => String::new(),
        }
    }

    pub fn activity_operation_activity_code(&self) -> String {
        match self {
            Self::ActivityOperation(error) => error.activity_code().to_string(),
            _ => String::new(),
        }
    }

    pub fn activity_operation_field_code(&self) -> String {
        match self {
            Self::ActivityOperation(error) => error.field_code().to_string(),
            _ => String::new(),
        }
    }

    pub fn activity_operation_count(&self) -> i64 {
        match self {
            Self::ActivityOperation(error) => error.count(),
            _ => -1,
        }
    }

    pub fn activity_operation_limit(&self) -> i64 {
        match self {
            Self::ActivityOperation(error) => error.limit(),
            _ => -1,
        }
    }

    pub fn activity_operation_value(&self) -> i64 {
        match self {
            Self::ActivityOperation(error) => error.value(),
            _ => -1,
        }
    }

    pub fn combat_action_kind_code(&self) -> String {
        match self {
            Self::CombatAction(error) => error.kind_code().to_string(),
            _ => String::new(),
        }
    }

    pub fn combat_action_position(&self) -> i64 {
        match self {
            Self::CombatAction(error) => error.position(),
            _ => -1,
        }
    }

    pub fn combat_action_skill_id(&self) -> i64 {
        match self {
            Self::CombatAction(error) => error.skill_id(),
            _ => -1,
        }
    }

    pub fn combat_action_item_id(&self) -> i64 {
        match self {
            Self::CombatAction(error) => error.item_id(),
            _ => -1,
        }
    }

    pub fn combat_wait_kind_code(&self) -> String {
        match self {
            Self::CombatWait(error) => error.kind_code().to_string(),
            _ => String::new(),
        }
    }

    pub fn combat_wait_combat_phase_code(&self) -> String {
        match self {
            Self::CombatWait(error) => error.combat_phase_code(),
            _ => String::new(),
        }
    }

    pub fn combat_wait_elapsed_ms(&self) -> i64 {
        match self {
            Self::CombatWait(error) => error.elapsed_ms(),
            _ => -1,
        }
    }

    pub fn combat_runtime_message(&self) -> String {
        match self {
            Self::CombatRuntime(error) => error.message(),
            _ => String::new(),
        }
    }

    pub fn combat_command_failure_kind(&self) -> Option<ScriptCombatCommandFailureKind> {
        match self {
            Self::CombatRuntime(error) => error.command_failure_kind(),
            _ => None,
        }
    }

    pub fn combat_command_failure_kind_code(&self) -> String {
        self.combat_command_failure_kind()
            .map(|kind| kind.code().to_string())
            .unwrap_or_default()
    }

    pub fn pending_response_kind_code(&self) -> String {
        match self {
            Self::PendingResponse(error) => error.kind_code().to_string(),
            _ => String::new(),
        }
    }

    pub fn pending_http_response_code(&self) -> String {
        match self {
            Self::PendingResponse(error) => error.pending_http_response_code(),
            _ => String::new(),
        }
    }

    pub fn expected_http_response_code(&self) -> String {
        match self {
            Self::PendingResponse(error) => error.expected_http_response_code(),
            _ => String::new(),
        }
    }

    pub fn actual_http_response_code(&self) -> String {
        match self {
            Self::PendingResponse(error) => error.actual_http_response_code(),
            _ => String::new(),
        }
    }

    pub fn pending_response_combat_phase_code(&self) -> String {
        match self {
            Self::PendingResponse(error) => error.combat_phase_code(),
            _ => String::new(),
        }
    }

    pub fn pending_response_net_target_code(&self) -> String {
        match self {
            Self::PendingResponse(error) => error.net_response_parse_target_code(),
            _ => String::new(),
        }
    }

    pub fn pending_response_expected_action_kind(&self) -> i64 {
        match self {
            Self::PendingResponse(error) => error.expected_action_kind(),
            _ => 0,
        }
    }

    pub fn pending_response_expected_spirit_index(&self) -> i64 {
        match self {
            Self::PendingResponse(error) => error.expected_spirit_index(),
            _ => 0,
        }
    }

    pub fn pending_response_actual_action_kind(&self) -> i64 {
        match self {
            Self::PendingResponse(error) => error.actual_action_kind(),
            _ => 0,
        }
    }

    pub fn pending_response_actual_spirit_index(&self) -> i64 {
        match self {
            Self::PendingResponse(error) => error.actual_spirit_index(),
            _ => 0,
        }
    }

    pub fn response_kind_code(&self) -> String {
        match self {
            Self::Response(error) => error.kind_code().to_string(),
            _ => String::new(),
        }
    }

    pub fn expected_response_code(&self) -> String {
        match self {
            Self::Response(error) => error.expected_response_code(),
            _ => String::new(),
        }
    }

    pub fn actual_response_code(&self) -> String {
        match self {
            Self::Response(error) => error.actual_response_code(),
            _ => String::new(),
        }
    }

    pub fn request_kind_code(&self) -> String {
        match self {
            Self::Request(error) => error.kind_code().to_string(),
            _ => String::new(),
        }
    }

    pub fn request_wait_context_code(&self) -> String {
        match self {
            Self::Request(error) => error.wait_context_code().to_string(),
            _ => String::new(),
        }
    }

    pub fn request_system_failure_kind_code(&self) -> String {
        match self {
            Self::Request(error) => error.system_failure_kind_code().to_string(),
            _ => String::new(),
        }
    }

    pub fn request_combat_intent_kind_code(&self) -> String {
        match self {
            Self::Request(error) => error.combat_intent_kind_code().to_string(),
            _ => String::new(),
        }
    }

    pub fn request_combat_validation_kind_code(&self) -> String {
        match self {
            Self::Request(error) => error.combat_validation_kind_code().to_string(),
            _ => String::new(),
        }
    }

    pub fn request_spirit_index(&self) -> i64 {
        match self {
            Self::Request(error) => error.spirit_index(),
            _ => -1,
        }
    }

    pub fn request_value(&self) -> i64 {
        match self {
            Self::Request(error) => error.value(),
            _ => -1,
        }
    }

    pub fn request_combat_protocol_error_kind_code(&self) -> String {
        match self {
            Self::Request(error) => error.combat_protocol_error_kind_code().to_string(),
            _ => String::new(),
        }
    }

    pub fn request_combat_protocol_error_value(&self) -> i64 {
        match self {
            Self::Request(error) => error.combat_protocol_error_value(),
            _ => -1,
        }
    }

    pub fn session_memory_kind_code(&self) -> String {
        match self {
            Self::SessionMemory(error) => error.kind_code().to_string(),
            _ => String::new(),
        }
    }

    pub fn session_memory_key(&self) -> String {
        match self {
            Self::SessionMemory(error) => error.key(),
            _ => String::new(),
        }
    }

    pub fn session_memory_expected_kind_code(&self) -> String {
        match self {
            Self::SessionMemory(error) => error.expected_kind_code(),
            _ => String::new(),
        }
    }

    pub fn session_memory_actual_kind_code(&self) -> String {
        match self {
            Self::SessionMemory(error) => error.actual_kind_code(),
            _ => String::new(),
        }
    }

    pub fn lookup_kind_code(&self) -> String {
        match self {
            Self::Lookup(error) => error.kind_code().to_string(),
            _ => String::new(),
        }
    }

    pub fn lookup_entity_code(&self) -> String {
        match self {
            Self::Lookup(error) => error.entity_code(),
            _ => String::new(),
        }
    }

    pub fn lookup_key(&self) -> String {
        match self {
            Self::Lookup(error) => error.key(),
            _ => String::new(),
        }
    }

    pub fn spirit_operation_kind_code(&self) -> String {
        match self {
            Self::SpiritOperation(error) => error.kind_code().to_string(),
            _ => String::new(),
        }
    }

    pub fn spirit_operation_spirit_id(&self) -> i64 {
        match self {
            Self::SpiritOperation(error) => error.spirit_id(),
            _ => -1,
        }
    }

    pub fn spirit_operation_catch_time(&self) -> i64 {
        match self {
            Self::SpiritOperation(error) => error.catch_time(),
            _ => -1,
        }
    }

    pub fn bridge_channel_code(&self) -> String {
        match self {
            Self::Bridge(bridge) => bridge.channel_code(),
            _ => String::new(),
        }
    }

    pub fn bridge_operation_code(&self) -> String {
        match self {
            Self::Bridge(bridge) => bridge.operation_code(),
            _ => String::new(),
        }
    }

    pub fn bridge_message(&self) -> String {
        match self {
            Self::Bridge(bridge) => bridge.message.clone(),
            _ => String::new(),
        }
    }

    pub fn net_response_parse_target(&self) -> String {
        self.net_response_parse_target_code()
    }

    pub fn net_response_parse_target_code(&self) -> String {
        match self {
            Self::NetResponseParse(failure) => failure.target_code(),
            _ => String::new(),
        }
    }

    pub fn net_response_parse_target_label(&self) -> String {
        match self {
            Self::NetResponseParse(failure) => failure.target_label(),
            _ => String::new(),
        }
    }

    pub fn net_response_parse_source_kind_code(&self) -> String {
        match self {
            Self::NetResponseParse(failure) => failure.source_kind_code(),
            _ => String::new(),
        }
    }

    pub fn net_response_parse_protocol_message(&self) -> String {
        match self {
            Self::NetResponseParse(failure) => failure.protocol_message(),
            _ => String::new(),
        }
    }

    pub fn net_response_parse_protocol_error_type(&self) -> String {
        self.net_response_parse_protocol_error_type_code()
    }

    pub fn net_response_parse_protocol_error_type_code(&self) -> String {
        match self {
            Self::NetResponseParse(failure) => failure.protocol_error_type_code(),
            _ => String::new(),
        }
    }

    pub fn net_response_parse_unexpected_cmd_id(&self) -> i64 {
        match self {
            Self::NetResponseParse(failure) => failure.unexpected_cmd_id(),
            _ => 0,
        }
    }

    pub fn return_code_kind_code(&self) -> String {
        match self {
            Self::ReturnCode(rejection) => rejection.kind_code().to_string(),
            _ => String::new(),
        }
    }

    pub fn return_code_value(&self) -> i64 {
        match self {
            Self::ReturnCode(rejection) => i64::from(rejection.code()),
            _ => 0,
        }
    }

    pub fn return_code_message(&self) -> String {
        match self {
            Self::ReturnCode(rejection) => rejection.message.clone(),
            _ => String::new(),
        }
    }

    pub fn http_business_result_code(&self) -> i64 {
        match self {
            Self::HttpBusiness(rejection) => rejection.result_code(),
            _ => 0,
        }
    }

    pub fn http_business_request_context(&self) -> String {
        match self {
            Self::HttpBusiness(rejection) => rejection.request_context(),
            _ => String::new(),
        }
    }

    pub fn http_business_message(&self) -> String {
        match self {
            Self::HttpBusiness(rejection) => rejection.message.clone(),
            _ => String::new(),
        }
    }
}

impl RocoErrorInfo {
    pub fn kind_code(&self) -> String {
        self.kind.kind_code()
    }

    pub fn detail_kind_code(&self) -> String {
        self.detail.kind_code().to_string()
    }

    pub fn network_kind_code(&self) -> String {
        match &self.kind {
            RocoErrorKind::Network { kind } => kind.code().to_string(),
            _ => String::new(),
        }
    }
}

impl RocoNetworkError {
    pub fn kind(&self) -> RocoNetworkErrorKind {
        match self {
            Self::ChannelClosed => RocoNetworkErrorKind::ChannelClosed,
            Self::HttpRequestFailed { .. } => RocoNetworkErrorKind::HttpRequestFailed,
            Self::HttpBridgeRequestFailed { .. } => RocoNetworkErrorKind::HttpBridgeRequestFailed,
            Self::NetBridgeRequestFailed { .. } => RocoNetworkErrorKind::NetBridgeRequestFailed,
            Self::NetResponseParseFailed { .. } => RocoNetworkErrorKind::NetResponseParseFailed,
        }
    }

    pub fn code(&self) -> &str {
        match self {
            Self::ChannelClosed => "network.channel_closed",
            Self::HttpRequestFailed { .. } => "network.http_request_failed",
            Self::HttpBridgeRequestFailed { .. } => "network.http_bridge_request_failed",
            Self::NetBridgeRequestFailed { .. } => "network.net_bridge_request_failed",
            Self::NetResponseParseFailed { .. } => "network.net_response_parse_failed",
        }
    }

    pub fn message(&self) -> String {
        self.to_string()
    }

    pub fn info(&self) -> RocoErrorInfo {
        let detail = match self {
            Self::NetResponseParseFailed { target, source } => {
                RocoErrorDetail::NetResponseParse(RocoNetResponseParseFailure {
                    target: target.clone(),
                    source: source.clone(),
                })
            }
            Self::HttpBridgeRequestFailed { bridge } | Self::NetBridgeRequestFailed { bridge } => {
                RocoErrorDetail::Bridge(bridge.clone())
            }
            _ => RocoErrorDetail::None,
        };
        RocoErrorInfo {
            kind: RocoErrorKind::Network { kind: self.kind() },
            code: self.code().to_string(),
            message: self.message(),
            detail,
        }
    }

    pub fn bridge_info(&self) -> Option<RocoBridgeErrorInfo> {
        match self {
            Self::HttpBridgeRequestFailed { bridge } | Self::NetBridgeRequestFailed { bridge } => {
                Some(bridge.clone())
            }
            _ => None,
        }
    }

    pub fn net_response_parse_failure(&self) -> Option<RocoNetResponseParseFailure> {
        match self {
            Self::NetResponseParseFailed { target, source } => Some(RocoNetResponseParseFailure {
                target: target.clone(),
                source: source.clone(),
            }),
            _ => None,
        }
    }
}

impl fmt::Display for RocoNetworkError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::ChannelClosed => f.write_str("Channel closed"),
            Self::HttpRequestFailed { message } => write!(f, "HTTP request failed: {message}"),
            Self::HttpBridgeRequestFailed { bridge } => {
                write!(
                    f,
                    "HTTP bridge request failed [{}]: {}",
                    bridge.operation_code, bridge.message
                )
            }
            Self::NetBridgeRequestFailed { bridge } => {
                write!(
                    f,
                    "Net bridge request failed [{}]: {}",
                    bridge.operation_code, bridge.message
                )
            }
            Self::NetResponseParseFailed { target, source } => {
                write!(
                    f,
                    "failed to parse network response {}: {source}",
                    target.label()
                )
            }
        }
    }
}

impl fmt::Display for RocoNetResponseParseSource {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Protocol {
                error_type, reason, ..
            } => {
                write!(f, "{} ({})", reason.message(), error_type.code())
            }
            Self::UnexpectedCommand { cmd_id } => {
                write!(f, "unexpected command cmd_id=0x{cmd_id:x}")
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum RocoTimeoutError {
    WaitingForResponse { serial_num: u32, timeout_ms: i64 },
}

impl RocoTimeoutError {
    pub fn message(&self) -> String {
        self.to_string()
    }
}

impl fmt::Display for RocoTimeoutError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::WaitingForResponse {
                serial_num,
                timeout_ms,
            } => write!(
                f,
                "Timeout waiting for response (serial_num={serial_num}, timeout={timeout_ms}ms)"
            ),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum RocoReturnCodeKind {
    GenericFailure,
    SafeCodeRequired,
    RoleUnavailable,
    Unrecognized { code: i32 },
}

impl RocoReturnCodeKind {
    pub fn code(&self) -> i32 {
        match self {
            Self::GenericFailure => -1,
            Self::SafeCodeRequired => -4,
            Self::RoleUnavailable => -7,
            Self::Unrecognized { code } => *code,
        }
    }

    pub fn kind_code(&self) -> &'static str {
        match self {
            Self::GenericFailure => "generic_failure",
            Self::SafeCodeRequired => "safe_code_required",
            Self::RoleUnavailable => "role_unavailable",
            Self::Unrecognized { .. } => "unrecognized",
        }
    }

    pub fn label(&self) -> &'static str {
        match self {
            Self::GenericFailure => "generic failure",
            Self::SafeCodeRequired => "safe code required",
            Self::RoleUnavailable => "role unavailable",
            Self::Unrecognized { .. } => "unrecognized return code",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RocoReturnCodeRejection {
    pub kind: RocoReturnCodeKind,
    pub message: String,
}

impl RocoReturnCodeRejection {
    pub fn code(&self) -> i32 {
        self.kind.code()
    }

    pub fn kind_code(&self) -> &'static str {
        self.kind.kind_code()
    }

    pub fn description(&self) -> String {
        if self.message.is_empty() {
            format!("{} ({})", self.kind.label(), self.code())
        } else {
            format!("{} ({}): {}", self.kind.label(), self.code(), self.message)
        }
    }
}

impl fmt::Display for RocoReturnCodeRejection {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.description())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RocoHttpBusinessRejection {
    pub result_code: i32,
    pub message: String,
    pub request_context: Option<RocoRequestContext>,
}

impl RocoHttpBusinessRejection {
    pub fn result_code(&self) -> i64 {
        i64::from(self.result_code)
    }

    pub fn request_context(&self) -> String {
        self.request_context
            .as_ref()
            .map(|context| context.raw.clone())
            .unwrap_or_default()
    }

    pub fn description(&self) -> String {
        let context = self.request_context();
        if context.is_empty() {
            format!(
                "HTTP business result_code={}: {}",
                self.result_code, self.message
            )
        } else {
            format!(
                "HTTP business {} result_code={}: {}",
                context, self.result_code, self.message
            )
        }
    }
}

impl fmt::Display for RocoHttpBusinessRejection {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.description())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum RocoServerRejectedError {
    ReturnCode {
        rejection: RocoReturnCodeRejection,
    },
    HttpBusiness {
        rejection: RocoHttpBusinessRejection,
    },
    HttpResponse {
        message: String,
    },
}

impl RocoServerRejectedError {
    pub const fn code(&self) -> &'static str {
        match self {
            Self::ReturnCode { .. } => "server_rejected.return_code",
            Self::HttpBusiness { .. } => "server_rejected.http_business",
            Self::HttpResponse { .. } => "server_rejected.http_response",
        }
    }

    pub fn return_code(&self) -> Option<RocoReturnCodeRejection> {
        match self {
            Self::ReturnCode { rejection } => Some(rejection.clone()),
            _ => None,
        }
    }

    pub fn http_business(&self) -> Option<RocoHttpBusinessRejection> {
        match self {
            Self::HttpBusiness { rejection } => Some(rejection.clone()),
            _ => None,
        }
    }
}

impl fmt::Display for RocoServerRejectedError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::ReturnCode { rejection } => write!(f, "return code rejected: {rejection}"),
            Self::HttpBusiness { rejection } => {
                write!(f, "HTTP business rejected: {rejection}")
            }
            Self::HttpResponse { message } => write!(f, "HTTP response rejected: {message}"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum RocoGeneralError {
    LockPoisoned { target: RocoGeneralLockTarget },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum RocoGeneralLockTarget {
    Stdlib,
}

impl RocoGeneralLockTarget {
    pub const fn code(self) -> &'static str {
        match self {
            Self::Stdlib => "stdlib",
        }
    }

    pub const fn label(self) -> &'static str {
        match self {
            Self::Stdlib => "stdlib",
        }
    }
}

impl RocoGeneralError {
    pub const fn code(&self) -> &'static str {
        match self {
            Self::LockPoisoned { .. } => "other.lock_poisoned",
        }
    }
}

impl fmt::Display for RocoGeneralError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::LockPoisoned { target } => write!(f, "{} lock poisoned", target.label()),
        }
    }
}

#[derive(Debug, Clone)]
pub enum RocoError {
    ScriptError(RocoScriptError),
    StdLib(RocoStdLibError),
    InvalidParam(RocoInvalidParamError),
    NetworkError(RocoNetworkError),
    TimeoutError(RocoTimeoutError),
    ServerRejected(RocoServerRejectedError),
    AssertionError(String),
    Other(RocoGeneralError),
}

impl fmt::Display for RocoError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            RocoError::ScriptError(error) => {
                write!(f, "Script error")?;
                match &error.location {
                    RocoScriptLocation::Unknown => {}
                    RocoScriptLocation::Anonymous { position } => write!(f, " at {}", position)?,
                    RocoScriptLocation::Source { source, position } => {
                        write!(f, " in {}:{}", source, position)?
                    }
                }
                write!(f, ": [{}] {}", error.kind, error.message)
            }
            RocoError::StdLib(error) => write!(f, "StdLib error: {}", error),
            RocoError::InvalidParam(error) => write!(f, "Invalid parameter: {}", error),
            RocoError::NetworkError(error) => write!(f, "Network error: {}", error),
            RocoError::TimeoutError(error) => write!(f, "Timeout error: {}", error),
            RocoError::ServerRejected(error) => write!(f, "Server rejected: {}", error),
            RocoError::AssertionError(msg) => write!(f, "Assertion failed: {}", msg),
            RocoError::Other(error) => write!(f, "Error: {}", error),
        }
    }
}

impl std::error::Error for RocoError {}

impl RocoError {
    pub fn kind(&self) -> RocoErrorKind {
        match self {
            Self::ScriptError(_) => RocoErrorKind::ScriptError,
            Self::StdLib(_) => RocoErrorKind::StdLib,
            Self::InvalidParam(_) => RocoErrorKind::InvalidParam,
            Self::NetworkError(error) => RocoErrorKind::Network { kind: error.kind() },
            Self::TimeoutError(_) => RocoErrorKind::Timeout,
            Self::ServerRejected(_) => RocoErrorKind::ServerRejected,
            Self::AssertionError(_) => RocoErrorKind::Assertion,
            Self::Other(_) => RocoErrorKind::Other,
        }
    }

    pub fn code(&self) -> String {
        match self {
            Self::ScriptError(error) => format!("script.{}", error.kind.as_str()),
            Self::StdLib(error) => error.code().to_string(),
            Self::InvalidParam(_) => "invalid_param".to_string(),
            Self::NetworkError(error) => error.code().to_string(),
            Self::TimeoutError(_) => "timeout.waiting_for_response".to_string(),
            Self::ServerRejected(error) => error.code().to_string(),
            Self::AssertionError(_) => "assertion".to_string(),
            Self::Other(error) => error.code().to_string(),
        }
    }

    pub fn message(&self) -> String {
        self.to_string()
    }

    pub fn unsupported_function(&self) -> Option<ScriptFunctionName> {
        match self {
            Self::StdLib(RocoStdLibError::Unsupported(ScriptUnsupportedError::Function {
                name,
            })) => Some(name.clone()),
            _ => None,
        }
    }

    pub fn return_code_rejection(&self) -> Option<RocoReturnCodeRejection> {
        match self {
            Self::ServerRejected(error) => error.return_code(),
            _ => None,
        }
    }

    pub fn http_business_rejection(&self) -> Option<RocoHttpBusinessRejection> {
        match self {
            Self::ServerRejected(error) => error.http_business(),
            _ => None,
        }
    }

    pub fn stdlib_bridge_failure(&self) -> Option<ScriptBridgeFailure> {
        match self {
            Self::StdLib(RocoStdLibError::Bridge(ScriptBridgeError::SendRequestFailed {
                failure,
            })) => Some(failure.clone()),
            _ => None,
        }
    }

    pub fn system_failure(&self) -> Option<ScriptSystemFailure> {
        match self {
            Self::StdLib(RocoStdLibError::System(error)) => error.failure(),
            _ => None,
        }
    }

    pub fn info(&self) -> RocoErrorInfo {
        let detail = match self {
            Self::InvalidParam(error) => RocoErrorDetail::InvalidParam(error.info()),
            Self::StdLib(RocoStdLibError::Unsupported(ScriptUnsupportedError::Function {
                name,
            })) => RocoErrorDetail::UnsupportedFunction(name.clone()),
            Self::StdLib(RocoStdLibError::FunctionContext(error)) => {
                RocoErrorDetail::FunctionContext(*error)
            }
            Self::StdLib(RocoStdLibError::Query(error)) => RocoErrorDetail::Query(*error),
            Self::StdLib(RocoStdLibError::StaticData(error)) => {
                RocoErrorDetail::StaticData(error.clone())
            }
            Self::StdLib(RocoStdLibError::System(error)) => error
                .failure()
                .map(RocoErrorDetail::SystemFailure)
                .unwrap_or(RocoErrorDetail::None),
            Self::StdLib(RocoStdLibError::Bridge(ScriptBridgeError::SendRequestFailed {
                failure,
            })) => RocoErrorDetail::StdlibBridge(failure.clone()),
            Self::StdLib(RocoStdLibError::ActivityOperation(error)) => {
                RocoErrorDetail::ActivityOperation(error.clone())
            }
            Self::StdLib(RocoStdLibError::CombatAction(error)) => {
                RocoErrorDetail::CombatAction(*error)
            }
            Self::StdLib(RocoStdLibError::CombatWait(error)) => {
                RocoErrorDetail::CombatWait(error.clone())
            }
            Self::StdLib(RocoStdLibError::CombatRuntime(error)) => {
                RocoErrorDetail::CombatRuntime(error.clone())
            }
            Self::StdLib(RocoStdLibError::PendingResponse(error)) => {
                RocoErrorDetail::PendingResponse(error.clone())
            }
            Self::StdLib(RocoStdLibError::Response(error)) => {
                RocoErrorDetail::Response(error.clone())
            }
            Self::StdLib(RocoStdLibError::Request(error)) => {
                RocoErrorDetail::Request(error.clone())
            }
            Self::StdLib(RocoStdLibError::SessionMemory(error)) => {
                RocoErrorDetail::SessionMemory(error.clone())
            }
            Self::StdLib(RocoStdLibError::Lookup(error)) => RocoErrorDetail::Lookup(error.clone()),
            Self::StdLib(RocoStdLibError::SpiritOperation(error)) => {
                RocoErrorDetail::SpiritOperation(error.clone())
            }
            Self::NetworkError(error) => error.info().detail,
            Self::ServerRejected(RocoServerRejectedError::ReturnCode { rejection }) => {
                RocoErrorDetail::ReturnCode(rejection.clone())
            }
            Self::ServerRejected(RocoServerRejectedError::HttpBusiness { rejection }) => {
                RocoErrorDetail::HttpBusiness(rejection.clone())
            }
            Self::Other(error) => RocoErrorDetail::General(error.clone()),
            _ => RocoErrorDetail::None,
        };
        RocoErrorInfo {
            kind: self.kind(),
            code: self.code(),
            message: self.message(),
            detail,
        }
    }
}

impl From<RocoStdLibError> for RocoError {
    fn from(error: RocoStdLibError) -> Self {
        RocoError::StdLib(error)
    }
}

impl From<ScriptFunctionContextError> for RocoStdLibError {
    fn from(error: ScriptFunctionContextError) -> Self {
        RocoStdLibError::FunctionContext(error)
    }
}

impl From<ScriptQueryError> for RocoStdLibError {
    fn from(error: ScriptQueryError) -> Self {
        RocoStdLibError::Query(error)
    }
}

impl From<ScriptCombatActionError> for RocoStdLibError {
    fn from(error: ScriptCombatActionError) -> Self {
        RocoStdLibError::CombatAction(error)
    }
}

impl From<ScriptCombatRuntimeError> for RocoStdLibError {
    fn from(error: ScriptCombatRuntimeError) -> Self {
        RocoStdLibError::CombatRuntime(error)
    }
}

impl From<ScriptCombatWaitError> for RocoStdLibError {
    fn from(error: ScriptCombatWaitError) -> Self {
        RocoStdLibError::CombatWait(error)
    }
}

impl From<ScriptPendingResponseError> for RocoStdLibError {
    fn from(error: ScriptPendingResponseError) -> Self {
        RocoStdLibError::PendingResponse(error)
    }
}

impl From<ScriptLookupError> for RocoStdLibError {
    fn from(error: ScriptLookupError) -> Self {
        RocoStdLibError::Lookup(error)
    }
}

impl From<ScriptSessionMemoryError> for RocoStdLibError {
    fn from(error: ScriptSessionMemoryError) -> Self {
        RocoStdLibError::SessionMemory(error)
    }
}

impl From<ScriptStaticDataError> for RocoStdLibError {
    fn from(error: ScriptStaticDataError) -> Self {
        RocoStdLibError::StaticData(error)
    }
}

impl From<ScriptSpiritOperationError> for RocoStdLibError {
    fn from(error: ScriptSpiritOperationError) -> Self {
        RocoStdLibError::SpiritOperation(error)
    }
}

impl From<ScriptSystemError> for RocoStdLibError {
    fn from(error: ScriptSystemError) -> Self {
        RocoStdLibError::System(error)
    }
}

impl From<ScriptActivityOperationError> for RocoStdLibError {
    fn from(error: ScriptActivityOperationError) -> Self {
        RocoStdLibError::ActivityOperation(error)
    }
}

impl From<ScriptBridgeError> for RocoStdLibError {
    fn from(error: ScriptBridgeError) -> Self {
        RocoStdLibError::Bridge(error)
    }
}

impl From<ScriptResponseError> for RocoStdLibError {
    fn from(error: ScriptResponseError) -> Self {
        RocoStdLibError::Response(error)
    }
}

impl From<ScriptRequestError> for RocoStdLibError {
    fn from(error: ScriptRequestError) -> Self {
        RocoStdLibError::Request(error)
    }
}

impl From<ScriptUnsupportedError> for RocoStdLibError {
    fn from(error: ScriptUnsupportedError) -> Self {
        RocoStdLibError::Unsupported(error)
    }
}

impl From<ScriptFunctionContextError> for RocoError {
    fn from(error: ScriptFunctionContextError) -> Self {
        RocoError::StdLib(RocoStdLibError::FunctionContext(error))
    }
}

impl From<ScriptQueryError> for RocoError {
    fn from(error: ScriptQueryError) -> Self {
        RocoError::StdLib(RocoStdLibError::Query(error))
    }
}

impl From<ScriptCombatActionError> for RocoError {
    fn from(error: ScriptCombatActionError) -> Self {
        RocoError::StdLib(RocoStdLibError::CombatAction(error))
    }
}

impl From<ScriptCombatRuntimeError> for RocoError {
    fn from(error: ScriptCombatRuntimeError) -> Self {
        RocoError::StdLib(RocoStdLibError::CombatRuntime(error))
    }
}

impl From<ScriptCombatWaitError> for RocoError {
    fn from(error: ScriptCombatWaitError) -> Self {
        RocoError::StdLib(RocoStdLibError::CombatWait(error))
    }
}

impl From<ScriptPendingResponseError> for RocoError {
    fn from(error: ScriptPendingResponseError) -> Self {
        RocoError::StdLib(RocoStdLibError::PendingResponse(error))
    }
}

impl From<ScriptLookupError> for RocoError {
    fn from(error: ScriptLookupError) -> Self {
        RocoError::StdLib(RocoStdLibError::Lookup(error))
    }
}

impl From<ScriptSessionMemoryError> for RocoError {
    fn from(error: ScriptSessionMemoryError) -> Self {
        RocoError::StdLib(RocoStdLibError::SessionMemory(error))
    }
}

impl From<ScriptStaticDataError> for RocoError {
    fn from(error: ScriptStaticDataError) -> Self {
        RocoError::StdLib(RocoStdLibError::StaticData(error))
    }
}

impl From<ScriptSpiritOperationError> for RocoError {
    fn from(error: ScriptSpiritOperationError) -> Self {
        RocoError::StdLib(RocoStdLibError::SpiritOperation(error))
    }
}

impl From<ScriptSystemError> for RocoError {
    fn from(error: ScriptSystemError) -> Self {
        RocoError::StdLib(RocoStdLibError::System(error))
    }
}

impl From<ScriptActivityOperationError> for RocoError {
    fn from(error: ScriptActivityOperationError) -> Self {
        RocoError::StdLib(RocoStdLibError::ActivityOperation(error))
    }
}

impl From<ScriptBridgeError> for RocoError {
    fn from(error: ScriptBridgeError) -> Self {
        RocoError::StdLib(RocoStdLibError::Bridge(error))
    }
}

impl From<ScriptResponseError> for RocoError {
    fn from(error: ScriptResponseError) -> Self {
        RocoError::StdLib(RocoStdLibError::Response(error))
    }
}

impl From<ScriptRequestError> for RocoError {
    fn from(error: ScriptRequestError) -> Self {
        RocoError::StdLib(RocoStdLibError::Request(error))
    }
}

impl From<ScriptUnsupportedError> for RocoError {
    fn from(error: ScriptUnsupportedError) -> Self {
        RocoError::StdLib(RocoStdLibError::Unsupported(error))
    }
}

#[cfg(test)]
mod tests {
    use super::{RocoErrorKind, RocoNetworkErrorKind, ScriptCombatCommandFailureKind};

    #[test]
    fn error_kind_keeps_family_and_structured_kind_codes_separate() {
        let kind = RocoErrorKind::Network {
            kind: RocoNetworkErrorKind::HttpBridgeRequestFailed,
        };

        assert_eq!(kind.family_code(), "network");
        assert_eq!(kind.kind_code(), "network.http_bridge_request_failed");
    }

    #[test]
    fn combat_command_failure_kind_has_stable_code() {
        assert_eq!(
            ScriptCombatCommandFailureKind::LineupSkill.code(),
            "lineup_skill"
        );
    }
}
