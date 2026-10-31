use crate::types::{DiffFile, DiffLineType, SlopCategory, SlopFinding, SlopSeverity};
use regex::Regex;
fn is_js_ts(path: &str) -> bool {
let lower = path.to_lowercase();
lower.ends_with(".ts") || lower.ends_with(".tsx") || lower.ends_with(".js") || lower.ends_with(".jsx") || lower.ends_with(".mjs") || lower.ends_with(".cjs")
}
fn is_python(path: &str) -> bool {
path.to_lowercase().ends_with(".py")
}
fn is_rust(path: &str) -> bool {
path.to_lowercase().ends_with(".rs")
}
fn has_js_throttle_guard(snippet: &str) -> bool {
snippet.contains("pLimit") || snippet.contains("p-limit") || snippet.contains("limit(") || snippet.contains("pMap") || snippet.contains("p-map") || snippet.contains("chunk") || snippet.contains("batch") || snippet.contains("concurrency") || snippet.contains("Semaphore") || snippet.contains("semaphore") || snippet.contains("pQueue") || snippet.contains("PQueue")
}
fn has_py_throttle_guard(snippet: &str) -> bool {
snippet.contains("Semaphore") || snippet.contains("semaphore") || snippet.contains("sem") || snippet.contains("as_completed") || snippet.contains("chunk") || snippet.contains("batch") || snippet.contains("Queue") || snippet.contains("queue")
}
fn has_rust_throttle_guard(snippet: &str) -> bool {
snippet.contains("buffer_unordered") || snippet.contains("buffered") || snippet.contains("chunks") || snippet.contains("Semaphore") || snippet.contains("semaphore")
}
pub fn detect_unbounded_concurrency_in_content(file_path: &str, content: &str) -> Vec<SlopFinding> {
let mut findings = Vec::new();
let js = is_js_ts(file_path);
let py = is_python(file_path);
let rs = is_rust(file_path);
if !js && !py && !rs {
return findings;
}
let lines: Vec<&str> = content.lines().collect();
let js_gather_re = Regex::new(r#"Promise\.(?:all|allSettled)\s*\("#).unwrap();
let js_map_re = Regex::new(r#"\.map\s*\(\s*(?:async\b|\([^)]*\)\s*=>|[a-zA-Z0-9_$]+\s*=>|function\b)"#).unwrap();
let py_gather_comp_re = Regex::new(r#"asyncio\.gather\s*\(\s*\*\s*\[[^\n]*?for\s+[a-zA-Z0-9_$,\s]+\s+in\s+"#).unwrap();
let py_gather_gen_re = Regex::new(r#"asyncio\.gather\s*\(\s*\*\s*\([^\n]*?for\s+[a-zA-Z0-9_$,\s]+\s+in\s+"#).unwrap();
let py_create_task_re = Regex::new(r#"\b(?:asyncio|loop)\.create_task\s*\("#).unwrap();
let py_loop_re = Regex::new(r#"^\s*(?:for\s+[a-zA-Z0-9_$,\s]+\s+in\s+|while\s+)"#).unwrap();
let rust_join_all_re = Regex::new(r#"(?:futures::future::join_all|future::join_all|\bjoin_all)\s*\("#).unwrap();
let rust_map_re = Regex::new(r#"\.map\s*\("#).unwrap();
for (idx, line) in lines.iter().enumerate() {
let line_num = idx + 1;
let trimmed = line.trim();
if trimmed.starts_with("//") || trimmed.starts_with("/*") || trimmed.starts_with('*') {
continue;
}
if py && trimmed.starts_with('#') {
continue;
}
if js {
let has_promise_wrapper = js_gather_re.is_match(trimmed);
let has_map_call = js_map_re.is_match(trimmed);
if has_promise_wrapper && has_map_call && !has_js_throttle_guard(trimmed) {
findings.push(SlopFinding {
file: file_path.to_string(),
line: line_num,
column: 1,
rule_id: "no-unbounded-concurrency".to_string(),
category: SlopCategory::AsyncAntiPattern,
severity: SlopSeverity::Warning,
message: "Unbounded concurrency detected: 'Promise.all/allSettled' over unthrottled '.map()' risks socket exhaustion and rate-limiting cascades".to_string(),
snippet: trimmed.to_string(),
fixable: false,
suggestion: Some("Throttle concurrent tasks using 'p-limit' or batch with chunking: 'const limit = pLimit(concurrency); await Promise.all(items.map(i => limit(() => fn(i))))'".to_string()),
});
continue;
}
if has_promise_wrapper && !has_js_throttle_guard(trimmed) {
for lookahead in 1..=3 {
if let Some(next_line) = lines.get(idx + lookahead) {
let next_trimmed = next_line.trim();
if js_map_re.is_match(next_trimmed) && !has_js_throttle_guard(next_trimmed) {
findings.push(SlopFinding {
file: file_path.to_string(),
line: line_num,
column: 1,
rule_id: "no-unbounded-concurrency".to_string(),
category: SlopCategory::AsyncAntiPattern,
severity: SlopSeverity::Warning,
message: "Unbounded concurrency detected: 'Promise.all/allSettled' over unthrottled '.map()' risks socket exhaustion and rate-limiting cascades".to_string(),
snippet: format!("{} {}", trimmed, next_trimmed),
fixable: false,
suggestion: Some("Throttle concurrent tasks using 'p-limit' or batch with chunking: 'const limit = pLimit(concurrency); await Promise.all(items.map(i => limit(() => fn(i))))'".to_string()),
});
break;
}
if next_trimmed.contains(';') || next_trimmed.starts_with("const ") || next_trimmed.starts_with("let ") {
break;
}
}
}
}
}
if py {
if (py_gather_comp_re.is_match(trimmed) || py_gather_gen_re.is_match(trimmed)) && !has_py_throttle_guard(trimmed) {
let mut guarded = false;
for lookback in (0..idx).rev().take(15) {
if let Some(prev_line) = lines.get(lookback) {
let prev_trimmed = prev_line.trim();
if has_py_throttle_guard(prev_trimmed) {
guarded = true;
break;
}
}
}
if !guarded {
findings.push(SlopFinding {
file: file_path.to_string(),
line: line_num,
column: 1,
rule_id: "no-unbounded-concurrency".to_string(),
category: SlopCategory::AsyncAntiPattern,
severity: SlopSeverity::Warning,
message: "Unbounded concurrency detected: 'asyncio.gather' over unbounded comprehension risks EMFILE file descriptor exhaustion".to_string(),
snippet: trimmed.to_string(),
fixable: false,
suggestion: Some("Throttle concurrent execution using 'asyncio.Semaphore(limit)' or process with 'asyncio.as_completed'".to_string()),
});
continue;
}
}
if py_create_task_re.is_match(trimmed) && !has_py_throttle_guard(trimmed) {
let mut in_loop = false;
for lookback in (0..idx).rev().take(6) {
if let Some(prev_line) = lines.get(lookback) {
let prev_trimmed = prev_line.trim();
if py_loop_re.is_match(prev_trimmed) {
in_loop = true;
break;
}
if prev_trimmed.starts_with("def ") || prev_trimmed.starts_with("class ") {
break;
}
}
}
if in_loop {
findings.push(SlopFinding {
file: file_path.to_string(),
line: line_num,
column: 1,
rule_id: "no-unbounded-concurrency".to_string(),
category: SlopCategory::AsyncAntiPattern,
severity: SlopSeverity::Warning,
message: "Unthrottled 'asyncio.create_task' spawned inside iteration loop without bounded semaphore or retention handle".to_string(),
snippet: trimmed.to_string(),
fixable: false,
suggestion: Some("Guard task creation with 'asyncio.Semaphore(limit)' or use an 'asyncio.Queue' worker pool".to_string()),
});
continue;
}
}
}
if rs {
let has_join = rust_join_all_re.is_match(trimmed);
let has_map = rust_map_re.is_match(trimmed);
if has_join && has_map && !has_rust_throttle_guard(trimmed) {
findings.push(SlopFinding {
file: file_path.to_string(),
line: line_num,
column: 1,
rule_id: "no-unbounded-concurrency".to_string(),
category: SlopCategory::AsyncAntiPattern,
severity: SlopSeverity::Warning,
message: "Unbounded concurrency detected: 'join_all' over dynamic collection iterator without backpressure".to_string(),
snippet: trimmed.to_string(),
fixable: false,
suggestion: Some("Replace 'join_all' with 'stream::iter(items).map(f).buffer_unordered(CONCURRENCY_LIMIT).collect::<Vec<_>>().await'".to_string()),
});
continue;
}
if has_join && !has_rust_throttle_guard(trimmed) {
for lookahead in 1..=3 {
if let Some(next_line) = lines.get(idx + lookahead) {
let next_trimmed = next_line.trim();
if rust_map_re.is_match(next_trimmed) && !has_rust_throttle_guard(next_trimmed) {
findings.push(SlopFinding {
file: file_path.to_string(),
line: line_num,
column: 1,
rule_id: "no-unbounded-concurrency".to_string(),
category: SlopCategory::AsyncAntiPattern,
severity: SlopSeverity::Warning,
message: "Unbounded concurrency detected: 'join_all' over dynamic collection iterator without backpressure".to_string(),
snippet: format!("{} {}", trimmed, next_trimmed),
fixable: false,
suggestion: Some("Replace 'join_all' with 'stream::iter(items).map(f).buffer_unordered(CONCURRENCY_LIMIT).collect::<Vec<_>>().await'".to_string()),
});
break;
}
if next_trimmed.contains(';') {
break;
}
}
}
}
}
}
findings
}
pub fn detect_unbounded_concurrency_in_diff(diff_files: &[DiffFile]) -> Vec<SlopFinding> {
let mut findings = Vec::new();
let js_gather_re = Regex::new(r#"Promise\.(?:all|allSettled)\s*\("#).unwrap();
let js_map_re = Regex::new(r#"\.map\s*\(\s*(?:async\b|\([^)]*\)\s*=>|[a-zA-Z0-9_$]+\s*=>|function\b)"#).unwrap();
let py_gather_comp_re = Regex::new(r#"asyncio\.gather\s*\(\s*\*\s*\[[^\n]*?for\s+[a-zA-Z0-9_$,\s]+\s+in\s+"#).unwrap();
let py_gather_gen_re = Regex::new(r#"asyncio\.gather\s*\(\s*\*\s*\([^\n]*?for\s+[a-zA-Z0-9_$,\s]+\s+in\s+"#).unwrap();
let py_create_task_re = Regex::new(r#"\b(?:asyncio|loop)\.create_task\s*\("#).unwrap();
let py_loop_re = Regex::new(r#"^\s*(?:for\s+[a-zA-Z0-9_$,\s]+\s+in\s+|while\s+)"#).unwrap();
let rust_join_all_re = Regex::new(r#"(?:futures::future::join_all|future::join_all|\bjoin_all)\s*\("#).unwrap();
let rust_map_re = Regex::new(r#"\.map\s*\("#).unwrap();
for file in diff_files {
if file.is_deleted {
continue;
}
let target_path = if !file.new_path.is_empty() { &file.new_path } else { &file.old_path };
let js = is_js_ts(target_path);
let py = is_python(target_path);
let rs = is_rust(target_path);
if !js && !py && !rs {
continue;
}
for chunk in &file.chunks {
let lines_len = chunk.lines.len();
for (idx, line) in chunk.lines.iter().enumerate() {
if line.line_type != DiffLineType::Add {
continue;
}
let trimmed = line.content.trim();
if trimmed.starts_with("//") || trimmed.starts_with("/*") || trimmed.starts_with('*') {
continue;
}
if py && trimmed.starts_with('#') {
continue;
}
let line_num = line.new_line_number.unwrap_or(1);
if js {
let has_promise = js_gather_re.is_match(trimmed);
let has_map = js_map_re.is_match(trimmed);
if has_promise && has_map && !has_js_throttle_guard(trimmed) {
findings.push(SlopFinding {
file: target_path.to_string(),
line: line_num,
column: 1,
rule_id: "no-unbounded-concurrency".to_string(),
category: SlopCategory::AsyncAntiPattern,
severity: SlopSeverity::Warning,
message: "Unbounded concurrency detected: 'Promise.all/allSettled' over unthrottled '.map()' risks socket exhaustion and rate-limiting cascades".to_string(),
snippet: trimmed.to_string(),
fixable: false,
suggestion: Some("Throttle concurrent tasks using 'p-limit' or batch with chunking: 'const limit = pLimit(concurrency); await Promise.all(items.map(i => limit(() => fn(i))))'".to_string()),
});
continue;
}
if has_promise && !has_js_throttle_guard(trimmed) {
for lookahead in 1..=3 {
if idx + lookahead < lines_len {
let next_dl = &chunk.lines[idx + lookahead];
let next_trimmed = next_dl.content.trim();
if js_map_re.is_match(next_trimmed) && !has_js_throttle_guard(next_trimmed) {
findings.push(SlopFinding {
file: target_path.to_string(),
line: line_num,
column: 1,
rule_id: "no-unbounded-concurrency".to_string(),
category: SlopCategory::AsyncAntiPattern,
severity: SlopSeverity::Warning,
message: "Unbounded concurrency detected: 'Promise.all/allSettled' over unthrottled '.map()' risks socket exhaustion and rate-limiting cascades".to_string(),
snippet: format!("{} {}", trimmed, next_trimmed),
fixable: false,
suggestion: Some("Throttle concurrent tasks using 'p-limit' or batch with chunking: 'const limit = pLimit(concurrency); await Promise.all(items.map(i => limit(() => fn(i))))'".to_string()),
});
break;
}
}
}
}
}
if py {
if (py_gather_comp_re.is_match(trimmed) || py_gather_gen_re.is_match(trimmed)) && !has_py_throttle_guard(trimmed) {
findings.push(SlopFinding {
file: target_path.to_string(),
line: line_num,
column: 1,
rule_id: "no-unbounded-concurrency".to_string(),
category: SlopCategory::AsyncAntiPattern,
severity: SlopSeverity::Warning,
message: "Unbounded concurrency detected: 'asyncio.gather' over unbounded comprehension risks EMFILE file descriptor exhaustion".to_string(),
snippet: trimmed.to_string(),
fixable: false,
suggestion: Some("Throttle concurrent execution using 'asyncio.Semaphore(limit)' or process with 'asyncio.as_completed'".to_string()),
});
continue;
}
if py_create_task_re.is_match(trimmed) && !has_py_throttle_guard(trimmed) {
let mut in_loop = false;
for lookback in (0..idx).rev().take(6) {
let prev_dl = &chunk.lines[lookback];
let prev_trimmed = prev_dl.content.trim();
if py_loop_re.is_match(prev_trimmed) {
in_loop = true;
break;
}
}
if in_loop {
findings.push(SlopFinding {
file: target_path.to_string(),
line: line_num,
column: 1,
rule_id: "no-unbounded-concurrency".to_string(),
category: SlopCategory::AsyncAntiPattern,
severity: SlopSeverity::Warning,
message: "Unthrottled 'asyncio.create_task' spawned inside iteration loop without bounded semaphore or retention handle".to_string(),
snippet: trimmed.to_string(),
fixable: false,
suggestion: Some("Guard task creation with 'asyncio.Semaphore(limit)' or use an 'asyncio.Queue' worker pool".to_string()),
});
continue;
}
}
}
if rs {
let has_join = rust_join_all_re.is_match(trimmed);
let has_map = rust_map_re.is_match(trimmed);
if has_join && has_map && !has_rust_throttle_guard(trimmed) {
findings.push(SlopFinding {
file: target_path.to_string(),
line: line_num,
column: 1,
rule_id: "no-unbounded-concurrency".to_string(),
category: SlopCategory::AsyncAntiPattern,
severity: SlopSeverity::Warning,
message: "Unbounded concurrency detected: 'join_all' over dynamic collection iterator without backpressure".to_string(),
snippet: trimmed.to_string(),
fixable: false,
suggestion: Some("Replace 'join_all' with 'stream::iter(items).map(f).buffer_unordered(CONCURRENCY_LIMIT).collect::<Vec<_>>().await'".to_string()),
});
continue;
}
if has_join && !has_rust_throttle_guard(trimmed) {
for lookahead in 1..=3 {
if idx + lookahead < lines_len {
let next_dl = &chunk.lines[idx + lookahead];
let next_trimmed = next_dl.content.trim();
if rust_map_re.is_match(next_trimmed) && !has_rust_throttle_guard(next_trimmed) {
findings.push(SlopFinding {
file: target_path.to_string(),
line: line_num,
column: 1,
rule_id: "no-unbounded-concurrency".to_string(),
category: SlopCategory::AsyncAntiPattern,
severity: SlopSeverity::Warning,
message: "Unbounded concurrency detected: 'join_all' over dynamic collection iterator without backpressure".to_string(),
snippet: format!("{} {}", trimmed, next_trimmed),
fixable: false,
suggestion: Some("Replace 'join_all' with 'stream::iter(items).map(f).buffer_unordered(CONCURRENCY_LIMIT).collect::<Vec<_>>().await'".to_string()),
});
break;
}
}
}
}
}
}
}
}
findings
}