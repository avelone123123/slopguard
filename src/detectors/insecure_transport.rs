use crate::types::{DiffFile, DiffLineType, SlopCategory, SlopFinding, SlopSeverity};
use regex::Regex;
fn get_insecure_transport_patterns() -> Vec<(Regex, &'static str)> {
    vec![
        (Regex::new(r#"NODE_TLS_REJECT_UNAUTHORIZED\s*=\s*['"]?0['"]?"#).unwrap(), "Node.js TLS certificate rejection globally disabled via NODE_TLS_REJECT_UNAUTHORIZED"),
        (Regex::new(r#"\brejectUnauthorized\s*:\s*false\b"#).unwrap(), "HTTPS Agent configured with rejectUnauthorized disabled"),
        (Regex::new(r#"\bstrictSSL\s*:\s*false\b"#).unwrap(), "Client request configured with strictSSL disabled"),
        (Regex::new(r#"\bverify\s*=\s*False\b"#).unwrap(), "Python HTTP client invoked with SSL verification disabled"),
        (Regex::new(r#"\bssl\._create_unverified_context\s*\("#).unwrap(), "Python SSL context created without certificate verification (_create_unverified_context)"),
        (Regex::new(r#"\burllib3\.disable_warnings\s*\("#).unwrap(), "Python urllib3 InsecureRequestWarning explicitly suppressed"),
        (Regex::new(r#"\bInsecureSkipVerify\s*:\s*true\b"#).unwrap(), "Go tls.Config configured with InsecureSkipVerify enabled"),
        (Regex::new(r#"\bdanger_accept_invalid_certs\s*\(\s*true\s*\)"#).unwrap(), "Rust client configured with danger_accept_invalid_certs enabled"),
        (Regex::new(r#"\bdanger_accept_invalid_hostnames\s*\(\s*true\s*\)"#).unwrap(), "Rust client configured with danger_accept_invalid_hostnames enabled"),
        (Regex::new(r#"\bCURLOPT_SSL_VERIFYPEER\s*=>\s*(?:false|0)\b"#).unwrap(), "PHP cURL configured with CURLOPT_SSL_VERIFYPEER disabled"),
        (Regex::new(r#"\bCURLOPT_SSL_VERIFYHOST\s*=>\s*(?:false|0)\b"#).unwrap(), "PHP cURL configured with CURLOPT_SSL_VERIFYHOST disabled"),
    ]
}
pub fn detect_insecure_transport_in_content(file_path: &str, content: &str) -> Vec<SlopFinding> {
    let mut findings = Vec::new();
    let patterns = get_insecure_transport_patterns();
    for (idx, line) in content.lines().enumerate() {
        let line_num = idx + 1;
        let trimmed = line.trim();
        for (re, desc) in &patterns {
            if re.is_match(trimmed) {
                findings.push(SlopFinding {
                    file: file_path.to_string(),
                    line: line_num,
                    column: 1,
                    rule_id: "no-disabled-tls".to_string(),
                    category: SlopCategory::Security,
                    severity: SlopSeverity::Error,
                    message: format!("Insecure transport detected: {desc}"),
                    snippet: trimmed.to_string(),
                    fixable: false,
                    suggestion: Some("Never disable TLS/SSL validation. Use valid signed certificates or inject custom test CA authorities".to_string()),
                });
                break;
            }
        }
    }
    findings
}
pub fn detect_insecure_transport_in_diff(diff_files: &[DiffFile]) -> Vec<SlopFinding> {
    let mut findings = Vec::new();
    let patterns = get_insecure_transport_patterns();
    for file in diff_files {
        let target_path = if !file.new_path.is_empty() { &file.new_path } else { &file.old_path };
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
                            rule_id: "no-disabled-tls".to_string(),
                            category: SlopCategory::Security,
                            severity: SlopSeverity::Error,
                            message: format!("Insecure transport detected: {desc}"),
                            snippet: trimmed.to_string(),
                            fixable: false,
                            suggestion: Some("Never disable TLS/SSL validation. Use valid signed certificates or inject custom test CA authorities".to_string()),
                        });
                        break;
                    }
                }
            }
        }
    }
    findings
}