use rhai::{Engine, Position};
use serde::{Deserialize, Serialize};

mod metadata;
mod semantic;

pub use metadata::{RocoLanguageFunctionDoc, RocoLanguageKeywordDoc, RocoLanguageMetadata};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RocoScriptDiagnostic {
    pub code: String,
    pub message: String,
    pub severity: RocoScriptDiagnosticSeverity,
    pub start_line: usize,
    pub start_column: usize,
    pub end_line: usize,
    pub end_column: usize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum RocoScriptDiagnosticSeverity {
    Error,
    Warning,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RocoBuiltinModuleFunctionDoc {
    pub module_path: String,
    pub name: String,
    pub signature: String,
    pub params: Vec<String>,
}

pub fn builtin_module_function_docs() -> Vec<RocoBuiltinModuleFunctionDoc> {
    semantic::builtin_module_function_docs()
}

pub fn rhai_language_metadata() -> RocoLanguageMetadata {
    metadata::rhai_language_metadata()
}

pub fn analyze_script(source: &str) -> Vec<RocoScriptDiagnostic> {
    let mut engine = Engine::new();
    engine.set_max_expr_depths(0, 0);

    match engine.compile(source) {
        Ok(ast) => semantic::analyze(&ast),
        Err(error) => vec![diagnostic_at_with_code(
            error.position(),
            1,
            "rhai.syntax",
            error.err_type().to_string(),
        )],
    }
}

fn diagnostic_at(position: Position, width: usize, message: String) -> RocoScriptDiagnostic {
    diagnostic_at_with_code(position, width, "roco.semantic", message)
}

fn diagnostic_at_with_code(
    position: Position,
    width: usize,
    code: &str,
    message: String,
) -> RocoScriptDiagnostic {
    let line = position.line().unwrap_or(1);
    let column = position.position().unwrap_or(1);
    RocoScriptDiagnostic {
        code: code.to_string(),
        message,
        severity: RocoScriptDiagnosticSeverity::Error,
        start_line: line,
        start_column: column,
        end_line: line,
        end_column: column.saturating_add(width.max(1)),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn valid_script_has_no_diagnostics() {
        assert!(analyze_script(
            r#"
                import "roco/combat" as roco_combat;
                let boss_id = 800501;
                roco_combat::challenge_boss(boss_id);
            "#
        )
        .is_empty());
    }

    #[test]
    fn invalid_script_reports_source_position() {
        let diagnostics = analyze_script("let value = ;");
        assert_eq!(diagnostics.len(), 1);
        assert_eq!(diagnostics[0].start_line, 1);
        assert!(diagnostics[0].start_column > 1);
        assert!(!diagnostics[0].message.is_empty());
    }
}
