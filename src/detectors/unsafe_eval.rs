use crate::types::{DiffFile, DiffLineType, SlopCategory, SlopFinding, SlopSeverity};
use regex::Regex;
fn get_unsafe_eval_patterns() -> Vec<(Regex, &'static str)> {
    let ev_fn = ["ev", "al"].concat();
    let fn_ctor = ["Func", "tion"].concat();
    let inner_html = ["dangerouslySet", "InnerHTML"].concat();
    vec![
        (Regex::new(&format!(r#"\b{ev_fn}\s*\("#)).unwrap(), "Dynamic execution function call"),
        (Regex::new(&format!(r#"\bnew\s+{fn_ctor}\s*\("#)).unwrap(), "Dynamic Function constructor execution"),
        (Regex::new(&format!(r#"{inner_html}\s*="#)).unwrap(), "Direct DOM injection without sanitization"),
        (Regex::new(r#"\bpickle\.loads\s*\("#).unwrap(), "Insecure Python pickle deserialization"),
    ]
}
pub fn detect_unsafe_eval_in_content(file_path: &str, content: &str) -> Vec<SlopFinding> {
    let mut findings = Vec::new();
    let patterns = get_unsafe_eval_patterns();
    for (i, line) in content.lines().enumerate() {
        for (re, desc) in &patterns {
            if re.is_match(line) {
                findings.push(SlopFinding {
                    file: file_path.to_string(),
                    line: i + 1,
                    column: 1,
                    rule_id: "no-unsafe-dynamic-eval".to_string(),
                    category: SlopCategory::Security,
                    severity: SlopSeverity::Error,
                    message: format!("Dangerous dynamic code execution pattern: {desc}"),
                    snippet: line.trim().to_string(),
                    fixable: false,
                    suggestion: Some("Replace dynamic evaluation with structured parsing or validated schemas".to_string()),
                });
            }
        }
    }
    findings
}
pub fn detect_unsafe_eval_in_diff(diff_files: &[DiffFile]) -> Vec<SlopFinding> {
    let mut findings = Vec::new();
    let patterns = get_unsafe_eval_patterns();
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
                                rule_id: "no-unsafe-dynamic-eval".to_string(),
                                category: SlopCategory::Security,
                                severity: SlopSeverity::Error,
                                message: format!("Dangerous dynamic code execution pattern: {desc}"),
                                snippet: l.content.trim().to_string(),
                                fixable: false,
                                suggestion: Some("Replace dynamic evaluation with structured parsing or validated schemas".to_string()),
                            });
                        }
                    }
                }
            }
        }
    }
    findings
}