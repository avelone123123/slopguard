use crate::types::{DiffFile, DiffLineType, SlopCategory, SlopFinding, SlopSeverity};
use regex::Regex;
fn is_js_ts(p: &str) -> bool {
    let l = p.to_lowercase();
    l.ends_with(".ts") || l.ends_with(".tsx") || l.ends_with(".js") || l.ends_with(".jsx") || l.ends_with(".mjs") || l.ends_with(".cjs")
}
fn is_python(p: &str) -> bool {
    p.to_lowercase().ends_with(".py")
}
fn is_rust(p: &str) -> bool {
    p.to_lowercase().ends_with(".rs")
}
fn is_go(p: &str) -> bool {
    p.to_lowercase().ends_with(".go")
}
fn is_php(p: &str) -> bool {
    p.to_lowercase().ends_with(".php")
}
fn has_path_guard(text: &str) -> bool {
    text.contains("basename")
        || text.contains("secure_filename")
        || text.contains("canonicalize")
        || text.contains("startsWith")
        || text.contains("starts_with")
        || text.contains("HasPrefix")
        || text.contains("is_relative_to")
        || text.contains("realpath")
        || text.contains("filepath.Base")
}
fn get_path_traversal_patterns(path: &str) -> Vec<(Regex, &'static str)> {
    let mut patterns = Vec::new();
    let specific = is_js_ts(path) || is_python(path) || is_rust(path) || is_go(path) || is_php(path);
    if !specific || is_js_ts(path) {
        patterns.push((Regex::new(r#"(?i)\b(?:fs\.(?:promises\.)?(?:readFile|readFileSync|createReadStream|createWriteStream|writeFile|writeFileSync|open|openSync|unlink|rm)|res\.(?:sendFile|download))\s*\([^)]*\breq\.(?:params|query|body)\b"#).unwrap(), "Node.js file operation with unsanitized request parameter"));
        patterns.push((Regex::new(r#"(?i)\bpath\.(?:join|resolve)\s*\([^)]*\breq\.(?:params|query|body)\b"#).unwrap(), "Node.js path.join/resolve with unsanitized request parameter"));
        patterns.push((Regex::new(r#"(?i)\b(?:fs\.(?:promises\.)?(?:readFile|readFileSync|createReadStream|createWriteStream|writeFile|writeFileSync)|res\.(?:sendFile|download))\s*\(\s*`[^`]*\$\{[^}]+\}`"#).unwrap(), "Node.js file operation with template literal path interpolation"));
        patterns.push((Regex::new(r#"(?i)\b(?:fs\.(?:promises\.)?(?:readFile|readFileSync|createReadStream|createWriteStream|writeFile|writeFileSync)|res\.(?:sendFile|download))\s*\(\s*(?:(?:"[^"]*"|'[^']*')\s*\+\s*[a-zA-Z_$]|[a-zA-Z_$][a-zA-Z0-9_$\.]*\s*\+\s*(?:"[^"]*"|'[^']*'))"#).unwrap(), "Node.js file operation with concatenated path string"));
        patterns.push((Regex::new(r#"(?i)\b(?:fs\.(?:promises\.)?(?:readFile|readFileSync|createReadStream|createWriteStream|writeFile|writeFileSync)|res\.(?:sendFile|download))\s*\(\s*path\.(?:join|resolve)\s*\([^)]*,\s*[a-zA-Z_$][a-zA-Z0-9_$]*\s*\)"#).unwrap(), "Node.js file operation with dynamic path.join variable without boundary check"));
    }
    if !specific || is_python(path) {
        patterns.push((Regex::new(r#"(?i)\bos\.path\.join\s*\([^)]*\b(?:request\.(?:args|form|values|GET|POST)|req\.params)\b"#).unwrap(), "Python os.path.join with unsanitized request parameter"));
        patterns.push((Regex::new(r#"(?i)\b(?:open|send_file|send_from_directory)\s*\([^)]*\b(?:request\.(?:args|form|values|GET|POST)|req\.params)\b"#).unwrap(), "Python file open or send_file with unsanitized request parameter"));
        patterns.push((Regex::new(r#"(?i)\b(?:open|send_file|send_from_directory)\s*\(\s*os\.path\.join\s*\([^)]*,\s*[a-zA-Z_][a-zA-Z0-9_]*\s*\)"#).unwrap(), "Python file open with dynamic os.path.join without path validation"));
        patterns.push((Regex::new(r#"(?i)\b(?:open|send_file|send_from_directory)\s*\(\s*f["'][^"']*\{[^}]+\}"#).unwrap(), "Python file open with f-string path interpolation"));
        patterns.push((Regex::new(r#"(?i)\b(?:open|send_file|send_from_directory)\s*\(\s*(?:(?:"[^"]*"|'[^']*')\s*\+\s*[a-zA-Z_]|[a-zA-Z_][a-zA-Z0-9_\.]*\s*\+\s*(?:"[^"]*"|'[^']*')|(?:"[^"]*"|'[^']*')\s*%\s*(?:\(|[a-zA-Z_]))"#).unwrap(), "Python file open with concatenated or formatted path string"));
    }
    if !specific || is_rust(path) {
        patterns.push((Regex::new(r#"(?i)\b(?:(?:std::)?fs::(?:read|read_to_string|write|read_link|remove_file)|File::(?:open|create)|OpenOptions::new)\s*\(\s*format!\s*\("#).unwrap(), "Rust filesystem access with dynamic format! macro path interpolation"));
        patterns.push((Regex::new(r#"(?i)\b(?:(?:std::)?fs::(?:read|read_to_string|write)|File::(?:open|create))\s*\(\s*(?:&?(?:Path::new|PathBuf::from)\s*\([^)]*\)\.join\s*\([^)]*\)|&?[a-zA-Z0-9_]+\.join\s*\([^)]*\))\s*\)"#).unwrap(), "Rust filesystem access using uncanonicalized path join"));
    }
    if !specific || is_go(path) {
        patterns.push((Regex::new(r#"(?i)\b(?:os\.(?:Open|OpenFile|ReadFile|WriteFile|Remove)|ioutil\.ReadFile|http\.ServeFile)\s*\([^)]*\br\.(?:URL\.Query\(\)\.Get|FormValue|PostFormValue)\b"#).unwrap(), "Go file operation with unsanitized HTTP query or form parameter"));
        patterns.push((Regex::new(r#"(?i)\bhttp\.ServeFile\s*\([^)]*,\s*filepath\.Join\s*\([^)]*\)"#).unwrap(), "Go http.ServeFile with dynamic filepath.Join without Clean prefix guard"));
        patterns.push((Regex::new(r#"(?i)\b(?:os\.(?:Open|OpenFile|ReadFile|WriteFile)|ioutil\.ReadFile)\s*\(\s*filepath\.Join\s*\([^)]*,\s*[a-zA-Z0-9_]+\s*\)"#).unwrap(), "Go file operation with dynamic filepath.Join without validation"));
        patterns.push((Regex::new(r#"(?i)\b(?:os\.(?:Open|OpenFile|ReadFile|WriteFile)|ioutil\.ReadFile|http\.ServeFile)\s*\(\s*(?:fmt\.Sprintf\s*\(|.*\+\s*[a-zA-Z_]|[a-zA-Z_].*\+)"#).unwrap(), "Go file operation with formatted or concatenated path string"));
    }
    if !specific || is_php(path) {
        patterns.push((Regex::new(r#"(?i)\b(?:file_get_contents|readfile|file_put_contents|fopen|include|require|include_once|require_once)\s*\([^)]*\$_(?:GET|POST|REQUEST|COOKIE)\["#).unwrap(), "PHP filesystem or inclusion operation with unvalidated superglobal parameter"));
        patterns.push((Regex::new(r#"(?i)\b(?:file_get_contents|readfile|file_put_contents|fopen|include|require|include_once|require_once)\s*\(\s*(?:"[^"]*\$[a-zA-Z_]|'[^']*\.\s*\$[a-zA-Z_]|"[^"]*"\s*\.\s*\$[a-zA-Z_])"#).unwrap(), "PHP filesystem operation with interpolated or concatenated variable path"));
    }
    patterns
}
pub fn detect_path_traversal_in_content(file_path: &str, content: &str) -> Vec<SlopFinding> {
    let mut findings = Vec::new();
    let patterns = get_path_traversal_patterns(file_path);
    let lines: Vec<&str> = content.lines().collect();
    for (i, line) in lines.iter().enumerate() {
        if has_path_guard(line) {
            continue;
        }
        let start = i.saturating_sub(4);
        let window = lines[start..=i].join(" ");
        if has_path_guard(&window) {
            continue;
        }
        for (re, desc) in &patterns {
            if re.is_match(line) {
                findings.push(SlopFinding {
                    file: file_path.to_string(),
                    line: i + 1,
                    column: 1,
                    rule_id: "no-path-traversal-risk".to_string(),
                    category: SlopCategory::Security,
                    severity: SlopSeverity::Error,
                    message: format!("Path traversal vulnerability: {desc}"),
                    snippet: line.trim().to_string(),
                    fixable: false,
                    suggestion: Some("Sanitize path using path.basename, secure_filename, or verify canonical prefix with starts_with".to_string()),
                });
                break;
            }
        }
    }
    findings
}
pub fn detect_path_traversal_in_diff(diff_files: &[DiffFile]) -> Vec<SlopFinding> {
    let mut findings = Vec::new();
    for f in diff_files {
        let path = if !f.new_path.is_empty() { &f.new_path } else { &f.old_path };
        let patterns = get_path_traversal_patterns(path);
        for c in &f.chunks {
            let chunk_text = c.lines.iter().map(|l| l.content.as_str()).collect::<Vec<_>>().join(" ");
            let chunk_guarded = has_path_guard(&chunk_text);
            for l in &c.lines {
                if l.line_type == DiffLineType::Add {
                    if has_path_guard(&l.content) || chunk_guarded {
                        continue;
                    }
                    for (re, desc) in &patterns {
                        if re.is_match(&l.content) {
                            findings.push(SlopFinding {
                                file: path.clone(),
                                line: l.new_line_number.unwrap_or(1),
                                column: 1,
                                rule_id: "no-path-traversal-risk".to_string(),
                                category: SlopCategory::Security,
                                severity: SlopSeverity::Error,
                                message: format!("Path traversal vulnerability: {desc}"),
                                snippet: l.content.trim().to_string(),
                                fixable: false,
                                suggestion: Some("Sanitize path using path.basename, secure_filename, or verify canonical prefix with starts_with".to_string()),
                            });
                            break;
                        }
                    }
                }
            }
        }
    }
    findings
}
fn get_toctou_patterns() -> Vec<(Regex, &'static str)> {
    vec![
        (Regex::new(r#"(?i)fs\.existsSync\s*\([^)]*\)\s*(?:\)?\s*(?:&&|;|\n|\{))[^}]*fs\.(?:writeFileSync|writeFile|appendFileSync|createWriteStream)"#).unwrap(), "Node.js check-then-write TOCTOU race condition (use 'wx' atomic exclusive flag)"),
        (Regex::new(r#"(?i)if\s+os\.path\.exists\s*\([^)]*\)\s*:[^:]*(?:open\s*\([^)]*['\"][wa][+]?['\"]|\.write\s*\()"#).unwrap(), "Python check-then-write TOCTOU race condition (use mode='x' atomic exclusive creation)"),
        (Regex::new(r#"(?i)if\s+[a-zA-Z0-9_.]+\.exists\(\)\s*:[^:]*open\s*\([^)]*['\"][wa][+]?['\"]"#).unwrap(), "Python Path.exists check-then-write TOCTOU race condition"),
        (Regex::new(r#"(?i)if\s+_,?\s*err\s*:=\s*os\.Stat\s*\([^)]*\);\s*os\.IsNotExist\(err\)\s*\{[^}]*os\.(?:Create|OpenFile\([^)]*O_CREATE)"#).unwrap(), "Go os.Stat check-then-create TOCTOU race condition (use os.O_EXCL flag)"),
        (Regex::new(r#"(?i)if\s+!?[a-zA-Z0-9_.]+\.exists\(\)\s*\{[^}]*(?:File::create|OpenOptions::new\(\)\.write)"#).unwrap(), "Rust path.exists() check-then-write TOCTOU race condition (use OpenOptions::new().create_new(true))"),
    ]
}
pub fn detect_speculative_file_race_toctou_in_content(file_path: &str, content: &str) -> Vec<SlopFinding> {
    let mut findings = Vec::new();
    let patterns = get_toctou_patterns();
    let lines: Vec<&str> = content.lines().collect();
    for (i, line) in lines.iter().enumerate() {
        let trimmed = line.trim();
        if trimmed.starts_with("//") || trimmed.starts_with('#') || trimmed.starts_with('*') || trimmed.contains("Regex::new") {
            continue;
        }
        if !trimmed.contains("existsSync") && !trimmed.contains("exists") && !trimmed.contains("Stat") && !trimmed.contains("IsNotExist") {
            continue;
        }
        for (re, desc) in &patterns {
            if re.is_match(trimmed) {
                findings.push(SlopFinding {
                    file: file_path.to_string(),
                    line: i + 1,
                    column: 1,
                    rule_id: "no-speculative-file-race-toctou".to_string(),
                    category: SlopCategory::FileRaceToctou,
                    severity: SlopSeverity::Error,
                    message: format!("Speculative file race condition and TOCTOU vulnerability: {desc}"),
                    snippet: trimmed.to_string(),
                    fixable: false,
                    suggestion: Some("Use atomic exclusive creation flags (O_CREAT | O_EXCL, mode='x', 'wx' flag)".to_string()),
                });
                break;
            }
        }
    }
    if findings.is_empty() {
        for (i, _) in lines.iter().enumerate() {
            let start = i.saturating_sub(4);
            let window = lines[start..=i].join("\n");
            for (re, desc) in &patterns {
                if re.is_match(&window) {
                    findings.push(SlopFinding {
                        file: file_path.to_string(),
                        line: start + 1,
                        column: 1,
                        rule_id: "no-speculative-file-race-toctou".to_string(),
                        category: SlopCategory::FileRaceToctou,
                        severity: SlopSeverity::Error,
                        message: format!("Speculative file race condition and TOCTOU vulnerability: {desc}"),
                        snippet: lines[start].trim().to_string(),
                        fixable: false,
                        suggestion: Some("Use atomic exclusive creation flags (O_CREAT | O_EXCL, mode='x', 'wx' flag)".to_string()),
                    });
                    return findings;
                }
            }
        }
    }
    findings
}
pub fn detect_speculative_file_race_toctou_in_diff(diff_files: &[DiffFile]) -> Vec<SlopFinding> {
    let mut findings = Vec::new();
    let patterns = get_toctou_patterns();
    for df in diff_files {
        let path = if !df.new_path.is_empty() { &df.new_path } else { &df.old_path };
        for chunk in &df.chunks {
            let mut chunk_matched = false;
            for dl in &chunk.lines {
                if dl.line_type != DiffLineType::Add {
                    continue;
                }
                let trimmed = dl.content.trim();
                if trimmed.starts_with("//") || trimmed.starts_with('#') || trimmed.starts_with('*') || trimmed.contains("Regex::new") {
                    continue;
                }
                if !trimmed.contains("existsSync") && !trimmed.contains("exists") && !trimmed.contains("Stat") && !trimmed.contains("IsNotExist") {
                    continue;
                }
                for (pat, desc) in &patterns {
                    if pat.is_match(trimmed) {
                        findings.push(SlopFinding {
                            file: path.to_string(),
                            line: dl.new_line_number.unwrap_or(chunk.new_start),
                            column: 1,
                            rule_id: "no-speculative-file-race-toctou".to_string(),
                            category: SlopCategory::FileRaceToctou,
                            severity: SlopSeverity::Error,
                            message: format!("Speculative file race condition and TOCTOU vulnerability: {desc}"),
                            snippet: trimmed.to_string(),
                            fixable: false,
                            suggestion: Some("Use atomic exclusive creation flags (O_CREAT | O_EXCL, mode='x', 'wx' flag)".to_string()),
                        });
                        chunk_matched = true;
                        break;
                    }
                }
            }
            if !chunk_matched {
                let chunk_text = chunk.lines.iter().filter(|l| l.line_type == DiffLineType::Add).map(|l| l.content.as_str()).collect::<Vec<_>>().join("\n");
                for (pat, desc) in &patterns {
                    if pat.is_match(&chunk_text) {
                        findings.push(SlopFinding {
                            file: path.to_string(),
                            line: chunk.new_start,
                            column: 1,
                            rule_id: "no-speculative-file-race-toctou".to_string(),
                            category: SlopCategory::FileRaceToctou,
                            severity: SlopSeverity::Error,
                            message: format!("Speculative file race condition and TOCTOU vulnerability: {desc}"),
                            snippet: chunk_text.lines().next().unwrap_or("").to_string(),
                            fixable: false,
                            suggestion: Some("Use atomic exclusive creation flags (O_CREAT | O_EXCL, mode='x', 'wx' flag)".to_string()),
                        });
                        break;
                    }
                }
            }
        }
    }
    findings
}