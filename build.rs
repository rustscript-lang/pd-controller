use std::{
    env, fs,
    path::{Path, PathBuf},
    process::Command,
};

fn main() {
    emit_git_build_metadata();

    let manifest_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").expect("CARGO_MANIFEST_DIR"));
    let dist_dir = manifest_dir.join("webui").join("dist");
    let out_dir = PathBuf::from(env::var("OUT_DIR").expect("OUT_DIR"));
    let output = out_dir.join("embedded_webui.rs");

    println!("cargo:rerun-if-changed={}", dist_dir.display());

    let mut files = Vec::new();
    if dist_dir.exists() {
        collect_files(&dist_dir, &dist_dir, &mut files);
        files.sort_by(|a, b| a.0.cmp(&b.0));
    }

    let mut source = String::new();
    if files.is_empty() {
        source.push_str("pub fn has_assets() -> bool { false }\n");
        source.push_str("pub fn get_asset(_path: &str) -> Option<&'static [u8]> { None }\n");
    } else {
        source.push_str("pub fn has_assets() -> bool { true }\n");
        source.push_str("pub fn get_asset(path: &str) -> Option<&'static [u8]> {\n");
        source.push_str("    match path {\n");
        for (relative, absolute) in files {
            source.push_str(&format!(
                "        \"{}\" => Some(include_bytes!(r#\"{}\"#)),\n",
                relative, absolute
            ));
        }
        source.push_str("        _ => None,\n");
        source.push_str("    }\n");
        source.push_str("}\n");
    }

    fs::write(&output, source).expect("failed to write embedded_webui.rs");
}

fn emit_git_build_metadata() {
    println!("cargo:rerun-if-env-changed=PD_BUILD_GIT_TAG");
    println!("cargo:rerun-if-env-changed=PD_BUILD_GIT_COMMIT");
    println!("cargo:rerun-if-env-changed=PD_BUILD_GIT_DIRTY");

    let git_tag = env::var("PD_BUILD_GIT_TAG").unwrap_or_else(|_| {
        run_git(["describe", "--tags", "--exact-match"]).unwrap_or_else(|| "untagged".to_string())
    });
    let git_commit = env::var("PD_BUILD_GIT_COMMIT").unwrap_or_else(|_| {
        run_git(["rev-parse", "--short=12", "HEAD"]).unwrap_or_else(|| "unknown".to_string())
    });
    let git_dirty = env::var("PD_BUILD_GIT_DIRTY").unwrap_or_else(|_| {
        match run_git(["status", "--porcelain", "--untracked-files=no"]) {
            Some(output) if !output.trim().is_empty() => "true".to_string(),
            _ => "false".to_string(),
        }
    });

    println!("cargo:rustc-env=PD_BUILD_GIT_TAG={git_tag}");
    println!("cargo:rustc-env=PD_BUILD_GIT_COMMIT={git_commit}");
    println!("cargo:rustc-env=PD_BUILD_GIT_DIRTY={git_dirty}");
}

fn run_git<const N: usize>(args: [&str; N]) -> Option<String> {
    let output = Command::new("git").args(args).output().ok()?;
    if !output.status.success() {
        return None;
    }
    String::from_utf8(output.stdout)
        .ok()
        .map(|value| value.trim().to_string())
}

fn collect_files(base: &Path, dir: &Path, files: &mut Vec<(String, String)>) {
    let Ok(entries) = fs::read_dir(dir) else {
        return;
    };
    for entry in entries.flatten() {
        let path = entry.path();
        if path.is_dir() {
            collect_files(base, &path, files);
            continue;
        }
        let Ok(relative) = path.strip_prefix(base) else {
            continue;
        };
        let rel = relative.to_string_lossy().replace('\\', "/");
        let abs = path.to_string_lossy().replace('\\', "/");
        files.push((rel, abs));
    }
}
