use crate::types::{DiffFile, DiffLineType, SlopCategory, SlopFinding, SlopSeverity};
use regex::Regex;
struct SpeculativeRule {
    pattern: Regex,
    language: &'static str,
    method_name: &'static str,
    suggestion: &'static str,
}
fn get_speculative_rules() -> Vec<SpeculativeRule> {
    vec![
        SpeculativeRule {
            pattern: Regex::new(r#"\bjson\.loadf\s*\("#).unwrap(),
            language: "python",
            method_name: "json.loadf()",
            suggestion: "json.loads(text) or json.load(file_obj)",
        },
        SpeculativeRule {
            pattern: Regex::new(r#"\bos\.path\.joinpath\s*\("#).unwrap(),
            language: "python",
            method_name: "os.path.joinpath()",
            suggestion: "os.path.join(a, b) or pathlib.Path(a).joinpath(b)",
        },
        SpeculativeRule {
            pattern: Regex::new(r#"\bhashlib\.[a-zA-Z0-9_]+\([^)]*\)\.hex\s*\("#).unwrap(),
            language: "python",
            method_name: "hashlib...hex()",
            suggestion: "hashlib.sha256(data).hexdigest()",
        },
        SpeculativeRule {
            pattern: Regex::new(r#"\basyncio\.sleep_until\s*\("#).unwrap(),
            language: "python",
            method_name: "asyncio.sleep_until()",
            suggestion: "asyncio.sleep(delay) calculating target_time - loop.time()",
        },
        SpeculativeRule {
            pattern: Regex::new(r#"\bdatetime\.utcnow\(\)\.timestamp_ms\s*\("#).unwrap(),
            language: "python",
            method_name: "datetime.utcnow().timestamp_ms()",
            suggestion: "int(datetime.datetime.now(datetime.timezone.utc).timestamp() * 1000)",
        },
        SpeculativeRule {
            pattern: Regex::new(r#"\bfs\.writeJsonSync\s*\("#).unwrap(),
            language: "javascript",
            method_name: "fs.writeJsonSync()",
            suggestion: "fs.writeFileSync(path, JSON.stringify(data, null, 2))",
        },
        SpeculativeRule {
            pattern: Regex::new(r#"\bcrypto\.randomUUIDv4\s*\("#).unwrap(),
            language: "javascript",
            method_name: "crypto.randomUUIDv4()",
            suggestion: "crypto.randomUUID()",
        },
        SpeculativeRule {
            pattern: Regex::new(r#"\bBuffer\.fromHex\s*\("#).unwrap(),
            language: "javascript",
            method_name: "Buffer.fromHex()",
            suggestion: "Buffer.from(hexStr, 'hex')",
        },
        SpeculativeRule {
            pattern: Regex::new(r#"\bArray\.prototype\.unique\b"#).unwrap(),
            language: "javascript",
            method_name: "Array.prototype.unique",
            suggestion: "[...new Set(array)]",
        },
        SpeculativeRule {
            pattern: Regex::new(r#"(?:\bOption::expect_or_default|\.expect_or_default)\s*\("#).unwrap(),
            language: "rust",
            method_name: "Option::expect_or_default()",
            suggestion: "opt.unwrap_or_default() or opt.unwrap_or_else(|| ...)",
        },
        SpeculativeRule {
            pattern: Regex::new(r#"(?:\bVec::push_all|\.push_all)\s*\("#).unwrap(),
            language: "rust",
            method_name: "Vec::push_all()",
            suggestion: "vec.extend_from_slice(slice) or vec.extend(iter)",
        },
        SpeculativeRule {
            pattern: Regex::new(r#"\bstd::fs::read_string\s*\("#).unwrap(),
            language: "rust",
            method_name: "std::fs::read_string()",
            suggestion: "std::fs::read_to_string(path)",
        },
    ]
}
fn matches_language(path: &str, lang: &str) -> bool {
    let lower = path.to_lowercase();
    match lang {
        "python" => lower.ends_with(".py"),
        "javascript" => lower.ends_with(".js") || lower.ends_with(".ts") || lower.ends_with(".jsx") || lower.ends_with(".tsx") || lower.ends_with(".mjs") || lower.ends_with(".cjs"),
        "rust" => lower.ends_with(".rs"),
        _ => true,
    }
}
pub fn detect_speculative_apis_in_content(file_path: &str, content: &str) -> Vec<SlopFinding> {
    let mut findings = Vec::new();
    let rules = get_speculative_rules();
    for (idx, line) in content.lines().enumerate() {
        let trimmed = line.trim();
        let line_num = idx + 1;
        if trimmed.starts_with("method_name:") || trimmed.starts_with("pattern:") || trimmed.contains("Regex::new") {
            continue;
        }
        for rule in &rules {
            if matches_language(file_path, rule.language) && rule.pattern.is_match(trimmed) {
                findings.push(SlopFinding {
                    file: file_path.to_string(),
                    line: line_num,
                    column: 1,
                    rule_id: "no-hallucinated-speculative-apis".to_string(),
                    category: SlopCategory::SpeculativeApi,
                    severity: SlopSeverity::Error,
                    message: format!("AI hallucinated speculative standard library API '{}' does not exist in {}", rule.method_name, rule.language),
                    snippet: trimmed.to_string(),
                    fixable: true,
                    suggestion: Some(rule.suggestion.to_string()),
                });
            }
        }
    }
    findings
}
pub fn detect_speculative_apis_in_diff(diff_files: &[DiffFile]) -> Vec<SlopFinding> {
    let mut findings = Vec::new();
    let rules = get_speculative_rules();
    for file in diff_files {
        if file.is_deleted {
            continue;
        }
        let target_path = if !file.new_path.is_empty() { &file.new_path } else { &file.old_path };
        for chunk in &file.chunks {
            for line in &chunk.lines {
                if line.line_type != DiffLineType::Add {
                    continue;
                }
                let trimmed = line.content.trim();
                let line_num = line.new_line_number.unwrap_or(1);
                if trimmed.starts_with("method_name:") || trimmed.starts_with("pattern:") || trimmed.contains("Regex::new") {
                    continue;
                }
                for rule in &rules {
                    if matches_language(target_path, rule.language) && rule.pattern.is_match(trimmed) {
                        findings.push(SlopFinding {
                            file: target_path.to_string(),
                            line: line_num,
                            column: 1,
                            rule_id: "no-hallucinated-speculative-apis".to_string(),
                            category: SlopCategory::SpeculativeApi,
                            severity: SlopSeverity::Error,
                            message: format!("AI hallucinated speculative standard library API '{}' does not exist in {}", rule.method_name, rule.language),
                            snippet: trimmed.to_string(),
                            fixable: true,
                            suggestion: Some(rule.suggestion.to_string()),
                        });
                    }
                }
            }
        }
    }
    findings
}