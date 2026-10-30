use crate::telemetry::{SharedTelemetry, load_persisted_telemetry};
use std::io::{Read, Write};
use std::net::TcpListener;
use std::path::PathBuf;
use std::process::Command;
use std::thread;
pub fn get_dashboard_html() -> &'static str {
    r##"<!DOCTYPE html>
<html lang="en">
<head>
<meta charset="UTF-8">
<meta name="viewport" content="width=device-width, initial-scale=1.0">
<title>SlopGuard - Live MCP Visual Telemetry</title>
<style>
*{box-sizing:border-box;margin:0;padding:0;font-family:-apple-system,BlinkMacSystemFont,"Segoe UI",Roboto,monospace}
body{background:#090d16;color:#f0f6fc;padding:24px;min-height:100vh}
.header{display:flex;justify-content:space-between;align-items:center;border-bottom:1px solid #1e293b;padding-bottom:16px;margin-bottom:24px}
.brand{display:flex;align-items:center;gap:12px}
.title{font-size:24px;font-weight:900;letter-spacing:-0.5px}
.title span{color:#00f2fe}
.status-pill{display:flex;align-items:center;gap:8px;background:#0d1d2e;border:1px solid #00f2fe44;padding:6px 14px;border-radius:20px;font-size:12px;font-weight:700;color:#00f2fe}
.pulse{width:8px;height:8px;background:#00f2fe;border-radius:50%;box-shadow:0 0 10px #00f2fe;animation:blink 1.5s infinite}
@keyframes blink{0%,100%{opacity:1}50%{opacity:0.3}}
.grid{display:grid;grid-template-columns:repeat(auto-fit,minmax(220px,1fr));gap:16px;margin-bottom:24px}
.card{background:#0f172a;border:1px solid #1e293b;border-radius:12px;padding:20px;position:relative;overflow:hidden}
.card-label{font-size:12px;color:#94a3b8;text-transform:uppercase;font-weight:700;margin-bottom:8px}
.card-value{font-size:36px;font-weight:800;color:#fff}
.card-sub{font-size:12px;color:#64748b;margin-top:4px}
.main-layout{display:grid;grid-template-columns:2fr 1fr;gap:20px}
@media(max-width:900px){.main-layout{grid-template-columns:1fr}}
.panel{background:#0f172a;border:1px solid #1e293b;border-radius:12px;padding:20px}
.panel-title{font-size:16px;font-weight:700;margin-bottom:16px;display:flex;justify-content:space-between;align-items:center}
.feed{display:flex;flex-direction:column;gap:10px;max-height:480px;overflow-y:auto}
.feed-item{background:#131d33;border-left:4px solid #00f2fe;padding:12px;border-radius:6px;display:flex;justify-content:space-between;align-items:center}
.feed-item.error{border-left-color:#e5484d}
.feed-tool{font-weight:700;font-size:13px;color:#38bdf8}
.feed-file{font-size:12px;color:#94a3b8;margin-top:2px}
.feed-badge{font-size:11px;font-weight:700;padding:3px 8px;border-radius:4px;background:#1e293b}
.feed-badge.pass{color:#34d399;background:#064e3b}
.feed-badge.intercept{color:#e5484d;background:#4c0519}
.chart-box{height:180px;width:100%;margin-top:10px}
</style>
</head>
<body>
<div class="header">
<div class="brand">
<svg width="32" height="32" viewBox="0 0 120 120" fill="none"><path d="M60 10 L105 32 C105 85 60 125 60 125 C60 125 15 85 15 32 Z" fill="#0f172a" stroke="#00f2fe" stroke-width="6"/><path d="M45 68 L56 79 L78 52" stroke="#00f2fe" stroke-width="8" stroke-linecap="round"/></svg>
<div class="title">SLOP<span>GUARD</span> <small style="font-size:12px;color:#64748b;font-weight:400">TELEMETRY v1.0</small></div>
</div>
<div class="status-pill"><div class="pulse"></div> MCP PROTOCOL ACTIVE</div>
</div>
<div class="grid">
<div class="card"><div class="card-label">AI Hygiene Score</div><div class="card-value" id="val-score">100<small style="font-size:18px;color:#64748b">/100</small></div><div class="card-sub" id="val-grade">Grade: A+ (Immaculate)</div></div>
<div class="card"><div class="card-label">Slop Neutralized</div><div class="card-value" id="val-blocked" style="color:#e5484d">0</div><div class="card-sub">Stubs & escapes blocked</div></div>
<div class="card"><div class="card-label">MCP Tool Latency</div><div class="card-value" id="val-latency" style="color:#00f2fe">1.8<small style="font-size:18px">ms</small></div><div class="card-sub">Sub-millisecond Rust speed</div></div>
<div class="card"><div class="card-label">Total Code Audits</div><div class="card-value" id="val-scans">0</div><div class="card-sub">Diff & snippet inspections</div></div>
</div>
<div class="main-layout">
<div class="panel">
<div class="panel-title"><span>Live MCP Agent Activity Feed</span><span style="font-size:12px;color:#64748b" id="feed-count">0 calls recorded</span></div>
<div class="feed" id="event-feed"><div style="color:#64748b;text-align:center;padding:40px">Waiting for Cursor / Claude Code MCP tool calls...</div></div>
</div>
<div class="panel">
<div class="panel-title"><span>Agent Telemetry Sparkline</span></div>
<div class="chart-box">
<svg width="100%" height="100%" viewBox="0 0 300 140" fill="none" id="spark-svg">
<path d="M0 120 L50 110 L100 115 L150 70 L200 95 L250 40 L300 60" stroke="#00f2fe" stroke-width="3" fill="none"/>
<path d="M0 120 L50 110 L100 115 L150 70 L200 95 L250 40 L300 60 L300 140 L0 140 Z" fill="#00f2fe" fill-opacity="0.1"/>
<text x="10" y="25" fill="#64748b" font-size="11">REALTIME LATENCY (MS)</text>
</svg>
</div>
<div style="margin-top:20px;border-top:1px solid #1e293b;padding-top:16px;font-size:12px;color:#94a3b8">
<div><strong>Target:</strong> <span id="val-target">Auto-connected Repository</span></div>
<div style="margin-top:4px"><strong>Guardrails:</strong> 9 active AST firewalls</div>
</div>
</div>
</div>
<script>
async function pollTelemetry(){
try{
const res=await fetch("/api/telemetry");
if(!res.ok)return;
const data=await res.json();
document.getElementById("val-score").innerHTML=data.current_hygiene_score+'<small style="font-size:18px;color:#64748b">/100</small>';
document.getElementById("val-grade").textContent=data.current_hygiene_score>=90?"Grade: A+ (Clean)":data.current_hygiene_score>=75?"Grade: B (Moderate)":"Grade: F (High Slop)";
document.getElementById("val-blocked").textContent=data.total_slop_blocked;
document.getElementById("val-latency").innerHTML=data.avg_latency_ms+'<small style="font-size:18px">ms</small>';
document.getElementById("val-scans").textContent=data.total_scans;
document.getElementById("feed-count").textContent=data.events.length+' calls recorded';
if(data.events&&data.events.length>0){
const feed=document.getElementById("event-feed");
feed.innerHTML=data.events.map(e=>`
<div class="feed-item ${e.slop_found>0?'error':''}">
<div>
<div class="feed-tool">${e.client_tool} <span style="font-size:11px;color:#64748b">${e.timestamp} (${e.latency_ms}ms)</span></div>
<div class="feed-file">${e.target_file||'git working tree'} - ${e.message}</div>
</div>
<div class="feed-badge ${e.slop_found>0?'intercept':'pass'}">${e.slop_found>0?'BLOCKED '+e.slop_found+' SLOP':'CLEAN'}</div>
</div>
`).join('');
}
}catch(err){}
}
setInterval(pollTelemetry,800);
pollTelemetry();
</script>
</body>
</html>"##
}
pub fn start_dashboard_server(port: u16, target_root: PathBuf, shared: SharedTelemetry) {
    let addr = format!("127.0.0.1:{port}");
    let listener = match TcpListener::bind(&addr) {
        Ok(l) => l,
        Err(_) => return,
    };
    for stream_res in listener.incoming() {
        if let Ok(mut stream) = stream_res {
            let mut buf = [0u8; 1024];
            let n = stream.read(&mut buf).unwrap_or(0);
            let req = String::from_utf8_lossy(&buf[..n]);
            if req.starts_with("GET /api/telemetry") {
                let state = if let Ok(s) = shared.lock() {
                    s.clone()
                } else {
                    load_persisted_telemetry(&target_root)
                };
                let body = serde_json::to_string(&state).unwrap_or_else(|_| "{}".to_string());
                let resp = format!("HTTP/1.1 200 OK\r\nContent-Type: application/json\r\nAccess-Control-Allow-Origin: *\r\nContent-Length: {}\r\n\r\n{}", body.len(), body);
                let _ = stream.write_all(resp.as_bytes());
            } else {
                let body = get_dashboard_html();
                let resp = format!("HTTP/1.1 200 OK\r\nContent-Type: text/html; charset=utf-8\r\nContent-Length: {}\r\n\r\n{}", body.len(), body);
                let _ = stream.write_all(resp.as_bytes());
            }
        }
    }
}
pub fn spawn_dashboard_thread(port: u16, target_root: PathBuf, shared: SharedTelemetry) {
    thread::spawn(move || {
        start_dashboard_server(port, target_root, shared);
    });
}
pub fn open_browser(url: &str) {
    #[cfg(target_os = "windows")]
    let _ = Command::new("cmd").args(["/c", "start", url]).spawn();
    #[cfg(target_os = "macos")]
    let _ = Command::new("open").arg(url).spawn();
    #[cfg(target_os = "linux")]
    let _ = Command::new("xdg-open").arg(url).spawn();
}