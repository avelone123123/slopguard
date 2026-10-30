use crate::types::{DiffFile, DiffLine, DiffLineType, SlopCategory, SlopFinding, SlopSeverity};
use regex::Regex;
fn is_js_ts(path: &str) -> bool {
    let lower = path.to_lowercase();
    lower.ends_with(".ts") || lower.ends_with(".tsx") || lower.ends_with(".js") || lower.ends_with(".jsx") || lower.ends_with(".mjs") || lower.ends_with(".cjs")
}
fn is_statement_handled(trimmed: &str) -> bool {
    trimmed.starts_with("const ")
        || trimmed.starts_with("let ")
        || trimmed.starts_with("var ")
        || trimmed.starts_with("return ")
        || trimmed.starts_with("await ")
        || trimmed.starts_with("yield ")
        || trimmed.starts_with("void ")
        || trimmed.starts_with("export ")
}
fn has_chained_catch(lines: &[&str], start_idx: usize) -> bool {
    for offset in 1..=4 {
        if let Some(next) = lines.get(start_idx + offset) {
            if next.contains(".catch(") {
                return true;
            }
            if next.contains(';') || next.trim().starts_with("const ") || next.trim().starts_with("let ") || next.trim().starts_with("function ") {
                return false;
            }
        }
    }
    false
}
fn has_chained_catch_diff(lines: &[DiffLine], start_idx: usize) -> bool {
    for offset in 1..=4 {
        if let Some(dl) = lines.get(start_idx + offset) {
            let next = &dl.content;
            if next.contains(".catch(") {
                return true;
            }
            if next.contains(';') || next.trim().starts_with("const ") || next.trim().starts_with("let ") || next.trim().starts_with("function ") {
                return false;
            }
        }
    }
    false
}
pub fn detect_floating_promises_in_content(file_path: &str, content: &str) -> Vec<SlopFinding> {
    let mut findings = Vec::new();
    if !is_js_ts(file_path) {
        return findings;
    }
    let for_each_re = Regex::new(r#"\.forEach\s*\(\s*(?:async\b|\(\s*async\b)"#).unwrap();
    let async_executor_re = Regex::new(r#"\bnew\s+Promise(?:\s*<[^>]+>)?\s*\(\s*async\s*(?:function\b[a-zA-Z0-9_$]*|(?:\([^)]*\)|[a-zA-Z_$][a-zA-Z0-9_$]*)\s*=>)"#).unwrap();
    let single_line_try_re = Regex::new(r#"\btry\s*\{[^}]*\breturn\s+([a-zA-Z0-9_$]+(?:\.[a-zA-Z0-9_$]+)*\s*\([^;{}]*\))"#).unwrap();
    let return_call_re = Regex::new(r#"^\s*return\s+([a-zA-Z0-9_$]+(?:\.[a-zA-Z0-9_$]+)*\s*\([^;{}]*\))"#).unwrap();
    let floating_then_re = Regex::new(r#"^[a-zA-Z_$][a-zA-Z0-9_$]*(?:\.[a-zA-Z0-9_$]+|\([^)]*\)|\[[^\]]*\])*\.then\s*\("#).unwrap();
    let lines: Vec<&str> = content.lines().collect();
    let mut try_depth: i32 = 0;
    for (idx, line) in lines.iter().enumerate() {
        let line_num = idx + 1;
        if line.contains("try {") || line.contains("try{") {
            try_depth = 1;
        } else if try_depth > 0 {
            for c in line.chars() {
                if c == '{' {
                    try_depth += 1;
                } else if c == '}' {
                    try_depth -= 1;
                }
            }
        }
        if for_each_re.is_match(line) {
            findings.push(SlopFinding {
                file: file_path.to_string(),
                line: line_num,
                column: 1,
                rule_id: "no-floating-promises".to_string(),
                category: SlopCategory::SilentCatch,
                severity: SlopSeverity::Error,
                message: "Async callback in forEach executes unawaited with unhandled rejections".to_string(),
                snippet: line.trim().to_string(),
                fixable: false,
                suggestion: Some("Replace with 'for (const item of items)' loop or 'await Promise.all(items.map(...))'".to_string()),
            });
        }
        if async_executor_re.is_match(line) {
            findings.push(SlopFinding {
                file: file_path.to_string(),
                line: line_num,
                column: 1,
                rule_id: "no-floating-promises".to_string(),
                category: SlopCategory::SilentCatch,
                severity: SlopSeverity::Error,
                message: "Async Promise executor function causes uncatchable rejections".to_string(),
                snippet: line.trim().to_string(),
                fixable: false,
                suggestion: Some("Avoid async executor functions in Promise constructor; handle asynchronous logic inside standard executor or wrap in try/catch".to_string()),
            });
        }
        if !line.contains("return await") && single_line_try_re.is_match(line) {
            findings.push(SlopFinding {
                file: file_path.to_string(),
                line: line_num,
                column: 1,
                rule_id: "no-floating-promises".to_string(),
                category: SlopCategory::SilentCatch,
                severity: SlopSeverity::Error,
                message: "Unawaited promise returned inside try block bypasses catch handler".to_string(),
                snippet: line.trim().to_string(),
                fixable: false,
                suggestion: Some("Add 'await' before returned promise inside try block: 'return await ...'".to_string()),
            });
        } else if !line.contains("return await") && try_depth > 0 && return_call_re.is_match(line) {
            findings.push(SlopFinding {
                file: file_path.to_string(),
                line: line_num,
                column: 1,
                rule_id: "no-floating-promises".to_string(),
                category: SlopCategory::SilentCatch,
                severity: SlopSeverity::Error,
                message: "Unawaited promise returned inside try block bypasses catch handler".to_string(),
                snippet: line.trim().to_string(),
                fixable: false,
                suggestion: Some("Add 'await' before returned promise inside try block: 'return await ...'".to_string()),
            });
        }
        let trimmed = line.trim();
        if !is_statement_handled(trimmed) && floating_then_re.is_match(trimmed) && !line.contains(".catch(") && !has_chained_catch(&lines, idx) {
            findings.push(SlopFinding {
                file: file_path.to_string(),
                line: line_num,
                column: 1,
                rule_id: "no-floating-promises".to_string(),
                category: SlopCategory::SilentCatch,
                severity: SlopSeverity::Error,
                message: "Floating Promise with unhandled rejection: .then() chain without .catch()".to_string(),
                snippet: line.trim().to_string(),
                fixable: false,
                suggestion: Some("Append .catch(...) handler or await the promise inside a try/catch block".to_string()),
            });
        }
        if try_depth > 0 && line.contains("catch") {
            try_depth = 0;
        }
    }
    findings
}
pub fn detect_floating_promises_in_diff(diff_files: &[DiffFile]) -> Vec<SlopFinding> {
    let mut findings = Vec::new();
    let for_each_re = Regex::new(r#"\.forEach\s*\(\s*(?:async\b|\(\s*async\b)"#).unwrap();
    let async_executor_re = Regex::new(r#"\bnew\s+Promise(?:\s*<[^>]+>)?\s*\(\s*async\s*(?:function\b[a-zA-Z0-9_$]*|(?:\([^)]*\)|[a-zA-Z_$][a-zA-Z0-9_$]*)\s*=>)"#).unwrap();
    let single_line_try_re = Regex::new(r#"\btry\s*\{[^}]*\breturn\s+([a-zA-Z0-9_$]+(?:\.[a-zA-Z0-9_$]+)*\s*\([^;{}]*\))"#).unwrap();
    let return_call_re = Regex::new(r#"^\s*return\s+([a-zA-Z0-9_$]+(?:\.[a-zA-Z0-9_$]+)*\s*\([^;{}]*\))"#).unwrap();
    let floating_then_re = Regex::new(r#"^[a-zA-Z_$][a-zA-Z0-9_$]*(?:\.[a-zA-Z0-9_$]+|\([^)]*\)|\[[^\]]*\])*\.then\s*\("#).unwrap();
    for f in diff_files {
        let path = if !f.new_path.is_empty() { &f.new_path } else { &f.old_path };
        if !is_js_ts(path) {
            continue;
        }
        for c in &f.chunks {
            let mut try_depth: i32 = 0;
            for (idx, l) in c.lines.iter().enumerate() {
                if l.content.contains("try {") || l.content.contains("try{") {
                    try_depth = 1;
                } else if try_depth > 0 {
                    for ch in l.content.chars() {
                        if ch == '{' {
                            try_depth += 1;
                        } else if ch == '}' {
                            try_depth -= 1;
                        }
                    }
                }
                if l.line_type == DiffLineType::Add {
                    let line_num = l.new_line_number.unwrap_or(1);
                    if for_each_re.is_match(&l.content) {
                        findings.push(SlopFinding {
                            file: path.clone(),
                            line: line_num,
                            column: 1,
                            rule_id: "no-floating-promises".to_string(),
                            category: SlopCategory::SilentCatch,
                            severity: SlopSeverity::Error,
                            message: "Async callback in forEach executes unawaited with unhandled rejections".to_string(),
                            snippet: l.content.trim().to_string(),
                            fixable: false,
                            suggestion: Some("Replace with 'for (const item of items)' loop or 'await Promise.all(items.map(...))'".to_string()),
                        });
                    }
                    if async_executor_re.is_match(&l.content) {
                        findings.push(SlopFinding {
                            file: path.clone(),
                            line: line_num,
                            column: 1,
                            rule_id: "no-floating-promises".to_string(),
                            category: SlopCategory::SilentCatch,
                            severity: SlopSeverity::Error,
                            message: "Async Promise executor function causes uncatchable rejections".to_string(),
                            snippet: l.content.trim().to_string(),
                            fixable: false,
                            suggestion: Some("Avoid async executor functions in Promise constructor; handle asynchronous logic inside standard executor or wrap in try/catch".to_string()),
                        });
                    }
                    if !l.content.contains("return await") && single_line_try_re.is_match(&l.content) {
                        findings.push(SlopFinding {
                            file: path.clone(),
                            line: line_num,
                            column: 1,
                            rule_id: "no-floating-promises".to_string(),
                            category: SlopCategory::SilentCatch,
                            severity: SlopSeverity::Error,
                            message: "Unawaited promise returned inside try block bypasses catch handler".to_string(),
                            snippet: l.content.trim().to_string(),
                            fixable: false,
                            suggestion: Some("Add 'await' before returned promise inside try block: 'return await ...'".to_string()),
                        });
                    } else if !l.content.contains("return await") && try_depth > 0 && return_call_re.is_match(&l.content) {
                        findings.push(SlopFinding {
                            file: path.clone(),
                            line: line_num,
                            column: 1,
                            rule_id: "no-floating-promises".to_string(),
                            category: SlopCategory::SilentCatch,
                            severity: SlopSeverity::Error,
                            message: "Unawaited promise returned inside try block bypasses catch handler".to_string(),
                            snippet: l.content.trim().to_string(),
                            fixable: false,
                            suggestion: Some("Add 'await' before returned promise inside try block: 'return await ...'".to_string()),
                        });
                    }
                    let trimmed = l.content.trim();
                    if !is_statement_handled(trimmed) && floating_then_re.is_match(trimmed) && !l.content.contains(".catch(") && !has_chained_catch_diff(&c.lines, idx) {
                        findings.push(SlopFinding {
                            file: path.clone(),
                            line: line_num,
                            column: 1,
                            rule_id: "no-floating-promises".to_string(),
                            category: SlopCategory::SilentCatch,
                            severity: SlopSeverity::Error,
                            message: "Floating Promise with unhandled rejection: .then() chain without .catch()".to_string(),
                            snippet: l.content.trim().to_string(),
                            fixable: false,
                            suggestion: Some("Append .catch(...) handler or await the promise inside a try/catch block".to_string()),
                        });
                    }
                }
                if try_depth > 0 && l.content.contains("catch") {
                    try_depth = 0;
                }
            }
        }
    }
    findings
}