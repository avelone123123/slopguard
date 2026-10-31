use crate::types::{DiffFile, DiffLineType, SlopCategory, SlopFinding, SlopSeverity};
use regex::Regex;
fn is_js_ts(p: &str) -> bool {
    let l = p.to_lowercase();
    l.ends_with(".ts") || l.ends_with(".tsx") || l.ends_with(".js") || l.ends_with(".jsx") || l.ends_with(".mjs") || l.ends_with(".cjs")
}
fn is_py(p: &str) -> bool {
    p.to_lowercase().ends_with(".py")
}
fn is_go(p: &str) -> bool {
    p.to_lowercase().ends_with(".go")
}
fn is_rs(p: &str) -> bool {
    p.to_lowercase().ends_with(".rs")
}
fn is_java(p: &str) -> bool {
    p.to_lowercase().ends_with(".java") || p.to_lowercase().ends_with(".kt")
}
fn is_php(p: &str) -> bool {
    p.to_lowercase().ends_with(".php")
}
fn is_cs(p: &str) -> bool {
    p.to_lowercase().ends_with(".cs")
}
fn get_cors_patterns(file_path: &str) -> Vec<(Regex, &'static str)> {
    let mut patterns = Vec::new();
    let specific = is_js_ts(file_path) || is_py(file_path) || is_go(file_path) || is_rs(file_path) || is_java(file_path) || is_php(file_path) || is_cs(file_path);
    if !specific || is_js_ts(file_path) {
        patterns.push((Regex::new(r#"(?i)\b(?:app|router|server)\.use\s*\(\s*cors\s*\(\s*(?:\{\s*\})?\s*\)\s*\)"#).unwrap(), "Express/Connect bare cors() middleware without origin restriction"));
        patterns.push((Regex::new(r#"(?i)['"]?Access-Control-Allow-Origin['"]?\s*[:=,]\s*['"]\*['"]"#).unwrap(), "Wildcard Access-Control-Allow-Origin header allows unauthorized cross-origin requests"));
        patterns.push((Regex::new(r#"(?i)\borigin\s*:\s*['"]\*['"]"#).unwrap(), "CORS origin option configured with wildcard asterisk"));
        patterns.push((Regex::new(r#"(?i)\borigin\s*:\s*true\b"#).unwrap(), "CORS origin option set to true reflects arbitrary request origins"));
    }
    if !specific || is_py(file_path) {
        patterns.push((Regex::new(r#"(?i)\ballow_origins\s*=\s*(?:\[\s*['"]\*['"]\s*\]|['"]\*['"])"#).unwrap(), "FastAPI/Starlette CORSMiddleware configured with wildcard allow_origins"));
        patterns.push((Regex::new(r#"(?i)\bCORS\s*\(\s*(?:app|api|server)?\s*\)"#).unwrap(), "Flask-CORS initialized with bare wildcard default"));
        patterns.push((Regex::new(r#"(?i)\borigins\s*=\s*(?:\[\s*['"]\*['"]\s*\]|['"]\*['"])"#).unwrap(), "Python CORS helper configured with wildcard origins parameter"));
        patterns.push((Regex::new(r#"(?i)\b(?:CORS_ALLOW_ALL_ORIGINS|CORS_ORIGIN_ALLOW_ALL)\s*=\s*True\b"#).unwrap(), "Django corsheaders configured with CORS_ALLOW_ALL_ORIGINS enabled"));
    }
    if !specific || is_go(file_path) {
        patterns.push((Regex::new(r#"(?i)\bcors\.(?:Default|AllowAll)\s*\(\s*\)"#).unwrap(), "Go cors middleware configured with permissive Default/AllowAll origin policy"));
        patterns.push((Regex::new(r#"\bAllowAllOrigins\s*:\s*true\b"#).unwrap(), "Go cors.Config configured with AllowAllOrigins enabled"));
        patterns.push((Regex::new(r#"\bAllowedOrigins\s*:\s*\[\]string\{['"]\*['"]\}"#).unwrap(), "Go cors.Config AllowedOrigins contains wildcard origin"));
    }
    if !specific || is_rs(file_path) {
        patterns.push((Regex::new(r#"\b(?:Cors|CorsLayer)::permissive\s*\(\s*\)"#).unwrap(), "Rust Actix/Tower CORS layer initialized in permissive mode"));
        patterns.push((Regex::new(r#"\.allow_origin\s*\(\s*(?:tower_http::cors::)?Any\s*\)"#).unwrap(), "Rust CORS layer configured with Any wildcard origin"));
    }
    if !specific || is_java(file_path) {
        patterns.push((Regex::new(r#"@CrossOrigin\s*\(\s*(?:origins\s*=\s*)?["']\*["']\s*\)"#).unwrap(), "Spring @CrossOrigin configured with wildcard origin"));
        patterns.push((Regex::new(r#"\.allowedOrigins\s*\(\s*["']\*["']\s*\)"#).unwrap(), "Spring CorsRegistry configured with wildcard allowedOrigins"));
        patterns.push((Regex::new(r#"@CrossOrigin\s*(?:\(\s*\))?\s*$"#).unwrap(), "Spring @CrossOrigin annotation without explicit origin allowlist"));
    }
    if !specific || is_php(file_path) {
        patterns.push((Regex::new(r#"(?i)\bheader\s*\(\s*['"]Access-Control-Allow-Origin:\s*\*['"]\s*\)"#).unwrap(), "PHP header configured with wildcard Access-Control-Allow-Origin"));
    }
    if !specific || is_cs(file_path) {
        patterns.push((Regex::new(r#"\.AllowAnyOrigin\s*\(\s*\)"#).unwrap(), ".NET CORS policy configured with AllowAnyOrigin"));
    }
    patterns
}
pub fn detect_insecure_cors_in_content(file_path: &str, content: &str) -> Vec<SlopFinding> {
    let mut findings = Vec::new();
    let patterns = get_cors_patterns(file_path);
    for (idx, line) in content.lines().enumerate() {
        let line_num = idx + 1;
        let trimmed = line.trim();
        for (re, desc) in &patterns {
            if re.is_match(trimmed) {
                findings.push(SlopFinding {
                    file: file_path.to_string(),
                    line: line_num,
                    column: 1,
                    rule_id: "no-insecure-cors".to_string(),
                    category: SlopCategory::Security,
                    severity: SlopSeverity::Error,
                    message: format!("Insecure CORS policy detected: {desc}"),
                    snippet: trimmed.to_string(),
                    fixable: false,
                    suggestion: Some("Specify explicit trusted origins instead of wildcard or permissive policies. Load allowed domains from secure environment configuration.".to_string()),
                });
                break;
            }
        }
    }
    findings
}
pub fn detect_insecure_cors_in_diff(diff_files: &[DiffFile]) -> Vec<SlopFinding> {
    let mut findings = Vec::new();
    for file in diff_files {
        let target_path = if !file.new_path.is_empty() { &file.new_path } else { &file.old_path };
        let patterns = get_cors_patterns(target_path);
        for chunk in &file.chunks {
            for line in &chunk.lines {
                if line.line_type != DiffLineType::Add {
                    continue;
                }
                let trimmed = line.content.trim();
                let line_num = line.new_line_number.unwrap_or(1);
                for (re, desc) in &patterns {
                    if re.is_match(trimmed) {
                        findings.push(SlopFinding {
                            file: target_path.to_string(),
                            line: line_num,
                            column: 1,
                            rule_id: "no-insecure-cors".to_string(),
                            category: SlopCategory::Security,
                            severity: SlopSeverity::Error,
                            message: format!("Insecure CORS policy detected: {desc}"),
                            snippet: trimmed.to_string(),
                            fixable: false,
                            suggestion: Some("Specify explicit trusted origins instead of wildcard or permissive policies. Load allowed domains from secure environment configuration.".to_string()),
                        });
                        break;
                    }
                }
            }
        }
    }
    findings
}