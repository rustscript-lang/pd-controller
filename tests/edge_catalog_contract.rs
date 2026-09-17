use std::{
    fs,
    path::{Path, PathBuf},
};

use edge::{ABI_VERSION, HOST_FUNCTION_COUNT, compile_edge_source_with_flavor, function_by_name};
#[cfg(feature = "mqtt")]
use vm::HostTypeSchema;
use vm::SourceFlavor;

/// Exact hex form of the compile catalog fingerprint for the active feature set.
/// Default (mqtt-off) stays on the frozen ABI25 HTTP/TLS/WS catalog. Enabling
/// `mqtt` joins MQTT hosts, so the catalog identity changes with that feature.
#[cfg(not(feature = "mqtt"))]
const EDGE_CATALOG_FINGERPRINT_HEX: &str = "ff4fc9ca114a15c1";
#[cfg(not(feature = "mqtt"))]
const EDGE_CATALOG_FINGERPRINT_U64: u64 = 0xff4fc9ca114a15c1;
#[cfg(feature = "mqtt")]
const EDGE_CATALOG_FINGERPRINT_HEX: &str = "b5660b84004c864c";
#[cfg(feature = "mqtt")]
const EDGE_CATALOG_FINGERPRINT_U64: u64 = 0xb5660b84004c864c;

const HTTP_SHORT_CIRCUIT: &str = "use http;\nhttp::response::set_body(\"catalog-contract\");\n";
const HTTP_SET_STATUS: &str = "use http;\nhttp::response::set_status(204);\n";
const TCP_NEW: &str = "use tcp;\nlet stream = tcp::stream::new();\n";
const TLS_FROM_SOCKET: &str = "use tcp;\nuse tls;\nlet stream = tcp::stream::new();\nlet session = tls::session::from_socket(stream);\n";
const WEBSOCKET_NEW: &str = "use websocket;\nlet ws = websocket::connection::new();\n";
const UDP_NEW: &str = "use udp;\nlet socket = udp::socket::new();\n";
const PROXY_DOWNSTREAM: &str = "use proxy;\nlet downstream = proxy::stream::downstream();\n";
#[cfg(feature = "mqtt")]
const MQTT_READ_EVENT: &str = r#"
use mqtt;
let connection = mqtt::connection::new();
let event = mqtt::connection::read_event(connection);
"#;
#[cfg(not(feature = "mqtt"))]
const MQTT_UNBOUND_SOURCE: &str =
    "use mqtt;\nmqtt::connection::new();\nmqtt::connection::read_event(0);\n";
const WEBRTC_UI_SOURCE: &str = "use vm;\nlet rtc: int = vm::webrtc::connection::new();\n";

fn compile_rss(source: &str) -> vm::CompiledProgram {
    compile_edge_source_with_flavor(source, SourceFlavor::RustScript).unwrap_or_else(|err| {
        panic!("source must compile through the migrated edge catalog: {err}\n{source}")
    })
}

fn assert_exact_catalog_schema(schema: &vm::HostImportSchema, expected_name: &str) {
    assert_eq!(schema.name, expected_name);
    assert_eq!(
        format!("{}", schema.fingerprint),
        EDGE_CATALOG_FINGERPRINT_HEX
    );
    assert_eq!(schema.fingerprint.as_u64(), EDGE_CATALOG_FINGERPRINT_U64);
}

fn assert_exact_catalog_imports(program: &vm::Program, expected_names: &[&str]) {
    let schemas = program.host_import_schemas();
    assert_eq!(
        schemas.len(),
        program.imports.len(),
        "host import schemas must stay aligned with imports"
    );
    for (import, schema) in program.imports.iter().zip(schemas.iter()) {
        let schema = schema.as_ref().unwrap_or_else(|| {
            panic!(
                "{} must carry an exact ABI25 catalog schema, not a missing/stale fallback",
                import.name
            )
        });
        assert_exact_catalog_schema(schema, &import.name);
    }
    for name in expected_names {
        let (index, _) = program
            .imports
            .iter()
            .enumerate()
            .find(|(_, import)| import.name == *name)
            .unwrap_or_else(|| panic!("fixture must import {name}"));
        let schema = schemas[index]
            .as_ref()
            .unwrap_or_else(|| panic!("{name} must have Some(schema) with the ABI25 fingerprint"));
        assert_exact_catalog_schema(schema, name);
    }
}

fn protocol_import_matches(name: &str, root: &str) -> bool {
    name.starts_with(&format!("{root}::")) || name.contains(&format!("::{root}::"))
}

fn assert_unbound_protocol_imports(program: &vm::Program, root: &str) {
    let schemas = program.host_import_schemas();
    assert_eq!(
        schemas.len(),
        program.imports.len(),
        "host import schemas must stay aligned with imports"
    );
    let mut seen = 0usize;
    for (import, schema) in program.imports.iter().zip(schemas.iter()) {
        if !protocol_import_matches(&import.name, root) {
            continue;
        }
        seen += 1;
        assert!(
            schema.is_none(),
            "{} must remain unbound (None schema) when {root} is omitted from the catalog, got {schema:?}",
            import.name
        );
    }
    assert!(
        seen > 0,
        "compiled source must include {root} imports to prove unbound None schemas; imports={:?}",
        program
            .imports
            .iter()
            .map(|import| import.name.as_str())
            .collect::<Vec<_>>()
    );
}

fn assert_fail_closed_protocol_error(err: impl std::fmt::Display, root: &str) {
    let message = err.to_string();
    assert!(
        message.contains("unknown host function") || message.contains(root),
        "fail-closed {root} compile error must identify the omitted host, got: {message}"
    );
}

fn assert_default_off_protocol_compile(source: &str, root: &str) {
    match compile_edge_source_with_flavor(source, SourceFlavor::RustScript) {
        Err(err) => assert_fail_closed_protocol_error(err, root),
        Ok(compiled) => assert_unbound_protocol_imports(&compiled.program, root),
    }
}

fn catalog_fingerprint(program: &vm::Program) -> vm::HostApiFingerprint {
    let schemas = program.host_import_schemas();
    assert_eq!(schemas.len(), program.imports.len());
    let mut fingerprint = None;
    for (import, schema) in program.imports.iter().zip(schemas.iter()) {
        let schema = schema
            .as_ref()
            .unwrap_or_else(|| panic!("{} must carry an exact catalog schema", import.name));
        match fingerprint {
            None => fingerprint = Some(schema.fingerprint),
            Some(existing) => assert_eq!(
                existing, schema.fingerprint,
                "every exact catalog import must share one fingerprint"
            ),
        }
    }
    fingerprint.expect("compiled edge program must carry catalog import fingerprints")
}

fn walk_rss_files(root: &Path, files: &mut Vec<PathBuf>) {
    let Ok(entries) = fs::read_dir(root) else {
        return;
    };
    for entry in entries.flatten() {
        let path = entry.path();
        let name = entry.file_name();
        let name = name.to_string_lossy();
        if name == ".git"
            || name == "target"
            || name == "node_modules"
            || name == "webui"
            || name == "tmp"
        {
            continue;
        }
        if path.is_dir() {
            walk_rss_files(&path, files);
            continue;
        }
        if path.extension().and_then(|ext| ext.to_str()) == Some("rss") {
            files.push(path);
        }
    }
}

#[cfg(feature = "mqtt")]
fn assert_mqtt_event_fields(schema: &HostTypeSchema) {
    match schema {
        HostTypeSchema::Named { name, fields } => {
            assert_eq!(name, "MqttEvent");
            let expected = [
                ("kind", HostTypeSchema::String),
                (
                    "topic",
                    HostTypeSchema::Optional(Box::new(HostTypeSchema::String)),
                ),
                (
                    "payload_text",
                    HostTypeSchema::Optional(Box::new(HostTypeSchema::String)),
                ),
                (
                    "payload_base64",
                    HostTypeSchema::Optional(Box::new(HostTypeSchema::String)),
                ),
                (
                    "qos",
                    HostTypeSchema::Optional(Box::new(HostTypeSchema::Int)),
                ),
                (
                    "retain",
                    HostTypeSchema::Optional(Box::new(HostTypeSchema::Bool)),
                ),
                (
                    "dup",
                    HostTypeSchema::Optional(Box::new(HostTypeSchema::Bool)),
                ),
                (
                    "reason",
                    HostTypeSchema::Optional(Box::new(HostTypeSchema::String)),
                ),
            ];
            assert_eq!(fields.len(), expected.len(), "MqttEvent field count");
            for (field, (expected_name, expected_ty)) in fields.iter().zip(expected) {
                assert_eq!(field.name, expected_name);
                assert_eq!(field.ty, expected_ty);
            }
        }
        other => panic!("expected Named MqttEvent, got {other:?}"),
    }
}

#[test]
fn published_abi_is_version_25() {
    assert_eq!(ABI_VERSION, 25);
    for index in 0..HOST_FUNCTION_COUNT {
        let function = edge::function_by_index(index)
            .unwrap_or_else(|| panic!("ABI function {index} must exist"));
        assert_eq!(function.index, index);
        assert_eq!(
            function_by_name(function.name).map(|item| item.index),
            Some(index)
        );
    }
    assert!(function_by_name("http::response::set_body").is_some());
}

#[test]
fn edge_catalog_fingerprint_is_golden_and_stable() {
    let first = compile_rss(HTTP_SHORT_CIRCUIT);
    let second = compile_rss(HTTP_SHORT_CIRCUIT);
    let fingerprint = catalog_fingerprint(&first.program);
    assert_eq!(fingerprint, catalog_fingerprint(&second.program));
    assert_eq!(format!("{fingerprint}"), EDGE_CATALOG_FINGERPRINT_HEX);
    assert_eq!(fingerprint.as_u64(), EDGE_CATALOG_FINGERPRINT_U64);
}

#[cfg(not(feature = "mqtt"))]
#[test]
fn mqtt_is_absent_from_the_default_production_catalog() {
    assert!(function_by_name("mqtt::connection::read_event").is_none());
    assert!(function_by_name("mqtt::connection::new").is_none());
    assert_default_off_protocol_compile(MQTT_UNBOUND_SOURCE, "mqtt");
}

#[test]
fn webrtc_ui_source_is_unbound_when_default_off() {
    assert!(function_by_name("webrtc::connection::new").is_none());
    assert_default_off_protocol_compile(WEBRTC_UI_SOURCE, "webrtc");
}

#[cfg(feature = "mqtt")]
#[test]
fn named_mqtt_event_is_the_exact_catalog_binding() {
    let spec = function_by_name("mqtt::connection::read_event")
        .expect("canonical mqtt::connection::read_event must be published when mqtt is enabled");
    assert_eq!(spec.name, "mqtt::connection::read_event");
    assert!(function_by_name("mqtt::connection::new").is_some());

    let compiled = compile_rss(MQTT_READ_EVENT);
    assert_exact_catalog_imports(
        &compiled.program,
        &["mqtt::connection::new", "mqtt::connection::read_event"],
    );

    let schema = compiled
        .program
        .host_import_schemas()
        .iter()
        .flatten()
        .find(|schema| schema.name == "mqtt::connection::read_event")
        .expect("mqtt::connection::read_event must have Some(schema)");
    assert_mqtt_event_fields(&schema.return_type);
    assert_eq!(
        format!("{}", schema.fingerprint),
        EDGE_CATALOG_FINGERPRINT_HEX
    );
}

#[test]
fn checked_in_rss_and_controller_fixtures_compile_through_the_edge_catalog() {
    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let mut files = Vec::new();
    walk_rss_files(&root, &mut files);
    files.sort();
    assert!(
        files.is_empty(),
        "pd-controller has no checked-in RSS fixtures; found {files:?}"
    );

    let fixtures: &[(&str, &[&str])] = &[
        (HTTP_SHORT_CIRCUIT, &["http::response::set_body"]),
        (HTTP_SET_STATUS, &["http::response::set_status"]),
        (TCP_NEW, &["tcp::stream::new"]),
        (
            TLS_FROM_SOCKET,
            &["tcp::stream::new", "tls::session::from_socket"],
        ),
        (WEBSOCKET_NEW, &["websocket::connection::new"]),
        (UDP_NEW, &["udp::socket::new"]),
        (PROXY_DOWNSTREAM, &["proxy::stream::downstream"]),
    ];
    for (source, expected) in fixtures {
        let compiled = compile_rss(source);
        assert_exact_catalog_imports(&compiled.program, expected);
    }

    #[cfg(feature = "mqtt")]
    {
        let compiled = compile_rss(MQTT_READ_EVENT);
        assert_exact_catalog_imports(
            &compiled.program,
            &["mqtt::connection::new", "mqtt::connection::read_event"],
        );
    }
}
