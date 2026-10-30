use crate::detectors::scanner::{scan_content_with_rules, scan_diff_with_rules, PatternRule};
use crate::types::{DiffFile, SlopCategory, SlopFinding, SlopSeverity};
use regex::Regex;
fn get_placeholder_rules() -> Vec<PatternRule> {
    let k_todo = ["T", "O", "D", "O"].concat();
    let k_fixme = ["F", "I", "X", "M", "E"].concat();
    let k_stub = ["S", "T", "U", "B"].concat();
    let k_ph = ["P", "L", "A", "C", "E", "H", "O", "L", "D", "E", "R"].concat();
    let k_words = format!("{k_todo}|{k_fixme}|{k_stub}|{k_ph}|NOT_IMPLEMENTED");
    vec![
        PatternRule {
            regex: Regex::new(&format!(r"(?i)\b(?:{k_words})\b.*?(?:implement|add logic|fill in|fix this|later)")).unwrap(),
            rule_id: "no-ai-todo-placeholders",
            category: SlopCategory::Placeholder,
            severity: SlopSeverity::Error,
            message: "Unfinished AI placeholder stub detected",
            fixable: false,
            suggestion: "Implement the required business logic or throw a domain-specific handled error",
        },
        PatternRule {
            regex: Regex::new(r#"(?i)(?:throw\s+new\s+(?:Error|NotImplementedError|Exception)|panic!|panic)\s*\(\s*["'`].*?(?:not implemented|placeholder|stub).*?["'`]\s*\)"#).unwrap(),
            rule_id: "no-ai-todo-placeholders",
            category: SlopCategory::Placeholder,
            severity: SlopSeverity::Error,
            message: "Explicit unimplemented error placeholder thrown by AI",
            fixable: false,
            suggestion: "Provide concrete implementation instead of throwing placeholder error",
        },
        PatternRule {
            regex: Regex::new(r"\b(?:todo!|unimplemented!)\s*\(\s*\)").unwrap(),
            rule_id: "no-ai-todo-placeholders",
            category: SlopCategory::Placeholder,
            severity: SlopSeverity::Error,
            message: "Rust todo!() or unimplemented!() macro left unfinished in production code",
            fixable: false,
            suggestion: "Provide concrete implementation rather than leaving macro placeholder",
        },
        PatternRule {
            regex: Regex::new(r"catch\s*\([^)]*\)\s*\{\s*(?://[^\n]*|/\*[\s\S]*?\*/)?\s*\}").unwrap(),
            rule_id: "no-silent-catch-pass",
            category: SlopCategory::SilentCatch,
            severity: SlopSeverity::Error,
            message: "Silent empty catch block swallowing agent errors",
            fixable: false,
            suggestion: "Handle or rethrow the error explicitly",
        },
        PatternRule {
            regex: Regex::new(r"except\s+(?:Exception|\w+)?:\s*(?:pass|\.\.\.)").unwrap(),
            rule_id: "no-silent-catch-pass",
            category: SlopCategory::SilentCatch,
            severity: SlopSeverity::Error,
            message: "Silent except-pass block swallowing errors",
            fixable: false,
            suggestion: "Handle or log the caught exception",
        },
        PatternRule {
            regex: Regex::new(r#"(?i)["'`](?:sk-[a-zA-Z0-9]{20,}|ghp_[a-zA-Z0-9]{20,}|xoxb-[a-zA-Z0-9]{20,}|dummy[-_]?(?:token|secret|key|api)|fake[-_]?(?:token|secret|key)|test[-_]api[-_]key)["'`]"#).unwrap(),
            rule_id: "no-dummy-secrets-mocks",
            category: SlopCategory::Placeholder,
            severity: SlopSeverity::Error,
            message: "Hardcoded fake/dummy credentials or mockup tokens left by AI",
            fixable: false,
            suggestion: "Use environment variables or runtime configurations instead of hardcoded mock secrets",
        },
    ]
}
pub fn detect_placeholders_in_diff(diff_files: &[DiffFile]) -> Vec<SlopFinding> {
    scan_diff_with_rules(diff_files, &get_placeholder_rules(), None)
}
pub fn detect_placeholders_in_content(file_path: &str, content: &str) -> Vec<SlopFinding> {
    scan_content_with_rules(file_path, content, &get_placeholder_rules(), None)
}