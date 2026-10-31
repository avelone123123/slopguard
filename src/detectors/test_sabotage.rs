use crate::types::{DiffFile, DiffLineType, SlopCategory, SlopFinding, SlopSeverity};
use regex::Regex;
fn is_test_file(path: &str) -> bool {
    let lower = path.to_lowercase();
    lower.contains(".test.") || lower.contains(".spec.") || lower.contains("/tests/") || lower.contains("\\tests\\") || lower.contains("/test/") || lower.contains("\\test\\") || lower.starts_with("test_") || lower.ends_with("_test.go") || lower.ends_with("_test.py") || lower.ends_with("_test.rs") || lower.ends_with("test.py") || lower.ends_with("test.ts") || lower.ends_with("test.js") || lower.ends_with("tests.rs")
}
fn get_js_test_skip_regex() -> Regex {
    Regex::new(r#"(?:\b(?:it|test|describe)\s*\.\s*skip\b|\b(?:xit|xtest|xdescribe)\s*\()"#).unwrap()
}
fn get_js_test_only_regex() -> Regex {
    Regex::new(r#"(?:\b(?:it|test|describe)\s*\.\s*only\b|\b(?:fit|fdescribe)\s*\()"#).unwrap()
}
fn get_py_test_decorator_regex() -> Regex {
    Regex::new(r#"^\s*@(?:pytest\.mark\.(?:skip|skipif|xfail)|unittest\.skip(?:If|Unless)?)\b"#).unwrap()
}
fn get_py_test_inline_regex() -> Regex {
    Regex::new(r#"(?:\bpytest\.skip\s*\(|\braise\s+unittest\.SkipTest\b)"#).unwrap()
}
fn get_rust_test_ignore_regex() -> Regex {
    Regex::new(r#"^#\[\s*ignore(?:\s*=.*)?\s*\]"#).unwrap()
}
pub fn detect_test_sabotage_in_content(file_path: &str, content: &str) -> Vec<SlopFinding> {
    let mut findings = Vec::new();
    let lower_path = file_path.to_lowercase();
    let is_js = lower_path.ends_with(".ts") || lower_path.ends_with(".tsx") || lower_path.ends_with(".js") || lower_path.ends_with(".jsx") || lower_path.ends_with(".mjs") || lower_path.ends_with(".cjs");
    let is_python = lower_path.ends_with(".py");
    let is_rust = lower_path.ends_with(".rs");
    let is_test = is_test_file(file_path);
    if !is_js && !is_python && !is_rust && !is_test {
        return findings;
    }
    let js_skip_re = get_js_test_skip_regex();
    let js_only_re = get_js_test_only_regex();
    let py_dec_re = get_py_test_decorator_regex();
    let py_inline_re = get_py_test_inline_regex();
    let rust_ignore_re = get_rust_test_ignore_regex();
    let has_test_attr = is_rust && (is_test || content.contains("#[test]"));
    for (idx, line) in content.lines().enumerate() {
        let line_num = idx + 1;
        let trimmed = line.trim();
        if trimmed.starts_with("//") || trimmed.starts_with("/*") || trimmed.starts_with('*') {
            continue;
        }
        if is_python && trimmed.starts_with('#') && !trimmed.starts_with("#[") {
            continue;
        }
        if (is_js || is_test) && js_only_re.is_match(trimmed) {
            findings.push(SlopFinding {
                file: file_path.to_string(),
                line: line_num,
                column: 1,
                rule_id: "no-test-sabotage".to_string(),
                category: SlopCategory::Security,
                severity: SlopSeverity::Error,
                message: "Test suite isolation trap: .only() detected, silently bypassing remaining test suite execution in CI".to_string(),
                snippet: trimmed.to_string(),
                fixable: false,
                suggestion: Some("Remove .only modifier to restore full test suite coverage and CI verification".to_string()),
            });
            continue;
        }
        if (is_js || is_test) && js_skip_re.is_match(trimmed) {
            findings.push(SlopFinding {
                file: file_path.to_string(),
                line: line_num,
                column: 1,
                rule_id: "no-test-sabotage".to_string(),
                category: SlopCategory::Security,
                severity: SlopSeverity::Error,
                message: "Test suite sabotage: skipped test detected (it.skip/test.skip/xit) muting test assertion execution".to_string(),
                snippet: trimmed.to_string(),
                fixable: false,
                suggestion: Some("Remove .skip modifier and fix the underlying assertion failure instead of bypassing the test".to_string()),
            });
            continue;
        }
        if (is_python || is_test) && (py_dec_re.is_match(trimmed) || py_inline_re.is_match(trimmed)) {
            findings.push(SlopFinding {
                file: file_path.to_string(),
                line: line_num,
                column: 1,
                rule_id: "no-test-sabotage".to_string(),
                category: SlopCategory::Security,
                severity: SlopSeverity::Error,
                message: "Test suite sabotage: Python test skip detected (@pytest.mark.skip/@unittest.skip) muting test coverage".to_string(),
                snippet: trimmed.to_string(),
                fixable: false,
                suggestion: Some("Remove test skip decorator or call and fix underlying implementation logic or mock fixtures".to_string()),
            });
            continue;
        }
        if (is_rust || is_test) && has_test_attr && rust_ignore_re.is_match(trimmed) {
            findings.push(SlopFinding {
                file: file_path.to_string(),
                line: line_num,
                column: 1,
                rule_id: "no-test-sabotage".to_string(),
                category: SlopCategory::Security,
                severity: SlopSeverity::Error,
                message: "Rust test sabotage: #[ignore] attribute bypasses test execution during cargo test".to_string(),
                snippet: trimmed.to_string(),
                fixable: false,
                suggestion: Some("Remove #[ignore] and resolve the failing assertion to maintain test integrity".to_string()),
            });
        }
    }
    findings
}
pub fn detect_test_sabotage_in_diff(diff_files: &[DiffFile]) -> Vec<SlopFinding> {
    let mut findings = Vec::new();
    let js_skip_re = get_js_test_skip_regex();
    let js_only_re = get_js_test_only_regex();
    let py_dec_re = get_py_test_decorator_regex();
    let py_inline_re = get_py_test_inline_regex();
    let rust_ignore_re = get_rust_test_ignore_regex();
    for file in diff_files {
        if file.is_deleted {
            continue;
        }
        let target_path = if !file.new_path.is_empty() { &file.new_path } else { &file.old_path };
        let lower_path = target_path.to_lowercase();
        let is_js = lower_path.ends_with(".ts") || lower_path.ends_with(".tsx") || lower_path.ends_with(".js") || lower_path.ends_with(".jsx") || lower_path.ends_with(".mjs") || lower_path.ends_with(".cjs");
        let is_python = lower_path.ends_with(".py");
        let is_rust = lower_path.ends_with(".rs");
        let is_test = is_test_file(target_path);
        if !is_js && !is_python && !is_rust && !is_test {
            continue;
        }
        for chunk in &file.chunks {
            for line in &chunk.lines {
                if line.line_type != DiffLineType::Add {
                    continue;
                }
                let trimmed = line.content.trim();
                let line_num = line.new_line_number.unwrap_or(1);
                if trimmed.starts_with("//") || trimmed.starts_with("/*") || trimmed.starts_with('*') {
                    continue;
                }
                if is_python && trimmed.starts_with('#') && !trimmed.starts_with("#[") {
                    continue;
                }
                if (is_js || is_test) && js_only_re.is_match(trimmed) {
                    findings.push(SlopFinding {
                        file: target_path.to_string(),
                        line: line_num,
                        column: 1,
                        rule_id: "no-test-sabotage".to_string(),
                        category: SlopCategory::Security,
                        severity: SlopSeverity::Error,
                        message: "Test suite isolation trap: .only() detected, silently bypassing remaining test suite execution in CI".to_string(),
                        snippet: trimmed.to_string(),
                        fixable: false,
                        suggestion: Some("Remove .only modifier to restore full test suite coverage and CI verification".to_string()),
                    });
                    continue;
                }
                if (is_js || is_test) && js_skip_re.is_match(trimmed) {
                    findings.push(SlopFinding {
                        file: target_path.to_string(),
                        line: line_num,
                        column: 1,
                        rule_id: "no-test-sabotage".to_string(),
                        category: SlopCategory::Security,
                        severity: SlopSeverity::Error,
                        message: "Test suite sabotage: skipped test detected (it.skip/test.skip/xit) muting test assertion execution".to_string(),
                        snippet: trimmed.to_string(),
                        fixable: false,
                        suggestion: Some("Remove .skip modifier and fix the underlying assertion failure instead of bypassing the test".to_string()),
                    });
                    continue;
                }
                if (is_python || is_test) && (py_dec_re.is_match(trimmed) || py_inline_re.is_match(trimmed)) {
                    findings.push(SlopFinding {
                        file: target_path.to_string(),
                        line: line_num,
                        column: 1,
                        rule_id: "no-test-sabotage".to_string(),
                        category: SlopCategory::Security,
                        severity: SlopSeverity::Error,
                        message: "Test suite sabotage: Python test skip detected (@pytest.mark.skip/@unittest.skip) muting test coverage".to_string(),
                        snippet: trimmed.to_string(),
                        fixable: false,
                        suggestion: Some("Remove test skip decorator or call and fix underlying implementation logic or mock fixtures".to_string()),
                    });
                    continue;
                }
                if (is_rust || is_test) && rust_ignore_re.is_match(trimmed) {
                    findings.push(SlopFinding {
                        file: target_path.to_string(),
                        line: line_num,
                        column: 1,
                        rule_id: "no-test-sabotage".to_string(),
                        category: SlopCategory::Security,
                        severity: SlopSeverity::Error,
                        message: "Rust test sabotage: #[ignore] attribute bypasses test execution during cargo test".to_string(),
                        snippet: trimmed.to_string(),
                        fixable: false,
                        suggestion: Some("Remove #[ignore] and resolve the failing assertion to maintain test integrity".to_string()),
                    });
                }
            }
        }
    }
    findings
}