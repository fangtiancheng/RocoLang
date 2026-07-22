use crate::error::{
    RocoError, RocoScriptError, RocoScriptErrorSource, RocoScriptLocation, RocoScriptPosition,
};

mod eval;
mod parse;

pub(super) fn location(source: Option<String>, position: rhai::Position) -> RocoScriptLocation {
    let Some(line) = position.line() else {
        return RocoScriptLocation::Unknown;
    };
    let position = match position.position() {
        Some(column) => RocoScriptPosition::LineColumn { line, column },
        None => RocoScriptPosition::Line { line },
    };
    match source {
        Some(source) if !source.is_empty() => RocoScriptLocation::Source { source, position },
        _ => RocoScriptLocation::Anonymous { position },
    }
}

pub(super) fn map_eval(error: Box<rhai::EvalAltResult>) -> RocoError {
    let position = error.position();
    let source = match error.as_ref() {
        rhai::EvalAltResult::ErrorInFunctionCall(_, source, ..) if !source.is_empty() => {
            Some(source.clone())
        }
        _ => None,
    };
    RocoError::ScriptError(RocoScriptError {
        location: location(source, position),
        source: RocoScriptErrorSource::Eval(eval::source(&error)),
    })
}

#[allow(deprecated)]
pub(super) fn map_parse(error: rhai::ParseError, source: Option<String>) -> RocoError {
    let position = error.position();
    RocoError::ScriptError(RocoScriptError {
        location: location(source, position),
        source: RocoScriptErrorSource::Parse(parse::source(error.err_type())),
    })
}
