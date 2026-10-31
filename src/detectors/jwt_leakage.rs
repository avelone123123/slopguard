use crate::types::{DiffFile, DiffLineType, SlopCategory, SlopFinding, SlopSeverity};
use regex::Regex;
pub fn detect_jwt_leakage_in_diff(diff_files: &[DiffFile]) -> Vec<SlopFinding> {
    let mut findings = Vec::new();
    let jwt_token_re = Regex::new(r"eyJ[A-Za-z0-9_-]{10,}\.[A-Za-z0-9_-]{10,}\.[A-Za-z0-9_-]{10,}").unwrap();
    let weak_secret_re = Regex::new(r#"(?i)(jwt|token|secret).*(sign|verify|decode)\s*\([^)]*["'](secret|123456|password|mysecret|test|jwtsecret)["']"#).unwrap();
    let alg_none_re = Regex::new(r#"(?i)algorithms?\s*[:=]\s*(\[?\s*["']none["']\s*\]?|None)"#).unwrap();
    for df in diff_files {
        let p_lower = df.new_path.to_lowercase();
        if p_lower.ends_with(".md") || p_lower.ends_with(".lock") {
            continue;
        }
        for chunk in &df.chunks {
            for dl in &chunk.lines {
                if dl.line_type != DiffLineType::Add {
                    continue;
                }
                let line_str = dl.content.trim();
                let line_num = dl.new_line_number.unwrap_or(0);
                if jwt_token_re.is_match(line_str) {
                    findings.push(SlopFinding {
                        file: df.new_path.clone(),
                        line: line_num,
                        column: 1,
                        rule_id: "no-hardcoded-jwt-secrets".to_string(),
                        category: SlopCategory::AuthLeakage,
                        severity: SlopSeverity::Error,
                        message: "Hardcoded JSON Web Token (JWT) detected in source code. AI assistants frequently embed dummy/expired production tokens that leak authorization artifacts.".to_string(),
                        snippet: line_str.to_string(),
                        fixable: false,
                        suggestion: Some("Inject JWT tokens dynamically through environment variables or secure credential vaults.".to_string()),
                    });
                }
                if weak_secret_re.is_match(line_str) {
                    findings.push(SlopFinding {
                        file: df.new_path.clone(),
                        line: line_num,
                        column: 1,
                        rule_id: "no-hardcoded-jwt-secrets".to_string(),
                        category: SlopCategory::AuthLeakage,
                        severity: SlopSeverity::Error,
                        message: "Weak or trivial secret key used in JWT signing/verification. Trivial secrets permit offline forgery attacks.".to_string(),
                        snippet: line_str.to_string(),
                        fixable: false,
                        suggestion: Some("Use strong 256-bit cryptographic secrets loaded from process.env.".to_string()),
                    });
                }
                if alg_none_re.is_match(line_str) {
                    findings.push(SlopFinding {
                        file: df.new_path.clone(),
                        line: line_num,
                        column: 1,
                        rule_id: "no-hardcoded-jwt-secrets".to_string(),
                        category: SlopCategory::AuthLeakage,
                        severity: SlopSeverity::Error,
                        message: "Insecure JWT algorithm 'none' permitted. This disables cryptographic signature verification entirely.".to_string(),
                        snippet: line_str.to_string(),
                        fixable: false,
                        suggestion: Some("Require RS256, ES256, or HS256 algorithm enforcement.".to_string()),
                    });
                }
            }
        }
    }
    findings
}
pub fn detect_jwt_leakage_in_content(file_path: &str, content: &str) -> Vec<SlopFinding> {
    let mut findings = Vec::new();
    let p_lower = file_path.to_lowercase();
    if p_lower.ends_with(".md") || p_lower.ends_with(".lock") {
        return findings;
    }
    let jwt_token_re = Regex::new(r"eyJ[A-Za-z0-9_-]{10,}\.[A-Za-z0-9_-]{10,}\.[A-Za-z0-9_-]{10,}").unwrap();
    let weak_secret_re = Regex::new(r#"(?i)(jwt|token|secret).*(sign|verify|decode)\s*\([^)]*["'](secret|123456|password|mysecret|test|jwtsecret)["']"#).unwrap();
    let alg_none_re = Regex::new(r#"(?i)algorithms?\s*[:=]\s*(\[?\s*["']none["']\s*\]?|None)"#).unwrap();
    for (idx, line) in content.lines().enumerate() {
        let line_str = line.trim();
        let line_num = idx + 1;
        if jwt_token_re.is_match(line_str) {
            findings.push(SlopFinding {
                file: file_path.to_string(),
                line: line_num,
                column: 1,
                rule_id: "no-hardcoded-jwt-secrets".to_string(),
                category: SlopCategory::AuthLeakage,
                severity: SlopSeverity::Error,
                message: "Hardcoded JSON Web Token (JWT) detected in source code.".to_string(),
                snippet: line_str.to_string(),
                fixable: false,
                suggestion: Some("Load tokens from environment variables or secure storage.".to_string()),
            });
        }
        if weak_secret_re.is_match(line_str) {
            findings.push(SlopFinding {
                file: file_path.to_string(),
                line: line_num,
                column: 1,
                rule_id: "no-hardcoded-jwt-secrets".to_string(),
                category: SlopCategory::AuthLeakage,
                severity: SlopSeverity::Error,
                message: "Weak or trivial secret key used in JWT signing/verification.".to_string(),
                snippet: line_str.to_string(),
                fixable: false,
                suggestion: Some("Use strong cryptographic secrets loaded from secure configuration.".to_string()),
            });
        }
        if alg_none_re.is_match(line_str) {
            findings.push(SlopFinding {
                file: file_path.to_string(),
                line: line_num,
                column: 1,
                rule_id: "no-hardcoded-jwt-secrets".to_string(),
                category: SlopCategory::AuthLeakage,
                severity: SlopSeverity::Error,
                message: "Insecure JWT algorithm 'none' permitted.".to_string(),
                snippet: line_str.to_string(),
                fixable: false,
                suggestion: Some("Require RS256 or HS256 algorithm enforcement.".to_string()),
            });
        }
    }
    findings
}