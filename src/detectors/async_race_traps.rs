use crate::types::{DiffFile, DiffLineType, SlopCategory, SlopFinding, SlopSeverity};
use regex::Regex;
fn is_supported_ext(p: &str) -> bool {
    let l = p.to_lowercase();
    l.ends_with(".js") || l.ends_with(".ts") || l.ends_with(".jsx") || l.ends_with(".tsx") || l.ends_with(".mjs") || l.ends_with(".cjs") || l.ends_with(".py") || l.ends_with(".rs")
}
pub fn detect_async_race_traps_in_diff(diff_files: &[DiffFile]) -> Vec<SlopFinding> {
    let mut findings = Vec::new();
    let js_race = Regex::new(r#"(?i)Promise\.race\s*\(\s*\[[^\]]+\]\s*\)"#).unwrap();
    let node_stream_pipe = Regex::new(r#"\.pipe\s*\([a-zA-Z0-9_.]+\)"#).unwrap();
    let py_gather = Regex::new(r#"(?i)asyncio\.gather\s*\([^)]+\)"#).unwrap();
    for df in diff_files {
        let path = if !df.new_path.is_empty() { &df.new_path } else { &df.old_path };
        if !is_supported_ext(path) {
            continue;
        }
        for chunk in &df.chunks {
            for dl in &chunk.lines {
                if dl.line_type != DiffLineType::Add {
                    continue;
                }
                let line_str = dl.content.trim();
                let line_num = dl.new_line_number.unwrap_or(0);
                let is_js_race_leak = js_race.is_match(line_str) && !line_str.contains(".catch");
                let is_stream_pipe_leak = node_stream_pipe.is_match(line_str) && !line_str.contains(".on('error'") && !line_str.contains(".on(\"error\"");
                let is_gather_leak = py_gather.is_match(line_str) && !line_str.contains("return_exceptions");
                let is_yield_leak = (line_str.starts_with("yield ") || line_str.starts_with("yield\t")) && (line_str.contains("fetch(") || line_str.contains("db.query(") || line_str.contains("axios.") || line_str.contains("client.get(")) && !line_str.contains("await ") && !line_str.contains("yield*");
                if is_js_race_leak || is_stream_pipe_leak || is_gather_leak || is_yield_leak {
                    findings.push(SlopFinding {
                        file: path.clone(),
                        line: line_num,
                        column: 1,
                        rule_id: "no-async-pipeline-races".to_string(),
                        category: SlopCategory::AsyncResourceLeak,
                        severity: SlopSeverity::Warning,
                        message: "Unshielded async race or unmonitored stream pipe prone to resource leakage or unhandled rejections.".to_string(),
                        snippet: line_str.to_string(),
                        fixable: false,
                        suggestion: Some("Attach error handlers or catch blocks, use stream.pipeline, or set return_exceptions=True in asyncio.gather.".to_string()),
                    });
                }
            }
        }
    }
    findings
}
pub fn detect_async_race_traps_in_content(file_path: &str, content: &str) -> Vec<SlopFinding> {
    let mut findings = Vec::new();
    if !is_supported_ext(file_path) {
        return findings;
    }
    let js_race = Regex::new(r#"(?i)Promise\.race\s*\(\s*\[[^\]]+\]\s*\)"#).unwrap();
    let node_stream_pipe = Regex::new(r#"\.pipe\s*\([a-zA-Z0-9_.]+\)"#).unwrap();
    let py_gather = Regex::new(r#"(?i)asyncio\.gather\s*\([^)]+\)"#).unwrap();
    let lines: Vec<&str> = content.lines().collect();
    for (idx, line) in lines.iter().enumerate() {
        let line_str = line.trim();
        let line_num = idx + 1;
        let next_line = lines.get(idx + 1).map(|s| s.trim()).unwrap_or("");
        let is_js_race_leak = js_race.is_match(line_str) && !line_str.contains(".catch") && !next_line.starts_with(".catch");
        let is_stream_pipe_leak = node_stream_pipe.is_match(line_str) && !line_str.contains(".on('error'") && !line_str.contains(".on(\"error\"") && !next_line.contains(".on('error'") && !next_line.contains(".on(\"error\"");
        let is_gather_leak = py_gather.is_match(line_str) && !line_str.contains("return_exceptions");
        let is_yield_leak = (line_str.starts_with("yield ") || line_str.starts_with("yield\t")) && (line_str.contains("fetch(") || line_str.contains("db.query(") || line_str.contains("axios.") || line_str.contains("client.get(")) && !line_str.contains("await ") && !line_str.contains("yield*");
        if is_js_race_leak || is_stream_pipe_leak || is_gather_leak || is_yield_leak {
            findings.push(SlopFinding {
                file: file_path.to_string(),
                line: line_num,
                column: 1,
                rule_id: "no-async-pipeline-races".to_string(),
                category: SlopCategory::AsyncResourceLeak,
                severity: SlopSeverity::Warning,
                message: "Unshielded async race or unmonitored stream pipe prone to resource leakage or unhandled rejections.".to_string(),
                snippet: line_str.to_string(),
                fixable: false,
                suggestion: Some("Attach error handlers or catch blocks, use stream.pipeline, or set return_exceptions=True in asyncio.gather.".to_string()),
            });
        }
    }
    findings
}