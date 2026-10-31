use crate::types::{DiffFile, DiffLineType, SlopCategory, SlopFinding, SlopSeverity};
use regex::Regex;
fn get_slopsquat_patterns() -> Vec<(Regex, &'static str)> {
    vec![
        (Regex::new(r#"(?i)["']?(?:crypto-secure-hash|starlette-reverse-proxy|supabase-admin-client|next-auth-helpers|express-cors-middleware|flask-jwt-router|axios-retry-interceptor|react-router-dom-v6|langchain-deepseek|torch-vision-utils|tailwind-merge-extra|fastapi-auth-bearer|boto3-auth-client|prisma-type-helpers)["']?"#).unwrap(), "AI-hallucinated dependency package signature vulnerable to supply chain slopsquatting"),
    ]
}
fn is_manifest_file(path_str: &str) -> bool {
    let lower = path_str.to_lowercase();
    lower.ends_with("package.json") || lower.ends_with("requirements.txt") || lower.ends_with("pyproject.toml") || lower.ends_with("cargo.toml")
}
pub fn detect_slopsquat_in_content(file_path: &str, content: &str) -> Vec<SlopFinding> {
    let mut findings = Vec::new();
    if !is_manifest_file(file_path) {
        return findings;
    }
    let patterns = get_slopsquat_patterns();
    for (idx, line) in content.lines().enumerate() {
        let line_num = idx + 1;
        let trimmed = line.trim();
        for (re, desc) in &patterns {
            if re.is_match(trimmed) {
                findings.push(SlopFinding {
                    file: file_path.to_string(),
                    line: line_num,
                    column: 1,
                    rule_id: "no-slopsquat-deps".to_string(),
                    category: SlopCategory::GhostDependency,
                    severity: SlopSeverity::Warning,
                    message: format!("Suspicious package declaration: {desc}"),
                    snippet: trimmed.to_string(),
                    fixable: false,
                    suggestion: Some("Verify package authenticity on official registry (npm/PyPI/crates.io). Hallucinated dependencies are vulnerable to slopsquatting attacks".to_string()),
                });
                break;
            }
        }
    }
    findings
}
pub fn detect_slopsquat_in_diff(diff_files: &[DiffFile]) -> Vec<SlopFinding> {
    let mut findings = Vec::new();
    let patterns = get_slopsquat_patterns();
    for file in diff_files {
        if file.is_deleted {
            continue;
        }
        let target_path = if !file.new_path.is_empty() { &file.new_path } else { &file.old_path };
        if !is_manifest_file(target_path) {
            continue;
        }
        for chunk in &file.chunks {
            for line in &chunk.lines {
                if line.line_type != DiffLineType::Add {
                    continue;
                }
                let trimmed = line.content.trim();
                let line_num = line.new_line_number.unwrap_or(1);
                for (re, desc) in &patterns {
                    if re.is_match(trimmed) {
                        findings.push(SlopFinding {
                            file: target_path.to_string(),
                            line: line_num,
                            column: 1,
                            rule_id: "no-slopsquat-deps".to_string(),
                            category: SlopCategory::GhostDependency,
                            severity: SlopSeverity::Warning,
                            message: format!("Suspicious package declaration: {desc}"),
                            snippet: trimmed.to_string(),
                            fixable: false,
                            suggestion: Some("Verify package authenticity on official registry (npm/PyPI/crates.io). Hallucinated dependencies are vulnerable to slopsquatting attacks".to_string()),
                        });
                        break;
                    }
                }
            }
        }
    }
    findings
}