use crate::types::{DiffFile, DiffLineType, SlopCategory, SlopFinding, SlopSeverity};
use regex::Regex;
struct HallucinatedExport {
    package: &'static str,
    export_name: &'static str,
    correct_alternative: &'static str,
}
fn get_known_hallucinations() -> Vec<HallucinatedExport> {
    vec![
        HallucinatedExport { package: "react", export_name: "useLocalStorage", correct_alternative: "Custom hook or third-party package like usehooks-ts" },
        HallucinatedExport { package: "react", export_name: "useFetch", correct_alternative: "@tanstack/react-query or custom useEffect fetch hook" },
        HallucinatedExport { package: "react", export_name: "useDebounce", correct_alternative: "use-debounce package or custom hook" },
        HallucinatedExport { package: "react", export_name: "useInterval", correct_alternative: "Custom useEffect timer hook" },
        HallucinatedExport { package: "axios", export_name: "createClient", correct_alternative: "axios.create()" },
        HallucinatedExport { package: "axios", export_name: "postJson", correct_alternative: "axios.post()" },
        HallucinatedExport { package: "lodash", export_name: "deepClone", correct_alternative: "cloneDeep from lodash" },
        HallucinatedExport { package: "lodash-es", export_name: "deepClone", correct_alternative: "cloneDeep from lodash-es" },
        HallucinatedExport { package: "lodash", export_name: "deepMerge", correct_alternative: "merge from lodash" },
        HallucinatedExport { package: "crypto", export_name: "generateUUID", correct_alternative: "crypto.randomUUID() or uuid package" },
    ]
}
fn check_line_hallucinations(line: &str, file: &str, line_num: usize, hallucinations: &[HallucinatedExport]) -> Vec<SlopFinding> {
    let mut findings = Vec::new();
    let re_import = Regex::new(r#"import\s*\{([^}]+)\}\s*from\s*["']([^"']+)["']"#).unwrap();
    for cap in re_import.captures_iter(line) {
        let imports_str = cap.get(1).map(|m| m.as_str()).unwrap_or("");
        let pkg = cap.get(2).map(|m| m.as_str()).unwrap_or("");
        let items: Vec<&str> = imports_str.split(',').map(|s| s.trim()).collect();
        for item in items {
            let base_item = item.split_whitespace().next().unwrap_or(item);
            for h in hallucinations {
                if pkg == h.package && base_item == h.export_name {
                    findings.push(SlopFinding {
                        file: file.to_string(),
                        line: line_num,
                        column: 1,
                        rule_id: "no-hallucinated-imports".to_string(),
                        category: SlopCategory::GhostDependency,
                        severity: SlopSeverity::Error,
                        message: format!("Hallucinated package export '{{ {} }}' from '{}' does not exist", h.export_name, h.package),
                        snippet: line.trim().to_string(),
                        fixable: false,
                        suggestion: Some(format!("Use: {}", h.correct_alternative)),
                    });
                }
            }
        }
    }
    findings
}
pub fn detect_hallucinated_imports_in_content(file_path: &str, content: &str) -> Vec<SlopFinding> {
    let mut findings = Vec::new();
    let hallucinations = get_known_hallucinations();
    for (i, line) in content.lines().enumerate() {
        findings.extend(check_line_hallucinations(line, file_path, i + 1, &hallucinations));
    }
    findings
}
pub fn detect_hallucinated_imports_in_diff(diff_files: &[DiffFile]) -> Vec<SlopFinding> {
    let mut findings = Vec::new();
    let hallucinations = get_known_hallucinations();
    for f in diff_files {
        let path = if !f.new_path.is_empty() { &f.new_path } else { &f.old_path };
        for c in &f.chunks {
            for l in &c.lines {
                if l.line_type == DiffLineType::Add {
                    findings.extend(check_line_hallucinations(&l.content, path, l.new_line_number.unwrap_or(1), &hallucinations));
                }
            }
        }
    }
    findings
}