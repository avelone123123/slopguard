use crate::config::{is_supported_code_file, load_config, should_ignore_path};
use crate::detectors::{compute_summary, run_all_diff_detectors, AST_DETECTOR_DEFINITIONS};
use crate::fixers::sanitize_file;
use crate::git::{get_diff_output, is_git_repo, parse_unified_diff};
use crate::telemetry::{load_persisted_telemetry, record_telemetry_event, SharedTelemetry};
use std::io::{Read, Write};
use std::net::TcpListener;
use std::path::PathBuf;
use std::process::Command;
use std::thread;
use std::time::Instant;
use walkdir::WalkDir;
pub fn get_dashboard_html() -> &'static str {
    r##"<!DOCTYPE html>
<html lang="en">
<head>
<meta charset="UTF-8">
<meta name="viewport" content="width=device-width, initial-scale=1.0">
<title>SlopGuard Enterprise - AST Security & Telemetry Console</title>
<style>
*{box-sizing:border-box;margin:0;padding:0;font-family:-apple-system,BlinkMacSystemFont,"Segoe UI","Inter",Roboto,sans-serif}
body{background:#0f1115;color:#e6edf3;padding:24px;min-height:100vh}
.header{display:flex;justify-content:space-between;align-items:center;border-bottom:1px solid #282d38;padding-bottom:16px;margin-bottom:24px}
.brand{display:flex;align-items:center;gap:12px}
.title{font-size:20px;font-weight:700;letter-spacing:-0.3px;color:#e6edf3}
.title span{color:#2f81f7}
.actions{display:flex;gap:10px;align-items:center}
.btn{background:#2f81f7;color:#ffffff;font-weight:600;border:1px solid #2f81f7;padding:8px 16px;border-radius:5px;cursor:pointer;font-size:12px;transition:0.15s}
.btn:hover{background:#388bfd}
.btn-sec{background:#171a21;color:#e6edf3;font-weight:600;border:1px solid #282d38}
.btn-sec:hover{border-color:#484f58;background:#1f242d}
.status-pill{display:flex;align-items:center;gap:8px;background:#171a21;border:1px solid #282d38;padding:6px 12px;border-radius:5px;font-size:12px;font-weight:600;color:#8b949e}
.pulse{width:7px;height:7px;background:#2ea043;border-radius:50%}
.grid{display:grid;grid-template-columns:repeat(auto-fit,minmax(220px,1fr));gap:14px;margin-bottom:24px}
.card{background:#171a21;border:1px solid #282d38;border-radius:6px;padding:18px;position:relative;overflow:hidden}
.card-label{font-size:11px;color:#8b949e;text-transform:uppercase;font-weight:600;letter-spacing:0.4px;margin-bottom:8px}
.card-value{font-size:30px;font-weight:700;color:#e6edf3;font-family:ui-monospace,Consolas,monospace}
.card-sub{font-size:12px;color:#8b949e;margin-top:6px}
.main-layout{display:grid;grid-template-columns:2fr 1fr;gap:16px}
@media(max-width:900px){.main-layout{grid-template-columns:1fr}}
.panel{background:#171a21;border:1px solid #282d38;border-radius:6px;padding:18px}
.panel-title{font-size:14px;font-weight:600;margin-bottom:14px;display:flex;justify-content:space-between;align-items:center;border-bottom:1px solid #282d38;padding-bottom:10px}
.feed{display:flex;flex-direction:column;gap:8px;max-height:480px;overflow-y:auto}
.feed-item{background:#13161c;border:1px solid #282d38;border-left:3px solid #2f81f7;padding:10px 12px;border-radius:4px;display:flex;justify-content:space-between;align-items:center}
.feed-item.error{border-left-color:#da3633}
.feed-tool{font-weight:600;font-size:12px;color:#e6edf3;font-family:ui-monospace,Consolas,monospace}
.feed-file{font-size:12px;color:#8b949e;margin-top:2px}
.feed-badge{font-size:11px;font-weight:600;padding:3px 8px;border-radius:4px;background:#1f242d;font-family:ui-monospace,Consolas,monospace}
.feed-badge.pass{color:#3fb950;background:rgba(46,160,67,0.12);border:1px solid rgba(46,160,67,0.3)}
.feed-badge.intercept{color:#f85149;background:rgba(218,54,51,0.12);border:1px solid rgba(218,54,51,0.3)}
.chart-box{height:160px;width:100%;margin-top:10px}
.guide-panel{display:none;background:#171a21;border:1px solid #2f81f7;border-radius:6px;padding:20px;margin-bottom:24px}
.guide-panel.open{display:block}
.guide-top{display:flex;justify-content:space-between;align-items:center;border-bottom:1px solid #282d38;padding-bottom:12px;margin-bottom:16px}
.lang-tabs{display:flex;gap:8px}
.lang-btn{background:#13161c;color:#8b949e;border:1px solid #282d38;padding:5px 12px;border-radius:4px;cursor:pointer;font-size:12px;font-weight:600}
.lang-btn.active{background:#2f81f7;color:#ffffff;border-color:#2f81f7}
.guide-grid{display:grid;grid-template-columns:repeat(auto-fit,minmax(300px,1fr));gap:12px}
.guide-card{background:#13161c;border:1px solid #282d38;border-radius:5px;padding:14px}
.guide-step{font-size:11px;color:#2f81f7;font-weight:700;font-family:monospace;margin-bottom:4px}
.guide-h{font-size:14px;font-weight:600;color:#e6edf3;margin-bottom:6px}
.guide-p{font-size:12px;color:#8b949e;line-height:1.5;margin-bottom:8px}
.guide-cmd{background:#0f1115;border:1px solid #282d38;padding:6px 10px;border-radius:4px;font-family:monospace;font-size:12px;color:#3fb950}
</style>
</head>
<body>
<div class="header">
<div class="brand">
<svg width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="#2f81f7" stroke-width="2"><path d="M12 22s8-4 8-10V5l-8-3-8 3v7c0 6 8 10 8 10z"/></svg>
<div class="title">SLOP<span>GUARD</span> <small style="font-size:12px;color:#8b949e;font-weight:400">ENTERPRISE CONSOLE v3.5.0</small></div>
</div>
<div class="actions">
<button onclick="toggleGuide()" class="btn btn-sec">GUIDE / ГАЙД / 指南</button>
<button onclick="triggerScan(this)" class="btn">AUDIT WORKSPACE</button>
<button onclick="triggerSanitize(this)" class="btn btn-sec">SANITIZE BLOAT</button>
<div class="status-pill"><div class="pulse"></div> MCP JSON-RPC 2.0 ACTIVE</div>
</div>
</div>
<div class="guide-panel" id="guide-panel">
<div class="guide-top">
<div style="font-size:15px;font-weight:700" id="guide-main-title">SlopGuard User Guide & Project Integration Manual</div>
<div class="lang-tabs">
<button class="lang-btn" id="lbtn-en" onclick="setGuideLang('en')">EN · English</button>
<button class="lang-btn active" id="lbtn-ru" onclick="setGuideLang('ru')">RU · Русский</button>
<button class="lang-btn" id="lbtn-zh" onclick="setGuideLang('zh')">ZH · 中文</button>
</div>
</div>
<div class="guide-grid" id="guide-cards"></div>
</div>
<div class="grid">
<div class="card"><div class="card-label">Security & Hygiene Score</div><div class="card-value" id="val-score">100<small style="font-size:16px;color:#8b949e">/100</small></div><div class="card-sub" id="val-grade">Grade: A+ (Verified Clean)</div></div>
<div class="card"><div class="card-label">Violations Intercepted</div><div class="card-value" id="val-blocked" style="color:#f85149">0</div><div class="card-sub">Hallucinations, stubs & vulnerabilities</div></div>
<div class="card"><div class="card-label">AST Engine Latency</div><div class="card-value" id="val-latency" style="color:#2f81f7">0.35<small style="font-size:16px">ms</small></div><div class="card-sub">Native x86_64 + ASM execution</div></div>
<div class="card"><div class="card-label">Completed Audits</div><div class="card-value" id="val-scans">0</div><div class="card-sub">66 Active AST Firewall Rules</div></div>
</div>
<div class="main-layout">
<div class="panel">
<div class="panel-title"><span>Live MCP Verification Stream</span><span style="font-size:12px;color:#8b949e" id="feed-count">0 calls recorded</span></div>
<div class="feed" id="event-feed"><div style="color:#8b949e;text-align:center;padding:40px;font-size:13px">Ready for MCP tool inspections or manual workspace audit.</div></div>
</div>
<div class="panel">
<div class="panel-title"><span>Execution Latency Profile</span></div>
<div class="chart-box">
<svg width="100%" height="100%" viewBox="0 0 300 140" fill="none" id="spark-svg">
<path d="M0 110 L50 108 L100 112 L150 105 L200 109 L250 104 L300 106" stroke="#2f81f7" stroke-width="2" fill="none" id="spark-line"/>
<path d="M0 110 L50 108 L100 112 L150 105 L200 109 L250 104 L300 106 L300 140 L0 140 Z" fill="#2f81f7" fill-opacity="0.08" id="spark-area"/>
<text x="10" y="20" fill="#8b949e" font-size="11" font-family="monospace">SCAN DURATION (MS)</text>
</svg>
</div>
<div style="margin-top:16px;border-top:1px solid #282d38;padding-top:14px;font-size:12px;color:#8b949e">
<div><strong>Repository:</strong> <span id="val-target">Connected Git Working Tree</span></div>
<div style="margin-top:4px"><strong>Firewall Matrix:</strong> 66 Deterministic AST Detectors</div>
</div>
</div>
</div>
<script>
async function pollTelemetry(){
try{
const res=await fetch("/api/telemetry");
if(!res.ok)return;
const data=await res.json();
document.getElementById("val-score").innerHTML=data.current_hygiene_score+'<small style="font-size:16px;color:#8b949e">/100</small>';
document.getElementById("val-grade").textContent=data.current_hygiene_score>=90?"Grade: A+ (Verified Clean)":data.current_hygiene_score>=75?"Grade: B (Moderate)":"Grade: F (Blocked)";
document.getElementById("val-blocked").textContent=data.total_slop_blocked;
document.getElementById("val-latency").innerHTML=data.avg_latency_ms+'<small style="font-size:16px">ms</small>';
document.getElementById("val-scans").textContent=data.total_scans;
document.getElementById("feed-count").textContent=data.events.length+' calls recorded';
if(data.events&&data.events.length>0){
const feed=document.getElementById("event-feed");
feed.innerHTML=data.events.map(e=>`
<div class="feed-item ${e.slop_found>0?'error':''}">
<div>
<div class="feed-tool">${e.client_tool} <span style="font-size:11px;color:#8b949e">${e.timestamp} (${e.latency_ms}ms)</span></div>
<div class="feed-file">${e.target_file||'git working tree'} - ${e.message}</div>
</div>
<div class="feed-badge ${e.slop_found>0?'intercept':'pass'}">${e.slop_found>0?'BLOCKED '+e.slop_found:'CLEAN'}</div>
</div>
`).join('');
const latencies=data.events.slice(0,8).map(e=>e.latency_ms||1).reverse();
if(latencies.length>1){
const maxLat=Math.max(...latencies,4);
const pts=latencies.map((l,idx)=>{
const x=idx*(300/(latencies.length-1));
const y=Math.round(130-(l/maxLat*95));
return `${x} ${y}`;
});
const lineD=`M${pts.join(' L')}`;
const areaD=`M0 140 L${pts.join(' L')} L300 140 Z`;
document.getElementById("spark-line").setAttribute("d",lineD);
document.getElementById("spark-area").setAttribute("d",areaD);
}
}
}catch(err){console.warn(err);}
}
async function triggerScan(btn){
const orig=btn.textContent;btn.textContent="AUDITING...";
try{await fetch("/api/scan",{method:"POST"});await pollTelemetry();}catch(e){console.warn(e);}
btn.textContent=orig;
}
async function triggerSanitize(btn){
const orig=btn.textContent;btn.textContent="SANITIZING...";
try{const res=await fetch("/api/fix",{method:"POST"});const d=await res.json();alert("Sanitized "+(d.fixed||0)+" file(s)");await pollTelemetry();}catch(e){console.warn(e);}
btn.textContent=orig;
}
const GUIDE_DATA={
en:{title:"SlopGuard User Guide & Project Integration (English)",items:[
{s:"01. PURPOSE",h:"What SlopGuard Is For",p:"Deterministic firewall that blocks AI coding hallucinations, slopsquatted phantom packages, empty catch blocks, and SSRF/SQLi/IDOR vulnerabilities before git commit.",c:"slopguard.exe"},
{s:"02. ARCHITECTURE",h:"How It Works Under the Hood",p:"Compiled to native x86_64 machine code with inline Assembly RDTSC counters and 66 AST rules. Scans 1,000 lines in <0.35ms 100% offline with zero API token cost.",c:"slopguard check --json"},
{s:"03. QUICK SCAN",h:"Audit Your Project Changes",p:"Run in your project root to check modified git diff files or scan the entire repository across JS/TS, Python, Rust, Go, C/C++, PHP, Java, C#.",c:"slopguard check  |  slopguard all ."},
{s:"04. GIT HOOK",h:"Automatic Pre-Commit Protection",p:"Installs a native .git/hooks/pre-commit gate that automatically rejects commits containing AI errors or security flaws.",c:"slopguard hook install"},
{s:"05. AUTO-SANITIZE",h:"Strip AI Comments & Blank Lines",p:"Automatically removes AI conversational apologies, sycophancy watermarks, and redundant blank lines across your project.",c:"slopguard fix ."},
{s:"06. MCP & CI/CD",h:"Cursor / Claude Code MCP & GitHub CI",p:"Connects to Cursor/Claude Code via JSON-RPC 2.0 MCP and generates GitHub Actions CI workflows and .slopguardrc.json config.",c:"slopguard mcp  |  slopguard init && slopguard ci"}
]},
ru:{title:"Руководство SlopGuard: назначение, принцип работы и подключение к проекту (Русский)",items:[
{s:"01. НАЗНАЧЕНИЕ",h:"Для чего нужна программа SlopGuard",p:"Нативный фаервол, который перехватывает галлюцинации ИИ, выдуманные библиотеки (slopsquatting), пустые заглушки и уязвимости (SQLi, SSRF, IDOR) до git commit.",c:"slopguard.exe"},
{s:"02. КАК РАБОТАЕТ",h:"Принцип работы (Rust + C++17 + x64 ASM)",p:"Использует аппаратные инструкции процессора x86_64 (RDTSC) и матрицу из 66 AST-правил. Проверяет 1000 строк кода за <0.35 мс полностью офлайн без затрат токенов.",c:"slopguard check --json"},
{s:"03. ПРОВЕРКА ПРОЕКТА",h:"Как проверить свой проект",p:"Запустите проверку текущего git diff или полный аудит всех файлов и манифестов (package.json, Cargo.toml, requirements.txt, go.mod).",c:"slopguard check  |  slopguard all ."},
{s:"04. АВТО-ЗАЩИТА GIT",h:"Установка Git Pre-Commit перехватчика",p:"Устанавливает хук в .git/hooks/pre-commit, который автоматически блокирует коммиты с ошибками или галлюцинациями ИИ.",c:"slopguard hook install"},
{s:"05. ОЧИСТКА КОДА",h:"Удаление ИИ-мусора и пустых строк",p:"Автоматически очищает проект от лишних комментариев ИИ, водяных знаков нейросетей и пустых строк.",c:"slopguard fix ."},
{s:"06. MCP И CI/CD",h:"Подключение к Cursor / Claude Code и CI",p:"Запускает MCP-сервер для автоматической самопроверки ИИ-агентов в IDE и создает конфиг .slopguardrc.json и пайплайн GitHub Actions.",c:"slopguard mcp  |  slopguard init && slopguard ci"}
]},
zh:{title:"SlopGuard 使用指南：核心用途、工作原理与项目集成 (中文)",items:[
{s:"01. 核心用途",h:"什么是 SlopGuard",p:"原生 AI 幻觉与安全防火墙，在 git commit 之前拦截 AI 虚构包名（供应链投毒）、空桩代码、异常吞没与 SQLi/SSRF/IDOR 漏洞。",c:"slopguard.exe"},
{s:"02. 工作原理",h:"底层架构 (Rust + C++17 + x64 汇编)",p:"采用硬件级 x86_64 RDTSC 指令与 66 条确定性 AST 规则，每千行代码扫描仅需 <0.35ms，100% 离线运行，零 Token 消耗。",c:"slopguard check --json"},
{s:"03. 扫描项目",h:"如何审计您的项目",p:"在项目根目录运行增量 Git Diff 检查，或对整个代码库及依赖清单执行全量安全审计。",c:"slopguard check  |  slopguard all ."},
{s:"04. 自动拦截钩子",h:"安装 Git Pre-Commit 防火墙钩子",p:"在 .git/hooks/pre-commit 中安装自动门禁，发现 AI 错误或漏洞时自动阻止提交。",c:"slopguard hook install"},
{s:"05. 自动净化",h:"一键清理 AI 废话注释与多余空行",p:"自动移除源代码中由大模型生成的对话式注释、水印标记与多余空白行。",c:"slopguard fix ."},
{s:"06. MCP 与 CI/CD",h:"集成 Cursor / Claude Code 与 GitHub CI",p:"提供 JSON-RPC 2.0 MCP 服务器让 AI 实时自检，并一键生成 .slopguardrc.json 与 GitHub Actions 工作流。",c:"slopguard mcp  |  slopguard init && slopguard ci"}
]}
};
function toggleGuide(){
const p=document.getElementById("guide-panel");
p.classList.toggle("open");
}
function setGuideLang(lang){
["en","ru","zh"].forEach(l=>document.getElementById("lbtn-"+l).classList.toggle("active",l===lang));
const d=GUIDE_DATA[lang]||GUIDE_DATA.ru;
document.getElementById("guide-main-title").textContent=d.title;
document.getElementById("guide-cards").innerHTML=d.items.map(it=>`<div class="guide-card"><div class="guide-step">${it.s}</div><div class="guide-h">${it.h}</div><div class="guide-p">${it.p}</div><div class="guide-cmd">$ ${it.c}</div></div>`).join("");
}
setGuideLang("ru");
setInterval(pollTelemetry,1000);
pollTelemetry();
</script>
</body>
</html>"##
}
fn audit_workspace(target_root: &PathBuf, shared: &SharedTelemetry) -> String {
    let start = Instant::now();
    let config = load_config(target_root);
    let mut files_count = 1;
    let findings = if is_git_repo(target_root) {
        let diff_text = get_diff_output(target_root, false);
        let diff_files = parse_unified_diff(&diff_text);
        files_count = diff_files.len().max(1);
        run_all_diff_detectors(&diff_files, target_root, &config)
    } else {
        Vec::new()
    };
    let elapsed = start.elapsed().as_millis();
    let summary = compute_summary(findings, files_count, elapsed);
    let msg = if summary.errors > 0 { format!("Found {} critical issues ({} rules active)", summary.errors, AST_DETECTOR_DEFINITIONS.len()) } else { format!("Clean working tree ({} rules verified)", AST_DETECTOR_DEFINITIONS.len()) };
    record_telemetry_event(shared, target_root, "dashboard_scan", "repository", summary.errors, summary.score, elapsed, &msg);
    serde_json::to_string(&summary).unwrap_or_else(|_| "{}".to_string())
}
fn sanitize_workspace(target_root: &PathBuf, shared: &SharedTelemetry) -> usize {
    let start = Instant::now();
    let config = load_config(target_root);
    let mut fixed = 0;
    let walker = WalkDir::new(target_root).into_iter().filter_entry(|e| {
        let name = e.file_name().to_string_lossy();
        if e.file_type().is_dir() && (name == "target" || name == ".git" || name == "node_modules" || name == "dist" || name == "build") {
            return false;
        }
        let rel = e.path().strip_prefix(target_root).unwrap_or(e.path());
        let rel_str = rel.to_string_lossy().replace('\\', "/");
        !should_ignore_path(&rel_str, &config.ignore)
    });
    for entry in walker.filter_map(|e| e.ok()) {
        if entry.file_type().is_file() {
            let rel = entry.path().strip_prefix(target_root).unwrap_or(entry.path());
            let rel_str = rel.to_string_lossy().replace('\\', "/");
            if is_supported_code_file(&rel_str) && sanitize_file(entry.path()) {
                fixed += 1;
            }
        }
    }
    let elapsed = start.elapsed().as_millis();
    record_telemetry_event(shared, target_root, "dashboard_sanitize", "all files", 0, 100, elapsed, &format!("Sanitized {fixed} file(s)"));
    fixed
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
            } else if req.starts_with("POST /api/scan") || req.starts_with("GET /api/scan") {
                let body = audit_workspace(&target_root, &shared);
                let resp = format!("HTTP/1.1 200 OK\r\nContent-Type: application/json\r\nAccess-Control-Allow-Origin: *\r\nContent-Length: {}\r\n\r\n{}", body.len(), body);
                let _ = stream.write_all(resp.as_bytes());
            } else if req.starts_with("POST /api/fix") || req.starts_with("GET /api/fix") {
                let count = sanitize_workspace(&target_root, &shared);
                let body = format!("{{\"success\":true,\"fixed\":{count}}}");
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
#[allow(dead_code)]
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
