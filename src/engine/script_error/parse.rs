use crate::error::RocoScriptParseErrorSource;

pub(super) fn source(error: &rhai::ParseErrorType) -> RocoScriptParseErrorSource {
    match error {
        rhai::ParseErrorType::UnexpectedEOF => RocoScriptParseErrorSource::UnexpectedEof,
        rhai::ParseErrorType::BadInput(_) => RocoScriptParseErrorSource::BadInput,
        rhai::ParseErrorType::UnknownOperator(operator) => {
            RocoScriptParseErrorSource::UnknownOperator {
                operator: operator.clone(),
            }
        }
        rhai::ParseErrorType::MissingToken(token, description) => {
            RocoScriptParseErrorSource::MissingToken {
                token: token.clone(),
                description: description.clone(),
            }
        }
        rhai::ParseErrorType::MissingSymbol(description) => {
            RocoScriptParseErrorSource::MissingSymbol {
                description: description.clone(),
            }
        }
        #[allow(deprecated)]
        rhai::ParseErrorType::MalformedCallExpr(description) => {
            RocoScriptParseErrorSource::DeprecatedMalformedCallExpr {
                description: description.clone(),
            }
        }
        rhai::ParseErrorType::MalformedIndexExpr(description) => {
            RocoScriptParseErrorSource::MalformedIndexExpr {
                description: description.clone(),
            }
        }
        #[allow(deprecated)]
        rhai::ParseErrorType::MalformedInExpr(description) => {
            RocoScriptParseErrorSource::DeprecatedMalformedInExpr {
                description: description.clone(),
            }
        }
        rhai::ParseErrorType::MalformedCapture(description) => {
            RocoScriptParseErrorSource::MalformedCapture {
                description: description.clone(),
            }
        }
        rhai::ParseErrorType::DuplicatedProperty(property) => {
            RocoScriptParseErrorSource::DuplicatedProperty {
                property: property.clone(),
            }
        }
        #[allow(deprecated)]
        rhai::ParseErrorType::DuplicatedSwitchCase => {
            RocoScriptParseErrorSource::DeprecatedDuplicatedSwitchCase
        }
        rhai::ParseErrorType::DuplicatedVariable(variable) => {
            RocoScriptParseErrorSource::DuplicatedVariable {
                variable: variable.clone(),
            }
        }
        rhai::ParseErrorType::WrongSwitchIntegerCase => {
            RocoScriptParseErrorSource::WrongSwitchIntegerCase
        }
        rhai::ParseErrorType::WrongSwitchDefaultCase => {
            RocoScriptParseErrorSource::WrongSwitchDefaultCase
        }
        rhai::ParseErrorType::WrongSwitchCaseCondition => {
            RocoScriptParseErrorSource::WrongSwitchCaseCondition
        }
        rhai::ParseErrorType::PropertyExpected => RocoScriptParseErrorSource::PropertyExpected,
        rhai::ParseErrorType::VariableExpected => RocoScriptParseErrorSource::VariableExpected,
        rhai::ParseErrorType::ForbiddenVariable(variable) => {
            RocoScriptParseErrorSource::ForbiddenVariable {
                variable: variable.clone(),
            }
        }
        rhai::ParseErrorType::Reserved(symbol) => RocoScriptParseErrorSource::Reserved {
            symbol: symbol.clone(),
        },
        rhai::ParseErrorType::MismatchedType(expected, actual) => {
            RocoScriptParseErrorSource::MismatchedType {
                expected: expected.clone(),
                actual: actual.clone(),
            }
        }
        rhai::ParseErrorType::ExprExpected(expression) => {
            RocoScriptParseErrorSource::ExprExpected {
                expression: expression.clone(),
            }
        }
        rhai::ParseErrorType::WrongDocComment => RocoScriptParseErrorSource::WrongDocComment,
        rhai::ParseErrorType::WrongFnDefinition => {
            RocoScriptParseErrorSource::WrongFunctionDefinition
        }
        rhai::ParseErrorType::FnDuplicatedDefinition(function, params) => {
            RocoScriptParseErrorSource::FunctionDuplicatedDefinition {
                function: function.clone(),
                params: *params,
            }
        }
        rhai::ParseErrorType::FnMissingName => RocoScriptParseErrorSource::FunctionMissingName,
        rhai::ParseErrorType::FnMissingParams(function) => {
            RocoScriptParseErrorSource::FunctionMissingParams {
                function: function.clone(),
            }
        }
        rhai::ParseErrorType::FnDuplicatedParam(function, parameter) => {
            RocoScriptParseErrorSource::FunctionDuplicatedParam {
                function: function.clone(),
                parameter: parameter.clone(),
            }
        }
        rhai::ParseErrorType::FnMissingBody(function) => {
            RocoScriptParseErrorSource::FunctionMissingBody {
                function: function.clone(),
            }
        }
        rhai::ParseErrorType::WrongExport => RocoScriptParseErrorSource::WrongExport,
        rhai::ParseErrorType::AssignmentToConstant(variable) => {
            RocoScriptParseErrorSource::AssignmentToConstant {
                variable: variable.clone(),
            }
        }
        rhai::ParseErrorType::AssignmentToInvalidLHS(description) => {
            RocoScriptParseErrorSource::AssignmentToInvalidLhs {
                description: description.clone(),
            }
        }
        rhai::ParseErrorType::VariableExists(variable) => {
            RocoScriptParseErrorSource::VariableExists {
                variable: variable.clone(),
            }
        }
        rhai::ParseErrorType::VariableUndefined(variable) => {
            RocoScriptParseErrorSource::VariableUndefined {
                variable: variable.clone(),
            }
        }
        rhai::ParseErrorType::ModuleUndefined(module) => {
            RocoScriptParseErrorSource::ModuleUndefined {
                module: module.clone(),
            }
        }
        rhai::ParseErrorType::ExprTooDeep => RocoScriptParseErrorSource::ExprTooDeep,
        rhai::ParseErrorType::TooManyFunctions => RocoScriptParseErrorSource::TooManyFunctions,
        rhai::ParseErrorType::LiteralTooLarge(type_name, max_size) => {
            RocoScriptParseErrorSource::LiteralTooLarge {
                type_name: type_name.clone(),
                max_size: *max_size,
            }
        }
        rhai::ParseErrorType::LoopBreak => RocoScriptParseErrorSource::LoopBreak,
        _ => RocoScriptParseErrorSource::UnclassifiedParse,
    }
}
