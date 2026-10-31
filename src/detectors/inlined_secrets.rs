use crate::types::{DiffFile, DiffLineType, SlopCategory, SlopFinding, SlopSeverity};
use regex::Regex;
fn get_inlined_secret_patterns() -> Vec<(Regex, &'static str)> {
    vec![
        (Regex::new(r#"(?i)\b(?:NEXT_PUBLIC_|VITE_|REACT_APP_|EXPO_PUBLIC_|NUXT_PUBLIC_)[a-zA-Z0-9_]*(?:SERVICE_ROLE|SECRET_KEY|PRIVATE_KEY|ADMIN_TOKEN|ADMIN_KEY|MASTER_KEY|DATABASE_URL)[a-zA-Z0-9_]*\b"#).unwrap(), "Client-inlined public variable exposing administrative master secret"),
        (Regex::new(r#"(?i)(?:NEXT_PUBLIC_|VITE_|REACT_APP_|EXPO_PUBLIC_|NUXT_PUBLIC_)[a-zA-Z0-9_]*\s*[:=]\s*["'](?:sb_secret_[a-zA-Z0-9_\-]{16,}|sk_live_[0-9a-zA-Z]{20,}|ghp_[a-zA-Z0-9]{25,}|sq0csp-[0-9a-zA-Z_\-]{20,}|[a-zA-Z0-9_\-\.]{10,}service_role[a-zA-Z0-9_\-\.]*)["']"#).unwrap(), "Direct assignment of production master token to client-exposed public variable"),
    ]
}
fn is_inlined_secret_target(path_str: &str) -> bool {
    let lower = path_str.to_lowercase();
    lower.ends_with(".env") || lower.contains(".env.") || lower.ends_with(".ts") || lower.ends_with(".tsx") || lower.ends_with(".js") || lower.ends_with(".jsx") || lower.ends_with(".mjs") || lower.ends_with(".cjs") || lower.contains("next.config.") || lower.contains("vite.config.") || lower.contains("nuxt.config.")
}
pub fn detect_inlined_secrets_in_content(file_path: &str, content: &str) -> Vec<SlopFinding> {
    let mut findings = Vec::new();
    if !is_inlined_secret_target(file_path) {
        return findings;
    }
    let patterns = get_inlined_secret_patterns();
    for (idx, line) in content.lines().enumerate() {
        let line_num = idx + 1;
        let trimmed = line.trim();
        for (re, desc) in &patterns {
            if re.is_match(trimmed) {
                findings.push(SlopFinding {
                    file: file_path.to_string(),
                    line: line_num,
                    column: 1,
                    rule_id: "no-inlined-secrets".to_string(),
                    category: SlopCategory::Security,
                    severity: SlopSeverity::Error,
                    message: format!("Client secret leak risk: {desc}"),
                    snippet: trimmed.to_string(),
                    fixable: false,
                    suggestion: Some("Move logic to server-side API routes (app/api/*, Server Actions) using server-only environment variables without public prefix".to_string()),
                });
                break;
            }
        }
    }
    findings
}
pub fn detect_inlined_secrets_in_diff(diff_files: &[DiffFile]) -> Vec<SlopFinding> {
    let mut findings = Vec::new();
    let patterns = get_inlined_secret_patterns();
    for file in diff_files {
        if file.is_deleted {
            continue;
        }
        let target_path = if !file.new_path.is_empty() { &file.new_path } else { &file.old_path };
        if !is_inlined_secret_target(target_path) {
            continue;
        }
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
                            rule_id: "no-inlined-secrets".to_string(),
                            category: SlopCategory::Security,
                            severity: SlopSeverity::Error,
                            message: format!("Client secret leak risk: {desc}"),
                            snippet: trimmed.to_string(),
                            fixable: false,
                            suggestion: Some("Move logic to server-side API routes (app/api/*, Server Actions) using server-only environment variables without public prefix".to_string()),
                        });
                        break;
                    }
                }
            }
        }
    }
    findings
}