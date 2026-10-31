use crate::types::{DiffFile, DiffLineType, SlopCategory, SlopFinding, SlopSeverity};
use regex::Regex;
fn is_ide_script(path: &str) -> bool {
    let l = path.to_lowercase();
    l.ends_with(".js") || l.ends_with(".ts") || l.ends_with(".tsx") || l.ends_with(".jsx") || l.ends_with(".mjs") || l.ends_with(".cjs")
}
#[allow(dead_code)]
pub const CONTRACT: crate::types::AstDetectorContract = crate::types::AstDetectorContract {
    rule_id: "no-ide-protocol-hallucinations",
    severity: SlopSeverity::Error,
    category: SlopCategory::IdePluginProtocolHallucination,
    target_languages: &["js", "ts", "tsx", "jsx"],
    analyze_diff: detect_ide_protocol_hallucinations_in_diff,
    analyze_content: detect_ide_protocol_hallucinations_in_content,
};
fn get_hallucinated_patterns() -> Vec<(Regex, &'static str)> {
    vec![
        (Regex::new(r#"vscode\.window\.showAIMessage\s*\("#).unwrap(), "Synthetic VS Code API vscode.window.showAIMessage does not exist"),
        (Regex::new(r#"vscode\.workspace\.registerSlopProvider\s*\("#).unwrap(), "Fictional extension method vscode.workspace.registerSlopProvider"),
        (Regex::new(r#"vscode\.commands\.executeCommand\s*\(\s*['"]cursor\.ai\."#).unwrap(), "Hallucinated Cursor internal command namespace invocation"),
        (Regex::new(r#"cursor\.editor\.injectPrompt\s*\("#).unwrap(), "Non-existent Cursor editor host injection primitive cursor.editor.injectPrompt"),
        (Regex::new(r#"cursor\.ai\.complete\s*\("#).unwrap(), "Invented Cursor host completion method cursor.ai.complete"),
        (Regex::new(r#"window\.ai\.generateText\s*\("#).unwrap(), "Unsupported experimental browser host API window.ai.generateText invocation"),
    ]
}
pub fn detect_ide_protocol_hallucinations_in_content(file_path: &str, content: &str) -> Vec<SlopFinding> {
    let mut findings = Vec::new();
    if !is_ide_script(file_path) {
        return findings;
    }
    let patterns = get_hallucinated_patterns();
    for (idx, line) in content.lines().enumerate() {
        let line_str = line.trim();
        for (re, desc) in &patterns {
            if re.is_match(line_str) {
                findings.push(SlopFinding {
                    file: file_path.to_string(),
                    line: idx + 1,
                    column: 1,
                    rule_id: "no-ide-protocol-hallucinations".to_string(),
                    category: SlopCategory::IdePluginProtocolHallucination,
                    severity: SlopSeverity::Error,
                    message: format!("Hallucinated IDE or Editor extension API invocation: {desc}"),
                    snippet: line_str.to_string(),
                    fixable: false,
                    suggestion: Some("Use official VS Code API (e.g. vscode.window.showInformationMessage) or verified extension points".to_string()),
                });
            }
        }
    }
    findings
}
pub fn detect_ide_protocol_hallucinations_in_diff(diff_files: &[DiffFile]) -> Vec<SlopFinding> {
    let mut findings = Vec::new();
    let patterns = get_hallucinated_patterns();
    for df in diff_files {
        let path = if !df.new_path.is_empty() { &df.new_path } else { &df.old_path };
        if !is_ide_script(path) {
            continue;
        }
        for chunk in &df.chunks {
            for dl in &chunk.lines {
                if dl.line_type != DiffLineType::Add {
                    continue;
                }
                let line_str = dl.content.trim();
                let line_num = dl.new_line_number.unwrap_or(0);
                for (re, desc) in &patterns {
                    if re.is_match(line_str) {
                        findings.push(SlopFinding {
                            file: path.clone(),
                            line: line_num,
                            column: 1,
                            rule_id: "no-ide-protocol-hallucinations".to_string(),
                            category: SlopCategory::IdePluginProtocolHallucination,
                            severity: SlopSeverity::Error,
                            message: format!("Hallucinated IDE or Editor extension API invocation: {desc}"),
                            snippet: line_str.to_string(),
                            fixable: false,
                            suggestion: Some("Use official VS Code API (e.g. vscode.window.showInformationMessage) or verified extension points".to_string()),
                        });
                    }
                }
            }
        }
    }
    findings
}