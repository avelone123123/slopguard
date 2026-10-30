use crate::types::{DiffFile, DiffLineType, SlopCategory, SlopFinding, SlopSeverity};
use regex::Regex;
use serde_json::Value;
use std::collections::HashSet;
use std::fs;
use std::path::Path;
fn get_node_builtins() -> HashSet<&'static str> {
    [
        "assert", "async_hooks", "buffer", "child_process", "cluster", "console", "constants",
        "crypto", "dgram", "diagnostics_channel", "dns", "domain", "events", "fs", "fs/promises",
        "http", "http2", "https", "inspector", "module", "net", "os", "path", "path/posix",
        "path/win32", "perf_hooks", "process", "punycode", "querystring", "readline", "repl",
        "stream", "stream/consumers", "stream/promises", "stream/web", "string_decoder", "timers",
        "timers/promises", "tls", "trace_events", "tty", "url", "util", "util/types", "v8", "vm",
        "wasi", "worker_threads", "zlib",
    ].into_iter().collect()
}
pub fn get_declared_dependencies(project_root: &Path) -> HashSet<String> {
    let mut deps = HashSet::new();
    let pkg_path = project_root.join("package.json");
    if pkg_path.exists() {
        if let Ok(content) = fs::read_to_string(&pkg_path) {
            if let Ok(val) = serde_json::from_str::<Value>(&content) {
                let sections = ["dependencies", "devDependencies", "peerDependencies", "optionalDependencies"];
                for sec in sections {
                    if let Some(obj) = val.get(sec).and_then(|v| v.as_object()) {
                        for k in obj.keys() {
                            deps.insert(k.clone());
                        }
                    }
                }
            }
        }
    }
    deps
}
fn extract_package_name(raw: &str) -> String {
    if raw.starts_with("node:") {
        return raw.to_string();
    }
    if raw.starts_with('@') {
        let parts: Vec<&str> = raw.split('/').collect();
        if parts.len() >= 2 {
            return format!("{}/{}", parts[0], parts[1]);
        }
    }
    raw.split('/').next().unwrap_or(raw).to_string()
}
fn is_js_ts(path_str: &str) -> bool {
    let lower = path_str.to_lowercase();
    lower.ends_with(".ts") || lower.ends_with(".tsx") || lower.ends_with(".js") || lower.ends_with(".jsx") || lower.ends_with(".mjs") || lower.ends_with(".cjs")
}
pub fn detect_ghost_dependencies_in_diff(diff_files: &[DiffFile], project_root: &Path, allowed_ghost_deps: &[String]) -> Vec<SlopFinding> {
    let mut findings = Vec::new();
    let declared = get_declared_dependencies(project_root);
    let allowed: HashSet<&str> = allowed_ghost_deps.iter().map(|s| s.as_str()).collect();
    let builtins = get_node_builtins();
    let re = Regex::new(r#"^\s*(?:import\s+(?:[\w*\s{},]*\s+from\s+)?|export\s+[\w*\s{},]*\s+from\s+|(?:const|let|var)\s+[\w*\s{},]+\s*=\s*require\s*\()\s*["']([^"']+)["']"#).unwrap();
    for file in diff_files {
        if file.is_deleted {
            continue;
        }
        let target_path = if !file.new_path.is_empty() { &file.new_path } else { &file.old_path };
        if !is_js_ts(target_path) {
            continue;
        }
        for chunk in &file.chunks {
            for line in &chunk.lines {
                if line.line_type != DiffLineType::Add {
                    continue;
                }
                let line_num = line.new_line_number.unwrap_or(1);
                if let Some(caps) = re.captures(&line.content) {
                    if let Some(import_match) = caps.get(1) {
                        let raw = import_match.as_str();
                        if raw.starts_with('.') || raw.starts_with('/') || raw.starts_with('~') || raw.starts_with("@/") || raw.starts_with("node:") {
                            continue;
                        }
                        let pkg = extract_package_name(raw);
                        if builtins.contains(pkg.as_str()) {
                            continue;
                        }
                        if !declared.contains(&pkg) && !allowed.contains(pkg.as_str()) {
                            findings.push(SlopFinding {
                                file: target_path.to_string(),
                                line: line_num,
                                column: import_match.start() + 1,
                                rule_id: "no-ghost-dependencies".to_string(),
                                category: SlopCategory::GhostDependency,
                                severity: SlopSeverity::Error,
                                message: format!("Ghost dependency '{pkg}' imported by AI but not declared in package.json"),
                                snippet: line.content.trim().to_string(),
                                fixable: false,
                                suggestion: Some(format!("Run 'npm install {pkg}' or remove hallucinated import")),
                            });
                        }
                    }
                }
            }
        }
    }
    findings
}
pub fn detect_ghost_dependencies_in_content(file_path: &str, content: &str, project_root: &Path, allowed_ghost_deps: &[String]) -> Vec<SlopFinding> {
    let mut findings = Vec::new();
    if !is_js_ts(file_path) {
        return findings;
    }
    let declared = get_declared_dependencies(project_root);
    let allowed: HashSet<&str> = allowed_ghost_deps.iter().map(|s| s.as_str()).collect();
    let builtins = get_node_builtins();
    let re = Regex::new(r#"^\s*(?:import\s+(?:[\w*\s{},]*\s+from\s+)?|export\s+[\w*\s{},]*\s+from\s+|(?:const|let|var)\s+[\w*\s{},]+\s*=\s*require\s*\()\s*["']([^"']+)["']"#).unwrap();
    for (idx, line) in content.lines().enumerate() {
        let line_num = idx + 1;
        if let Some(caps) = re.captures(line) {
            if let Some(import_match) = caps.get(1) {
                let raw = import_match.as_str();
                if raw.starts_with('.') || raw.starts_with('/') || raw.starts_with('~') || raw.starts_with("@/") || raw.starts_with("node:") {
                    continue;
                }
                let pkg = extract_package_name(raw);
                if builtins.contains(pkg.as_str()) {
                    continue;
                }
                if !declared.contains(&pkg) && !allowed.contains(pkg.as_str()) {
                    findings.push(SlopFinding {
                        file: file_path.to_string(),
                        line: line_num,
                        column: import_match.start() + 1,
                        rule_id: "no-ghost-dependencies".to_string(),
                        category: SlopCategory::GhostDependency,
                        severity: SlopSeverity::Error,
                        message: format!("Ghost dependency '{pkg}' imported by AI but not declared in package.json"),
                        snippet: line.trim().to_string(),
                        fixable: false,
                        suggestion: Some(format!("Run 'npm install {pkg}' or remove hallucinated import")),
                    });
                }
            }
        }
    }
    findings
}