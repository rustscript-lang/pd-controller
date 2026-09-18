//! Per-package source proof for frozen pd-edge and RustScript pins.
//!
//! Uses only `std` so a broken git pin can still be diagnosed without compiling
//! the workspace graph.

use std::path::PathBuf;

const PD_EDGE_GIT: &str = "https://github.com/rustscript-lang/pd-edge.git";
const PD_EDGE_REV: &str = "5f4f889e349bdfbd5534deb42bd13b616a6114f5";
const RUSTSCRIPT_GIT: &str = "https://github.com/rustscript-lang/rustscript.git";
const RUSTSCRIPT_REV: &str = "b1d6cffede77f49410bf63525f30b9a46b02dc01";
const ABBREVIATED_EDGE_REV: &str = "5f4f889";
const ABBREVIATED_RUSTSCRIPT_REV: &str = "b1d6cff";

fn manifest() -> String {
    std::fs::read_to_string(PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("Cargo.toml"))
        .expect("read Cargo.toml")
}

fn lockfile() -> String {
    std::fs::read_to_string(PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("Cargo.lock"))
        .expect("read Cargo.lock")
}

fn dependency_line<'a>(manifest: &'a str, key: &str) -> &'a str {
    let prefix = format!("{key} = {{");
    manifest
        .lines()
        .find(|line| line.trim_start().starts_with(&prefix))
        .unwrap_or_else(|| panic!("Cargo.toml must declare {key}"))
}

fn quoted_rev<'a>(dependency: &'a str, key: &str) -> Option<&'a str> {
    let needle = format!("{key} = \"");
    let start = dependency.find(&needle)? + needle.len();
    let end = start + dependency[start..].find('"')?;
    Some(&dependency[start..end])
}

fn assert_full_sha_pin(dependency: &str, crate_name: &str, expected: &str, abbreviated: &str) {
    let rev =
        quoted_rev(dependency, "rev").unwrap_or_else(|| panic!("{crate_name} must declare rev"));
    assert_eq!(
        rev.len(),
        40,
        "{crate_name} rev must be the full 40-character SHA, not an abbreviation: {rev}"
    );
    assert!(
        rev.chars().all(|ch| ch.is_ascii_hexdigit()),
        "{crate_name} rev must be hexadecimal: {rev}"
    );
    assert_eq!(
        rev, expected,
        "{crate_name} must pin the frozen full SHA {expected}, got {rev}"
    );
    assert_ne!(
        rev, abbreviated,
        "{crate_name} must not pin the abbreviated SHA {abbreviated}"
    );
    assert!(
        !dependency.contains("path ="),
        "{crate_name} must not depend on sibling checkout state: {dependency}"
    );
    assert!(
        !dependency.contains("branch ="),
        "{crate_name} must not pin a git branch: {dependency}"
    );
}

#[derive(Debug)]
struct LockPackage {
    name: String,
    version: String,
    source: Option<String>,
}

fn lock_packages(lock: &str) -> Vec<LockPackage> {
    let mut packages = Vec::new();
    let mut name = None;
    let mut version = None;
    let mut source = None;
    for line in lock.lines() {
        let line = line.trim();
        if line == "[[package]]" {
            if let (Some(name), Some(version)) = (name.take(), version.take()) {
                packages.push(LockPackage {
                    name,
                    version,
                    source: source.take(),
                });
            }
            continue;
        }
        if let Some(value) = line
            .strip_prefix("name = \"")
            .and_then(|rest| rest.strip_suffix('"'))
        {
            name = Some(value.to_string());
            continue;
        }
        if let Some(value) = line
            .strip_prefix("version = \"")
            .and_then(|rest| rest.strip_suffix('"'))
        {
            version = Some(value.to_string());
            continue;
        }
        if let Some(value) = line
            .strip_prefix("source = \"")
            .and_then(|rest| rest.strip_suffix('"'))
        {
            source = Some(value.to_string());
        }
    }
    if let (Some(name), Some(version)) = (name, version) {
        packages.push(LockPackage {
            name,
            version,
            source,
        });
    }
    packages
}

fn git_source(git: &str, rev: &str) -> String {
    format!("git+{git}?rev={rev}#{rev}")
}

fn assert_lock_source(packages: &[LockPackage], name: &str, version: &str, expected: &str) {
    let matches: Vec<&LockPackage> = packages
        .iter()
        .filter(|package| package.name == name && package.version == version)
        .collect();
    assert_eq!(
        matches.len(),
        1,
        "Cargo.lock must declare exactly one {name} {version}, found {}",
        matches.len()
    );
    let source = matches[0]
        .source
        .as_deref()
        .unwrap_or_else(|| panic!("Cargo.lock {name} {version} must declare a source"));
    assert_eq!(
        source, expected,
        "Cargo.lock {name} {version} must use the frozen git SHA source"
    );
}

#[test]
fn pd_edge_uses_the_frozen_full_git_sha() {
    let manifest = manifest();
    let dependency = dependency_line(&manifest, "edge");
    assert!(
        dependency.contains(&format!("git = \"{PD_EDGE_GIT}\"")),
        "pd-edge must use the canonical HTTPS Git remote: {dependency}"
    );
    assert_full_sha_pin(dependency, "pd-edge", PD_EDGE_REV, ABBREVIATED_EDGE_REV);
}

#[test]
fn pd_vm_uses_the_frozen_full_git_sha() {
    let manifest = manifest();
    let dependency = dependency_line(&manifest, "vm");
    assert!(
        dependency.contains(&format!("git = \"{RUSTSCRIPT_GIT}\"")),
        "pd-vm must use the canonical HTTPS Git remote: {dependency}"
    );
    assert_full_sha_pin(
        dependency,
        "pd-vm",
        RUSTSCRIPT_REV,
        ABBREVIATED_RUSTSCRIPT_REV,
    );
}

#[test]
fn lockfile_pins_pd_edge_and_rustscript_crates_per_package() {
    let packages = lock_packages(&lockfile());
    let edge_source = git_source(PD_EDGE_GIT, PD_EDGE_REV);
    let rustscript_source = git_source(RUSTSCRIPT_GIT, RUSTSCRIPT_REV);

    assert_lock_source(&packages, "pd-edge", "0.1.0", &edge_source);
    assert_lock_source(&packages, "pd-edge-abi", "0.1.0", &edge_source);
    assert_lock_source(&packages, "pd-edge-host-function", "0.1.0", &edge_source);
    assert_lock_source(&packages, "pd-vm", "0.1.0", &rustscript_source);
    assert_lock_source(&packages, "pd-host-function", "0.1.0", &rustscript_source);
    assert_lock_source(&packages, "pd-host-schema", "0.1.0", &rustscript_source);

    let registry_family: Vec<&LockPackage> = packages
        .iter()
        .filter(|package| {
            package
                .source
                .as_deref()
                .is_some_and(|source| source.starts_with("registry+"))
                && (package.name.starts_with("pd-edge")
                    || package.name.starts_with("pd-host-")
                    || package.name.starts_with("pd-vm"))
        })
        .collect();
    assert!(
        registry_family.is_empty(),
        "lockfile must not resolve registry pd-edge*/pd-host-*/pd-vm* packages, found {registry_family:?}"
    );

    for package in &packages {
        if let Some(source) = &package.source {
            assert!(
                !source.contains(&format!("rev={ABBREVIATED_EDGE_REV}\"")),
                "{} {} must not resolve an abbreviated pd-edge SHA: {source}",
                package.name,
                package.version
            );
            assert!(
                !source.contains(&format!("rev={ABBREVIATED_RUSTSCRIPT_REV}\"")),
                "{} {} must not resolve an abbreviated RustScript SHA: {source}",
                package.name,
                package.version
            );
            assert!(
                !source.starts_with("path+") && !source.starts_with("file+"),
                "{} {} must not resolve a path/file source: {source}",
                package.name,
                package.version
            );
        }
    }
}
