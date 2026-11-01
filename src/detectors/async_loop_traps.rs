use crate::types::{DiffFile, DiffLineType, SlopCategory, SlopFinding, SlopSeverity};
use regex::Regex;
fn check_file_type(path: &str) -> (bool, bool, bool) {
let l = path.to_lowercase();
(l.ends_with(".ts") || l.ends_with(".tsx") || l.ends_with(".js") || l.ends_with(".jsx") || l.ends_with(".mjs") || l.ends_with(".cjs"), l.ends_with(".py"), l.ends_with(".rs"))
}
fn get_js_loop_regex() -> Regex {
Regex::new(r#"^\s*(?:for\s*\(|while\s*\(|for\s+await\s*\()"#).unwrap()
}
fn get_js_unawaited_async_regex() -> Regex {
Regex::new(r#"(?:^|[^\w$.])(?:fetch\s*\(|axios\.(?:get|post|put|delete|patch|request)\s*\(|http\.(?:get|request)\s*\(|https\.(?:get|request)\s*\(|db\.(?:query|execute|save|find)\s*\(|prisma\.[a-zA-Z0-9_]+\.(?:find|create|update|delete|upsert)\s*\(|fs\.promises\.[a-zA-Z0-9_]+\s*\(|sendEmail\s*\(|dispatchTask\s*\()"#).unwrap()
}
fn get_py_loop_regex() -> Regex {
Regex::new(r#"^\s*(?:for\s+[a-zA-Z0-9_,\s]+\s+in\s+|while\s+)"#).unwrap()
}
fn get_py_unawaited_async_regex() -> Regex {
Regex::new(r#"(?:^|[^\w$.])(?:aiohttp\.[a-zA-Z0-9_.]+\s*\(|asyncio\.sleep\s*\(|asyncio\.create_task\s*\(|client\.(?:get|post|put|delete|request)\s*\(|session\.(?:get|post|put|delete|request)\s*\(|cursor\.(?:execute|fetchall|fetchone)\s*\()"#).unwrap()
}
fn get_rust_loop_regex() -> Regex {
Regex::new(r#"^\s*(?:for\s+[a-zA-Z0-9_,\s]+\s+in\s+|while\s+|loop\s*\{)"#).unwrap()
}
fn get_rust_unawaited_async_regex() -> Regex {
Regex::new(r#"(?:tokio::spawn\s*\(|async\s+move\s*\{)"#).unwrap()
}
fn has_js_sync_guard(line: &str) -> bool {
line.contains("await ") || line.contains(".then") || line.contains(".push(") || line.contains("Promise.all") || line.contains("pLimit") || line.contains("p-limit") || line.contains("yield ") || line.contains("return ")
}
fn has_py_sync_guard(line: &str) -> bool {
line.contains("await ") || line.contains(".append(") || line.contains(".add(") || line.contains("asyncio.gather") || line.contains("Semaphore") || line.contains("yield ") || line.contains("return ")
}
fn has_rust_sync_guard(line: &str) -> bool {
line.contains(".await") || line.contains(".push(") || line.contains("join_all") || line.contains("Semaphore") || line.contains("join_set") || line.contains("JoinSet")
}
#[allow(dead_code)]
pub const CONTRACT: crate::types::AstDetectorContract = crate::types::AstDetectorContract {
rule_id: "no-unawaited-async-in-loops",
severity: SlopSeverity::Error,
category: SlopCategory::UnawaitedAsyncLoopTrap,
target_languages: &["js", "ts", "jsx", "tsx", "py", "rs"],
analyze_diff: detect_async_loop_traps_in_diff,
analyze_content: detect_async_loop_traps_in_content,
};
pub fn detect_async_loop_traps_in_content(file_path: &str, content: &str) -> Vec<SlopFinding> {
let mut findings = Vec::new();
let (js, py, rs) = check_file_type(file_path);
if !js && !py && !rs {
return findings;
}
let lines: Vec<&str> = content.lines().collect();
let js_loop_re = get_js_loop_regex();
let js_async_re = get_js_unawaited_async_regex();
let foreach_async_re = Regex::new(r#"\.forEach\s*\(\s*async\s*(?:\([^)]*\)|[a-zA-Z0-9_$]+|\bfunction\b)"#).unwrap();
let map_async_re = Regex::new(r#"\.map\s*\(\s*async\s*(?:\([^)]*\)|[a-zA-Z0-9_$]+|\bfunction\b)"#).unwrap();
let py_loop_re = get_py_loop_regex();
let py_async_re = get_py_unawaited_async_regex();
let rs_loop_re = get_rust_loop_regex();
let rs_async_re = get_rust_unawaited_async_regex();
for (idx, line) in lines.iter().enumerate() {
let line_num = idx + 1;
let trimmed = line.trim();
if trimmed.starts_with("//") || trimmed.starts_with("/*") || trimmed.starts_with('*') {
continue;
}
if py && trimmed.starts_with('#') {
continue;
}
if js && foreach_async_re.is_match(trimmed) {
findings.push(SlopFinding {
file: file_path.to_string(),
line: line_num,
column: 1,
rule_id: "no-unawaited-async-in-loops".to_string(),
category: SlopCategory::AsyncIterationHazard,
severity: SlopSeverity::Error,
message: "Synchronous forEach with async closure does not await promises, causing unhandled errors and detached execution".to_string(),
snippet: trimmed.to_string(),
fixable: false,
suggestion: Some("Use 'for (const item of items)' with await or 'await Promise.all(items.map(...))'".to_string()),
});
continue;
}
if js && map_async_re.is_match(trimmed) && !trimmed.contains("Promise.all") && !trimmed.contains("Promise.allSettled") {
findings.push(SlopFinding {
file: file_path.to_string(),
line: line_num,
column: 1,
rule_id: "no-unawaited-async-in-loops".to_string(),
category: SlopCategory::AsyncIterationHazard,
severity: SlopSeverity::Error,
message: "Array.prototype.map with async callback without Promise.all creates floating unawaited promises (CWE-835)".to_string(),
snippet: trimmed.to_string(),
fixable: false,
suggestion: Some("Wrap with 'await Promise.all(items.map(...))' or use sequential 'for...of'".to_string()),
});
continue;
}
if (trimmed.starts_with("yield ") || trimmed.starts_with("yield\t")) && (trimmed.contains("fetch(") || trimmed.contains("fetch (") || trimmed.contains("axios.") || trimmed.contains("db.query(") || trimmed.contains("client.get(")) && !trimmed.contains("await ") && !trimmed.contains("yield*") {
findings.push(SlopFinding {
file: file_path.to_string(),
line: line_num,
column: 1,
rule_id: "no-unawaited-async-in-loops".to_string(),
category: SlopCategory::AsyncIterationHazard,
severity: SlopSeverity::Error,
message: "Async generator yielding raw unawaited promise or coroutine without await or yield*".to_string(),
snippet: trimmed.to_string(),
fixable: false,
suggestion: Some("Await the promise before yielding or delegate with 'yield*'".to_string()),
});
continue;
}
if js && js_loop_re.is_match(trimmed) {
if (trimmed.contains("fetch(") || trimmed.contains("fetch (") || trimmed.contains("axios.")) && (trimmed.contains("await ") || trimmed.contains("await\t")) && !trimmed.contains("signal:") && !trimmed.contains("signal") {
findings.push(SlopFinding {
file: file_path.to_string(),
line: line_num,
column: 1,
rule_id: "no-unawaited-async-in-loops".to_string(),
category: SlopCategory::AsyncIterationHazard,
severity: SlopSeverity::Error,
message: "Loop fetch lacking AbortController cancellation signal risks socket exhaustion during aborted iterations".to_string(),
snippet: trimmed.to_string(),
fixable: false,
suggestion: Some("Pass an AbortSignal ('{ signal: controller.signal }') to cancel pending requests on loop exit".to_string()),
});
continue;
}
if js_async_re.is_match(trimmed) && !has_js_sync_guard(trimmed) {
findings.push(SlopFinding {
file: file_path.to_string(),
line: line_num,
column: 1,
rule_id: "no-unawaited-async-in-loops".to_string(),
category: SlopCategory::AsyncIterationHazard,
severity: SlopSeverity::Error,
message: "Unawaited asynchronous operation directly inside loop causes unbounded concurrent micro-tasks".to_string(),
snippet: trimmed.to_string(),
fixable: false,
suggestion: Some("Await the async operation synchronously or accumulate tasks into a bounded pool via 'p-limit'".to_string()),
});
continue;
}
for offset in 1..=12 {
if let Some(body_line) = lines.get(idx + offset) {
let body_trimmed = body_line.trim();
if body_trimmed.starts_with("//") || body_trimmed.starts_with("/*") || body_trimmed.starts_with('*') {
continue;
}
if (body_trimmed.contains("fetch(") || body_trimmed.contains("fetch (") || body_trimmed.contains("axios.")) && (body_trimmed.contains("await ") || body_trimmed.contains("await\t")) && !body_trimmed.contains("signal:") && !body_trimmed.contains("signal") {
findings.push(SlopFinding {
file: file_path.to_string(),
line: idx + offset + 1,
column: 1,
rule_id: "no-unawaited-async-in-loops".to_string(),
category: SlopCategory::AsyncIterationHazard,
severity: SlopSeverity::Error,
message: "Loop fetch lacking AbortController cancellation signal risks socket exhaustion during aborted iterations".to_string(),
snippet: body_trimmed.to_string(),
fixable: false,
suggestion: Some("Pass an AbortSignal ('{ signal: controller.signal }') to cancel pending requests on loop exit".to_string()),
});
break;
}
if js_async_re.is_match(body_trimmed) && !has_js_sync_guard(body_trimmed) {
findings.push(SlopFinding {
file: file_path.to_string(),
line: idx + offset + 1,
column: 1,
rule_id: "no-unawaited-async-in-loops".to_string(),
category: SlopCategory::AsyncIterationHazard,
severity: SlopSeverity::Error,
message: "Unawaited asynchronous call inside loop body drops promises and risks socket starvation".to_string(),
snippet: body_trimmed.to_string(),
fixable: false,
suggestion: Some("Refactor to 'await' the promise or throttle using 'p-limit' or batch processing".to_string()),
});
break;
}
if body_trimmed == "}" || body_trimmed.starts_with("};") {
break;
}
}
}
}
if py && py_loop_re.is_match(trimmed) {
if py_async_re.is_match(trimmed) && !has_py_sync_guard(trimmed) {
findings.push(SlopFinding {
file: file_path.to_string(),
line: line_num,
column: 1,
rule_id: "no-unawaited-async-in-loops".to_string(),
category: SlopCategory::AsyncIterationHazard,
severity: SlopSeverity::Error,
message: "Unawaited asynchronous coroutine inside Python loop creates floating tasks".to_string(),
snippet: trimmed.to_string(),
fixable: false,
suggestion: Some("Await coroutine with 'await' or accumulate into 'asyncio.Semaphore' pool".to_string()),
});
continue;
}
let base_indent = line.len() - line.trim_start().len();
let mut py_guarded = false;
for off in 1..=12 {
if let Some(bl) = lines.get(idx + off) {
let bt = bl.trim();
if bt.is_empty() || bt.starts_with('#') {
continue;
}
let ci = bl.len() - bl.trim_start().len();
if ci <= base_indent {
break;
}
if has_py_sync_guard(bt) {
py_guarded = true;
break;
}
}
}
if py_guarded {
continue;
}
for offset in 1..=12 {
if let Some(body_line) = lines.get(idx + offset) {
let body_trimmed = body_line.trim();
if body_trimmed.is_empty() || body_trimmed.starts_with('#') {
continue;
}
let current_indent = body_line.len() - body_line.trim_start().len();
if current_indent <= base_indent {
break;
}
if py_async_re.is_match(body_trimmed) && !has_py_sync_guard(body_trimmed) {
findings.push(SlopFinding {
file: file_path.to_string(),
line: idx + offset + 1,
column: 1,
rule_id: "no-unawaited-async-in-loops".to_string(),
category: SlopCategory::AsyncIterationHazard,
severity: SlopSeverity::Error,
message: "Unawaited coroutine invocation in Python loop body leads to unhandled runtime warnings".to_string(),
snippet: body_trimmed.to_string(),
fixable: false,
suggestion: Some("Use 'await' or manage concurrent tasks with 'asyncio.as_completed'".to_string()),
});
break;
}
}
}
}
if rs {
if (trimmed.contains("for ") || trimmed.contains("while ") || trimmed.contains("loop ")) && trimmed.contains("tokio::spawn") && !has_rust_sync_guard(trimmed) {
findings.push(SlopFinding {
file: file_path.to_string(),
line: line_num,
column: 1,
rule_id: "no-unawaited-async-in-loops".to_string(),
category: SlopCategory::AsyncIterationHazard,
severity: SlopSeverity::Error,
message: "Unawaited tokio::spawn in loop without JoinSet".to_string(),
snippet: trimmed.to_string(),
fixable: false,
suggestion: Some("Collect JoinHandle instances into JoinSet and await them".to_string()),
});
continue;
}
if rs_loop_re.is_match(trimmed) {
let mut rs_guarded = false;
for off in 1..=12 {
if let Some(bl) = lines.get(idx + off) {
let bt = bl.trim();
if bt.contains(".push(") || bt.contains("join_set") || bt.contains("JoinSet") || bt.contains("join_all") || bt.contains("Semaphore") || bt.contains("handles.") {
rs_guarded = true;
break;
}
if bt == "}" || bt.starts_with("};") {
break;
}
}
}
if rs_guarded {
continue;
}
for offset in 1..=12 {
if let Some(body_line) = lines.get(idx + offset) {
let body_trimmed = body_line.trim();
if body_trimmed.starts_with("//") || body_trimmed.starts_with("/*") || body_trimmed.starts_with('*') {
continue;
}
if rs_async_re.is_match(body_trimmed) && !has_rust_sync_guard(body_trimmed) {
findings.push(SlopFinding {
file: file_path.to_string(),
line: idx + offset + 1,
column: 1,
rule_id: "no-unawaited-async-in-loops".to_string(),
category: SlopCategory::AsyncIterationHazard,
severity: SlopSeverity::Error,
message: "Spawning unbuffered tasks inside loop without bounded JoinSet or Semaphore causes thread pool saturation".to_string(),
snippet: body_trimmed.to_string(),
fixable: false,
suggestion: Some("Throttle task dispatch using 'tokio::sync::Semaphore' or use 'futures::stream::buffer_unordered'".to_string()),
});
break;
}
if body_trimmed == "}" || body_trimmed.starts_with("};") {
break;
}
}
}
}
}
}
findings
}
pub fn detect_async_loop_traps_in_diff(diff_files: &[DiffFile]) -> Vec<SlopFinding> {
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
findings.extend(detect_async_loop_traps_in_content(target_path, &content));
}
findings
}