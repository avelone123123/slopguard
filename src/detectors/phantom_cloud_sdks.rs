use crate::types::{DiffFile, DiffLineType, SlopCategory, SlopFinding, SlopSeverity};
use regex::Regex;
fn is_manifest_file(p: &str) -> bool {
    let l = p.to_lowercase();
    l.ends_with("package.json") || l.ends_with("requirements.txt") || l.ends_with("pyproject.toml") || l.ends_with("cargo.toml")
}
pub fn detect_phantom_cloud_sdks_in_diff(diff_files: &[DiffFile]) -> Vec<SlopFinding> {
    let mut findings = Vec::new();
    let phantom_npm_sdk = Regex::new(r#"(?i)"(@aws-sdk/client-all|aws-sdk-v4|azure-identity-v2|google-cloud-storage-next|firebase-admin-next|openai-client-official)"\s*:"#).unwrap();
    let phantom_pypi_sdk = Regex::new(r#"(?i)^\s*(boto4|openai-async-client|langchain-extended|google-cloud-all|transformers-plus)\s*([=>~<]|$)"#).unwrap();
    let phantom_crate_sdk = Regex::new(r#"(?i)^\s*(tokio-plus|reqwest-next|serde-extended|aws-sdk-complete)\s*="#).unwrap();
    for df in diff_files {
        let path = if !df.new_path.is_empty() { &df.new_path } else { &df.old_path };
        if !is_manifest_file(path) {
            continue;
        }
        let lower = path.to_lowercase();
        let is_npm = lower.ends_with("package.json");
        let is_pypi = lower.ends_with("requirements.txt") || lower.ends_with("pyproject.toml");
        let is_cargo = lower.ends_with("cargo.toml");
        for chunk in &df.chunks {
            for dl in &chunk.lines {
                if dl.line_type != DiffLineType::Add {
                    continue;
                }
                let line_str = dl.content.trim();
                let line_num = dl.new_line_number.unwrap_or(0);
                let matched = if is_npm {
                    phantom_npm_sdk.is_match(line_str)
                } else if is_pypi {
                    phantom_pypi_sdk.is_match(line_str)
                } else if is_cargo {
                    phantom_crate_sdk.is_match(line_str)
                } else {
                    phantom_npm_sdk.is_match(line_str) || phantom_pypi_sdk.is_match(line_str) || phantom_crate_sdk.is_match(line_str)
                };
                if matched {
                    findings.push(SlopFinding {
                        file: path.clone(),
                        line: line_num,
                        column: 1,
                        rule_id: "no-phantom-cloud-sdks".to_string(),
                        category: SlopCategory::SupplyChainPoisoning,
                        severity: SlopSeverity::Error,
                        message: "Hallucinated or poisoned phantom cloud SDK dependency detected in manifest.".to_string(),
                        snippet: line_str.to_string(),
                        fixable: false,
                        suggestion: Some("Use official, verified cloud provider SDK packages.".to_string()),
                    });
                }
            }
        }
    }
    findings
}
pub fn detect_phantom_cloud_sdks_in_content(file_path: &str, content: &str) -> Vec<SlopFinding> {
    let mut findings = Vec::new();
    if !is_manifest_file(file_path) {
        return findings;
    }
    let phantom_npm_sdk = Regex::new(r#"(?i)"(@aws-sdk/client-all|aws-sdk-v4|azure-identity-v2|google-cloud-storage-next|firebase-admin-next|openai-client-official)"\s*:"#).unwrap();
    let phantom_pypi_sdk = Regex::new(r#"(?i)^\s*(boto4|openai-async-client|langchain-extended|google-cloud-all|transformers-plus)\s*([=>~<]|$)"#).unwrap();
    let phantom_crate_sdk = Regex::new(r#"(?i)^\s*(tokio-plus|reqwest-next|serde-extended|aws-sdk-complete)\s*="#).unwrap();
    let lower = file_path.to_lowercase();
    let is_npm = lower.ends_with("package.json");
    let is_pypi = lower.ends_with("requirements.txt") || lower.ends_with("pyproject.toml");
    let is_cargo = lower.ends_with("cargo.toml");
    for (idx, line) in content.lines().enumerate() {
        let line_str = line.trim();
        let line_num = idx + 1;
        let matched = if is_npm {
            phantom_npm_sdk.is_match(line_str)
        } else if is_pypi {
            phantom_pypi_sdk.is_match(line_str)
        } else if is_cargo {
            phantom_crate_sdk.is_match(line_str)
        } else {
            phantom_npm_sdk.is_match(line_str) || phantom_pypi_sdk.is_match(line_str) || phantom_crate_sdk.is_match(line_str)
        };
        if matched {
            findings.push(SlopFinding {
                file: file_path.to_string(),
                line: line_num,
                column: 1,
                rule_id: "no-phantom-cloud-sdks".to_string(),
                category: SlopCategory::SupplyChainPoisoning,
                severity: SlopSeverity::Error,
                message: "Hallucinated or poisoned phantom cloud SDK dependency detected in manifest.".to_string(),
                snippet: line_str.to_string(),
                fixable: false,
                suggestion: Some("Use official, verified cloud provider SDK packages.".to_string()),
            });
        }
    }
    findings
}