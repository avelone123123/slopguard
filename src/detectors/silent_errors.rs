use crate::types::{DiffFile, DiffLineType, SlopCategory, SlopFinding, SlopSeverity};
use regex::Regex;
fn is_js_ts(path: &str) -> bool {
    let lower = path.to_lowercase();
    lower.ends_with(".ts") || lower.ends_with(".tsx") || lower.ends_with(".js") || lower.ends_with(".jsx") || lower.ends_with(".mjs") || lower.ends_with(".cjs")
}
fn is_py(path: &str) -> bool {
    path.to_lowercase().ends_with(".py")
}
fn is_go(path: &str) -> bool {
    path.to_lowercase().ends_with(".go")
}
fn is_multiline_catch_swallowed(lines: &[&str], idx: usize) -> bool {
    let mut has_body = false;
    for offset in 1..=8 {
        let Some(next_line) = lines.get(idx + offset) else { break; };
        let next_trim = next_line.trim();
        if next_trim.contains('}') {
            break;
        }
        if next_trim.is_empty() {
            continue;
        }
        has_body = true;
        if next_trim.contains("throw ") || next_trim.contains("return ") || next_trim.contains("reject(") || next_trim.contains("process.exit(") || next_trim.contains("dispatch(") || next_trim.contains("next(") {
            return false;
        }
        if !next_trim.starts_with("console.") {
            return false;
        }
    }
    has_body
}
pub fn detect_silent_errors_in_content(file_path: &str, content: &str) -> Vec<SlopFinding> {
    let mut findings = Vec::new();
    let js_ts = is_js_ts(file_path);
    let py = is_py(file_path);
    let go = is_go(file_path);
    if !js_ts && !py && !go {
        return findings;
    }
    let chained_catch_re = Regex::new(r#"\.catch\s*\(\s*(?:\([^)]*\)|[a-zA-Z_$][a-zA-Z0-9_$]*|function\s*\([^)]*\))\s*=>?\s*\{?\s*(?:console\.(?:log|error|warn|info|debug)\([^)]*\);?)?\s*\}?\s*\)"#).unwrap();
    let single_line_catch_swallow_re = Regex::new(r#"catch\s*(?:\([^)]*\))?\s*\{\s*(?:console\.(?:log|error|warn|info|debug)\([^)]*\);?)?\s*\}"#).unwrap();
    let multiline_catch_start_re = Regex::new(r#"catch\s*(?:\([^)]*\))?\s*\{"#).unwrap();
    let supabase_unchecked_re = Regex::new(r#"(?:const|let)\s*\{\s*(?:data|data\s*:\s*[a-zA-Z0-9_$]+)\s*\}\s*=\s*await\s+(?:supabase|supabaseClient|client|api)\b"#).unwrap();
    let py_except_single_re = Regex::new(r#"except(?:\s+[\w\s,()]+)?(?:\s+as\s+\w+)?:\s*(?:pass|\.\.\.|print\([^)]*\)\s*$)"#).unwrap();
    let py_except_start_re = Regex::new(r#"^\s*except(?:\s+[\w\s,()]+)?(?:\s+as\s+\w+)?:\s*$"#).unwrap();
    let go_blank_err_re = Regex::new(r#"^\s*_\s*=\s*err\b|;\s*_\s*=\s*err\b"#).unwrap();
    let lines: Vec<&str> = content.lines().collect();
    for (idx, line) in lines.iter().enumerate() {
        let line_num = idx + 1;
        let trimmed = line.trim();
        if js_ts {
            if chained_catch_re.is_match(line) {
                findings.push(SlopFinding {
                    file: file_path.to_string(),
                    line: line_num,
                    column: 1,
                    rule_id: "no-silent-error-swallowing".to_string(),
                    category: SlopCategory::SilentCatch,
                    severity: SlopSeverity::Error,
                    message: "Chained .catch() silently swallows Promise rejection without propagation or handling".to_string(),
                    snippet: trimmed.to_string(),
                    fixable: false,
                    suggestion: Some("Log the rejection with proper telemetry or rethrow: .catch((err) => { logger.error(err); throw err; })".to_string()),
                });
            }
            if single_line_catch_swallow_re.is_match(line) && !line.contains("throw") && !line.contains("return") {
                findings.push(SlopFinding {
                    file: file_path.to_string(),
                    line: line_num,
                    column: 1,
                    rule_id: "no-silent-error-swallowing".to_string(),
                    category: SlopCategory::SilentCatch,
                    severity: SlopSeverity::Error,
                    message: "Catch block logs error but swallows execution, causing indeterminate application state".to_string(),
                    snippet: trimmed.to_string(),
                    fixable: false,
                    suggestion: Some("Re-throw the error or return a designated fallback result instead of silently continuing".to_string()),
                });
            } else if multiline_catch_start_re.is_match(line) && !line.contains('}') && is_multiline_catch_swallowed(&lines, idx) {
                findings.push(SlopFinding {
                    file: file_path.to_string(),
                    line: line_num,
                    column: 1,
                    rule_id: "no-silent-error-swallowing".to_string(),
                    category: SlopCategory::SilentCatch,
                    severity: SlopSeverity::Error,
                    message: "Catch block logs error but swallows execution, causing indeterminate application state".to_string(),
                    snippet: trimmed.to_string(),
                    fixable: false,
                    suggestion: Some("Re-throw the error or return a designated fallback result instead of silently continuing".to_string()),
                });
            }
            if supabase_unchecked_re.is_match(line) {
                findings.push(SlopFinding {
                    file: file_path.to_string(),
                    line: line_num,
                    column: 1,
                    rule_id: "no-silent-error-swallowing".to_string(),
                    category: SlopCategory::SilentCatch,
                    severity: SlopSeverity::Warning,
                    message: "Destructuring API response without error tuple causes silent failure when data is null".to_string(),
                    snippet: trimmed.to_string(),
                    fixable: false,
                    suggestion: Some("Destructure both data and error: 'const { data, error } = await ...' and handle if (error)".to_string()),
                });
            }
        }
        if py {
            if py_except_single_re.is_match(trimmed) {
                findings.push(SlopFinding {
                    file: file_path.to_string(),
                    line: line_num,
                    column: 1,
                    rule_id: "no-silent-error-swallowing".to_string(),
                    category: SlopCategory::SilentCatch,
                    severity: SlopSeverity::Error,
                    message: "Python except block silently suppresses exception without re-raising".to_string(),
                    snippet: trimmed.to_string(),
                    fixable: false,
                    suggestion: Some("Re-raise the exception with 'raise' or use 'logger.exception()' with intentional fallback".to_string()),
                });
            } else if py_except_start_re.is_match(line) {
                if let Some(next_line) = lines.get(idx + 1) {
                    let next_trim = next_line.trim();
                    if next_trim == "pass" || next_trim == "..." || (next_trim.starts_with("print(") && !next_line.contains("raise")) {
                        findings.push(SlopFinding {
                            file: file_path.to_string(),
                            line: line_num,
                            column: 1,
                            rule_id: "no-silent-error-swallowing".to_string(),
                            category: SlopCategory::SilentCatch,
                            severity: SlopSeverity::Error,
                            message: "Python except block silently suppresses exception without re-raising".to_string(),
                            snippet: trimmed.to_string(),
                            fixable: false,
                            suggestion: Some("Re-raise the exception with 'raise' or use 'logger.exception()' with intentional fallback".to_string()),
                        });
                    }
                }
            }
        }
        if go && go_blank_err_re.is_match(trimmed) {
            findings.push(SlopFinding {
                file: file_path.to_string(),
                line: line_num,
                column: 1,
                rule_id: "no-silent-error-swallowing".to_string(),
                category: SlopCategory::SilentCatch,
                severity: SlopSeverity::Warning,
                message: "Go error swallowed via blank identifier '_ = err' without inspection".to_string(),
                snippet: trimmed.to_string(),
                fixable: false,
                suggestion: Some("Handle the error explicitly: 'if err != nil { return err }'".to_string()),
            });
        }
    }
    findings
}
pub fn detect_silent_errors_in_diff(diff_files: &[DiffFile]) -> Vec<SlopFinding> {
    let mut findings = Vec::new();
    let chained_catch_re = Regex::new(r#"\.catch\s*\(\s*(?:\([^)]*\)|[a-zA-Z_$][a-zA-Z0-9_$]*|function\s*\([^)]*\))\s*=>?\s*\{?\s*(?:console\.(?:log|error|warn|info|debug)\([^)]*\);?)?\s*\}?\s*\)"#).unwrap();
    let single_line_catch_swallow_re = Regex::new(r#"catch\s*(?:\([^)]*\))?\s*\{\s*(?:console\.(?:log|error|warn|info|debug)\([^)]*\);?)?\s*\}"#).unwrap();
    let supabase_unchecked_re = Regex::new(r#"(?:const|let)\s*\{\s*(?:data|data\s*:\s*[a-zA-Z0-9_$]+)\s*\}\s*=\s*await\s+(?:supabase|supabaseClient|client|api)\b"#).unwrap();
    let py_except_single_re = Regex::new(r#"except(?:\s+[\w\s,()]+)?(?:\s+as\s+\w+)?:\s*(?:pass|\.\.\.|print\([^)]*\)\s*$)"#).unwrap();
    let go_blank_err_re = Regex::new(r#"^\s*_\s*=\s*err\b|;\s*_\s*=\s*err\b"#).unwrap();
    for file in diff_files {
        if file.is_deleted {
            continue;
        }
        let target_path = if !file.new_path.is_empty() { &file.new_path } else { &file.old_path };
        let js_ts = is_js_ts(target_path);
        let py = is_py(target_path);
        let go = is_go(target_path);
        if !js_ts && !py && !go {
            continue;
        }
        for chunk in &file.chunks {
            for line in &chunk.lines {
                if line.line_type != DiffLineType::Add {
                    continue;
                }
                let line_num = line.new_line_number.unwrap_or(1);
                let trimmed = line.content.trim();
                if js_ts {
                    if chained_catch_re.is_match(&line.content) {
                        findings.push(SlopFinding {
                            file: target_path.to_string(),
                            line: line_num,
                            column: 1,
                            rule_id: "no-silent-error-swallowing".to_string(),
                            category: SlopCategory::SilentCatch,
                            severity: SlopSeverity::Error,
                            message: "Chained .catch() silently swallows Promise rejection without propagation or handling".to_string(),
                            snippet: trimmed.to_string(),
                            fixable: false,
                            suggestion: Some("Log the rejection with proper telemetry or rethrow: .catch((err) => { logger.error(err); throw err; })".to_string()),
                        });
                    }
                    if single_line_catch_swallow_re.is_match(&line.content) && !line.content.contains("throw") && !line.content.contains("return") {
                        findings.push(SlopFinding {
                            file: target_path.to_string(),
                            line: line_num,
                            column: 1,
                            rule_id: "no-silent-error-swallowing".to_string(),
                            category: SlopCategory::SilentCatch,
                            severity: SlopSeverity::Error,
                            message: "Catch block logs error but swallows execution, causing indeterminate application state".to_string(),
                            snippet: trimmed.to_string(),
                            fixable: false,
                            suggestion: Some("Re-throw the error or return a designated fallback result instead of silently continuing".to_string()),
                        });
                    }
                    if supabase_unchecked_re.is_match(&line.content) {
                        findings.push(SlopFinding {
                            file: target_path.to_string(),
                            line: line_num,
                            column: 1,
                            rule_id: "no-silent-error-swallowing".to_string(),
                            category: SlopCategory::SilentCatch,
                            severity: SlopSeverity::Warning,
                            message: "Destructuring API response without error tuple causes silent failure when data is null".to_string(),
                            snippet: trimmed.to_string(),
                            fixable: false,
                            suggestion: Some("Destructure both data and error: 'const { data, error } = await ...' and handle if (error)".to_string()),
                        });
                    }
                }
                if py && py_except_single_re.is_match(trimmed) {
                    findings.push(SlopFinding {
                        file: target_path.to_string(),
                        line: line_num,
                        column: 1,
                        rule_id: "no-silent-error-swallowing".to_string(),
                        category: SlopCategory::SilentCatch,
                        severity: SlopSeverity::Error,
                        message: "Python except block silently suppresses exception without re-raising".to_string(),
                        snippet: trimmed.to_string(),
                        fixable: false,
                        suggestion: Some("Re-raise the exception with 'raise' or use 'logger.exception()' with intentional fallback".to_string()),
                    });
                }
                if go && go_blank_err_re.is_match(trimmed) {
                    findings.push(SlopFinding {
                        file: target_path.to_string(),
                        line: line_num,
                        column: 1,
                        rule_id: "no-silent-error-swallowing".to_string(),
                        category: SlopCategory::SilentCatch,
                        severity: SlopSeverity::Warning,
                        message: "Go error swallowed via blank identifier '_ = err' without inspection".to_string(),
                        snippet: trimmed.to_string(),
                        fixable: false,
                        suggestion: Some("Handle the error explicitly: 'if err != nil { return err }'".to_string()),
                    });
                }
            }
        }
    }
    findings
}