use crate::types::{DiffFile, SlopCategory, SlopFinding, SlopSeverity};
use regex::Regex;
use std::collections::HashSet;
use std::fs;
use std::path::{Path, PathBuf};
fn extract_relative_imports(content: &str) -> Vec<String> {
    let mut imports = Vec::new();
    let re_esm = Regex::new(r#"(?:import|from|require)\s*\(?\s*["'](\.[^"']+)["']"#).unwrap();
    for cap in re_esm.captures_iter(content) {
        if let Some(m) = cap.get(1) {
            imports.push(m.as_str().to_string());
        }
    }
    let re_py = Regex::new(r#"from\s+(\.[a-zA-Z0-9_.]+)\s+import"#).unwrap();
    for cap in re_py.captures_iter(content) {
        if let Some(m) = cap.get(1) {
            imports.push(m.as_str().to_string());
        }
    }
    imports
}
fn resolve_import_path(base_file: &Path, rel_import: &str) -> Option<PathBuf> {
    let parent = base_file.parent()?;
    let target = parent.join(rel_import);
    let candidates = [
        target.clone(),
        target.with_extension("ts"),
        target.with_extension("tsx"),
        target.with_extension("js"),
        target.with_extension("jsx"),
        target.with_extension("py"),
        target.join("index.ts"),
        target.join("index.js"),
    ];
    for c in candidates {
        if c.is_file() {
            return Some(c);
        }
    }
    None
}
fn normalize_path_str(p: &Path, root: &Path) -> String {
    let rel = p.strip_prefix(root).unwrap_or(p);
    let s = rel.to_string_lossy().replace('\\', "/");
    s.replace("/./", "/").replace("./", "")
}
pub fn detect_circular_dependencies_in_content(file_path: &str, content: &str, project_root: &Path) -> Vec<SlopFinding> {
    let mut findings = Vec::new();
    let full_base = project_root.join(file_path);
    let norm_base = normalize_path_str(&full_base, project_root);
    let imports = extract_relative_imports(content);
    let mut visited = HashSet::new();
    for imp in imports {
        if let Some(resolved) = resolve_import_path(&full_base, &imp) {
            let norm_dep = normalize_path_str(&resolved, project_root);
            if norm_dep == norm_base {
                findings.push(SlopFinding {
                    file: file_path.to_string(),
                    line: 1,
                    column: 1,
                    rule_id: "no-circular-dependencies".to_string(),
                    category: SlopCategory::GhostDependency,
                    severity: SlopSeverity::Error,
                    message: format!("Self-referential circular import: '{}' imports itself", file_path),
                    snippet: imp,
                    fixable: false,
                    suggestion: Some("Remove self-referencing module import".to_string()),
                });
                continue;
            }
            if let Ok(dep_content) = fs::read_to_string(&resolved) {
                let second_hops = extract_relative_imports(&dep_content);
                for second_imp in second_hops {
                    if let Some(second_resolved) = resolve_import_path(&resolved, &second_imp) {
                        let norm_second = normalize_path_str(&second_resolved, project_root);
                        if norm_second == norm_base && visited.insert(norm_dep.clone()) {
                            findings.push(SlopFinding {
                                file: file_path.to_string(),
                                line: 1,
                                column: 1,
                                rule_id: "no-circular-dependencies".to_string(),
                                category: SlopCategory::GhostDependency,
                                severity: SlopSeverity::Error,
                                message: format!("Circular dependency cycle detected: '{}' <-> '{}'", norm_base, norm_dep),
                                snippet: format!("import from '{}' creates a runtime cycle", norm_dep),
                                fixable: false,
                                suggestion: Some(format!("Extract shared interfaces or types into a separate module to break the '{}' <-> '{}' cycle", norm_base, norm_dep)),
                            });
                        }
                    }
                }
            }
        }
    }
    findings
}
pub fn detect_circular_dependencies_in_diff(diff_files: &[DiffFile], project_root: &Path) -> Vec<SlopFinding> {
    let mut findings = Vec::new();
    for f in diff_files {
        let path = if !f.new_path.is_empty() { &f.new_path } else { &f.old_path };
        let full_path = project_root.join(path);
        if full_path.is_file() {
            if let Ok(content) = fs::read_to_string(&full_path) {
                findings.extend(detect_circular_dependencies_in_content(path, &content, project_root));
            }
        }
    }
    findings
}