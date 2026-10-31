use crate::types::{DiffFile, DiffLineType, SlopCategory, SlopFinding, SlopSeverity};
use regex::Regex;
fn check_file_type(path: &str) -> (bool, bool, bool) {
let l = path.to_lowercase();
(l.ends_with(".ts") || l.ends_with(".tsx") || l.ends_with(".js") || l.ends_with(".jsx") || l.ends_with(".mjs") || l.ends_with(".cjs"), l.ends_with(".py"), l.ends_with(".rs"))
}
fn get_js_async_foreach_regex() -> Regex {
Regex::new(r#"\.forEach\s*\(\s*(?:async\s*(?:\([^)]*\)|[a-zA-Z0-9_$]+)\s*=>|async\s+function\b|\(\s*async\b)"#).unwrap()
}
fn get_js_async_reduce_regex() -> Regex {
Regex::new(r#"\.(?:reduce|reduceRight)\s*\(\s*(?:async\s*(?:\([^)]*\)|[a-zA-Z0-9_$]+)\s*=>|async\s+function\b|\(\s*async\b)"#).unwrap()
}
fn get_py_async_sync_iter_regex() -> Regex {
Regex::new(r#"\b(?:map|filter)\s*\(\s*(?:async\s+lambda[^,]*|[a-zA-Z0-9_]*async[a-zA-Z0-9_]*|[a-zA-Z0-9_]*coro[a-zA-Z0-9_]*)\s*,"#).unwrap()
}
fn get_rust_async_foreach_regex() -> Regex {
Regex::new(r#"\.for_each\s*\(\s*(?:move\s*)?\|[^|]*\|\s*(?:async\b|\{\s*async\b)"#).unwrap()
}
#[allow(dead_code)]
pub const CONTRACT: crate::types::AstDetectorContract = crate::types::AstDetectorContract {
rule_id: "no-async-in-sync-callbacks",
severity: SlopSeverity::Error,
category: SlopCategory::AsyncIterationHazard,
target_languages: &[".js", ".ts", ".jsx", ".tsx", ".py", ".rs"],
analyze_diff: detect_async_callback_traps_in_diff,
analyze_content: detect_async_callback_traps_in_content,
};
pub fn detect_async_callback_traps_in_content(file_path: &str, content: &str) -> Vec<SlopFinding> {
let mut findings = Vec::new();
let (js, py, rs) = check_file_type(file_path);
if !js && !py && !rs {
return findings;
}
let js_foreach_re = get_js_async_foreach_regex();
let js_reduce_re = get_js_async_reduce_regex();
let py_iter_re = get_py_async_sync_iter_regex();
let rs_foreach_re = get_rust_async_foreach_regex();
for (idx, line) in content.lines().enumerate() {
let line_num = idx + 1;
let trimmed = line.trim();
if trimmed.starts_with("//") || trimmed.starts_with("/*") || trimmed.starts_with('*') {
continue;
}
if py && trimmed.starts_with('#') {
continue;
}
if js {
if js_foreach_re.is_match(trimmed) {
findings.push(SlopFinding {
file: file_path.to_string(),
line: line_num,
column: 1,
rule_id: "no-async-in-sync-callbacks".to_string(),
category: SlopCategory::AsyncIterationHazard,
severity: SlopSeverity::Error,
message: "Async callback passed to synchronous '.forEach()' floats unawaited and silently drops rejected promises".to_string(),
snippet: trimmed.to_string(),
fixable: false,
suggestion: Some("Refactor to 'for (const item of items) { await ... }' or 'await Promise.all(items.map(async (item) => ...))'".to_string()),
});
continue;
}
if js_reduce_re.is_match(trimmed) {
findings.push(SlopFinding {
file: file_path.to_string(),
line: line_num,
column: 1,
rule_id: "no-async-in-sync-callbacks".to_string(),
category: SlopCategory::AsyncIterationHazard,
severity: SlopSeverity::Error,
message: "Async callback in '.reduce()' creates unhandled promise accumulator trap and sequencing hazards".to_string(),
snippet: trimmed.to_string(),
fixable: false,
suggestion: Some("Sequentially await promises inside 'for (const item of items)' loop instead of async '.reduce()'".to_string()),
});
continue;
}
}
if py && py_iter_re.is_match(trimmed) {
findings.push(SlopFinding {
file: file_path.to_string(),
line: line_num,
column: 1,
rule_id: "no-async-in-sync-callbacks".to_string(),
category: SlopCategory::AsyncIterationHazard,
severity: SlopSeverity::Error,
message: "Coroutine or async callback passed to synchronous 'map()' / 'filter()' executes without event loop scheduling".to_string(),
snippet: trimmed.to_string(),
fixable: false,
suggestion: Some("Schedule coroutines with 'await asyncio.gather(*(coroutine(item) for item in items))' or use an 'async for' loop".to_string()),
});
continue;
}
if rs && rs_foreach_re.is_match(trimmed) {
findings.push(SlopFinding {
file: file_path.to_string(),
line: line_num,
column: 1,
rule_id: "no-async-in-sync-callbacks".to_string(),
category: SlopCategory::AsyncIterationHazard,
severity: SlopSeverity::Error,
message: "Async block in synchronous iterator '.for_each()' creates unpolled Future that is immediately dropped".to_string(),
snippet: trimmed.to_string(),
fixable: false,
suggestion: Some("Use 'stream.for_each_concurrent(limit, |item| async move { ... }).await' or iterate with 'while let Some(item) = stream.next().await'".to_string()),
});
}
}
findings
}
pub fn detect_async_callback_traps_in_diff(diff_files: &[DiffFile]) -> Vec<SlopFinding> {
let mut findings = Vec::new();
for file in diff_files {
if file.is_deleted {
continue;
}
let target_path = if !file.new_path.is_empty() { &file.new_path } else { &file.old_path };
let (js, py, rs) = check_file_type(target_path);
if !js && !py && !rs {
continue;
}
let mut added_lines = Vec::new();
for chunk in &file.chunks {
for line in &chunk.lines {
if line.line_type == DiffLineType::Add {
added_lines.push(line.content.as_str());
}
}
}
let content = added_lines.join("\n");
findings.extend(detect_async_callback_traps_in_content(target_path, &content));
}
findings
}