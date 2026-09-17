use std::{
    fs,
    path::{Path, PathBuf},
};

use edge::{ABI_VERSION, HOST_FUNCTION_COUNT, compile_edge_source_with_flavor, function_by_name};
use vm::{HostTypeSchema, SourceFlavor};

/// Exact hex form of the default pd-edge compile catalog fingerprint.
/// Bump together with [`EDGE_CATALOG_FINGERPRINT_U64`] when the frozen edge
/// ABI surface or pd-vm fingerprint encoding changes.
const EDGE_CATALOG_FINGERPRINT_HEX: &str = "ff4fc9ca114a15c1";
const EDGE_CATALOG_FINGERPRINT_U64: u64 = 0xff4fc9ca114a15c1;

const HTTP_SHORT_CIRCUIT: &str = "use http;\nhttp::response::set_body(\"catalog-contract\");\n";
const MQTT_READ_EVENT: &str = r#"
use mqtt;
let connection = mqtt::connection::new();
let event = mqtt::connection::read_event(connection);
"#;

fn compile_rss(source: &str) -> vm::CompiledProgram {
    compile_edge_source_with_flavor(source, SourceFlavor::RustScript).unwrap_or_else(|err| {
        panic!("source must compile through the migrated edge catalog: {err}\n{source}")
    })
}

fn catalog_fingerprint(program: &vm::Program) -> vm::HostApiFingerprint {
    let schemas = program.host_import_schemas();
    let mut fingerprint = None;
    for schema in schemas.iter().flatten() {
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

#[test]
fn named_mqtt_event_is_the_exact_catalog_binding_when_present() {
    let compiled = compile_rss(MQTT_READ_EVENT);
    assert!(
        compiled
            .program
            .imports
            .iter()
            .any(|import| import.name == "mqtt::connection::read_event"),
        "migrated edge compile catalog must bind mqtt::connection::read_event"
    );

    let schema = compiled
        .program
        .host_import_schemas()
        .iter()
        .flatten()
        .find(|schema| schema.name == "mqtt::connection::read_event");

    match (function_by_name("mqtt::connection::read_event"), schema) {
        (Some(spec), Some(schema)) => {
            assert_eq!(spec.name, "mqtt::connection::read_event");
            match &schema.return_type {
                HostTypeSchema::Named { name, .. } => {
                    assert_eq!(name, "MqttEvent");
                }
                other => panic!("expected Named MqttEvent, got {other:?}"),
            }
            assert_eq!(
                format!("{}", schema.fingerprint),
                EDGE_CATALOG_FINGERPRINT_HEX
            );
        }
        (None, None) => {
            assert_eq!(ABI_VERSION, 25);
            assert!(
                function_by_name("http::response::set_body").is_some(),
                "default controller catalog keeps HTTP hosts when MQTT stays ABI-gated"
            );
        }
        (abi, exact) => panic!(
            "mqtt ABI publication and exact catalog schema must stay aligned; abi={:?} exact={}",
            abi.map(|spec| spec.name),
            exact.is_some()
        ),
    }
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

    let fixtures = [
        HTTP_SHORT_CIRCUIT,
        "use http;\nhttp::response::set_status(204);\n",
        "use vm;\nvm::http::response::set_body(\"vm-ns\");\n",
        "use tcp;\nlet stream = tcp::stream::new();\n",
        "use tcp;\nuse tls;\nlet stream = tcp::stream::new();\nlet session = tls::session::from_socket(stream);\n",
        "use websocket;\nlet ws = websocket::connection::new();\n",
        "use udp;\nlet socket = udp::socket::new();\n",
        "use proxy;\nlet downstream = proxy::stream::downstream();\n",
        MQTT_READ_EVENT,
    ];
    for source in fixtures {
        let compiled = compile_rss(source);
        assert!(
            !compiled.program.host_import_schemas().is_empty()
                || compiled
                    .program
                    .imports
                    .iter()
                    .any(|import| import.name.contains("http")
                        || import.name.contains("tcp")
                        || import.name.contains("tls")
                        || import.name.contains("websocket")
                        || import.name.contains("udp")
                        || import.name.contains("proxy")
                        || import.name.contains("response")),
            "fixture must record host imports:\n{source}"
        );
    }
}
