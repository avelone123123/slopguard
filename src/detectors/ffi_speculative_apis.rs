use crate::types::{DiffFile, DiffLineType, SlopCategory, SlopFinding, SlopSeverity};
use regex::Regex;
fn is_ffi_source(path: &str) -> bool {
    let l = path.to_lowercase();
    l.ends_with(".rs") || l.ends_with(".c") || l.ends_with(".cpp") || l.ends_with(".h") || l.ends_with(".js") || l.ends_with(".ts") || l.ends_with(".wat")
}
#[allow(dead_code)]
pub const CONTRACT: crate::types::AstDetectorContract = crate::types::AstDetectorContract {
    rule_id: crate::types::RULE_NO_SPECULATIVE_FFI_WASM,
    severity: SlopSeverity::Error,
    category: SlopCategory::SpeculativeNativeFfi,
    target_languages: &["rs", "c", "cpp", "js", "ts", "wat"],
    analyze_diff: detect_ffi_speculative_apis_in_diff,
    analyze_content: detect_ffi_speculative_apis_in_content,
};
fn get_speculative_ffi_patterns() -> Vec<(Regex, &'static str, &'static str)> {
    vec![
        (Regex::new(r#"extern\s+["']C["']\s*\{[^}]*fn\s+(?:slop_|__ai_|fast_crypto_hash|sys_fast_memcpy|cursor_native_)"#).unwrap(), "Synthetic extern C ABI symbol hallucination", "Declare verified system library symbol with canonical linkage"),
        (Regex::new(r#"\b(?:slop_fast_alloc|slop_sys_call|cursor_internal_eval|ai_native_bridge)\s*\("#).unwrap(), "Hallucinated native FFI foreign symbol invocation", "Replace with official platform APIs or libc bindings"),
        (Regex::new(r#"WebAssembly\.instantiate(?:Streaming)?\s*\([^,]+,\s*\{[^}]*env:\s*\{[^}]*__(?:ai|cursor|synth)"#).unwrap(), "Speculative non-standard WebAssembly runtime environment import", "Use standardized WASI host imports (wasi_snapshot_preview1)"),
        (Regex::new(r#"\b(?:cursor_native_api|vscode_native_node|__cursorNative)\b"#).unwrap(), "Fictional native IDE host binding namespace", "Access IDE functions through verified extension message passing APIs"),
        (Regex::new(r#"\bimport\s+["']env["']\s+["'](?:slop_|ai_kernel_|wasm_crypto_accelerate)"#).unwrap(), "Invented WAT/WASM environment import descriptor", "Declare valid WASI imports"),
    ]
}
pub fn detect_ffi_speculative_apis_in_content(file_path: &str, content: &str) -> Vec<SlopFinding> {
    let mut findings = Vec::new();
    if !is_ffi_source(file_path) {
        return findings;
    }
    let patterns = get_speculative_ffi_patterns();
    for (idx, line) in content.lines().enumerate() {
        let trimmed = line.trim();
        if !trimmed.contains("extern") && !trimmed.contains("slop_") && !trimmed.contains("cursor") && !trimmed.contains("ai_") && !trimmed.contains("WebAssembly") && !trimmed.contains("vscode") && !trimmed.contains("__") && !trimmed.contains("env") {
            continue;
        }
        for (re, desc, sugg) in &patterns {
            if re.is_match(trimmed) {
                findings.push(SlopFinding {
                    file: file_path.to_string(),
                    line: idx + 1,
                    column: 1,
                    rule_id: crate::types::RULE_NO_SPECULATIVE_FFI_WASM.to_string(),
                    category: SlopCategory::SpeculativeNativeFfi,
                    severity: SlopSeverity::Error,
                    message: format!("Speculative FFI/WASM hallucination: {desc}"),
                    snippet: trimmed.to_string(),
                    fixable: false,
                    suggestion: Some(sugg.to_string()),
                });
                break;
            }
        }
    }
    findings
}
pub fn detect_ffi_speculative_apis_in_diff(diff_files: &[DiffFile]) -> Vec<SlopFinding> {
    let mut findings = Vec::new();
    let patterns = get_speculative_ffi_patterns();
    for df in diff_files {
        let path = if !df.new_path.is_empty() { &df.new_path } else { &df.old_path };
        if !is_ffi_source(path) {
            continue;
        }
        for chunk in &df.chunks {
            for dl in &chunk.lines {
                if dl.line_type != DiffLineType::Add {
                    continue;
                }
                let line_str = dl.content.trim();
                let line_num = dl.new_line_number.unwrap_or(0);
                if !line_str.contains("extern") && !line_str.contains("slop_") && !line_str.contains("cursor") && !line_str.contains("ai_") && !line_str.contains("WebAssembly") && !line_str.contains("vscode") && !line_str.contains("__") && !line_str.contains("env") {
                    continue;
                }
                for (re, desc, sugg) in &patterns {
                    if re.is_match(line_str) {
                        findings.push(SlopFinding {
                            file: path.clone(),
                            line: line_num,
                            column: 1,
                            rule_id: crate::types::RULE_NO_SPECULATIVE_FFI_WASM.to_string(),
                            category: SlopCategory::SpeculativeNativeFfi,
                            severity: SlopSeverity::Error,
                            message: format!("Speculative FFI/WASM hallucination: {desc}"),
                            snippet: line_str.to_string(),
                            fixable: false,
                            suggestion: Some(sugg.to_string()),
                        });
                        break;
                    }
                }
            }
        }
    }
    findings
}