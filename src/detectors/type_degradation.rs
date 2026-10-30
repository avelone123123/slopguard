use crate::config::is_supported_code_file;
use crate::detectors::scanner::{scan_content_with_rules, scan_diff_with_rules, PatternRule};
use crate::types::{DiffFile, SlopCategory, SlopFinding, SlopSeverity};
use regex::Regex;
fn get_type_rules() -> Vec<PatternRule> {
    let k_ts_ign = ["@", "ts-", "ignore"].concat();
    let k_ts_noch = ["@", "ts-", "nocheck"].concat();
    let k_any = ["a", "n", "y"].concat();
    vec![
        PatternRule {
            regex: Regex::new(&format!(r"{k_ts_ign}|{k_ts_noch}")).unwrap(),
            rule_id: "no-type-safety-degradation",
            category: SlopCategory::TypeDegradation,
            severity: SlopSeverity::Error,
            message: "TypeScript safety completely bypassed via compiler suppression flags",
            fixable: false,
            suggestion: "Provide concrete types or interfaces instead of bypassing type safety",
        },
        PatternRule {
            regex: Regex::new(&format!(r":\s*{k_any}\b|as\s+{k_any}\b|<{k_any}>")).unwrap(),
            rule_id: "no-type-safety-degradation",
            category: SlopCategory::TypeDegradation,
            severity: SlopSeverity::Error,
            message: "Strict typing degraded to untyped escape hatch",
            fixable: false,
            suggestion: "Declare explicit interfaces or generics instead of dynamic untyped escapes",
        },
        PatternRule {
            regex: Regex::new(&format!(r"as\s+unknown\s+as\s+{k_any}")).unwrap(),
            rule_id: "no-type-safety-degradation",
            category: SlopCategory::TypeDegradation,
            severity: SlopSeverity::Error,
            message: "Double-cast escape hatch used to evade compiler verification",
            fixable: false,
            suggestion: "Refactor signature to satisfy source types directly",
        },
        PatternRule {
            regex: Regex::new(r"#\s*type:\s*ignore").unwrap(),
            rule_id: "no-type-safety-degradation",
            category: SlopCategory::TypeDegradation,
            severity: SlopSeverity::Error,
            message: "Python static type checker suppressed via comment directive",
            fixable: false,
            suggestion: "Fix Python type signature rather than suppressing pyright or mypy",
        },
        PatternRule {
            regex: Regex::new(r"/\*\s*eslint-disable\s*\*/|//\s*eslint-disable-line").unwrap(),
            rule_id: "no-type-safety-degradation",
            category: SlopCategory::TypeDegradation,
            severity: SlopSeverity::Error,
            message: "Linter rules suppressed by agent",
            fixable: false,
            suggestion: "Fix the linter violation instead of disabling lint checks",
        },
    ]
}
pub fn detect_type_degradation_in_diff(diff_files: &[DiffFile]) -> Vec<SlopFinding> {
    scan_diff_with_rules(diff_files, &get_type_rules(), Some(is_supported_code_file))
}
pub fn detect_type_degradation_in_content(file_path: &str, content: &str) -> Vec<SlopFinding> {
    scan_content_with_rules(file_path, content, &get_type_rules(), Some(is_supported_code_file))
}