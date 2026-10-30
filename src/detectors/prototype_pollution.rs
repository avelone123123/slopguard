use crate::types::{DiffFile, DiffLineType, SlopCategory, SlopFinding, SlopSeverity};
use regex::Regex;
fn is_js_ts(path: &str) -> bool {
    let lower = path.to_lowercase();
    lower.ends_with(".ts") || lower.ends_with(".tsx") || lower.ends_with(".js") || lower.ends_with(".jsx") || lower.ends_with(".mjs") || lower.ends_with(".cjs")
}
fn has_prototype_guard(text: &str) -> bool {
    (text.contains("__proto__") && (text.contains("===") || text.contains("!==") || text.contains("==") || text.contains("!=") || text.contains(".indexOf") || text.contains(".includes")))
        || (text.contains("constructor") && (text.contains("===") || text.contains("!==") || text.contains("==") || text.contains("!=")))
        || (text.contains("prototype") && (text.contains("===") || text.contains("!==") || text.contains("==") || text.contains("!=")))
        || text.contains("hasOwnProperty")
        || text.contains("hasOwn")
        || text.contains("Object.create(null)")
        || text.contains("new Map")
        || text.contains("new Set")
}
pub fn detect_prototype_pollution_in_content(file_path: &str, content: &str) -> Vec<SlopFinding> {
    let mut findings = Vec::new();
    if !is_js_ts(file_path) {
        return findings;
    }
    let direct_proto_re = Regex::new(r#"\b[a-zA-Z0-9_$]+(?:\.|\s*\[\s*["']?)__proto__["']?\s*\]?(?:\.[a-zA-Z0-9_$]+|\[[^\]]+\])*\s*=(?:[^=]|$)"#).unwrap();
    let object_proto_re = Regex::new(r#"\bObject\.prototype(?:\s*\[[^\]]+\]|\.[a-zA-Z0-9_$]+)+\s*=(?:[^=]|$)"#).unwrap();
    let path_split_re = Regex::new(r#"\.split\s*\(\s*['"][./\\]+['"]\s*\)"#).unwrap();
    let key_iter_re = Regex::new(r#"(?:Object\.(?:keys|entries|values|getOwnPropertyNames)\s*\(|for\s*\(\s*(?:const|let|var)?\s*[a-zA-Z0-9_$]+\s+in\b)"#).unwrap();
    let indexed_assign_re = Regex::new(r#"\b[a-zA-Z0-9_$]+\s*\[\s*[a-zA-Z0-9_$]+(?:\.[a-zA-Z0-9_$]+)*\s*\]\s*=(?:[^=]|$)"#).unwrap();
    let recursive_call_re = Regex::new(r#"\b(?:deepMerge|merge|clone|extend|patch|copy)\s*\("#).unwrap();
    let step_assign_re = Regex::new(r#"\b([a-zA-Z0-9_$]+)\s*=\s*([a-zA-Z0-9_$]+)\s*\["#).unwrap();
    let lines: Vec<&str> = content.lines().collect();
    for (i, line) in lines.iter().enumerate() {
        if direct_proto_re.is_match(line) {
            findings.push(SlopFinding {
                file: file_path.to_string(),
                line: i + 1,
                column: 1,
                rule_id: "no-prototype-pollution".to_string(),
                category: SlopCategory::Security,
                severity: SlopSeverity::Error,
                message: "Direct mutation of __proto__ creates global prototype pollution vulnerability".to_string(),
                snippet: line.trim().to_string(),
                fixable: false,
                suggestion: Some("Never mutate __proto__ directly; use Object.create() or structuredClone()".to_string()),
            });
        } else if object_proto_re.is_match(line) {
            findings.push(SlopFinding {
                file: file_path.to_string(),
                line: i + 1,
                column: 1,
                rule_id: "no-prototype-pollution".to_string(),
                category: SlopCategory::Security,
                severity: SlopSeverity::Error,
                message: "Direct mutation of Object.prototype pollutes base object prototype globally".to_string(),
                snippet: line.trim().to_string(),
                fixable: false,
                suggestion: Some("Avoid modifying Object.prototype; use local instance methods or Maps".to_string()),
            });
        }
    }
    let mut i = 0;
    while i < lines.len() {
        let line = lines[i];
        let trimmed = line.trim_start();
        let is_block_start = line.contains("function ") || line.contains("function(") || line.contains("=>") || line.contains(" = function") || trimmed.starts_with("for ") || trimmed.starts_with("for(");
        if is_block_start {
            let start_line = i + 1;
            let mut depth: i32 = 0;
            let mut block_lines = Vec::new();
            let mut j = i;
            while j < lines.len() {
                block_lines.push(lines[j]);
                for ch in lines[j].chars() {
                    if ch == '{' { depth += 1; }
                    else if ch == '}' { depth -= 1; }
                }
                if depth <= 0 && j > i {
                    break;
                }
                j += 1;
            }
            let block_text = block_lines.join("\n");
            if !has_prototype_guard(&block_text) {
                let has_key_iter = key_iter_re.is_match(&block_text);
                let has_path_split = path_split_re.is_match(&block_text);
                let has_indexed_assign = indexed_assign_re.is_match(&block_text);
                let has_traversal = step_assign_re.captures_iter(&block_text).any(|c| c.get(1).map(|m| m.as_str()) == c.get(2).map(|m| m.as_str()));
                let has_deep_step = recursive_call_re.is_match(&block_text) || has_traversal || (block_text.contains("typeof") && block_text.contains("object"));
                if (has_key_iter && has_indexed_assign && has_deep_step) || (has_path_split && (has_indexed_assign || has_deep_step)) {
                    let msg = if has_path_split {
                        "Unguarded path-traversal setter allows prototype pollution via __proto__ or constructor"
                    } else {
                        "Unguarded recursive merge allows prototype pollution via __proto__ or constructor keys"
                    };
                    findings.push(SlopFinding {
                        file: file_path.to_string(),
                        line: start_line,
                        column: 1,
                        rule_id: "no-prototype-pollution".to_string(),
                        category: SlopCategory::Security,
                        severity: SlopSeverity::Error,
                        message: msg.to_string(),
                        snippet: line.trim().to_string(),
                        fixable: false,
                        suggestion: Some("Sanitize keys by ignoring '__proto__', 'constructor', and 'prototype', or use Object.create(null)".to_string()),
                    });
                }
            }
            i = j + 1;
        } else {
            i += 1;
        }
    }
    findings
}
pub fn detect_prototype_pollution_in_diff(diff_files: &[DiffFile]) -> Vec<SlopFinding> {
    let mut findings = Vec::new();
    let direct_proto_re = Regex::new(r#"\b[a-zA-Z0-9_$]+(?:\.|\s*\[\s*["']?)__proto__["']?\s*\]?(?:\.[a-zA-Z0-9_$]+|\[[^\]]+\])*\s*=(?:[^=]|$)"#).unwrap();
    let object_proto_re = Regex::new(r#"\bObject\.prototype(?:\s*\[[^\]]+\]|\.[a-zA-Z0-9_$]+)+\s*=(?:[^=]|$)"#).unwrap();
    for f in diff_files {
        let path = if !f.new_path.is_empty() { &f.new_path } else { &f.old_path };
        if !is_js_ts(path) {
            continue;
        }
        for c in &f.chunks {
            let mut added_lines = Vec::new();
            for l in &c.lines {
                if l.line_type == DiffLineType::Add {
                    added_lines.push(l);
                    if direct_proto_re.is_match(&l.content) {
                        findings.push(SlopFinding {
                            file: path.clone(),
                            line: l.new_line_number.unwrap_or(1),
                            column: 1,
                            rule_id: "no-prototype-pollution".to_string(),
                            category: SlopCategory::Security,
                            severity: SlopSeverity::Error,
                            message: "Direct mutation of __proto__ creates global prototype pollution vulnerability".to_string(),
                            snippet: l.content.trim().to_string(),
                            fixable: false,
                            suggestion: Some("Never mutate __proto__ directly; use Object.create() or structuredClone()".to_string()),
                        });
                    } else if object_proto_re.is_match(&l.content) {
                        findings.push(SlopFinding {
                            file: path.clone(),
                            line: l.new_line_number.unwrap_or(1),
                            column: 1,
                            rule_id: "no-prototype-pollution".to_string(),
                            category: SlopCategory::Security,
                            severity: SlopSeverity::Error,
                            message: "Direct mutation of Object.prototype pollutes base object prototype globally".to_string(),
                            snippet: l.content.trim().to_string(),
                            fixable: false,
                            suggestion: Some("Avoid modifying Object.prototype; use local instance methods or Maps".to_string()),
                        });
                    }
                }
            }
            let chunk_content = added_lines.iter().map(|l| l.content.as_str()).collect::<Vec<_>>().join("\n");
            if !chunk_content.is_empty() {
                let chunk_findings = detect_prototype_pollution_in_content(path, &chunk_content);
                for mut cf in chunk_findings {
                    if !cf.message.starts_with("Direct mutation") {
                        let actual_line = added_lines.get(cf.line.saturating_sub(1)).and_then(|l| l.new_line_number).unwrap_or(cf.line);
                        cf.line = actual_line;
                        findings.push(cf);
                    }
                }
            }
        }
    }
    findings
}