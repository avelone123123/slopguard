use crate::config::should_ignore_path;
use crate::types::{DiffFile, DiffLineType, FunctionSignature, SlopCategory, SlopFinding, SlopSeverity};
use regex::Regex;
use std::collections::HashSet;
use std::fs;
use std::path::Path;
use walkdir::WalkDir;
fn tokenize_code(code: &str) -> HashSet<String> {
    let re_str = Regex::new(r#"["'`][^"'`]*["'`]"#).unwrap();
    let re_num = Regex::new(r"\b\d+\b").unwrap();
    let re_word = Regex::new(r"[a-zA-Z_$][a-zA-Z0-9_$]*").unwrap();
    let s1 = re_str.replace_all(code, "STR");
    let s2 = re_num.replace_all(&s1, "NUM");
    re_word.find_iter(&s2).map(|m| m.as_str().to_lowercase()).collect()
}
fn calculate_jaccard_similarity(set_a: &HashSet<String>, set_b: &HashSet<String>) -> f64 {
    if set_a.is_empty() || set_b.is_empty() {
        return 0.0;
    }
    let intersection = set_a.intersection(set_b).count();
    let union = set_a.len() + set_b.len() - intersection;
    if union == 0 { 0.0 } else { intersection as f64 / union as f64 }
}
fn normalize_identifier(name: &str) -> String {
    name.replace(['-', '_'], "").to_lowercase()
}
pub fn extract_functions_from_file(file_path: &str, content: &str) -> Vec<FunctionSignature> {
    let mut results = Vec::new();
    let fn_re = Regex::new(r"(?:export\s+)?(?:async\s+)?(?:function\s+([a-zA-Z0-9_$]+)|(?:const|let|var)\s+([a-zA-Z0-9_$]+)\s*=\s*(?:async\s*)?(?:\([^)]*\)|[a-zA-Z0-9_$]+)\s*=>|fn\s+([a-zA-Z0-9_$]+)\s*\(|def\s+([a-zA-Z0-9_$]+)\s*\()").unwrap();
    let lines: Vec<&str> = content.lines().collect();
    for (idx, line) in lines.iter().enumerate() {
        if let Some(caps) = fn_re.captures(line) {
            let name = caps.get(1).or_else(|| caps.get(2)).or_else(|| caps.get(3)).or_else(|| caps.get(4)).map(|m| m.as_str().to_string()).unwrap_or_default();
            if name.len() < 3 {
                continue;
            }
            let end_idx = (idx + 12).min(lines.len());
            let snippet = lines[idx..end_idx].join("\n");
            let tokens = tokenize_code(&snippet);
            if tokens.len() >= 8 {
                results.push(FunctionSignature { name, file: file_path.to_string(), line: idx + 1, body_snippet: snippet, token_set: tokens });
            }
        }
    }
    results
}
pub fn index_existing_functions(project_root: &Path, ignore_patterns: &[String]) -> Vec<FunctionSignature> {
    let mut signatures = Vec::new();
    let walker = WalkDir::new(project_root).into_iter().filter_entry(|e| {
        let name = e.file_name().to_string_lossy();
        if e.file_type().is_dir() && (name == "target" || name == ".git" || name == "node_modules" || name == "dist" || name == "build") {
            return false;
        }
        let rel = e.path().strip_prefix(project_root).unwrap_or(e.path());
        let rel_str = rel.to_string_lossy().replace('\\', "/");
        !should_ignore_path(&rel_str, ignore_patterns)
    });
    for entry in walker.filter_map(|e| e.ok()) {
        if entry.file_type().is_file() {
            let rel = entry.path().strip_prefix(project_root).unwrap_or(entry.path());
            let rel_str = rel.to_string_lossy().replace('\\', "/");
            let is_code = rel_str.ends_with(".ts") || rel_str.ends_with(".js") || rel_str.ends_with(".rs") || rel_str.ends_with(".py");
            if is_code {
                if let Ok(content) = fs::read_to_string(entry.path()) {
                    signatures.extend(extract_functions_from_file(&rel_str, &content));
                }
            }
        }
    }
    signatures
}
pub fn detect_duplicate_utilities(diff_files: &[DiffFile], existing: &[FunctionSignature]) -> Vec<SlopFinding> {
    let mut findings = Vec::new();
    for file in diff_files {
        if file.is_deleted {
            continue;
        }
        let target_path = if !file.new_path.is_empty() { &file.new_path } else { &file.old_path };
        let added_lines: Vec<&str> = file.chunks.iter().flat_map(|c| c.lines.iter()).filter(|l| l.line_type == DiffLineType::Add).map(|l| l.content.as_str()).collect();
        let added_content = added_lines.join("\n");
        let new_signatures = extract_functions_from_file(target_path, &added_content);
        for new_fn in new_signatures {
            let norm_new = normalize_identifier(&new_fn.name);
            for old_fn in existing {
                if old_fn.file == *target_path {
                    continue;
                }
                let norm_old = normalize_identifier(&old_fn.name);
                let name_match = norm_new == norm_old || (norm_new.contains(&norm_old) && norm_old.len() > 6) || (norm_old.contains(&norm_new) && norm_new.len() > 6);
                let sim = calculate_jaccard_similarity(&new_fn.token_set, &old_fn.token_set);
                if (name_match && sim >= 0.50) || sim >= 0.85 {
                    findings.push(SlopFinding {
                        file: target_path.to_string(),
                        line: new_fn.line,
                        column: 1,
                        rule_id: "no-duplicate-shadow-utilities".to_string(),
                        category: SlopCategory::Duplicate,
                        severity: SlopSeverity::Warning,
                        message: format!("Shadow utility '{}' duplicates existing '{}' in {}:{} ({}% structural overlap)", new_fn.name, old_fn.name, old_fn.file, old_fn.line, (sim * 100.0).round() as usize),
                        snippet: format!("function {}(...)", new_fn.name),
                        fixable: false,
                        suggestion: Some(format!("Reuse and reference '{}' from '{}' instead of declaring a duplicate", old_fn.name, old_fn.file)),
                    });
                    break;
                }
            }
        }
    }
    findings
}