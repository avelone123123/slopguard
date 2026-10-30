use crate::types::{DiffFile, DiffLineType, SlopCategory, SlopFinding, SlopSeverity};
use regex::Regex;
fn get_secret_patterns() -> Vec<(Regex, &'static str)> {
    vec![
        (Regex::new(r#"(?i)(?:key|secret|token|password|auth)\s*[:=]\s*["'](?:sk-(?:proj|ant)-[a-zA-Z0-9_\-]{16,}|hf_[a-zA-Z0-9]{16,})["']"#).unwrap(), "Leaked AI platform API key (OpenAI/Anthropic/HuggingFace)"),
        (Regex::new(r#"(?i)(?:key|secret|token|auth)\s*[:=]\s*["'](?:ghp_[a-zA-Z0-9]{30,}|gho_[a-zA-Z0-9]{30,})["']"#).unwrap(), "Leaked GitHub Personal Access or OAuth Token"),
        (Regex::new(r#"\bAKIA[0-9A-Z]{16}\b"#).unwrap(), "Exposed AWS Access Key ID"),
        (Regex::new(r#"-----BEGIN (?:RSA |EC |OPENSSH |DSA |PGP )?PRIVATE KEY-----"#).unwrap(), "Unencrypted private cryptographic key block"),
        (Regex::new(r#"\bsk_live_[0-9a-zA-Z]{24,}\b"#).unwrap(), "Live Stripe production secret key"),
        (Regex::new(r#"\bxox[baprs]-[0-9]{10,}-[a-zA-Z0-9]{20,}\b"#).unwrap(), "Exposed Slack bot or user access token"),
    ]
}
pub fn detect_secrets_in_content(file_path: &str, content: &str) -> Vec<SlopFinding> {
    let mut findings = Vec::new();
    let patterns = get_secret_patterns();
    for (i, line) in content.lines().enumerate() {
        for (re, desc) in &patterns {
            if re.is_match(line) {
                findings.push(SlopFinding {
                    file: file_path.to_string(),
                    line: i + 1,
                    column: 1,
                    rule_id: "no-hardcoded-secrets".to_string(),
                    category: SlopCategory::Security,
                    severity: SlopSeverity::Error,
                    message: format!("Hardcoded security credential detected: {desc}"),
                    snippet: line.trim().to_string(),
                    fixable: false,
                    suggestion: Some("Store secrets in environment variables or an external vault".to_string()),
                });
            }
        }
    }
    findings
}
pub fn detect_secrets_in_diff(diff_files: &[DiffFile]) -> Vec<SlopFinding> {
    let mut findings = Vec::new();
    let patterns = get_secret_patterns();
    for f in diff_files {
        let path = if !f.new_path.is_empty() { &f.new_path } else { &f.old_path };
        for c in &f.chunks {
            for l in &c.lines {
                if l.line_type == DiffLineType::Add {
                    for (re, desc) in &patterns {
                        if re.is_match(&l.content) {
                            findings.push(SlopFinding {
                                file: path.clone(),
                                line: l.new_line_number.unwrap_or(1),
                                column: 1,
                                rule_id: "no-hardcoded-secrets".to_string(),
                                category: SlopCategory::Security,
                                severity: SlopSeverity::Error,
                                message: format!("Hardcoded security credential detected: {desc}"),
                                snippet: l.content.trim().to_string(),
                                fixable: false,
                                suggestion: Some("Store secrets in environment variables or an external vault".to_string()),
                            });
                        }
                    }
                }
            }
        }
    }
    findings
}