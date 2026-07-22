use super::*;

impl RocoEngine {
    pub(super) fn script_position(position: rhai::Position) -> Option<RocoScriptPosition> {
        position.line().map(|line| match position.position() {
            Some(column) => RocoScriptPosition::LineColumn { line, column },
            None => RocoScriptPosition::Line { line },
        })
    }

    pub(super) fn script_location(
        source: Option<String>,
        position: rhai::Position,
    ) -> RocoScriptLocation {
        let Some(position) = Self::script_position(position) else {
            return RocoScriptLocation::Unknown;
        };
        match source {
            Some(source) if !source.is_empty() => RocoScriptLocation::Source { source, position },
            _ => RocoScriptLocation::Anonymous { position },
        }
    }

    pub(super) fn script_parse_error_source(
        error: &rhai::ParseErrorType,
    ) -> RocoScriptParseErrorSource {
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

    pub(super) fn script_eval_error_source(
        error: &rhai::EvalAltResult,
    ) -> RocoScriptEvalErrorSource {
        match error {
            rhai::EvalAltResult::ErrorSystem(message, _) => RocoScriptEvalErrorSource::System {
                message: message.clone(),
            },
            rhai::EvalAltResult::ErrorParsing(source, _) => RocoScriptEvalErrorSource::Parsing {
                source: Self::script_parse_error_source(source),
            },
            rhai::EvalAltResult::ErrorVariableExists(variable, _) => {
                RocoScriptEvalErrorSource::VariableExists {
                    variable: variable.clone(),
                }
            }
            rhai::EvalAltResult::ErrorForbiddenVariable(variable, _) => {
                RocoScriptEvalErrorSource::ForbiddenVariable {
                    variable: variable.clone(),
                }
            }
            rhai::EvalAltResult::ErrorVariableNotFound(variable, _) => {
                RocoScriptEvalErrorSource::VariableNotFound {
                    variable: variable.clone(),
                }
            }
            rhai::EvalAltResult::ErrorPropertyNotFound(property, _) => {
                RocoScriptEvalErrorSource::PropertyNotFound {
                    property: property.clone(),
                }
            }
            rhai::EvalAltResult::ErrorIndexNotFound(_, _) => {
                RocoScriptEvalErrorSource::IndexNotFound
            }
            rhai::EvalAltResult::ErrorFunctionNotFound(signature, _) => {
                RocoScriptEvalErrorSource::FunctionNotFound {
                    signature: signature.clone(),
                }
            }
            rhai::EvalAltResult::ErrorModuleNotFound(module, _) => {
                RocoScriptEvalErrorSource::ModuleNotFound {
                    module: module.clone(),
                }
            }
            rhai::EvalAltResult::ErrorInFunctionCall(function, source, cause, _) => {
                RocoScriptEvalErrorSource::FunctionCall {
                    function: function.clone(),
                    source: source.clone(),
                    cause: Box::new(Self::script_eval_error_source(cause)),
                }
            }
            rhai::EvalAltResult::ErrorInModule(module, cause, _) => {
                RocoScriptEvalErrorSource::Module {
                    module: module.clone(),
                    cause: Box::new(Self::script_eval_error_source(cause)),
                }
            }
            rhai::EvalAltResult::ErrorUnboundThis(_) => RocoScriptEvalErrorSource::UnboundThis,
            rhai::EvalAltResult::ErrorMismatchDataType(expected, actual, _) => {
                RocoScriptEvalErrorSource::MismatchDataType {
                    expected: expected.clone(),
                    actual: actual.clone(),
                }
            }
            rhai::EvalAltResult::ErrorMismatchOutputType(expected, actual, _) => {
                RocoScriptEvalErrorSource::MismatchOutputType {
                    expected: expected.clone(),
                    actual: actual.clone(),
                }
            }
            rhai::EvalAltResult::ErrorIndexingType(type_name, _) => {
                RocoScriptEvalErrorSource::IndexingType {
                    type_name: type_name.clone(),
                }
            }
            rhai::EvalAltResult::ErrorArrayBounds(len, index, _) => {
                RocoScriptEvalErrorSource::ArrayBounds {
                    len: *len,
                    index: *index,
                }
            }
            rhai::EvalAltResult::ErrorStringBounds(len, index, _) => {
                RocoScriptEvalErrorSource::StringBounds {
                    len: *len,
                    index: *index,
                }
            }
            rhai::EvalAltResult::ErrorBitFieldBounds(len, index, _) => {
                RocoScriptEvalErrorSource::BitFieldBounds {
                    len: *len,
                    index: *index,
                }
            }
            rhai::EvalAltResult::ErrorFor(_) => RocoScriptEvalErrorSource::ForLoopNonIterable,
            rhai::EvalAltResult::ErrorDataRace(variable, _) => {
                RocoScriptEvalErrorSource::DataRace {
                    variable: variable.clone(),
                }
            }
            rhai::EvalAltResult::ErrorNonPureMethodCallOnConstant(method, _) => {
                RocoScriptEvalErrorSource::NonPureMethodCallOnConstant {
                    method: method.clone(),
                }
            }
            rhai::EvalAltResult::ErrorAssignmentToConstant(variable, _) => {
                RocoScriptEvalErrorSource::AssignmentToConstant {
                    variable: variable.clone(),
                }
            }
            rhai::EvalAltResult::ErrorDotExpr(expression, _) => {
                RocoScriptEvalErrorSource::DotExpr {
                    expression: expression.clone(),
                }
            }
            rhai::EvalAltResult::ErrorArithmetic(message, _) => {
                RocoScriptEvalErrorSource::Arithmetic {
                    message: message.clone(),
                }
            }
            rhai::EvalAltResult::ErrorTooManyOperations(_) => {
                RocoScriptEvalErrorSource::TooManyOperations
            }
            rhai::EvalAltResult::ErrorTooManyVariables(_) => {
                RocoScriptEvalErrorSource::TooManyVariables
            }
            rhai::EvalAltResult::ErrorTooManyModules(_) => {
                RocoScriptEvalErrorSource::TooManyModules
            }
            rhai::EvalAltResult::ErrorStackOverflow(_) => RocoScriptEvalErrorSource::StackOverflow,
            rhai::EvalAltResult::ErrorDataTooLarge(type_name, _) => {
                RocoScriptEvalErrorSource::DataTooLarge {
                    type_name: type_name.clone(),
                }
            }
            rhai::EvalAltResult::ErrorTerminated(_, _) => RocoScriptEvalErrorSource::Terminated,
            rhai::EvalAltResult::ErrorCustomSyntax(message, symbols, _) => {
                RocoScriptEvalErrorSource::CustomSyntax {
                    message: message.clone(),
                    symbols: symbols.clone(),
                }
            }
            rhai::EvalAltResult::ErrorRuntime(value, _) => RocoScriptEvalErrorSource::Runtime {
                value: Self::script_runtime_error_value(value),
            },
            rhai::EvalAltResult::LoopBreak(is_break, _, _) => {
                RocoScriptEvalErrorSource::LoopBreak {
                    is_break: *is_break,
                }
            }
            rhai::EvalAltResult::Return(_, _) => RocoScriptEvalErrorSource::Return,
            rhai::EvalAltResult::Exit(_, _) => RocoScriptEvalErrorSource::Exit,
            _ => RocoScriptEvalErrorSource::UnclassifiedEval,
        }
    }

    pub(super) fn script_runtime_error_value(value: &Dynamic) -> RocoScriptRuntimeErrorValue {
        value
            .clone()
            .try_cast::<RocoScriptRuntimeErrorValue>()
            .unwrap_or_else(|| RocoScriptRuntimeErrorValue::message(value.to_string()))
    }

    pub(super) fn map_eval_error(error: Box<rhai::EvalAltResult>) -> RocoError {
        let position = error.position();
        let location_source = match error.as_ref() {
            rhai::EvalAltResult::ErrorInFunctionCall(_, source, ..) if !source.is_empty() => {
                Some(source.clone())
            }
            _ => None,
        };
        let error_source = RocoScriptErrorSource::Eval(Self::script_eval_error_source(&error));
        RocoError::ScriptError(RocoScriptError {
            location: Self::script_location(location_source, position),
            source: error_source,
        })
    }

    #[allow(deprecated)]
    pub(super) fn map_parse_error(error: rhai::ParseError, source: Option<String>) -> RocoError {
        let position = error.position();
        let error_source =
            RocoScriptErrorSource::Parse(Self::script_parse_error_source(error.err_type()));
        RocoError::ScriptError(RocoScriptError {
            location: Self::script_location(source, position),
            source: error_source,
        })
    }
}
