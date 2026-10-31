use crate::types::{DiffFile, DiffLineType, SlopCategory, SlopFinding, SlopSeverity};
use regex::Regex;
pub fn detect_broken_access_control_in_diff(diff_files: &[DiffFile]) -> Vec<SlopFinding> {
    let mut findings = Vec::new();
    let unscoped_sql_delete = Regex::new(r#"(?i)DELETE\s+FROM\s+\w+\s+WHERE\s+id\s*=\s*(\$1|\?|:id|\$\{req\.params\.id\}|req\.params\.id)\s*["'`]?\s*;?$"#).unwrap();
    let unscoped_sql_update = Regex::new(r#"(?i)UPDATE\s+\w+\s+SET\s+[^;]+WHERE\s+id\s*=\s*(\$1|\?|:id|\$\{req\.params\.id\}|req\.params\.id)\s*["'`]?\s*;?$"#).unwrap();
    let unscoped_orm_destroy = Regex::new(r#"(?i)\w+\.(destroy|delete|remove)\s*\(\s*\{\s*(where\s*:\s*\{\s*)?id\s*:\s*req\.params\.id\s*\}?\s*\)"#).unwrap();
    for df in diff_files {
        let p_lower = df.new_path.to_lowercase();
        if p_lower.ends_with(".md") || p_lower.ends_with(".lock") {
            continue;
        }
        for chunk in &df.chunks {
            for dl in &chunk.lines {
                if dl.line_type != DiffLineType::Add {
                    continue;
                }
                let line_str = dl.content.trim();
                let line_num = dl.new_line_number.unwrap_or(0);
                if unscoped_sql_delete.is_match(line_str) || unscoped_sql_update.is_match(line_str) || unscoped_orm_destroy.is_match(line_str) {
                    findings.push(SlopFinding {
                        file: df.new_path.clone(),
                        line: line_num,
                        column: 1,
                        rule_id: "no-unscoped-idor-queries".to_string(),
                        category: SlopCategory::BrokenAccessControl,
                        severity: SlopSeverity::Error,
                        message: "Unscoped record modification by primary key without owner or tenant verification (IDOR hazard).".to_string(),
                        snippet: line_str.to_string(),
                        fixable: false,
                        suggestion: Some("Append AND user_id = :auth_user_id to ensure callers cannot mutate other tenants' records.".to_string()),
                    });
                }
            }
        }
    }
    findings
}
pub fn detect_broken_access_control_in_content(file_path: &str, content: &str) -> Vec<SlopFinding> {
    let mut findings = Vec::new();
    let p_lower = file_path.to_lowercase();
    if p_lower.ends_with(".md") || p_lower.ends_with(".lock") {
        return findings;
    }
    let unscoped_sql_delete = Regex::new(r#"(?i)DELETE\s+FROM\s+\w+\s+WHERE\s+id\s*=\s*(\$1|\?|:id|\$\{req\.params\.id\}|req\.params\.id)\s*["'`]?\s*;?$"#).unwrap();
    let unscoped_sql_update = Regex::new(r#"(?i)UPDATE\s+\w+\s+SET\s+[^;]+WHERE\s+id\s*=\s*(\$1|\?|:id|\$\{req\.params\.id\}|req\.params\.id)\s*["'`]?\s*;?$"#).unwrap();
    let unscoped_orm_destroy = Regex::new(r#"(?i)\w+\.(destroy|delete|remove)\s*\(\s*\{\s*(where\s*:\s*\{\s*)?id\s*:\s*req\.params\.id\s*\}?\s*\)"#).unwrap();
    for (idx, line) in content.lines().enumerate() {
        let line_str = line.trim();
        let line_num = idx + 1;
        if unscoped_sql_delete.is_match(line_str) || unscoped_sql_update.is_match(line_str) || unscoped_orm_destroy.is_match(line_str) {
            findings.push(SlopFinding {
                file: file_path.to_string(),
                line: line_num,
                column: 1,
                rule_id: "no-unscoped-idor-queries".to_string(),
                category: SlopCategory::BrokenAccessControl,
                severity: SlopSeverity::Error,
                message: "Unscoped record modification by primary key without tenant check (IDOR hazard).".to_string(),
                snippet: line_str.to_string(),
                fixable: false,
                suggestion: Some("Scope database query to authenticated user ID.".to_string()),
            });
        }
    }
    findings
}