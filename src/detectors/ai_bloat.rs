use crate::detectors::scanner::{scan_content_with_rules, scan_diff_with_rules, PatternRule};
use crate::types::{DiffFile, SlopCategory, SlopFinding, SlopSeverity};
use regex::Regex;
fn get_bloat_rules() -> Vec<PatternRule> {
    let k_todo = ["T", "O", "D", "O"].concat();
    vec![
        PatternRule {
            regex: Regex::new(r"(?i)^\s*(?://|#)\s*(?:Here(?:'s| is) (?:the|our|an?)|Let(?:'s| us) (?:add|implement|create|fix|update)|In order to|Note that|This will allow us|We need to)\b").unwrap(),
            rule_id: "no-ai-conversational-bloat",
            category: SlopCategory::AiBloat,
            severity: SlopSeverity::Warning,
            message: "Conversational AI comment bloat detected in production code",
            fixable: true,
            suggestion: "Run 'slopguard fix' to automatically strip AI comments",
        },
        PatternRule {
            regex: Regex::new(&format!(r"(?i)^\s*(?://|#)\s*(?:Step \d+:|{k_todo}:?\s*(?:implement|add)\s*(?:logic|later|code))\b")).unwrap(),
            rule_id: "no-ai-conversational-bloat",
            category: SlopCategory::AiBloat,
            severity: SlopSeverity::Warning,
            message: "Conversational AI comment bloat detected in production code",
            fixable: true,
            suggestion: "Run 'slopguard fix' to automatically strip AI comments",
        },
        PatternRule {
            regex: Regex::new(r"(?i)^\s*(?://|#)\s*(?:Function|Method|Class) to\b").unwrap(),
            rule_id: "no-ai-conversational-bloat",
            category: SlopCategory::AiBloat,
            severity: SlopSeverity::Warning,
            message: "Conversational AI comment bloat detected in production code",
            fixable: true,
            suggestion: "Run 'slopguard fix' to automatically strip AI comments",
        },
    ]
}
pub fn detect_ai_comment_bloat_in_diff(diff_files: &[DiffFile]) -> Vec<SlopFinding> {
    scan_diff_with_rules(diff_files, &get_bloat_rules(), None)
}
pub fn detect_ai_comment_bloat_in_content(file_path: &str, content: &str) -> Vec<SlopFinding> {
    scan_content_with_rules(file_path, content, &get_bloat_rules(), None)
}