use crate::types::{DiffFile, DiffLineType, SlopCategory, SlopFinding, SlopSeverity};
use regex::Regex;
fn get_redos_structural_patterns() -> Vec<(Regex, &'static str)> {
    vec![
        (Regex::new(r#"\([^()\\|]+(?:\+|\*|\{\d+,?\d*\})\)(?:\+|\*|\{\d+,?\d*\})"#).unwrap(), "Nested repetition on group causing exponential backtracking"),
        (Regex::new(r#"\(\.\*|\.\+\)[+*]"#).unwrap(), "Unbounded dot wildcard quantifier combination"),
        (Regex::new(r#"\((?:[a-zA-Z0-9_-]+\|[a-zA-Z0-9_-]+)\)[+*]"#).unwrap(), "Overlapping alternation repetition vulnerability"),
        (Regex::new(r#"\([^)]*\\w\+[^)]*\)\+"#).unwrap(), "Nested word-class quantifier repetition"),
        (Regex::new(r#"\([^)]*\\d\+[^)]*\)\+"#).unwrap(), "Nested digit-class quantifier repetition"),
    ]
}
fn get_call_site_patterns() -> Vec<Regex> {
    vec![
        Regex::new(r#"/(?:[^\n/]|\\.)*(?:\([^)/]+(?:\+|\*|\{\d+,?\d*\})\)[+*]|\(\.\*|\.\+\)[+*])(?:[^\n/]|\\.)*/[gimsuy]*"#).unwrap(),
        Regex::new(r#"(?i)(?:RegExp|re\.(?:compile|search|match|findall|sub)|regexp\.(?:Compile|MustCompile)|Regex::new)\s*\(\s*r?["'][^"']*(\([^"'\)]*(\+|\*|\{\d+,?\d*\})[^"'\)]*\)[+*]|\(\.\*|\.\+\)[+*])[^"']*["']"#).unwrap(),
        Regex::new(r#"(?i)(?:pattern|regex|rule)\s*=\s*r?["'][^"']*(\([^"'\)]*(\+|\*|\{\d+,?\d*\})[^"'\)]*\)[+*]|\(\.\*|\.\+\)[+*])[^"']*["']"#).unwrap(),
    ]
}
fn evaluate_line_for_redos(line: &str, call_sites: &[Regex], structural: &[(Regex, &'static str)]) -> Option<String> {
    for cs in call_sites {
        if cs.is_match(line) {
            return Some("Catastrophic backtracking regular expression detected in call site".to_string());
        }
    }
    for (re, desc) in structural {
        if (line.contains('/') || line.contains("Regex") || line.contains("regexp") || line.contains("re.") || line.contains("Pattern") || line.contains("compile") || line.contains("match")) && re.is_match(line) {
            return Some(desc.to_string());
        }
    }
    None
}
pub fn detect_redos_in_content(file_path: &str, content: &str) -> Vec<SlopFinding> {
    let mut findings = Vec::new();
    let call_sites = get_call_site_patterns();
    let structural = get_redos_structural_patterns();
    let lines: Vec<&str> = content.lines().collect();
    for (i, line) in lines.iter().enumerate() {
        if let Some(desc) = evaluate_line_for_redos(line, &call_sites, &structural) {
            findings.push(SlopFinding {
                file: file_path.to_string(),
                line: i + 1,
                column: 1,
                rule_id: "no-redos-vulnerability".to_string(),
                category: SlopCategory::Security,
                severity: SlopSeverity::Error,
                message: format!("ReDoS vulnerability detected: {desc}"),
                snippet: line.trim().to_string(),
                fixable: false,
                suggestion: Some("Refactor nested quantifiers to atomic groups, use possessive repetition, or deploy linear regex engines (RE2/Rust regex)".to_string()),
            });
        }
    }
    findings
}
pub fn detect_redos_in_diff(diff_files: &[DiffFile]) -> Vec<SlopFinding> {
    let mut findings = Vec::new();
    let call_sites = get_call_site_patterns();
    let structural = get_redos_structural_patterns();
    for f in diff_files {
        let path = if !f.new_path.is_empty() { &f.new_path } else { &f.old_path };
        for c in &f.chunks {
            for l in &c.lines {
                if l.line_type == DiffLineType::Add {
                    if let Some(desc) = evaluate_line_for_redos(&l.content, &call_sites, &structural) {
                        findings.push(SlopFinding {
                            file: path.clone(),
                            line: l.new_line_number.unwrap_or(1),
                            column: 1,
                            rule_id: "no-redos-vulnerability".to_string(),
                            category: SlopCategory::Security,
                            severity: SlopSeverity::Error,
                            message: format!("ReDoS vulnerability detected: {desc}"),
                            snippet: l.content.trim().to_string(),
                            fixable: false,
                            suggestion: Some("Refactor nested quantifiers to atomic groups, use possessive repetition, or deploy linear regex engines (RE2/Rust regex)".to_string()),
                        });
                    }
                }
            }
        }
    }
    findings
}