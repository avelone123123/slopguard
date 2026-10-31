use crate::types::{DiffFile, DiffLineType, SlopCategory, SlopFinding, SlopSeverity};
use regex::Regex;
fn is_test_file(path: &str) -> bool {
    let lower = path.to_lowercase();
    lower.contains(".test.") || lower.contains(".spec.") || lower.contains("/tests/") || lower.contains("\\tests\\") || lower.contains("/test/") || lower.contains("\\test\\") || lower.starts_with("test_") || lower.ends_with("_test.go") || lower.ends_with("_test.py") || lower.ends_with("_test.rs")
}
fn is_js_ts(path: &str) -> bool {
    let lower = path.to_lowercase();
    lower.ends_with(".ts") || lower.ends_with(".tsx") || lower.ends_with(".js") || lower.ends_with(".jsx") || lower.ends_with(".mjs") || lower.ends_with(".cjs")
}
fn is_py(path: &str) -> bool {
    path.to_lowercase().ends_with(".py")
}
fn is_tautological_expect(line: &str, expect_re: &Regex) -> bool {
    if let Some(caps) = expect_re.captures(line) {
        let left = caps.get(1).map(|m| m.as_str().trim()).unwrap_or("");
        let right = caps.get(2).map(|m| m.as_str().trim()).unwrap_or("");
        if !left.is_empty() && left == right {
            return true;
        }
    }
    false
}
pub fn detect_coverage_theater_in_content(file_path: &str, content: &str) -> Vec<SlopFinding> {
    let mut findings = Vec::new();
    let js_ts = is_js_ts(file_path);
    let py = is_py(file_path);
    let expect_re = Regex::new(r#"\bexpect\s*\(\s*([a-zA-Z0-9_$'"\s]+?)\s*\)\s*\.(?:toBe|toEqual|toStrictEqual)\s*\(\s*([a-zA-Z0-9_$'"\s]+?)\s*\)"#).unwrap();
    let empty_js_test_re = Regex::new(r#"\b(?:it|test)\s*\(\s*["'][^"']+["']\s*,\s*(?:async\s*)?(?:\(\s*\)|[a-zA-Z_$]+)\s*=>\s*\{\s*\}\s*\)"#).unwrap();
    let empty_js_test_fn_re = Regex::new(r#"\b(?:it|test)\s*\(\s*["'][^"']+["']\s*,\s*(?:async\s*)?function\s*\([^)]*\)\s*\{\s*\}\s*\)"#).unwrap();
    let py_assert_tautology_re = Regex::new(r#"^\s*assert\s+(?:True|False|1\s*==\s*1|0\s*==\s*0)\b"#).unwrap();
    let py_unittest_tautology_re = Regex::new(r#"\bself\.assert(?:True\s*\(\s*True\s*\)|False\s*\(\s*False\s*\)|Equal\s*\(\s*1\s*,\s*1\s*\)|Equal\s*\(\s*True\s*,\s*True\s*\))"#).unwrap();
    let py_empty_test_re = Regex::new(r#"^\s*def\s+test_[a-zA-Z0-9_]+\s*\([^)]*\)\s*:\s*(?:pass|\.\.\.)\s*$"#).unwrap();
    let lines: Vec<&str> = content.lines().collect();
    for (idx, line) in lines.iter().enumerate() {
        let line_num = idx + 1;
        let trimmed = line.trim();
        if (js_ts || is_test_file(file_path)) && is_tautological_expect(trimmed, &expect_re) {
            findings.push(SlopFinding {
                file: file_path.to_string(),
                line: line_num,
                column: 1,
                rule_id: "no-coverage-theater".to_string(),
                category: SlopCategory::AiBloat,
                severity: SlopSeverity::Warning,
                message: "Tautological assertion in test creates vacuous coverage without validation".to_string(),
                snippet: trimmed.to_string(),
                fixable: false,
                suggestion: Some("Assert against real dynamic outputs and expected state transformations instead of identical literals".to_string()),
            });
        }
        if (js_ts || is_test_file(file_path)) && (empty_js_test_re.is_match(trimmed) || empty_js_test_fn_re.is_match(trimmed)) {
            findings.push(SlopFinding {
                file: file_path.to_string(),
                line: line_num,
                column: 1,
                rule_id: "no-coverage-theater".to_string(),
                category: SlopCategory::AiBloat,
                severity: SlopSeverity::Warning,
                message: "Empty test block detected: function completes without any assertions or logic execution".to_string(),
                snippet: trimmed.to_string(),
                fixable: false,
                suggestion: Some("Implement concrete test assertions or remove the empty test stub".to_string()),
            });
        }
        if (py || is_test_file(file_path)) && (py_assert_tautology_re.is_match(trimmed) || py_unittest_tautology_re.is_match(trimmed)) {
            findings.push(SlopFinding {
                file: file_path.to_string(),
                line: line_num,
                column: 1,
                rule_id: "no-coverage-theater".to_string(),
                category: SlopCategory::AiBloat,
                severity: SlopSeverity::Warning,
                message: "Vacuous assertion (assert True / assert 1 == 1) detected in Python test suite".to_string(),
                snippet: trimmed.to_string(),
                fixable: false,
                suggestion: Some("Replace tautology with actual verification of the function return value or raised exception".to_string()),
            });
        }
        let is_py_empty = py_empty_test_re.is_match(trimmed) || (trimmed.starts_with("def test_") && trimmed.ends_with(':') && lines.get(idx + 1).map(|next| { let t = next.trim(); t == "pass" || t == "..." }).unwrap_or(false));
        if (py || is_test_file(file_path)) && is_py_empty {
            findings.push(SlopFinding {
                file: file_path.to_string(),
                line: line_num,
                column: 1,
                rule_id: "no-coverage-theater".to_string(),
                category: SlopCategory::AiBloat,
                severity: SlopSeverity::Warning,
                message: "Empty Python test stub (pass/...) detected without verification logic".to_string(),
                snippet: trimmed.to_string(),
                fixable: false,
                suggestion: Some("Implement real test body assertions or mark as pending test fixture".to_string()),
            });
        }
    }
    findings
}
pub fn detect_coverage_theater_in_diff(diff_files: &[DiffFile]) -> Vec<SlopFinding> {
    let mut findings = Vec::new();
    let expect_re = Regex::new(r#"\bexpect\s*\(\s*([a-zA-Z0-9_$'"\s]+?)\s*\)\s*\.(?:toBe|toEqual|toStrictEqual)\s*\(\s*([a-zA-Z0-9_$'"\s]+?)\s*\)"#).unwrap();
    let empty_js_test_re = Regex::new(r#"\b(?:it|test)\s*\(\s*["'][^"']+["']\s*,\s*(?:async\s*)?(?:\(\s*\)|[a-zA-Z_$]+)\s*=>\s*\{\s*\}\s*\)"#).unwrap();
    let empty_js_test_fn_re = Regex::new(r#"\b(?:it|test)\s*\(\s*["'][^"']+["']\s*,\s*(?:async\s*)?function\s*\([^)]*\)\s*\{\s*\}\s*\)"#).unwrap();
    let py_assert_tautology_re = Regex::new(r#"^\s*assert\s+(?:True|False|1\s*==\s*1|0\s*==\s*0)\b"#).unwrap();
    let py_unittest_tautology_re = Regex::new(r#"\bself\.assert(?:True\s*\(\s*True\s*\)|False\s*\(\s*False\s*\)|Equal\s*\(\s*1\s*,\s*1\s*\)|Equal\s*\(\s*True\s*,\s*True\s*\))"#).unwrap();
    let py_empty_test_re = Regex::new(r#"^\s*def\s+test_[a-zA-Z0-9_]+\s*\([^)]*\)\s*:\s*(?:pass|\.\.\.)\s*$"#).unwrap();
    for file in diff_files {
        let path = if !file.new_path.is_empty() { &file.new_path } else { &file.old_path };
        let js_ts = is_js_ts(path);
        let py = is_py(path);
        for chunk in &file.chunks {
            for (line_idx, line) in chunk.lines.iter().enumerate() {
                if line.line_type != DiffLineType::Add {
                    continue;
                }
                let trimmed = line.content.trim();
                let line_num = line.new_line_number.unwrap_or(1);
                if (js_ts || is_test_file(path)) && is_tautological_expect(trimmed, &expect_re) {
                    findings.push(SlopFinding {
                        file: path.to_string(),
                        line: line_num,
                        column: 1,
                        rule_id: "no-coverage-theater".to_string(),
                        category: SlopCategory::AiBloat,
                        severity: SlopSeverity::Warning,
                        message: "Tautological assertion in test creates vacuous coverage without validation".to_string(),
                        snippet: trimmed.to_string(),
                        fixable: false,
                        suggestion: Some("Assert against real dynamic outputs and expected state transformations instead of identical literals".to_string()),
                    });
                }
                if (js_ts || is_test_file(path)) && (empty_js_test_re.is_match(trimmed) || empty_js_test_fn_re.is_match(trimmed)) {
                    findings.push(SlopFinding {
                        file: path.to_string(),
                        line: line_num,
                        column: 1,
                        rule_id: "no-coverage-theater".to_string(),
                        category: SlopCategory::AiBloat,
                        severity: SlopSeverity::Warning,
                        message: "Empty test block detected: function completes without any assertions or logic execution".to_string(),
                        snippet: trimmed.to_string(),
                        fixable: false,
                        suggestion: Some("Implement concrete test assertions or remove the empty test stub".to_string()),
                    });
                }
                if (py || is_test_file(path)) && (py_assert_tautology_re.is_match(trimmed) || py_unittest_tautology_re.is_match(trimmed)) {
                    findings.push(SlopFinding {
                        file: path.to_string(),
                        line: line_num,
                        column: 1,
                        rule_id: "no-coverage-theater".to_string(),
                        category: SlopCategory::AiBloat,
                        severity: SlopSeverity::Warning,
                        message: "Vacuous assertion (assert True / assert 1 == 1) detected in Python test suite".to_string(),
                        snippet: trimmed.to_string(),
                        fixable: false,
                        suggestion: Some("Replace tautology with actual verification of the function return value or raised exception".to_string()),
                    });
                }
                let is_py_empty = py_empty_test_re.is_match(trimmed) || (trimmed.starts_with("def test_") && trimmed.ends_with(':') && chunk.lines.get(line_idx + 1).map(|next| { let t = next.content.trim(); t == "pass" || t == "..." }).unwrap_or(false));
                if (py || is_test_file(path)) && is_py_empty {
                    findings.push(SlopFinding {
                        file: path.to_string(),
                        line: line_num,
                        column: 1,
                        rule_id: "no-coverage-theater".to_string(),
                        category: SlopCategory::AiBloat,
                        severity: SlopSeverity::Warning,
                        message: "Empty Python test stub (pass/...) detected without verification logic".to_string(),
                        snippet: trimmed.to_string(),
                        fixable: false,
                        suggestion: Some("Implement real test body assertions or mark as pending test fixture".to_string()),
                    });
                }
            }
        }
    }
    findings
}