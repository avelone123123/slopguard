use crate::types::{DiffFile, DiffLineType, SlopCategory, SlopFinding, SlopSeverity};
use regex::Regex;
pub struct PatternRule {
    pub regex: Regex,
    pub rule_id: &'static str,
    pub category: SlopCategory,
    pub severity: SlopSeverity,
    pub message: &'static str,
    pub fixable: bool,
    pub suggestion: &'static str,
}
#[allow(dead_code)]
pub fn scan_diff(diff_files: &[crate::types::DiffFile], config: &crate::types::SlopConfig) -> Vec<SlopFinding> {
    crate::detectors::run_all_diff_detectors(diff_files, std::path::Path::new("."), config)
}
pub fn scan_diff_with_rules(diff_files: &[DiffFile], rules: &[PatternRule], file_filter: Option<fn(&str) -> bool>) -> Vec<SlopFinding> {
    let mut findings = Vec::new();
    for file in diff_files {
        if file.is_deleted {
            continue;
        }
        let target_path = if !file.new_path.is_empty() { &file.new_path } else { &file.old_path };
        if let Some(filter) = file_filter {
            if !filter(target_path) {
                continue;
            }
        }
        for chunk in &file.chunks {
            for line in &chunk.lines {
                if line.line_type != DiffLineType::Add {
                    continue;
                }
                let line_num = line.new_line_number.unwrap_or(1);
                for rule in rules {
                    if let Some(m) = rule.regex.find(&line.content) {
                        findings.push(SlopFinding {
                            file: target_path.to_string(),
                            line: line_num,
                            column: m.start() + 1,
                            rule_id: rule.rule_id.to_string(),
                            category: rule.category.clone(),
                            severity: rule.severity.clone(),
                            message: rule.message.to_string(),
                            snippet: line.content.trim().to_string(),
                            fixable: rule.fixable,
                            suggestion: Some(rule.suggestion.to_string()),
                        });
                    }
                }
            }
        }
    }
    findings
}
pub fn scan_content_with_rules(file_path: &str, content: &str, rules: &[PatternRule], file_filter: Option<fn(&str) -> bool>) -> Vec<SlopFinding> {
    let mut findings = Vec::new();
    if let Some(filter) = file_filter {
        if !filter(file_path) {
            return findings;
        }
    }
    for (idx, line) in content.lines().enumerate() {
        let line_num = idx + 1;
        for rule in rules {
            if let Some(m) = rule.regex.find(line) {
                findings.push(SlopFinding {
                    file: file_path.to_string(),
                    line: line_num,
                    column: m.start() + 1,
                    rule_id: rule.rule_id.to_string(),
                    category: rule.category.clone(),
                    severity: rule.severity.clone(),
                    message: rule.message.to_string(),
                    snippet: line.trim().to_string(),
                    fixable: rule.fixable,
                    suggestion: Some(rule.suggestion.to_string()),
                });
            }
        }
    }
    findings
}