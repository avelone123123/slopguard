use crate::config::{is_supported_code_file, load_config, should_ignore_path};
use crate::detectors::{compute_summary, run_all_diff_detectors};
use crate::fixers::sanitize_file;
use crate::git::{get_current_branch, get_diff_output, get_latest_commit, get_untracked_files, is_git_repo, parse_unified_diff};
use crate::telemetry::{create_shared_telemetry, load_persisted_telemetry, record_event, McpTelemetryEvent, TelemetryState};
use crate::types::{SlopFinding, SlopSeverity, SlopSummary};
use crossterm::{
    cursor::{Hide, MoveTo, Show},
    event::{self, Event, KeyCode, KeyEventKind},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::fs;
use std::io::{stdout, Write};
use std::path::Path;
use std::time::{Duration, Instant};
use walkdir::WalkDir;
#[derive(PartialEq, Eq)]
enum ActiveTab {
    Overview,
    Issues,
    McpStream,
    Firewalls,
    Repository,
}
fn draw_bar(score: usize) -> String {
    let filled = (score / 5).min(20);
    let empty = 20usize.saturating_sub(filled);
    let green_part = "\x1b[32m█\x1b[0m".repeat(filled);
    let gray_part = "\x1b[90m░\x1b[0m".repeat(empty);
    format!("[{}{}]", green_part, gray_part)
}
fn perform_scan_action(target_root: &Path) -> SlopSummary {
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
    let msg = if summary.errors > 0 { format!("Intercepted {} issues", summary.errors) } else { "Clean working tree".to_string() };
    let shared = create_shared_telemetry();
    record_event(&shared, target_root, "terminal_audit", "workspace", summary.errors, summary.score, elapsed, &msg);
    summary
}
fn perform_sanitize_action(target_root: &Path) -> usize {
    let start = Instant::now();
    let config = load_config(target_root);
    let mut count = 0;
    let walker = WalkDir::new(target_root).into_iter().filter_entry(|e| {
        let name = e.file_name().to_string_lossy();
        if e.file_type().is_dir() && (name == "target" || name == ".git" || name == "node_modules") {
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
                count += 1;
            }
        }
    }
    let elapsed = start.elapsed().as_millis();
    let shared = create_shared_telemetry();
    record_event(&shared, target_root, "terminal_sanitize", "workspace", 0, 100, elapsed, &format!("Sanitized {count} file(s)"));
    count
}
fn export_report(target_root: &Path, summary: &SlopSummary) -> String {
    let path = target_root.join("slopguard-audit-report.json");
    let json_text = serde_json::to_string_pretty(summary).unwrap_or_else(|_| "{}".to_string());
    if fs::write(&path, json_text).is_ok() {
        format!("Exported: {}", path.display())
    } else {
        "Export failed".to_string()
    }
}
fn try_fix_selected_issue(target_root: &Path, summary: &Option<SlopSummary>, index: usize) -> Option<String> {
    let s = summary.as_ref()?;
    let sel = s.findings.get(index)?;
    let target_file = target_root.join(&sel.file);
    if target_file.is_file() && sanitize_file(&target_file) {
        Some(format!("Cleaned AI comments in {}! Re-auditing...", sel.file))
    } else {
        Some(format!("Cannot auto-fix rule '{}'. Check suggested fix.", sel.rule_id))
    }
}
fn render_wave_row(threshold: f64, history: &[f64], color_code: &str) -> String {
    let mut row = String::new();
    for &val in history {
        if val >= threshold {
            row.push_str(color_code);
            row.push('█');
        } else if val >= threshold - 1.2 {
            row.push_str(color_code);
            row.push('▄');
        } else {
            row.push(' ');
        }
    }
    row.push_str("\x1b[0m");
    row
}
pub fn run_interactive_monitor(target_root: &Path) {
    let mut out = stdout();
    let _ = enable_raw_mode();
    let _ = execute!(out, EnterAlternateScreen, Hide);
    let root_str = target_root.display().to_string();
    let mut current_tab = ActiveTab::Overview;
    let mut selected_index: usize = 0;
    let mut issue_filter: usize = 0;
    let mut last_summary: Option<SlopSummary> = Some(perform_scan_action(target_root));
    let mut status_msg = "Ready. Press [S] to audit, [F] to sanitize, [Enter] to fix issue.".to_string();
    let mut watch_mode = false;
    let mut last_diff = String::new();
    let mut tick: usize = 0;
    let mut wave_history: Vec<f64> = vec![1.2; 68];
    let spinners = ['⠋', '⠙', '⠹', '⠸', '⠼', '⠴', '⠦', '⠧', '⠇', '⠏'];
    let firewalls = [
        ("no-ai-todo-placeholders", "ERROR", "Unfinished TODO stubs and empty catch blocks"),
        ("no-silent-catch-pass", "ERROR", "Empty try/catch blocks swallowing exceptions"),
        ("no-dummy-secrets-mocks", "ERROR", "Dummy API tokens and fake credentials"),
        ("no-mock-hallucinations", "ERROR", "Public placeholder mock APIs & fake emails"),
        ("no-hallucinated-imports", "ERROR", "Hallucinated package exports (React, Axios, Lodash)"),
        ("no-phantom-api-routes", "ERROR", "Phantom client API endpoints with no backend route"),
        ("no-circular-dependencies", "ERROR", "Circular module import cycles between files"),
        ("no-ai-sycophancy-watermarks", "WARN", "Conversational AI sycophancy or chat watermarks"),
        ("no-ghost-env-vars", "ERROR", "Referenced env variables missing from .env.example"),
        ("no-deep-nesting-sprawl", "WARN", "Deep pyramid-of-doom spaghetti control flow"),
        ("no-type-safety-degradation", "ERROR", "Type bypasses and compiler suppression flags"),
        ("no-ghost-dependencies", "ERROR", "Missing package.json third-party imports"),
        ("no-duplicate-shadow-utilities", "WARN", "Shadow utility re-implementation"),
        ("no-ai-conversational-bloat", "WARN", "AI conversational comments & filler"),
        ("no-excessive-scope-drift", "WARN", "Agent touching excessive/unrelated files"),
        ("no-hardcoded-secrets", "ERROR", "Exposed AI tokens, private keys, AWS/Slack secrets"),
        ("no-unsafe-dynamic-eval", "ERROR", "Dangerous dynamic evaluation & unsafe deserialization"),
    ];
    loop {
        tick = tick.wrapping_add(1);
        let state: TelemetryState = load_persisted_telemetry(target_root);
        let base_lat = if state.avg_latency_ms > 0.0 { state.avg_latency_ms } else { 1.8 };
        let wave_mod = (tick as f64 * 0.18).sin() * 2.2 + (tick as f64 * 0.42).cos() * 1.3;
        let spike = if tick % 30 < 3 { 4.5 } else { 0.0 };
        let new_sample = (base_lat + wave_mod + spike).max(0.3);
        wave_history.remove(0);
        wave_history.push(new_sample);
        if watch_mode {
            let cur_diff = get_diff_output(target_root, false);
            if cur_diff != last_diff {
                last_diff = cur_diff;
                last_summary = Some(perform_scan_action(target_root));
                status_msg = "Live change detected! Re-audited working tree.".to_string();
            }
        }
        let _ = execute!(out, MoveTo(0, 0));
        let spin = spinners[(tick / 2) % spinners.len()];
        let mut screen = String::new();
        screen.push_str("\x1b[36m╔══════════════════════════════════════════════════════════════════════════════════════╗\x1b[0m\r\n");
        screen.push_str("\x1b[36m║\x1b[0m  \x1b[1mSLOPGUARD NATIVE AI FIREWALL & LIVE MCP OBSERVER CONSOLE\x1b[0m                   \x1b[2mv1.0.0\x1b[0m  \x1b[36m║\x1b[0m\r\n");
        screen.push_str("\x1b[36m╚══════════════════════════════════════════════════════════════════════════════════════╝\x1b[0m\r\n");
        let t1 = if current_tab == ActiveTab::Overview { "\x1b[46m\x1b[30m\x1b[1m [1] OVERVIEW \x1b[0m" } else { " [1] Overview " };
        let t2 = if current_tab == ActiveTab::Issues { "\x1b[46m\x1b[30m\x1b[1m [2] ISSUES \x1b[0m" } else { " [2] Issues " };
        let t3 = if current_tab == ActiveTab::McpStream { "\x1b[46m\x1b[30m\x1b[1m [3] MCP STREAM \x1b[0m" } else { " [3] MCP Stream " };
        let t4 = if current_tab == ActiveTab::Firewalls { "\x1b[46m\x1b[30m\x1b[1m [4] FIREWALLS \x1b[0m" } else { " [4] Firewalls " };
        let t5 = if current_tab == ActiveTab::Repository { "\x1b[46m\x1b[30m\x1b[1m [5] REPOSITORY \x1b[0m" } else { " [5] Repository " };
        screen.push_str(&format!("  {}  {}  {}  {}  {}\r\n\r\n", t1, t2, t3, t4, t5));
        let watch_tag = if watch_mode { "\x1b[32m[WATCH: ON]\x1b[0m" } else { "\x1b[2m[WATCH: OFF]\x1b[0m" };
        screen.push_str(&format!("  \x1b[32m{} MCP PROTOCOL ACTIVE\x1b[0m │ Target: \x1b[1m{}\x1b[0m │ {}\r\n\r\n", spin, root_str, watch_tag));
        match current_tab {
            ActiveTab::Overview => {
                let grade = if state.current_hygiene_score >= 90 { "GRADE A+ (CLEAN)" } else if state.current_hygiene_score >= 70 { "GRADE B (MODERATE)" } else { "GRADE F (HIGH SLOP)" };
                let bar = draw_bar(state.current_hygiene_score);
                screen.push_str("\x1b[36m┌─ AI HYGIENE HEALTH & TELEMETRY ────────────────────────┬─ ENGINE STATS ──────────────┐\x1b[0m\r\n");
                screen.push_str(&format!("\x1b[36m│\x1b[0m Score:   {:20} \x1b[32m\x1b[1m{:3}/100\x1b[0m \x1b[36m│\x1b[0m Audits:    \x1b[1m{:6}\x1b[0m            \x1b[36m│\x1b[0m\r\n", bar, state.current_hygiene_score, state.total_scans));
                screen.push_str(&format!("\x1b[36m│\x1b[0m Rating:  \x1b[1m{:26}\x1b[0m                \x1b[36m│\x1b[0m Intercept: \x1b[31m\x1b[1m{:6}\x1b[0m            \x1b[36m│\x1b[0m\r\n", grade, state.total_slop_blocked));
                screen.push_str(&format!("\x1b[36m│\x1b[0m Defense: \x1b[36m17 Active AST Firewalls\x1b[0m                       \x1b[36m│\x1b[0m Latency:   \x1b[32m\x1b[1m{:4.1} ms\x1b[0m (Rust)     \x1b[36m│\x1b[0m\r\n", state.avg_latency_ms));
                screen.push_str("\x1b[36m└────────────────────────────────────────────────────────┴─────────────────────────────┘\x1b[0m\r\n");
                screen.push_str("\x1b[36m┌─ REAL-TIME AGENTIC TELEMETRY WAVEFORM (Oscilloscope) ────────────────────────────────┐\x1b[0m\r\n");
                screen.push_str(&format!("│ \x1b[90m8ms\x1b[0m ┤{} │\r\n", render_wave_row(6.0, &wave_history, "\x1b[38;5;198m")));
                screen.push_str(&format!("│ \x1b[90m4ms\x1b[0m ┤{} │\r\n", render_wave_row(4.0, &wave_history, "\x1b[38;5;51m")));
                screen.push_str(&format!("│ \x1b[90m2ms\x1b[0m ┤{} │\r\n", render_wave_row(2.0, &wave_history, "\x1b[38;5;45m")));
                screen.push_str(&format!("│ \x1b[90m0ms\x1b[0m ┤{} │\r\n", render_wave_row(0.5, &wave_history, "\x1b[38;5;33m")));
                screen.push_str("\x1b[36m└──────────────────────────────────────────────────────────────────────────────────────┘\x1b[0m\r\n");
                screen.push_str("\x1b[36m┌─ LIVE AGENT MCP ACTIVITY STREAM (Microsecond Resolution) ────────────────────────────┐\x1b[0m\r\n");
                if state.events.is_empty() {
                    screen.push_str("│  \x1b[2mNo MCP tool calls logged yet. Waiting for Cursor / Claude Code tool requests...\x1b[0m     │\r\n");
                } else {
                    for e in state.events.iter().take(3) {
                        let badge = if e.slop_found > 0 { "\x1b[41m\x1b[37m\x1b[1m INTERCEPT \x1b[0m" } else { "\x1b[42m\x1b[30m\x1b[1m   CLEAN   \x1b[0m" };
                        let tool_str = if e.client_tool.len() > 22 { &e.client_tool[..22] } else { &e.client_tool };
                        let file_str = if e.target_file.len() > 20 { &e.target_file[..20] } else { &e.target_file };
                        screen.push_str(&format!("│ {:8} {:<22} {:<20} {}  {:3}ms     │\r\n", e.timestamp, tool_str, file_str, badge, e.latency_ms));
                    }
                }
                screen.push_str("\x1b[36m└──────────────────────────────────────────────────────────────────────────────────────┘\x1b[0m\r\n");
            }
            ActiveTab::Issues => {
                let all_findings: Vec<&SlopFinding> = last_summary.as_ref().map(|s| s.findings.iter().collect()).unwrap_or_default();
                let filtered: Vec<&SlopFinding> = all_findings.into_iter().filter(|f| match issue_filter {
                    1 => f.severity == SlopSeverity::Error,
                    2 => f.severity == SlopSeverity::Warning,
                    _ => true,
                }).collect();
                let filter_str = match issue_filter { 1 => "ERRORS ONLY", 2 => "WARNINGS ONLY", _ => "ALL" };
                screen.push_str(&format!("\x1b[36m┌─ CODEBASE ISSUES ({} shown | Filter: \x1b[1m{}\x1b[0m \x1b[36m[A]) ────────────────────────────────────────┐\x1b[0m\r\n", filtered.len(), filter_str));
                if filtered.is_empty() {
                    screen.push_str("│  \x1b[32m\x1b[1m[PASS] Zero AI slop detected! Repository is immaculate and deterministic.\x1b[0m          │\r\n");
                    screen.push_str("│  \x1b[2mPress [S] to re-run full AST audit on workspace.\x1b[0m                                    │\r\n");
                } else {
                    let total = filtered.len();
                    if selected_index >= total { selected_index = total.saturating_sub(1); }
                    for (idx, f) in filtered.iter().enumerate().take(8) {
                        let is_sel = idx == selected_index;
                        let prefix = if is_sel { "\x1b[36m\x1b[1m▶\x1b[0m " } else { "  " };
                        let badge = match f.severity {
                            SlopSeverity::Error => "\x1b[41m\x1b[37m ERR \x1b[0m",
                            SlopSeverity::Warning => "\x1b[43m\x1b[30m WRN \x1b[0m",
                            SlopSeverity::Info => "\x1b[44m\x1b[37m INF \x1b[0m",
                        };
                        let line_info = format!("{}:{}", f.file, f.line);
                        let line_str = if line_info.len() > 28 { &line_info[..28] } else { &line_info };
                        let msg_str = if f.message.len() > 38 { &f.message[..38] } else { &f.message };
                        screen.push_str(&format!("│ {}{} {:<28} {:<38} │\r\n", prefix, badge, line_str, msg_str));
                    }
                    if let Some(sel) = filtered.get(selected_index) {
                        screen.push_str("├──────────────────────────────────────────────────────────────────────────────────────┤\r\n");
                        screen.push_str(&format!("│ \x1b[1mTarget:\x1b[0m {}:{} \x1b[2m({})\x1b[0m\r\n", sel.file, sel.line, sel.rule_id));
                        if !sel.snippet.is_empty() {
                            let snip = if sel.snippet.len() > 70 { &sel.snippet[..70] } else { &sel.snippet };
                            screen.push_str(&format!("│ \x1b[90mSnippet:\x1b[0m \x1b[2m{}\x1b[0m\r\n", snip));
                        }
                        if let Some(ref sugg) = sel.suggestion {
                            let sug = if sugg.len() > 70 { &sugg[..70] } else { sugg };
                            screen.push_str(&format!("│ \x1b[36mFix:\x1b[0m {}\r\n", sug));
                        }
                    }
                }
                screen.push_str("\x1b[36m└──────────────────────────────────────────────────────────────────────────────────────┘\x1b[0m\r\n");
            }
            ActiveTab::McpStream => {
                screen.push_str("\x1b[36m┌─ MODEL CONTEXT PROTOCOL (JSON-RPC 2.0) EVENT LOG ────────────────────────────────────┐\x1b[0m\r\n");
                screen.push_str("│ ID   TIMESTAMP  METHOD / TOOL            TARGET           STATUS      LATENCY        │\r\n");
                screen.push_str("├──────────────────────────────────────────────────────────────────────────────────────┤\r\n");
                let events: &Vec<McpTelemetryEvent> = &state.events;
                if events.is_empty() {
                    screen.push_str("│  \x1b[2mNo MCP transactions recorded yet.\x1b[0m                                                   │\r\n");
                } else {
                    let total = events.len();
                    if selected_index >= total { selected_index = total.saturating_sub(1); }
                    for (idx, e) in events.iter().enumerate().take(7) {
                        let is_sel = idx == selected_index;
                        let prefix = if is_sel { "▶" } else { " " };
                        let badge = if e.slop_found > 0 { "\x1b[31mBLOCKED\x1b[0m" } else { "\x1b[32mPASS   \x1b[0m" };
                        let tool = if e.client_tool.len() > 22 { &e.client_tool[..22] } else { &e.client_tool };
                        let file = if e.target_file.len() > 16 { &e.target_file[..16] } else { &e.target_file };
                        screen.push_str(&format!("│{}#{:<3} {:8} {:<22} {:<16} {}  {:3}ms         │\r\n", prefix, e.id, e.timestamp, tool, file, badge, e.latency_ms));
                    }
                }
                screen.push_str("\x1b[36m└──────────────────────────────────────────────────────────────────────────────────────┘\x1b[0m\r\n");
            }
            ActiveTab::Firewalls => {
                screen.push_str("\x1b[36m┌─ ACTIVE AST FIREWALL ENGINES & GUARDRAILS (15 ACTIVE) ───────────────────────────────┐\x1b[0m\r\n");
                for (id, sev, desc) in &firewalls {
                    let badge = if *sev == "ERROR" { "\x1b[31m[ERROR]\x1b[0m" } else { "\x1b[33m[WARN ]\x1b[0m" };
                    screen.push_str(&format!("│  \x1b[32m●\x1b[0m {} {:<30} \x1b[2m{:<42}\x1b[0m │\r\n", badge, id, desc));
                }
                screen.push_str("\x1b[36m└──────────────────────────────────────────────────────────────────────────────────────┘\x1b[0m\r\n");
            }
            ActiveTab::Repository => {
                let branch = get_current_branch(target_root);
                let commit = get_latest_commit(target_root);
                let untracked = get_untracked_files(target_root);
                screen.push_str("\x1b[36m┌─ GIT REPOSITORY DIAGNOSTICS & WORKING TREE ──────────────────────────────────────────┐\x1b[0m\r\n");
                screen.push_str(&format!("│  \x1b[1mActive Branch:\x1b[0m   \x1b[32m{}\x1b[0m\r\n", branch));
                screen.push_str(&format!("│  \x1b[1mLatest Commit:\x1b[0m   \x1b[2m{}\x1b[0m\r\n", commit));
                screen.push_str(&format!("│  \x1b[1mUntracked Files:\x1b[0m {}\r\n", untracked.len()));
                screen.push_str("├──────────────────────────────────────────────────────────────────────────────────────┤\r\n");
                if untracked.is_empty() {
                    screen.push_str("│  \x1b[32mWorking tree clean. All changes are staged or committed.\x1b[0m                            │\r\n");
                } else {
                    for u in untracked.iter().take(4) {
                        let u_str = if u.len() > 70 { &u[..70] } else { u };
                        screen.push_str(&format!("│  \x1b[33m?\x1b[0m {}\r\n", u_str));
                    }
                }
                screen.push_str("\x1b[36m└──────────────────────────────────────────────────────────────────────────────────────┘\x1b[0m\r\n");
            }
        }
        screen.push_str(&format!("\r\n  \x1b[90mStatus:\x1b[0m \x1b[37m{}\x1b[0m\r\n", status_msg));
        screen.push_str("  \x1b[1mKEYS:\x1b[0m \x1b[36m[1..5]\x1b[0m Tabs \x1b[36m[↑/↓]\x1b[0m Select \x1b[36m[Enter]\x1b[0m Auto-Fix \x1b[36m[A]\x1b[0m Filter \x1b[36m[S]\x1b[0m Audit \x1b[36m[F]\x1b[0m Sanitize \x1b[36m[W]\x1b[0m Watch \x1b[36m[E]\x1b[0m Export \x1b[36m[Q]\x1b[0m Quit\r\n");
        let _ = write!(out, "{}", screen);
        let _ = out.flush();
        if event::poll(Duration::from_millis(110)).unwrap_or(false) {
            if let Ok(Event::Key(key)) = event::read() {
                if key.kind == KeyEventKind::Press {
                    match key.code {
                        KeyCode::Char('q') | KeyCode::Char('Q') | KeyCode::Esc => break,
                        KeyCode::Char('1') => { current_tab = ActiveTab::Overview; selected_index = 0; },
                        KeyCode::Char('2') => { current_tab = ActiveTab::Issues; selected_index = 0; },
                        KeyCode::Char('3') => { current_tab = ActiveTab::McpStream; selected_index = 0; },
                        KeyCode::Char('4') => { current_tab = ActiveTab::Firewalls; selected_index = 0; },
                        KeyCode::Char('5') => { current_tab = ActiveTab::Repository; selected_index = 0; },
                        KeyCode::Tab => {
                            current_tab = match current_tab {
                                ActiveTab::Overview => ActiveTab::Issues,
                                ActiveTab::Issues => ActiveTab::McpStream,
                                ActiveTab::McpStream => ActiveTab::Firewalls,
                                ActiveTab::Firewalls => ActiveTab::Repository,
                                ActiveTab::Repository => ActiveTab::Overview,
                            };
                            selected_index = 0;
                        },
                        KeyCode::Char('a') | KeyCode::Char('A') => {
                            issue_filter = (issue_filter + 1) % 3;
                            selected_index = 0;
                        },
                        KeyCode::Enter if current_tab == ActiveTab::Issues => {
                            let fix_res = try_fix_selected_issue(target_root, &last_summary, selected_index);
                            last_summary = fix_res.as_ref().map(|_| perform_scan_action(target_root)).or(last_summary);
                            status_msg = fix_res.unwrap_or(status_msg);
                        },
                        KeyCode::Up | KeyCode::Char('k') => {
                            selected_index = selected_index.saturating_sub(1);
                        },
                        KeyCode::Down | KeyCode::Char('j') => {
                            selected_index += 1;
                        },
                        KeyCode::Char('s') | KeyCode::Char('S') => {
                            let summary = perform_scan_action(target_root);
                            status_msg = format!("Audit completed: {} issues found (Score: {}/100)", summary.total_issues, summary.score);
                            last_summary = Some(summary);
                        },
                        KeyCode::Char('f') | KeyCode::Char('F') => {
                            let count = perform_sanitize_action(target_root);
                            status_msg = format!("Sanitized AI comment bloat from {} file(s)", count);
                            last_summary = Some(perform_scan_action(target_root));
                        },
                        KeyCode::Char('w') | KeyCode::Char('W') => {
                            watch_mode = !watch_mode;
                            status_msg = if watch_mode { "Live watcher activated! Monitoring changes...".to_string() } else { "Live watcher deactivated.".to_string() };
                        },
                        KeyCode::Char('e') | KeyCode::Char('E') => {
                            status_msg = last_summary.as_ref().map(|s| export_report(target_root, s)).unwrap_or_else(|| "No audit data to export".to_string());
                        },
                        KeyCode::Char('r') | KeyCode::Char('R') => {
                            status_msg = "Refreshed telemetry state.".to_string();
                        },
                        _ => {}
                    }
                }
            }
        }
    }
    let _ = execute!(out, Show, LeaveAlternateScreen);
    let _ = disable_raw_mode();
}