use serde::{Deserialize, Serialize};

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
