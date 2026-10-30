use crate::types::{DiffFile, DiffLineType, SlopCategory, SlopFinding, SlopSeverity};
use regex::Regex;
fn get_watermark_regexes() -> Vec<Regex> {
    vec![
        Regex::new(r"(?i)^\s*(?://|#)\s*(?:As an AI|Here is the (?:updated|revised|complete|working|full) (?:code|implementation)|Hope this helps|Feel free to (?:modify|adjust|change))\b").unwrap(),
        Regex::new(r"(?i)^\s*(?://|#)\s*(?:In a (?:real|production) (?:world|environment|app)|Let me know if you (?:need|have any)|Ensure you have installed|Make sure to run npm)\b").unwrap(),
        Regex::new(r"(?i)^\s*(?://|#)\s*(?:This code is (?:guaranteed|designed) to|Happy coding|Best regards|I have refactored the)\b").unwrap(),
    ]
}
pub fn detect_watermarks_in_content(file_path: &str, content: &str) -> Vec<SlopFinding> {
    let mut findings = Vec::new();
    let regexes = get_watermark_regexes();
    for (i, line) in content.lines().enumerate() {
        for re in &regexes {
            if re.is_match(line) {
                findings.push(SlopFinding {
                    file: file_path.to_string(),
                    line: i + 1,
                    column: 1,
                    rule_id: "no-ai-sycophancy-watermarks".to_string(),
                    category: SlopCategory::AiBloat,
                    severity: SlopSeverity::Warning,
                    message: "Conversational AI sycophancy or chat watermark comment left in code".to_string(),
                    snippet: line.trim().to_string(),
                    fixable: true,
                    suggestion: Some("Remove conversational chat watermark artifact before committing".to_string()),
                });
            }
        }
    }
    findings
}
pub fn detect_watermarks_in_diff(diff_files: &[DiffFile]) -> Vec<SlopFinding> {
    let mut findings = Vec::new();
    let regexes = get_watermark_regexes();
    for f in diff_files {
        let path = if !f.new_path.is_empty() { &f.new_path } else { &f.old_path };
        for c in &f.chunks {
            for l in &c.lines {
                if l.line_type == DiffLineType::Add {
                    for re in &regexes {
                        if re.is_match(&l.content) {
                            findings.push(SlopFinding {
                                file: path.clone(),
                                line: l.new_line_number.unwrap_or(1),
                                column: 1,
                                rule_id: "no-ai-sycophancy-watermarks".to_string(),
                                category: SlopCategory::AiBloat,
                                severity: SlopSeverity::Warning,
                                message: "Conversational AI sycophancy or chat watermark comment left in code".to_string(),
                                snippet: l.content.trim().to_string(),
                                fixable: true,
                                suggestion: Some("Remove conversational chat watermark artifact".to_string()),
                            });
                        }
                    }
                }
            }
        }
    }
    findings
}