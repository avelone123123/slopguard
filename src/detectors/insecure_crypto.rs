use crate::types::{DiffFile, DiffLineType, SlopCategory, SlopFinding, SlopSeverity};
use regex::Regex;
fn is_js_ts(path: &str) -> bool {
    let lower = path.to_lowercase();
    lower.ends_with(".ts") || lower.ends_with(".tsx") || lower.ends_with(".js") || lower.ends_with(".jsx") || lower.ends_with(".mjs") || lower.ends_with(".cjs")
}
fn is_py(path: &str) -> bool {
    path.to_lowercase().ends_with(".py")
}
fn get_sensitive_identifier_regex() -> Regex {
    Regex::new(r#"(?i)\b[a-zA-Z0-9_$]*(?:token|secret|api_?key|nonce|password|passwd|auth|session|salt|reset|otp|pin|csrf)[a-zA-Z0-9_$]*\b"#).unwrap()
}
fn get_js_weak_prng_regex() -> Regex {
    Regex::new(r#"\bMath\.random\s*\("#).unwrap()
}
fn get_py_weak_prng_regex() -> Regex {
    Regex::new(r#"\brandom\.(?:random|choice|choices|randint|sample|randrange)\s*\("#).unwrap()
}
fn get_weak_hash_patterns() -> Vec<(Regex, &'static str)> {
    vec![
        (Regex::new(r#"(?i)\bcreateHash\s*\(\s*['"](?:md5|sha1)['"]\s*\)"#).unwrap(), "crypto.createHash initialized with cryptographically broken digest (MD5/SHA-1)"),
        (Regex::new(r#"(?i)\bhashlib\.(?:md5|sha1)\s*\("#).unwrap(), "hashlib invoked with broken hash algorithm (MD5/SHA-1)"),
        (Regex::new(r#"(?i)\bCryptoJS\.(?:MD5|SHA1)\s*\("#).unwrap(), "CryptoJS invoked with broken digest (MD5/SHA-1)"),
        (Regex::new(r#"\b(?:md5|sha1)\.New\s*\(\s*\)"#).unwrap(), "Go crypto digest initialized with broken hash algorithm (MD5/SHA-1)"),
        (Regex::new(r#"(?i)\b(?:md5|sha1)\s*\(\s*\$(?:password|pass|passwd|pwd|token|secret|salt|auth|pin|code|key)\b"#).unwrap(), "PHP weak hash invoked on credential or token variable"),
    ]
}
pub fn detect_insecure_crypto_in_content(file_path: &str, content: &str) -> Vec<SlopFinding> {
    let mut findings = Vec::new();
    let sensitive_re = get_sensitive_identifier_regex();
    let js_prng_re = get_js_weak_prng_regex();
    let py_prng_re = get_py_weak_prng_regex();
    let hash_patterns = get_weak_hash_patterns();
    let is_js = is_js_ts(file_path);
    let is_python = is_py(file_path);
    for (idx, line) in content.lines().enumerate() {
        let line_num = idx + 1;
        let trimmed = line.trim();
        if (is_js || (!is_python && !trimmed.contains("def "))) && js_prng_re.is_match(trimmed) && sensitive_re.is_match(trimmed) {
            findings.push(SlopFinding {
                file: file_path.to_string(),
                line: line_num,
                column: 1,
                rule_id: "no-insecure-crypto".to_string(),
                category: SlopCategory::Security,
                severity: SlopSeverity::Error,
                message: "Insecure PRNG used for security-sensitive token/credential (CWE-330): Math.random is cryptographically predictable".to_string(),
                snippet: trimmed.to_string(),
                fixable: false,
                suggestion: Some("Use crypto.randomBytes(n), crypto.randomUUID(), or crypto.getRandomValues() for cryptographic entropy".to_string()),
            });
            continue;
        }
        if (is_python || (!is_js && trimmed.contains("def "))) && py_prng_re.is_match(trimmed) && sensitive_re.is_match(trimmed) {
            findings.push(SlopFinding {
                file: file_path.to_string(),
                line: line_num,
                column: 1,
                rule_id: "no-insecure-crypto".to_string(),
                category: SlopCategory::Security,
                severity: SlopSeverity::Error,
                message: "Insecure PRNG used for security-sensitive token/credential (CWE-330): Python random module is predictable".to_string(),
                snippet: trimmed.to_string(),
                fixable: false,
                suggestion: Some("Use the standard secrets module (secrets.token_hex(), secrets.token_urlsafe(), secrets.choice())".to_string()),
            });
            continue;
        }
        for (re, desc) in &hash_patterns {
            if re.is_match(trimmed) {
                findings.push(SlopFinding {
                    file: file_path.to_string(),
                    line: line_num,
                    column: 1,
                    rule_id: "no-insecure-crypto".to_string(),
                    category: SlopCategory::Security,
                    severity: SlopSeverity::Error,
                    message: format!("Broken hash algorithm detected (CWE-328): {desc}"),
                    snippet: trimmed.to_string(),
                    fixable: false,
                    suggestion: Some("Migrate to SHA-256/SHA-512 for digests, or use Argon2id/bcrypt/scrypt for credential hashing".to_string()),
                });
                break;
            }
        }
    }
    findings
}
pub fn detect_insecure_crypto_in_diff(diff_files: &[DiffFile]) -> Vec<SlopFinding> {
    let mut findings = Vec::new();
    let sensitive_re = get_sensitive_identifier_regex();
    let js_prng_re = get_js_weak_prng_regex();
    let py_prng_re = get_py_weak_prng_regex();
    let hash_patterns = get_weak_hash_patterns();
    for file in diff_files {
        if file.is_deleted {
            continue;
        }
        let target_path = if !file.new_path.is_empty() { &file.new_path } else { &file.old_path };
        let is_js = is_js_ts(target_path);
        let is_python = is_py(target_path);
        for chunk in &file.chunks {
            for line in &chunk.lines {
                if line.line_type != DiffLineType::Add {
                    continue;
                }
                let trimmed = line.content.trim();
                let line_num = line.new_line_number.unwrap_or(1);
                if (is_js || (!is_python && !trimmed.contains("def "))) && js_prng_re.is_match(trimmed) && sensitive_re.is_match(trimmed) {
                    findings.push(SlopFinding {
                        file: target_path.to_string(),
                        line: line_num,
                        column: 1,
                        rule_id: "no-insecure-crypto".to_string(),
                        category: SlopCategory::Security,
                        severity: SlopSeverity::Error,
                        message: "Insecure PRNG used for security-sensitive token/credential (CWE-330): Math.random is cryptographically predictable".to_string(),
                        snippet: trimmed.to_string(),
                        fixable: false,
                        suggestion: Some("Use crypto.randomBytes(n), crypto.randomUUID(), or crypto.getRandomValues() for cryptographic entropy".to_string()),
                    });
                    continue;
                }
                if (is_python || (!is_js && trimmed.contains("def "))) && py_prng_re.is_match(trimmed) && sensitive_re.is_match(trimmed) {
                    findings.push(SlopFinding {
                        file: target_path.to_string(),
                        line: line_num,
                        column: 1,
                        rule_id: "no-insecure-crypto".to_string(),
                        category: SlopCategory::Security,
                        severity: SlopSeverity::Error,
                        message: "Insecure PRNG used for security-sensitive token/credential (CWE-330): Python random module is predictable".to_string(),
                        snippet: trimmed.to_string(),
                        fixable: false,
                        suggestion: Some("Use the standard secrets module (secrets.token_hex(), secrets.token_urlsafe(), secrets.choice())".to_string()),
                    });
                    continue;
                }
                for (re, desc) in &hash_patterns {
                    if re.is_match(trimmed) {
                        findings.push(SlopFinding {
                            file: target_path.to_string(),
                            line: line_num,
                            column: 1,
                            rule_id: "no-insecure-crypto".to_string(),
                            category: SlopCategory::Security,
                            severity: SlopSeverity::Error,
                            message: format!("Broken hash algorithm detected (CWE-328): {desc}"),
                            snippet: trimmed.to_string(),
                            fixable: false,
                            suggestion: Some("Migrate to SHA-256/SHA-512 for digests, or use Argon2id/bcrypt/scrypt for credential hashing".to_string()),
                        });
                        break;
                    }
                }
            }
        }
    }
    findings
}