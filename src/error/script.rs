use super::*;

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
    pub location: RocoScriptLocation,
    pub source: RocoScriptErrorSource,
}

impl RocoScriptError {
    pub const fn kind(&self) -> RocoScriptErrorKind {
        self.source.kind()
    }

    pub fn message(&self) -> String {
        self.source.message()
    }
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
        source: Box<RocoJsonErrorInfo>,
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
            source: Box::new(RocoJsonErrorInfo::from_serde_json_error(error)),
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
    pub const fn kind(&self) -> RocoScriptErrorKind {
        match self {
            Self::Parse(_) => RocoScriptErrorKind::Parse,
            Self::Eval(RocoScriptEvalErrorSource::FunctionCall { .. }) => {
                RocoScriptErrorKind::FunctionCall
            }
            Self::Eval(RocoScriptEvalErrorSource::Module { .. }) => RocoScriptErrorKind::Module,
            Self::Eval(RocoScriptEvalErrorSource::Terminated) => RocoScriptErrorKind::Terminated,
            Self::Eval(RocoScriptEvalErrorSource::Runtime { .. }) => RocoScriptErrorKind::Runtime,
            Self::Eval(_) => RocoScriptErrorKind::Eval,
        }
    }

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
