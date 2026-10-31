use crate::types::{DiffFile, DiffLineType, SlopCategory, SlopFinding, SlopSeverity};
use regex::Regex;
pub fn detect_xxe_ssrf_in_diff(diff_files: &[DiffFile]) -> Vec<SlopFinding> {
    let mut findings = Vec::new();
    let xxe_re = Regex::new(r#"(?i)(resolve_entities\s*=\s*True|disallow_doctype\s*=\s*False|LIBXML_NOENT|setFeature\([^)]*external-general-entities[^)]*true|FEATURE_SECURE_PROCESSING\s*,\s*false)"#).unwrap();
    let ssrf_metadata_re = Regex::new(r#"(?i)(169\.254\.169\.254|metadata\.google\.internal|100\.100\.100\.200)"#).unwrap();
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
                if xxe_re.is_match(line_str) {
                    findings.push(SlopFinding {
                        file: df.new_path.clone(),
                        line: line_num,
                        column: 1,
                        rule_id: "no-unsafe-xml-or-ssrf".to_string(),
                        category: SlopCategory::SsrfHazard,
                        severity: SlopSeverity::Error,
                        message: "Unsafe XML parser configuration enabling external entity resolution (XXE vulnerability).".to_string(),
                        snippet: line_str.to_string(),
                        fixable: false,
                        suggestion: Some("Disable external DTD resolution and entity processing in XML parsers.".to_string()),
                    });
                }
                if ssrf_metadata_re.is_match(line_str) {
                    findings.push(SlopFinding {
                        file: df.new_path.clone(),
                        line: line_num,
                        column: 1,
                        rule_id: "no-unsafe-xml-or-ssrf".to_string(),
                        category: SlopCategory::SsrfHazard,
                        severity: SlopSeverity::Error,
                        message: "Hardcoded cloud instance metadata IP address detected (SSRF hazard).".to_string(),
                        snippet: line_str.to_string(),
                        fixable: false,
                        suggestion: Some("Block outgoing network requests to cloud metadata endpoints and internal IPs.".to_string()),
                    });
                }
            }
        }
    }
    findings
}
pub fn detect_xxe_ssrf_in_content(file_path: &str, content: &str) -> Vec<SlopFinding> {
    let mut findings = Vec::new();
    let p_lower = file_path.to_lowercase();
    if p_lower.ends_with(".md") || p_lower.ends_with(".lock") {
        return findings;
    }
    let xxe_re = Regex::new(r#"(?i)(resolve_entities\s*=\s*True|disallow_doctype\s*=\s*False|LIBXML_NOENT|setFeature\([^)]*external-general-entities[^)]*true|FEATURE_SECURE_PROCESSING\s*,\s*false)"#).unwrap();
    let ssrf_metadata_re = Regex::new(r#"(?i)(169\.254\.169\.254|metadata\.google\.internal|100\.100\.100\.200)"#).unwrap();
    for (idx, line) in content.lines().enumerate() {
        let line_str = line.trim();
        let line_num = idx + 1;
        if xxe_re.is_match(line_str) {
            findings.push(SlopFinding {
                file: file_path.to_string(),
                line: line_num,
                column: 1,
                rule_id: "no-unsafe-xml-or-ssrf".to_string(),
                category: SlopCategory::SsrfHazard,
                severity: SlopSeverity::Error,
                message: "Unsafe XML parser configuration enabling external entity resolution (XXE).".to_string(),
                snippet: line_str.to_string(),
                fixable: false,
                suggestion: Some("Disable external entity parsing in XML libraries.".to_string()),
            });
        }
        if ssrf_metadata_re.is_match(line_str) {
            findings.push(SlopFinding {
                file: file_path.to_string(),
                line: line_num,
                column: 1,
                rule_id: "no-unsafe-xml-or-ssrf".to_string(),
                category: SlopCategory::SsrfHazard,
                severity: SlopSeverity::Error,
                message: "Cloud instance metadata IP address detected (SSRF hazard).".to_string(),
                snippet: line_str.to_string(),
                fixable: false,
                suggestion: Some("Ensure user input cannot access internal cloud metadata IPs.".to_string()),
            });
        }
    }
    findings
}