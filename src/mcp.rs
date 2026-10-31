use crate::config::load_config;
use crate::dashboard::spawn_dashboard_thread;
use crate::detectors::{compute_summary, run_all_content_detectors, run_all_diff_detectors};
use crate::fixers::sanitize_file;
use crate::git::{get_diff_output, get_git_root, is_git_repo, parse_unified_diff};
use crate::telemetry::{create_shared_telemetry, record_event, SharedTelemetry};
use serde_json::{json, Value};
use std::io::{self, BufRead, Write};
use std::path::Path;
use std::time::Instant;
pub fn start_mcp_server() {
    let stdin = io::stdin();
    let mut stdout = io::stdout();
    let cwd = std::env::current_dir().unwrap_or_else(|_| Path::new(".").to_path_buf());
    let target_root = if is_git_repo(&cwd) { get_git_root(&cwd) } else { cwd.clone() };
    let shared = create_shared_telemetry();
    spawn_dashboard_thread(4545, target_root.clone(), shared.clone());
    for line in stdin.lock().lines() {
        if let Ok(line_str) = line {
            let trimmed = line_str.trim();
            if trimmed.is_empty() {
                continue;
            }
            if let Ok(req) = serde_json::from_str::<Value>(trimmed) {
                if let Some(resp) = handle_mcp_message(&req, &target_root, &shared) {
                    let _ = writeln!(stdout, "{}", serde_json::to_string(&resp).unwrap());
                    let _ = stdout.flush();
                }
            }
        }
    }
}
fn handle_mcp_message(msg: &Value, target_root: &Path, shared: &SharedTelemetry) -> Option<Value> {
    let id = msg.get("id");
    let method = msg.get("method").and_then(|v| v.as_str()).unwrap_or("");
    if method == "initialize" {
        return Some(json!({
            "jsonrpc": "2.0",
            "id": id,
            "result": {
                "protocolVersion": "2024-11-05",
                "capabilities": { "tools": {} },
                "serverInfo": { "name": "slopguard", "version": env!("CARGO_PKG_VERSION") }
            }
        }));
    }
    if method == "notifications/initialized" {
        return None;
    }
    if method == "ping" {
        return Some(json!({ "jsonrpc": "2.0", "id": id, "result": {} }));
    }
    if method == "tools/list" {
        return Some(json!({
            "jsonrpc": "2.0",
            "id": id,
            "result": {
                "tools": [
                    {
                        "name": "slopguard_scan_diff",
                        "description": "Scan active working tree or staged git diff for AI-generated code slop, type bypasses, dummy tokens, and duplicate utilities",
                        "inputSchema": {
                            "type": "object",
                            "properties": {
                                "stagedOnly": { "type": "boolean", "description": "Only inspect staged git changes" }
                            }
                        }
                    },
                    {
                        "name": "slopguard_check_snippet",
                        "description": "Inspect an in-memory code snippet for AI placeholders, type degradation, and conversational comment bloat before writing it to disk",
                        "inputSchema": {
                            "type": "object",
                            "properties": {
                                "filePath": { "type": "string", "description": "Target relative file path (e.g. src/auth.ts)" },
                                "code": { "type": "string", "description": "Raw code snippet to analyze" }
                            },
                            "required": ["filePath", "code"]
                        }
                    },
                    {
                        "name": "slopguard_sanitize_file",
                        "description": "Automatically remove AI conversational comment bloat and redundant placeholders from a file",
                        "inputSchema": {
                            "type": "object",
                            "properties": {
                                "filePath": { "type": "string", "description": "File path to clean" }
                            },
                            "required": ["filePath"]
                        }
                    }
                ]
            }
        }));
    }
    if method == "tools/call" {
        let params = msg.get("params");
        let tool_name = params.and_then(|p| p.get("name")).and_then(|v| v.as_str()).unwrap_or("");
        let args = params.and_then(|p| p.get("arguments"));
        let config = load_config(target_root);
        if tool_name == "slopguard_scan_diff" {
            let start = Instant::now();
            let staged = args.and_then(|a| a.get("stagedOnly")).and_then(|v| v.as_bool()).unwrap_or(false);
            let diff_text = get_diff_output(target_root, staged);
            let diff_files = parse_unified_diff(&diff_text);
            let findings = run_all_diff_detectors(&diff_files, target_root, &config);
            let elapsed = start.elapsed().as_millis();
            let summary = compute_summary(findings, diff_files.len(), elapsed);
            let msg = if summary.errors > 0 { format!("Intercepted {} critical issues", summary.errors) } else { "Clean working tree".to_string() };
            record_event(shared, target_root, "slopguard_scan_diff", "git diff", summary.errors, summary.score, elapsed, &msg);
            return Some(json!({
                "jsonrpc": "2.0",
                "id": id,
                "result": {
                    "content": [{ "type": "text", "text": serde_json::to_string_pretty(&summary).unwrap() }]
                }
            }));
        }
        if tool_name == "slopguard_check_snippet" {
            let start = Instant::now();
            let file_path = args.and_then(|a| a.get("filePath")).and_then(|v| v.as_str()).unwrap_or("snippet.ts");
            let code = args.and_then(|a| a.get("code")).and_then(|v| v.as_str()).unwrap_or("");
            let findings = run_all_content_detectors(file_path, code, target_root, &config);
            let elapsed = start.elapsed().as_millis();
            let summary = compute_summary(findings, 1, elapsed);
            let msg = if summary.errors > 0 { format!("Blocked {} code slop issues", summary.errors) } else { "Verified clean snippet".to_string() };
            record_event(shared, target_root, "slopguard_check_snippet", file_path, summary.errors, summary.score, elapsed, &msg);
            return Some(json!({
                "jsonrpc": "2.0",
                "id": id,
                "result": {
                    "content": [{ "type": "text", "text": serde_json::to_string_pretty(&summary).unwrap() }]
                }
            }));
        }
        if tool_name == "slopguard_sanitize_file" {
            let start = Instant::now();
            let file_path = args.and_then(|a| a.get("filePath")).and_then(|v| v.as_str()).unwrap_or("");
            let ok = sanitize_file(Path::new(file_path));
            let elapsed = start.elapsed().as_millis();
            record_event(shared, target_root, "slopguard_sanitize_file", file_path, 0, 100, elapsed, if ok { "Sanitized AI bloat" } else { "File unmodified" });
            return Some(json!({
                "jsonrpc": "2.0",
                "id": id,
                "result": {
                    "content": [{ "type": "text", "text": json!({ "success": ok, "filePath": file_path }).to_string() }]
                }
            }));
        }
        return Some(json!({ "jsonrpc": "2.0", "id": id, "error": { "code": -32601, "message": format!("Unknown tool '{tool_name}'") } }));
    }
    id.map(|id_val| json!({ "jsonrpc": "2.0", "id": id_val, "error": { "code": -32601, "message": "Method not found" } }))
}