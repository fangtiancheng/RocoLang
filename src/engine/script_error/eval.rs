use rhai::Dynamic;

use crate::error::{RocoScriptEvalErrorSource, RocoScriptRuntimeErrorValue};

use super::parse;

pub(super) fn source(error: &rhai::EvalAltResult) -> RocoScriptEvalErrorSource {
    match error {
        rhai::EvalAltResult::ErrorSystem(message, _) => RocoScriptEvalErrorSource::System {
            message: message.clone(),
        },
        rhai::EvalAltResult::ErrorParsing(source, _) => RocoScriptEvalErrorSource::Parsing {
            source: parse::source(source),
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
        rhai::EvalAltResult::ErrorIndexNotFound(_, _) => RocoScriptEvalErrorSource::IndexNotFound,
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
                cause: Box::new(self::source(cause)),
            }
        }
        rhai::EvalAltResult::ErrorInModule(module, cause, _) => RocoScriptEvalErrorSource::Module {
            module: module.clone(),
            cause: Box::new(self::source(cause)),
        },
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
        rhai::EvalAltResult::ErrorDataRace(variable, _) => RocoScriptEvalErrorSource::DataRace {
            variable: variable.clone(),
        },
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
        rhai::EvalAltResult::ErrorDotExpr(expression, _) => RocoScriptEvalErrorSource::DotExpr {
            expression: expression.clone(),
        },
        rhai::EvalAltResult::ErrorArithmetic(message, _) => RocoScriptEvalErrorSource::Arithmetic {
            message: message.clone(),
        },
        rhai::EvalAltResult::ErrorTooManyOperations(_) => {
            RocoScriptEvalErrorSource::TooManyOperations
        }
        rhai::EvalAltResult::ErrorTooManyVariables(_) => {
            RocoScriptEvalErrorSource::TooManyVariables
        }
        rhai::EvalAltResult::ErrorTooManyModules(_) => RocoScriptEvalErrorSource::TooManyModules,
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
            value: runtime_value(value),
        },
        rhai::EvalAltResult::LoopBreak(is_break, _, _) => RocoScriptEvalErrorSource::LoopBreak {
            is_break: *is_break,
        },
        rhai::EvalAltResult::Return(_, _) => RocoScriptEvalErrorSource::Return,
        rhai::EvalAltResult::Exit(_, _) => RocoScriptEvalErrorSource::Exit,
        _ => RocoScriptEvalErrorSource::UnclassifiedEval,
    }
}

fn runtime_value(value: &Dynamic) -> RocoScriptRuntimeErrorValue {
    value
        .clone()
        .try_cast::<RocoScriptRuntimeErrorValue>()
        .unwrap_or_else(|| RocoScriptRuntimeErrorValue::message(value.to_string()))
}
