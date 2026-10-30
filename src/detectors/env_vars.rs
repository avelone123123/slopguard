use crate::types::{DiffFile, DiffLineType, SlopCategory, SlopFinding, SlopSeverity};
use regex::Regex;
use std::collections::HashSet;
use std::fs;
use std::path::Path;
fn get_known_env_vars(project_root: &Path) -> HashSet<String> {
    let mut vars = HashSet::new();
    let candidates = [".env.example", ".env.sample", ".env.template", ".env.local.example", ".env"];
    for name in &candidates {
        let p = project_root.join(name);
        if p.is_file() {
            if let Ok(content) = fs::read_to_string(&p) {
                for line in content.lines() {
                    let trimmed = line.trim();
                    if trimmed.is_empty() || trimmed.starts_with('#') {
                        continue;
                    }
                    if let Some(pos) = trimmed.find('=') {
                        let k = trimmed[..pos].trim();
                        if !k.is_empty() {
                            vars.insert(k.to_string());
                        }
                    }
                }
            }
        }
    }
    vars
}
fn is_standard_env(v: &str) -> bool {
    let standard = [
        "NODE_ENV", "PORT", "HOST", "PATH", "PWD", "HOME", "USER", "SHELL",
        "CI", "DEBUG", "TZ", "CARGO_MANIFEST_DIR", "RUST_LOG", "RUST_BACKTRACE",
        "VERCEL", "NETLIFY", "GITHUB_ACTIONS", "DATABASE_URL", "API_URL"
    ];
    standard.contains(&v)
}
fn extract_env_vars_from_line(line: &str) -> Vec<String> {
    let mut found = Vec::new();
    let re_node = Regex::new(r#"(?:process\.env|import\.meta\.env)\.([A-Z0-9_]{3,})"#).unwrap();
    for cap in re_node.captures_iter(line) {
        if let Some(m) = cap.get(1) {
            found.push(m.as_str().to_string());
        }
    }
    let re_call = Regex::new(r#"(?:env::var|os\.environ\.get|os\.getenv|os\.Getenv)\s*\(\s*["']([A-Z0-9_]{3,})["']"#).unwrap();
    for cap in re_call.captures_iter(line) {
        if let Some(m) = cap.get(1) {
            found.push(m.as_str().to_string());
        }
    }
    found
}
pub fn detect_ghost_env_vars_in_content(file_path: &str, content: &str, project_root: &Path) -> Vec<SlopFinding> {
    let mut findings = Vec::new();
    let known = get_known_env_vars(project_root);
    if known.is_empty() {
        return findings;
    }
    for (i, line) in content.lines().enumerate() {
        let envs = extract_env_vars_from_line(line);
        for var in envs {
            if !is_standard_env(&var) && !known.contains(&var) {
                findings.push(SlopFinding {
                    file: file_path.to_string(),
                    line: i + 1,
                    column: 1,
                    rule_id: "no-ghost-env-vars".to_string(),
                    category: SlopCategory::GhostEnv,
                    severity: SlopSeverity::Error,
                    message: format!("Ghost environment variable '{var}' referenced but missing from .env.example / .env"),
                    snippet: line.trim().to_string(),
                    fixable: false,
                    suggestion: Some(format!("Add '{var}=' to your .env.example template to prevent production failures")),
                });
            }
        }
    }
    findings
}
pub fn detect_ghost_env_vars_in_diff(diff_files: &[DiffFile], project_root: &Path) -> Vec<SlopFinding> {
    let mut findings = Vec::new();
    let known = get_known_env_vars(project_root);
    if known.is_empty() {
        return findings;
    }
    for f in diff_files {
        let path = if !f.new_path.is_empty() { &f.new_path } else { &f.old_path };
        for c in &f.chunks {
            for l in &c.lines {
                if l.line_type == DiffLineType::Add {
                    let envs = extract_env_vars_from_line(&l.content);
                    for var in envs {
                        if !is_standard_env(&var) && !known.contains(&var) {
                            findings.push(SlopFinding {
                                file: path.clone(),
                                line: l.new_line_number.unwrap_or(1),
                                column: 1,
                                rule_id: "no-ghost-env-vars".to_string(),
                                category: SlopCategory::GhostEnv,
                                severity: SlopSeverity::Error,
                                message: format!("Ghost environment variable '{var}' referenced but missing from .env.example / .env"),
                                snippet: l.content.trim().to_string(),
                                fixable: false,
                                suggestion: Some(format!("Declare '{var}=' in .env.example so teammates and CI have it")),
                            });
                        }
                    }
                }
            }
        }
    }
    findings
}