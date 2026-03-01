use super::ParseError;
use super::source_map::{SourceMap, Span};

pub fn render_source_error(source_map: &SourceMap, err: &ParseError, _styled: bool) -> String {
    let code_prefix = err
        .code
        .as_deref()
        .map(|code| format!("error[{code}]"))
        .unwrap_or_else(|| "error".to_string());

    if let Some(span) = err.span
        && let Some(rendered) = render_span_snippet(source_map, span, &err.message)
    {
        return format!("{code_prefix}: {}", rendered.trim_end());
    }

    format!("{code_prefix}: line {}: {}", err.line, err.message)
}

fn render_span_snippet(source_map: &SourceMap, span: Span, message: &str) -> Option<String> {
    let file = source_map.file(span.source_id)?;
    let (line, col) = file.line_col_for_offset(span.lo)?;
    let line_text = file.line_text(line)?;
    let pointer_width = span.len().max(1);
    let pointer = format!(
        "{}{}",
        " ".repeat(col.saturating_sub(1)),
        "^".repeat(pointer_width)
    );
    Some(format!(
        "{message}\n --> {}:{line}:{col}\n  |\n{line:>3} | {line_text}\n  | {pointer}",
        file.name
    ))
}
