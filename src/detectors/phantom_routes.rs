use crate::types::{DiffFile, DiffLineType, SlopCategory, SlopFinding, SlopSeverity};
use regex::Regex;
use std::collections::HashSet;
use std::path::Path;
use walkdir::WalkDir;
fn index_existing_api_routes(project_root: &Path) -> HashSet<String> {
    let mut routes = HashSet::new();
    let walker = WalkDir::new(project_root).into_iter().filter_entry(|e| {
        let n = e.file_name().to_string_lossy();
        if e.file_type().is_dir() && (n == "target" || n == ".git" || n == "node_modules" || n == "dist") {
            return false;
        }
        true
    });
    for entry in walker.filter_map(|e| e.ok()) {
        if entry.file_type().is_file() {
            let rel = entry.path().strip_prefix(project_root).unwrap_or(entry.path());
            let rel_str = rel.to_string_lossy().replace('\\', "/");
            if rel_str.contains("/api/") || rel_str.starts_with("api/") {
                let mut clean = rel_str.replace("app/api/", "/api/").replace("pages/api/", "/api/").replace("src/api/", "/api/").replace("api/", "/api/");
                if let Some(pos) = clean.rfind('.') {
                    clean.truncate(pos);
                }
                clean = clean.replace("/route", "").replace("/index", "");
                routes.insert(clean);
            }
        }
    }
    routes
}
fn extract_client_api_calls(line: &str) -> Vec<String> {
    let mut found = Vec::new();
    let re = Regex::new(r#"(?:fetch|axios\.(?:get|post|put|delete|patch)|\$fetch)\s*\(\s*["'](/api/[^"'?#]+)["']"#).unwrap();
    for cap in re.captures_iter(line) {
        if let Some(m) = cap.get(1) {
            let mut route = m.as_str().to_string();
            while route.ends_with('/') && route.len() > 1 {
                route.pop();
            }
            found.push(route);
        }
    }
    found
}
pub fn detect_phantom_routes_in_content(file_path: &str, content: &str, project_root: &Path) -> Vec<SlopFinding> {
    let mut findings = Vec::new();
    if file_path.contains("/api/") || file_path.starts_with("api/") {
        return findings;
    }
    let routes = index_existing_api_routes(project_root);
    if routes.is_empty() {
        return findings;
    }
    for (i, line) in content.lines().enumerate() {
        let calls = extract_client_api_calls(line);
        for call in calls {
            let is_matched = routes.iter().any(|r| call == *r || call.starts_with(&format!("{r}/")));
            if !is_matched {
                findings.push(SlopFinding {
                    file: file_path.to_string(),
                    line: i + 1,
                    column: 1,
                    rule_id: "no-phantom-api-routes".to_string(),
                    category: SlopCategory::Placeholder,
                    severity: SlopSeverity::Error,
                    message: format!("Phantom API endpoint '{call}' called by AI with no matching backend route handler"),
                    snippet: line.trim().to_string(),
                    fixable: false,
                    suggestion: Some(format!("Create backend route handler for '{call}' or update URL to existing endpoint")),
                });
            }
        }
    }
    findings
}
pub fn detect_phantom_routes_in_diff(diff_files: &[DiffFile], project_root: &Path) -> Vec<SlopFinding> {
    let mut findings = Vec::new();
    let routes = index_existing_api_routes(project_root);
    if routes.is_empty() {
        return findings;
    }
    for f in diff_files {
        let path = if !f.new_path.is_empty() { &f.new_path } else { &f.old_path };
        if path.contains("/api/") || path.starts_with("api/") {
            continue;
        }
        for c in &f.chunks {
            for l in &c.lines {
                if l.line_type == DiffLineType::Add {
                    let calls = extract_client_api_calls(&l.content);
                    for call in calls {
                        let is_matched = routes.iter().any(|r| call == *r || call.starts_with(&format!("{r}/")));
                        if !is_matched {
                            findings.push(SlopFinding {
                                file: path.clone(),
                                line: l.new_line_number.unwrap_or(1),
                                column: 1,
                                rule_id: "no-phantom-api-routes".to_string(),
                                category: SlopCategory::Placeholder,
                                severity: SlopSeverity::Error,
                                message: format!("Phantom API endpoint '{call}' called by AI with no matching backend route handler"),
                                snippet: l.content.trim().to_string(),
                                fixable: false,
                                suggestion: Some(format!("Ensure backend route exists for '{call}' before calling it in client code")),
                            });
                        }
                    }
                }
            }
        }
    }
    findings
}