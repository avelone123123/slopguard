use crate::types::{DiffFile, SlopCategory, SlopFinding, SlopSeverity};
use regex::Regex;
pub fn detect_scope_drift(diff_files: &[DiffFile], max_drift_files: usize) -> Vec<SlopFinding> {
    let mut findings = Vec::new();
    let modified_paths: Vec<&str> = diff_files.iter().map(|f| if !f.new_path.is_empty() { f.new_path.as_str() } else { f.old_path.as_str() }).collect();
    if modified_paths.len() > max_drift_files {
        let first = modified_paths.first().copied().unwrap_or("repository");
        let sample = modified_paths.iter().take(5).copied().collect::<Vec<&str>>().join(", ");
        findings.push(SlopFinding {
            file: first.to_string(),
            line: 1,
            column: 1,
            rule_id: "no-excessive-scope-drift".to_string(),
            category: SlopCategory::ScopeDrift,
            severity: SlopSeverity::Warning,
            message: format!("Excessive agent scope drift: {} files modified across single turn (threshold: {})", modified_paths.len(), max_drift_files),
            snippet: format!("{sample}... (+{} more)", modified_paths.len() - 5),
            fixable: false,
            suggestion: Some("Split large agent refactorings into smaller, verified atomic steps".to_string()),
        });
    }
    let sensitive_patterns = [
        Regex::new(r"^\.env(?:\..+)?$").unwrap(),
        Regex::new(r"^\.github/workflows/.+$").unwrap(),
        Regex::new(r"^(?:package-lock\.json|yarn\.lock|pnpm-lock\.yaml|Cargo\.lock)$").unwrap(),
        Regex::new(r"^(?:tsconfig\.json|jsconfig\.json)$").unwrap(),
    ];
    for fpath in modified_paths {
        let filename = fpath.split('/').next_back().unwrap_or(fpath);
        for pattern in &sensitive_patterns {
            if pattern.is_match(filename) || pattern.is_match(fpath) {
                findings.push(SlopFinding {
                    file: fpath.to_string(),
                    line: 1,
                    column: 1,
                    rule_id: "no-excessive-scope-drift".to_string(),
                    category: SlopCategory::ScopeDrift,
                    severity: SlopSeverity::Warning,
                    message: format!("Sensitive boundary file '{fpath}' modified unexpectedly by agent"),
                    snippet: fpath.to_string(),
                    fixable: false,
                    suggestion: Some("Confirm whether critical environment or configuration file was meant to be changed".to_string()),
                });
            }
        }
    }
    findings
}