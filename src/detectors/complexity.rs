use crate::types::{DiffFile, DiffLineType, SlopCategory, SlopFinding, SlopSeverity};
fn calculate_indent_level(line: &str) -> usize {
    let mut spaces = 0;
    for c in line.chars() {
        if c == ' ' {
            spaces += 1;
        } else if c == '\t' {
            spaces += 4;
        } else {
            break;
        }
    }
    spaces / 4
}
fn has_control_keyword(line: &str) -> bool {
    let trimmed = line.trim_start();
    trimmed.starts_with("if ") || trimmed.starts_with("if(") ||
    trimmed.starts_with("for ") || trimmed.starts_with("for(") ||
    trimmed.starts_with("while ") || trimmed.starts_with("while(") ||
    trimmed.starts_with("switch ") || trimmed.starts_with("switch(") ||
    trimmed.starts_with("match ") || trimmed.starts_with("try ") ||
    trimmed.starts_with("catch ") || trimmed.starts_with("catch(") ||
    trimmed.starts_with("case ")
}
pub fn detect_nesting_sprawl_in_content(file_path: &str, content: &str, max_depth: usize) -> Vec<SlopFinding> {
    let mut findings = Vec::new();
    for (i, line) in content.lines().enumerate() {
        if line.trim().is_empty() {
            continue;
        }
        let depth = calculate_indent_level(line);
        if depth >= max_depth && has_control_keyword(line) {
            findings.push(SlopFinding {
                file: file_path.to_string(),
                line: i + 1,
                column: 1,
                rule_id: "no-deep-nesting-sprawl".to_string(),
                category: SlopCategory::Complexity,
                severity: SlopSeverity::Warning,
                message: format!("Deep nesting pyramid detected (depth: {depth} >= {max_depth})"),
                snippet: line.trim().to_string(),
                fixable: false,
                suggestion: Some("Flatten complex control flow using guard clauses, early returns, or extracted helper functions".to_string()),
            });
        }
    }
    findings
}
pub fn detect_nesting_sprawl_in_diff(diff_files: &[DiffFile], max_depth: usize) -> Vec<SlopFinding> {
    let mut findings = Vec::new();
    for f in diff_files {
        let path = if !f.new_path.is_empty() { &f.new_path } else { &f.old_path };
        for c in &f.chunks {
            for l in &c.lines {
                if l.line_type == DiffLineType::Add && !l.content.trim().is_empty() {
                    let depth = calculate_indent_level(&l.content);
                    if depth >= max_depth && has_control_keyword(&l.content) {
                        findings.push(SlopFinding {
                            file: path.clone(),
                            line: l.new_line_number.unwrap_or(1),
                            column: 1,
                            rule_id: "no-deep-nesting-sprawl".to_string(),
                            category: SlopCategory::Complexity,
                            severity: SlopSeverity::Warning,
                            message: format!("Deep nesting pyramid introduced in diff (depth: {depth} >= {max_depth})"),
                            snippet: l.content.trim().to_string(),
                            fixable: false,
                            suggestion: Some("Flatten nested logic using guard clauses or early returns".to_string()),
                        });
                    }
                }
            }
        }
    }
    findings
}