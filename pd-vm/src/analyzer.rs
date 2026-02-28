use crate::compiler::{SourceError, SourceFlavor, compile_source_with_flavor};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct LintDiagnostic {
    pub line: usize,
    pub message: String,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct LintReport {
    pub diagnostics: Vec<LintDiagnostic>,
}

impl LintReport {
    pub fn ok() -> Self {
        Self {
            diagnostics: Vec::new(),
        }
    }

    pub fn has_errors(&self) -> bool {
        !self.diagnostics.is_empty()
    }
}

pub fn lint_source_with_flavor(source: &str, flavor: SourceFlavor) -> LintReport {
    match compile_source_with_flavor(source, flavor) {
        Ok(_) => LintReport::ok(),
        Err(SourceError::Parse(err)) => LintReport {
            diagnostics: vec![LintDiagnostic {
                line: err.line,
                message: err.message,
            }],
        },
        Err(SourceError::Compile(err)) => LintReport {
            diagnostics: vec![LintDiagnostic {
                line: 0,
                message: format!("compile error: {err:?}"),
            }],
        },
    }
}

pub fn lint_source(source: &str) -> LintReport {
    lint_source_with_flavor(source, SourceFlavor::RustScript)
}
