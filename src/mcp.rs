use crate::config::load_config;
use crate::detectors::{compute_summary, run_all_content_detectors, run_all_diff_detectors};
use crate::fixers::sanitize_file;
use crate::git::{get_diff_output, parse_unified_diff};
use serde_json::{json, Value};
use std::io::{self, BufRead, Write};
use std::path::Path;
use std::time::Instant;
pub fn start_mcp_server() {
    let stdin = io::stdin();
    let mut stdout = io::stdout();
    for line in stdin.lock().lines() {
        if let Ok(line_str) = line {
            let trimmed = line_str.trim();
            if trimmed.is_empty() {
                continue;
            }
            if let Ok(req) = serde_json::from_str::<Value>(trimmed) {
                if let Some(resp) = handle_mcp_message(&req) {
                    let _ = writeln!(stdout, "{}", serde_json::to_string(&resp).unwrap());
                    let _ = stdout.flush();
                }
            }
        }
    }
}
fn handle_mcp_message(msg: &Value) -> Option<Value> {
    let id = msg.get("id");
    let method = msg.get("method").and_then(|v| v.as_str()).unwrap_or("");
    if method == "initialize" {
        return Some(json!({
            "jsonrpc": "2.0",
            "id": id,
            "result": {
                "protocolVersion": "2024-11-05",
                "capabilities": { "tools": {} },
                "serverInfo": { "name": "slopguard", "version": "1.0.0" }
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
        let cwd = std::env::current_dir().unwrap_or_else(|_| Path::new(".").to_path_buf());
        let config = load_config(&cwd);
        if tool_name == "slopguard_scan_diff" {
            let start = Instant::now();
            let staged = args.and_then(|a| a.get("stagedOnly")).and_then(|v| v.as_bool()).unwrap_or(false);
            let diff_text = get_diff_output(&cwd, staged);
            let diff_files = parse_unified_diff(&diff_text);
            let findings = run_all_diff_detectors(&diff_files, &cwd, &config);
            let summary = compute_summary(findings, diff_files.len(), start.elapsed().as_millis());
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
            let findings = run_all_content_detectors(file_path, code, &cwd, &config);
            let summary = compute_summary(findings, 1, start.elapsed().as_millis());
            return Some(json!({
                "jsonrpc": "2.0",
                "id": id,
                "result": {
                    "content": [{ "type": "text", "text": serde_json::to_string_pretty(&summary).unwrap() }]
                }
            }));
        }
        if tool_name == "slopguard_sanitize_file" {
            let file_path = args.and_then(|a| a.get("filePath")).and_then(|v| v.as_str()).unwrap_or("");
            let ok = sanitize_file(Path::new(file_path));
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