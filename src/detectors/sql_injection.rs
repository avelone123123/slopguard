use crate::types::{DiffFile, DiffLineType, SlopCategory, SlopFinding, SlopSeverity};
use regex::Regex;
fn get_sql_injection_patterns() -> Vec<(Regex, &'static str)> {
    vec![
        (Regex::new(r#"(?i)`[^`]*\b(?:SELECT|INSERT\s+INTO|UPDATE|DELETE\s+FROM|DROP\s+TABLE|ALTER\s+TABLE|UNION\s+SELECT)\b[^`]*\$\{[^}]+\}[^`]*`"#).unwrap(), "Template literal raw SQL interpolation"),
        (Regex::new(r#"(?i)`[^`]*\$\{[^}]+\}[^`]*\b(?:SELECT|INSERT\s+INTO|UPDATE|DELETE\s+FROM|DROP\s+TABLE|ALTER\s+TABLE|UNION\s+SELECT)\b[^`]*`"#).unwrap(), "Template literal raw SQL interpolation"),
        (Regex::new(r#"(?i)(?:"[^"]*\b(?:SELECT|INSERT\s+INTO|UPDATE|DELETE\s+FROM|DROP\s+TABLE|ALTER\s+TABLE|UNION\s+SELECT)\b[^"]*"|'[^']*\b(?:SELECT|INSERT\s+INTO|UPDATE|DELETE\s+FROM|DROP\s+TABLE|ALTER\s+TABLE|UNION\s+SELECT)\b[^']*')\s*\+\s*[a-zA-Z_$]"#).unwrap(), "SQL query string concatenation with variable"),
        (Regex::new(r#"(?i)[a-zA-Z_0-9\)\.]\s*\+\s*(?:"[^"]*\b(?:SELECT|INSERT\s+INTO|UPDATE|DELETE\s+FROM|DROP\s+TABLE|ALTER\s+TABLE|UNION\s+SELECT)\b[^"]*"|'[^']*\b(?:SELECT|INSERT\s+INTO|UPDATE|DELETE\s+FROM|DROP\s+TABLE|ALTER\s+TABLE|UNION\s+SELECT)\b[^']*')"#).unwrap(), "SQL query string concatenation with variable"),
        (Regex::new(r#"(?i)\bf(?:"[^"]*\b(?:SELECT|INSERT|UPDATE|DELETE|DROP|ALTER|UNION)\b[^"]*\{[^}]+\}|'[^']*\b(?:SELECT|INSERT|UPDATE|DELETE|DROP|ALTER|UNION)\b[^']*\{[^}]+\})"#).unwrap(), "Python f-string SQL query interpolation"),
        (Regex::new(r#"(?i)\bf(?:"[^"]*\{[^}]+\}[^"]*\b(?:SELECT|INSERT|UPDATE|DELETE|DROP|ALTER|UNION)\b|'[^']*\{[^}]+\}[^']*\b(?:SELECT|INSERT|UPDATE|DELETE|DROP|ALTER|UNION)\b)"#).unwrap(), "Python f-string SQL query interpolation"),
        (Regex::new(r#"(?i)(?:"[^"]*\b(?:SELECT|INSERT|UPDATE|DELETE|DROP|ALTER|UNION)\b[^"]*"|'[^']*\b(?:SELECT|INSERT|UPDATE|DELETE|DROP|ALTER|UNION)\b[^']*')\s*%\s*(?:\(|[a-zA-Z_])"#).unwrap(), "Insecure Python '%' string formatting in SQL query"),
        (Regex::new(r#"(?i)\bformat!\s*\(\s*(?:"[^"]*\b(?:SELECT|INSERT|UPDATE|DELETE|DROP|ALTER|UNION)\b[^"]*\{[^}]*\}|'[^']*\b(?:SELECT|INSERT|UPDATE|DELETE|DROP|ALTER|UNION)\b[^']*\{[^}]*\})"#).unwrap(), "Rust format! macro SQL query interpolation"),
        (Regex::new(r#"(?i)\bfmt\.Sprintf\s*\(\s*(?:"[^"]*\b(?:SELECT|INSERT|UPDATE|DELETE|DROP|ALTER|UNION)\b[^"]*%[svdqx]|'[^']*\b(?:SELECT|INSERT|UPDATE|DELETE|DROP|ALTER|UNION)\b[^']*%[svdqx])"#).unwrap(), "Go fmt.Sprintf SQL query formatting"),
        (Regex::new(r#"(?i)"[^"\\]*\b(?:SELECT|INSERT|UPDATE|DELETE|DROP|ALTER|UNION)\b[^"\\]*\$[a-zA-Z_]"#).unwrap(), "PHP double-quote variable interpolation in SQL"),
        (Regex::new(r#"(?i)"[^"\\]*\$[a-zA-Z_][^"\\]*\b(?:SELECT|INSERT|UPDATE|DELETE|DROP|ALTER|UNION)\b"#).unwrap(), "PHP double-quote variable interpolation in SQL"),
        (Regex::new(r#"(?i)(?:"[^"]*\b(?:SELECT|INSERT\s+INTO|UPDATE|DELETE\s+FROM|DROP\s+TABLE|ALTER\s+TABLE|UNION\s+SELECT)\b[^"]*"|'[^']*\b(?:SELECT|INSERT\s+INTO|UPDATE|DELETE\s+FROM|DROP\s+TABLE|ALTER\s+TABLE|UNION\s+SELECT)\b[^']*')\s*\.\s*\$[a-zA-Z_]"#).unwrap(), "PHP SQL string concatenation with variable"),
    ]
}
pub fn detect_sql_injection_in_content(file_path: &str, content: &str) -> Vec<SlopFinding> {
    let mut findings = Vec::new();
    let patterns = get_sql_injection_patterns();
    for (i, line) in content.lines().enumerate() {
        for (re, desc) in &patterns {
            if re.is_match(line) {
                findings.push(SlopFinding {
                    file: file_path.to_string(),
                    line: i + 1,
                    column: 1,
                    rule_id: "no-sql-injection-risk".to_string(),
                    category: SlopCategory::Security,
                    severity: SlopSeverity::Error,
                    message: format!("Insecure SQL query construction: {desc}"),
                    snippet: line.trim().to_string(),
                    fixable: false,
                    suggestion: Some("Use parameterized queries ($1, ?) or ORM query builders instead of string interpolation".to_string()),
                });
                break;
            }
        }
    }
    findings
}
pub fn detect_sql_injection_in_diff(diff_files: &[DiffFile]) -> Vec<SlopFinding> {
    let mut findings = Vec::new();
    let patterns = get_sql_injection_patterns();
    for f in diff_files {
        let path = if !f.new_path.is_empty() { &f.new_path } else { &f.old_path };
        for c in &f.chunks {
            for l in &c.lines {
                if l.line_type == DiffLineType::Add {
                    for (re, desc) in &patterns {
                        if re.is_match(&l.content) {
                            findings.push(SlopFinding {
                                file: path.clone(),
                                line: l.new_line_number.unwrap_or(1),
                                column: 1,
                                rule_id: "no-sql-injection-risk".to_string(),
                                category: SlopCategory::Security,
                                severity: SlopSeverity::Error,
                                message: format!("Insecure SQL query construction: {desc}"),
                                snippet: l.content.trim().to_string(),
                                fixable: false,
                                suggestion: Some("Use parameterized queries ($1, ?) or ORM query builders instead of string interpolation".to_string()),
                            });
                            break;
                        }
                    }
                }
            }
        }
    }
    findings
}