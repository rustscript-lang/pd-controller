use vm::{SourceFlavor, lint_source_with_flavor};

#[test]
fn lint_reports_no_errors_for_all_supported_frontends() {
    let cases = [
        (
            SourceFlavor::RustScript,
            include_str!("../examples/example.rss"),
        ),
        (
            SourceFlavor::JavaScript,
            include_str!("../examples/example.js"),
        ),
        (SourceFlavor::Lua, include_str!("../examples/example.lua")),
        (
            SourceFlavor::Scheme,
            include_str!("../examples/example.scm"),
        ),
    ];

    for (flavor, source) in cases {
        let report = lint_source_with_flavor(source, flavor);
        assert!(
            !report.has_errors(),
            "lint should succeed for {flavor:?}, got diagnostics: {:?}",
            report.diagnostics
        );
    }
}

#[test]
fn lint_reports_syntax_errors_for_all_supported_frontends() {
    let cases = [
        (SourceFlavor::RustScript, "let value = ;"),
        (SourceFlavor::JavaScript, "let value = ;"),
        (SourceFlavor::Lua, "local value = "),
        (SourceFlavor::Scheme, "(define value"),
    ];

    for (flavor, source) in cases {
        let report = lint_source_with_flavor(source, flavor);
        assert!(report.has_errors(), "lint should fail for {flavor:?}",);
        assert!(
            !report.diagnostics[0].message.trim().is_empty(),
            "expected non-empty diagnostic message for {flavor:?}",
        );
    }
}
