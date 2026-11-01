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
fn get_command_injection_patterns(path: &str) -> Vec<(Regex, &'static str)> {
    let mut patterns = Vec::new();
    let specific = is_js_ts(path) || is_python(path) || is_rust(path) || is_go(path) || is_php(path);
    if !specific || is_js_ts(path) {
        patterns.push((Regex::new(r#"(?i)\b(?:child_process\.)?(?:exec|execSync)\s*\(\s*`[^`]*\$\{[^}]+\}"#).unwrap(), "Node.js exec/execSync template literal interpolation"));
        patterns.push((Regex::new(r#"(?i)\b(?:child_process\.)?(?:exec|execSync)\s*\(\s*(?:(?:"[^"]*"|'[^']*')\s*\+\s*[a-zA-Z_$]|[a-zA-Z_$][a-zA-Z0-9_$\.]*\s*\+\s*(?:"[^"]*"|'[^']*'))"#).unwrap(), "Node.js exec/execSync string concatenation"));
        patterns.push((Regex::new(r#"(?i)\b(?:child_process\.)?(?:spawn|spawnSync|fork|execFile|execFileSync)\s*\(.+shell\s*:\s*true"#).unwrap(), "Node.js child_process with shell: true option"));
    }
    if !specific || is_python(path) {
        patterns.push((Regex::new(r#"(?i)\bos\.(?:system|popen)\s*\(\s*f["'][^"']*\{[^}]+\}"#).unwrap(), "Python os.system/popen f-string command interpolation"));
        patterns.push((Regex::new(r#"(?i)\bos\.(?:system|popen)\s*\(\s*(?:(?:"[^"]*"|'[^']*')\s*\+\s*[a-zA-Z_]|[a-zA-Z_][a-zA-Z0-9_\.]*\s*\+\s*(?:"[^"]*"|'[^']*')|(?:"[^"]*"|'[^']*')\s*%\s*(?:\(|[a-zA-Z_]))"#).unwrap(), "Python os.system/popen string concatenation or formatting"));
        patterns.push((Regex::new(r#"(?i)\bsubprocess\.(?:run|Popen|call|check_output|check_call)\s*\(.+shell\s*=\s*True"#).unwrap(), "Python subprocess execution with shell=True"));
        patterns.push((Regex::new(r#"(?i)\bsubprocess\.(?:run|Popen|call|check_output|check_call)\s*\(\s*(?:f["'][^"']*\{[^}]+\}|(?:"[^"]*"|'[^']*')\s*\+\s*[a-zA-Z_]|[a-zA-Z_][a-zA-Z0-9_\.]*\s*\+\s*(?:"[^"]*"|'[^']*'))"#).unwrap(), "Python subprocess command interpolation without argument array"));
    }
    if !specific || is_rust(path) {
        patterns.push((Regex::new(r#"(?i)\bCommand::new\s*\(\s*"(?:sh|bash|zsh|cmd|cmd\.exe|powershell|pwsh)"\s*\)(?:\s*\.[a-zA-Z0-9_]+\s*\([^)]*\))*\s*\.arg\s*\(\s*format!\s*\("#).unwrap(), "Rust Command shell invocation with format! macro"));
        patterns.push((Regex::new(r#"(?i)\bCommand::new\s*\(\s*format!\s*\("#).unwrap(), "Rust Command binary path dynamic format! interpolation"));
    }
    if !specific || is_go(path) {
        patterns.push((Regex::new(r#"(?i)\bexec\.Command\s*\(\s*"(?:sh|bash|zsh|cmd|cmd\.exe|powershell)"\s*,\s*"(?:-c|/C|-Command)"\s*,\s*(?:fmt\.Sprintf\s*\(|.*\+\s*[a-zA-Z_]|[a-zA-Z_].*\+)"#).unwrap(), "Go exec.Command shell invocation with formatted command string"));
        patterns.push((Regex::new(r#"(?i)\bexec\.Command\s*\(\s*fmt\.Sprintf\s*\("#).unwrap(), "Go exec.Command dynamic format string"));
    }
    if !specific || is_php(path) {
        patterns.push((Regex::new(r#"(?i)\b(?:exec|shell_exec|system|passthru|popen|proc_open)\s*\(\s*(?:"[^"]*\$[a-zA-Z_]|'[^']*\.\s*\$[a-zA-Z_]|"[^"]*"\s*\.\s*\$[a-zA-Z_]|\$[a-zA-Z_])"#).unwrap(), "PHP shell command execution with variable interpolation"));
        patterns.push((Regex::new(r#"`[^`]*\$[a-zA-Z_][^`]*`"#).unwrap(), "PHP backtick shell execution with variable interpolation"));
    }
    patterns
}
pub fn detect_command_injection_in_content(file_path: &str, content: &str) -> Vec<SlopFinding> {
    let mut findings = Vec::new();
    let patterns = get_command_injection_patterns(file_path);
    let lines: Vec<&str> = content.lines().collect();
    for (i, line) in lines.iter().enumerate() {
        for (re, desc) in &patterns {
            if re.is_match(line) {
                findings.push(SlopFinding {
                    file: file_path.to_string(),
                    line: i + 1,
                    column: 1,
                    rule_id: "no-command-injection-risk".to_string(),
                    category: SlopCategory::Security,
                    severity: SlopSeverity::Error,
                    message: format!("Command injection vulnerability: {desc}"),
                    snippet: line.trim().to_string(),
                    fixable: false,
                    suggestion: Some("Pass arguments as a safe discrete vector/array instead of invoking shell interpolation".to_string()),
                });
                break;
            }
        }
        if (line.contains("shell: true") || line.contains("shell:true")) && is_js_ts(file_path) {
            let start = i.saturating_sub(4);
            let prev_window = lines[start..i].join(" ");
            if (prev_window.contains("spawn") || prev_window.contains("fork") || prev_window.contains("execFile")) && !findings.iter().any(|f| f.line == i + 1) {
                findings.push(SlopFinding {
                    file: file_path.to_string(),
                    line: i + 1,
                    column: 1,
                    rule_id: "no-command-injection-risk".to_string(),
                    category: SlopCategory::Security,
                    severity: SlopSeverity::Error,
                    message: "Command injection vulnerability: Node.js child_process with shell: true option".to_string(),
                    snippet: line.trim().to_string(),
                    fixable: false,
                    suggestion: Some("Avoid shell: true and pass arguments as an array directly to spawn or execFile".to_string()),
                });
            }
        }
        if (line.contains("shell=True") || line.contains("shell = True")) && is_python(file_path) {
            let start = i.saturating_sub(4);
            let prev_window = lines[start..i].join(" ");
            if prev_window.contains("subprocess.") && !findings.iter().any(|f| f.line == i + 1) {
                findings.push(SlopFinding {
                    file: file_path.to_string(),
                    line: i + 1,
                    column: 1,
                    rule_id: "no-command-injection-risk".to_string(),
                    category: SlopCategory::Security,
                    severity: SlopSeverity::Error,
                    message: "Command injection vulnerability: Python subprocess execution with shell=True".to_string(),
                    snippet: line.trim().to_string(),
                    fixable: false,
                    suggestion: Some("Pass command arguments as a list with shell=False".to_string()),
                });
            }
        }
    }
    findings
}
pub fn detect_command_injection_in_diff(diff_files: &[DiffFile]) -> Vec<SlopFinding> {
    let mut findings = Vec::new();
    for f in diff_files {
        let path = if !f.new_path.is_empty() { &f.new_path } else { &f.old_path };
        let patterns = get_command_injection_patterns(path);
        for c in &f.chunks {
            for l in &c.lines {
                if l.line_type == DiffLineType::Add {
                    for (re, desc) in &patterns {
                        if re.is_match(&l.content) {
                            findings.push(SlopFinding {
                                file: path.clone(),
                                line: l.new_line_number.unwrap_or(1),
                                column: 1,
                                rule_id: "no-command-injection-risk".to_string(),
                                category: SlopCategory::Security,
                                severity: SlopSeverity::Error,
                                message: format!("Command injection vulnerability: {desc}"),
                                snippet: l.content.trim().to_string(),
                                fixable: false,
                                suggestion: Some("Pass arguments as a safe discrete vector/array instead of invoking shell interpolation".to_string()),
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
fn get_speculative_subprocess_patterns() -> Vec<Regex> {
    vec![
        Regex::new(r#"(?i)\bsubprocess\.(?:Popen|run|call|check_call|check_output)\s*\((?:[^)]*shell\s*=\s*True[^)]*(?:f["']|format\(|\s*%)|[^)]*(?:f["']|format\(|\s*%)[^)]*shell\s*=\s*True)"#).unwrap(),
        Regex::new(r#"(?i)\bos\.(?:system|popen)\s*\(\s*(?:f["'][^"']*\{|format\(|["'][^"']*"\s*%\s*)"#).unwrap(),
        Regex::new(r#"(?i)\b(?:child_process\.)?(?:exec|execSync)\s*\(\s*`[^`]*\$\{[^}]+\}"#).unwrap(),
        Regex::new(r#"(?i)\b(?:child_process\.)?(?:spawn|spawnSync)\s*\((?:[^)]*shell\s*:\s*true[^)]*[`+]|[^)]*[`+][^)]*shell\s*:\s*true)"#).unwrap(),
        Regex::new(r#"(?i)\bexec\.Command\s*\(\s*"(?:sh|bash|zsh|cmd|powershell)"\s*,\s*"(?:-c|/C|-Command)"\s*,\s*fmt\.Sprintf"#).unwrap(),
        Regex::new(r#"(?i)\bCommand::new\s*\(\s*"(?:sh|bash|zsh|cmd|powershell)"\s*\)(?:\s*\.[a-zA-Z0-9_]+\s*\([^)]*\))*\s*\.arg\s*\(\s*format!\s*\("#).unwrap(),
    ]
}
pub fn detect_speculative_subprocess_injection_in_content(file_path: &str, content: &str) -> Vec<SlopFinding> {
    let mut findings = Vec::new();
    let patterns = get_speculative_subprocess_patterns();
    for (idx, line) in content.lines().enumerate() {
        let trimmed = line.trim();
        if trimmed.starts_with("//") || trimmed.starts_with('#') || trimmed.starts_with('*') || trimmed.contains("Regex::new") {
            continue;
        }
        for pat in &patterns {
            if pat.is_match(trimmed) {
                findings.push(SlopFinding {
                    file: file_path.to_string(),
                    line: idx + 1,
                    column: 1,
                    rule_id: "no-speculative-subprocess-injection".to_string(),
                    category: SlopCategory::SpeculativeSubprocessInjection,
                    severity: SlopSeverity::Error,
                    message: "Unchecked dynamic subprocess or shell interpolation vulnerability (CWE-78)".to_string(),
                    snippet: trimmed.to_string(),
                    fixable: false,
                    suggestion: Some("Pass command arguments as an array/vector without invoking a shell interpreter".to_string()),
                });
                break;
            }
        }
    }
    findings
}
pub fn detect_speculative_subprocess_injection_in_diff(diff_files: &[DiffFile]) -> Vec<SlopFinding> {
    let mut findings = Vec::new();
    let patterns = get_speculative_subprocess_patterns();
    for df in diff_files {
        let path = if !df.new_path.is_empty() { &df.new_path } else { &df.old_path };
        for chunk in &df.chunks {
            for dl in &chunk.lines {
                if dl.line_type != DiffLineType::Add {
                    continue;
                }
                let trimmed = dl.content.trim();
                if trimmed.starts_with("//") || trimmed.starts_with('#') || trimmed.starts_with('*') || trimmed.contains("Regex::new") {
                    continue;
                }
                for pat in &patterns {
                    if pat.is_match(trimmed) {
                        findings.push(SlopFinding {
                            file: path.to_string(),
                            line: dl.new_line_number.unwrap_or(chunk.new_start),
                            column: 1,
                            rule_id: "no-speculative-subprocess-injection".to_string(),
                            category: SlopCategory::SpeculativeSubprocessInjection,
                            severity: SlopSeverity::Error,
                            message: "Unchecked dynamic subprocess or shell interpolation vulnerability (CWE-78)".to_string(),
                            snippet: trimmed.to_string(),
                            fixable: false,
                            suggestion: Some("Pass command arguments as an array/vector without invoking a shell interpreter".to_string()),
                        });
                        break;
                    }
                }
            }
        }
    }
    findings
}