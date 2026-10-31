use crate::types::{DiffFile, DiffLineType, SlopCategory, SlopFinding, SlopSeverity};
use regex::Regex;
fn is_supported_ext(p: &str) -> bool {
    let l = p.to_lowercase();
    l.ends_with(".js") || l.ends_with(".ts") || l.ends_with(".jsx") || l.ends_with(".tsx") || l.ends_with(".mjs") || l.ends_with(".cjs") || l.ends_with(".py") || l.ends_with(".rs") || l.ends_with(".go")
}
fn has_traversal_defense(line: &str) -> bool {
    line.contains("canonicalize")
        || line.contains("secure_filename")
        || line.contains("os.path.commonpath")
        || line.contains("is_relative_to")
        || line.contains("path=safe")
        || line.contains("starts_with")
        || line.contains("startsWith")
        || line.contains("filepath.Clean")
}
pub fn detect_deep_path_traversal_in_diff(diff_files: &[DiffFile]) -> Vec<SlopFinding> {
    let mut findings = Vec::new();
    let py_zip_slip = Regex::new(r#"(?i)(tarfile\.open\([^)]*\)\.extractall\(|zipfile\.ZipFile\([^)]*\)\.extractall\()"#).unwrap();
    let node_traversal_stream = Regex::new(r#"(?i)\bfs\.(createReadStream|createWriteStream|readFile|readFileSync)\s*\([^)]*req\.(params|query|body)"#).unwrap();
    let node_traversal_join = Regex::new(r#"(?i)\bpath\.(join|resolve)\s*\([^)]*req\.(params|query|body)"#).unwrap();
    let rust_file_format = Regex::new(r#"(?i)File::(create|open)\s*\(\s*format!\s*\([^)]*\{[^}]*\}/\{[^}]*"#).unwrap();
    for df in diff_files {
        let path = if !df.new_path.is_empty() { &df.new_path } else { &df.old_path };
        if !is_supported_ext(path) {
            continue;
        }
        for chunk in &df.chunks {
            let chunk_text = chunk.lines.iter().map(|l| l.content.as_str()).collect::<Vec<_>>().join(" ");
            let chunk_guarded = has_traversal_defense(&chunk_text);
            for dl in &chunk.lines {
                if dl.line_type != DiffLineType::Add {
                    continue;
                }
                let line_str = dl.content.trim();
                let line_num = dl.new_line_number.unwrap_or(0);
                if has_traversal_defense(line_str) || chunk_guarded {
                    continue;
                }
                if py_zip_slip.is_match(line_str)
                    || node_traversal_stream.is_match(line_str)
                    || node_traversal_join.is_match(line_str)
                    || rust_file_format.is_match(line_str)
                {
                    findings.push(SlopFinding {
                        file: path.clone(),
                        line: line_num,
                        column: 1,
                        rule_id: "no-deep-path-traversal".to_string(),
                        category: SlopCategory::PathTraversalSecurity,
                        severity: SlopSeverity::Error,
                        message: "Deep path traversal or zip-slip vulnerability detected without boundary containment.".to_string(),
                        snippet: line_str.to_string(),
                        fixable: false,
                        suggestion: Some("Validate and constrain paths using canonicalize, secure_filename, or is_relative_to.".to_string()),
                    });
                }
            }
        }
    }
    findings
}
pub fn detect_deep_path_traversal_in_content(file_path: &str, content: &str) -> Vec<SlopFinding> {
    let mut findings = Vec::new();
    if !is_supported_ext(file_path) {
        return findings;
    }
    let py_zip_slip = Regex::new(r#"(?i)(tarfile\.open\([^)]*\)\.extractall\(|zipfile\.ZipFile\([^)]*\)\.extractall\()"#).unwrap();
    let node_traversal_stream = Regex::new(r#"(?i)\bfs\.(createReadStream|createWriteStream|readFile|readFileSync)\s*\([^)]*req\.(params|query|body)"#).unwrap();
    let node_traversal_join = Regex::new(r#"(?i)\bpath\.(join|resolve)\s*\([^)]*req\.(params|query|body)"#).unwrap();
    let rust_file_format = Regex::new(r#"(?i)File::(create|open)\s*\(\s*format!\s*\([^)]*\{[^}]*\}/\{[^}]*"#).unwrap();
    let lines: Vec<&str> = content.lines().collect();
    for (idx, line) in lines.iter().enumerate() {
        let line_str = line.trim();
        let line_num = idx + 1;
        if has_traversal_defense(line_str) {
            continue;
        }
        let start = idx.saturating_sub(4);
        let window = lines[start..=idx].join(" ");
        if has_traversal_defense(&window) {
            continue;
        }
        if py_zip_slip.is_match(line_str)
            || node_traversal_stream.is_match(line_str)
            || node_traversal_join.is_match(line_str)
            || rust_file_format.is_match(line_str)
        {
            findings.push(SlopFinding {
                file: file_path.to_string(),
                line: line_num,
                column: 1,
                rule_id: "no-deep-path-traversal".to_string(),
                category: SlopCategory::PathTraversalSecurity,
                severity: SlopSeverity::Error,
                message: "Deep path traversal or zip-slip vulnerability detected without boundary containment.".to_string(),
                snippet: line_str.to_string(),
                fixable: false,
                suggestion: Some("Validate and constrain paths using canonicalize, secure_filename, or is_relative_to.".to_string()),
            });
        }
    }
    findings
}