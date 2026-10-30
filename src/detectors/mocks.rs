use crate::detectors::scanner::{scan_content_with_rules, scan_diff_with_rules, PatternRule};
use crate::types::{DiffFile, SlopCategory, SlopFinding, SlopSeverity};
use regex::Regex;
fn get_mock_rules() -> Vec<PatternRule> {
    vec![
        PatternRule {
            regex: Regex::new(r#"(?i)["'`](?:https?://)?(?:[a-zA-Z0-9_.-]+\.)?(?:jsonplaceholder\.typicode\.com|httpbin\.org|reqres\.in|dummyjson\.com)(?:/[^\s"'`]*)?["'`]"#).unwrap(),
            rule_id: "no-mock-hallucinations",
            category: SlopCategory::Placeholder,
            severity: SlopSeverity::Error,
            message: "Hallucinated public mock API URL left in production code by AI",
            fixable: false,
            suggestion: "Replace public placeholder mock API with configured domain endpoint",
        },
        PatternRule {
            regex: Regex::new(r#"["'`]eyJ[a-zA-Z0-9_-]{10,}\.eyJ[a-zA-Z0-9_-]{10,}\.[a-zA-Z0-9_-]{10,}["'`]"#).unwrap(),
            rule_id: "no-mock-hallucinations",
            category: SlopCategory::Placeholder,
            severity: SlopSeverity::Error,
            message: "Hardcoded mock JWT token string detected",
            fixable: false,
            suggestion: "Inject authentication tokens via secure session or environment variables",
        },
        PatternRule {
            regex: Regex::new(r#"(?i)["'`](?:john(?:_doe)?@example\.com|testuser\d*@example\.com|admin@dummy\.com)["'`]"#).unwrap(),
            rule_id: "no-mock-hallucinations",
            category: SlopCategory::Placeholder,
            severity: SlopSeverity::Warning,
            message: "Mock dummy email address placeholder detected",
            fixable: false,
            suggestion: "Remove mock identity stubs before deploying to production",
        },
    ]
}
pub fn detect_mocks_in_diff(diff_files: &[DiffFile]) -> Vec<SlopFinding> {
    scan_diff_with_rules(diff_files, &get_mock_rules(), None)
}
pub fn detect_mocks_in_content(file_path: &str, content: &str) -> Vec<SlopFinding> {
    scan_content_with_rules(file_path, content, &get_mock_rules(), None)
}