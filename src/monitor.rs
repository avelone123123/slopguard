use crate::config::{is_supported_code_file, load_config, should_ignore_path};
use crate::detectors::{compute_summary, run_all_diff_detectors};
use crate::fixers::sanitize_file;
use crate::git::{get_diff_output, is_git_repo, parse_unified_diff};
use crate::telemetry::{create_shared_telemetry, load_persisted_telemetry, record_event};
use crossterm::{
    cursor::{Hide, MoveTo, Show},
    event::{self, Event, KeyCode, KeyEventKind},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::io::{stdout, Write};
use std::path::Path;
use std::time::{Duration, Instant};
use walkdir::WalkDir;
fn draw_bar(score: usize) -> String {
    let filled = score / 5;
    let empty = 20usize.saturating_sub(filled);
    format!("[{}{}]", "█".repeat(filled), "░".repeat(empty))
}
fn perform_scan_action(target_root: &Path) {
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
}
fn perform_sanitize_action(target_root: &Path) {
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
}
pub fn run_interactive_monitor(target_root: &Path) {
    let mut out = stdout();
    let _ = enable_raw_mode();
    let _ = execute!(out, EnterAlternateScreen, Hide);
    let root_str = target_root.display().to_string();
    loop {
        let state = load_persisted_telemetry(target_root);
        let _ = execute!(out, MoveTo(0, 0));
        let grade = if state.current_hygiene_score >= 90 { "GRADE A+ (CLEAN)" } else if state.current_hygiene_score >= 70 { "GRADE B (MODERATE)" } else { "GRADE F (HIGH SLOP)" };
        let bar = draw_bar(state.current_hygiene_score);
        let mut screen = String::new();
        screen.push_str("\x1b[36m╔══════════════════════════════════════════════════════════════════════════════════════╗\x1b[0m\r\n");
        screen.push_str("\x1b[36m║\x1b[0m  \x1b[1mSLOPGUARD NATIVE AI FIREWALL & LIVE MCP TELEMETRY MONITOR\x1b[0m                  \x1b[2mv1.0.0\x1b[0m  \x1b[36m║\x1b[0m\r\n");
        screen.push_str("\x1b[36m╚══════════════════════════════════════════════════════════════════════════════════════╝\x1b[0m\r\n\r\n");
        screen.push_str(&format!("  \x1b[32m● MCP PROTOCOL ACTIVE\x1b[0m (STDIO) │ Target: \x1b[1m{}\x1b[0m\r\n", root_str));
        screen.push_str("  Listeners: Cursor, Claude Code, Windsurf, Antigravity\r\n\r\n");
        screen.push_str("\x1b[36m┌─ AI HYGIENE HEALTH ────────────────────────┐┌─ REALTIME ENGINE TELEMETRY ───────────┐\x1b[0m\r\n");
        screen.push_str(&format!("\x1b[36m│\x1b[0m Score:   \x1b[32m\x1b[1m{:3}/100\x1b[0m {:20} \x1b[36m││\x1b[0m Audits Recorded:   \x1b[1m{:6}\x1b[0m             \x1b[36m│\x1b[0m\r\n", state.current_hygiene_score, bar, state.total_scans));
        screen.push_str(&format!("\x1b[36m│\x1b[0m Rating:  \x1b[1m{:26}\x1b[0m     \x1b[36m││\x1b[0m Slop Neutralized: \x1b[31m\x1b[1m{:6}\x1b[0m             \x1b[36m│\x1b[0m\r\n", grade, state.total_slop_blocked));
        screen.push_str(&format!("\x1b[36m│\x1b[0m Defense: \x1b[36m11 Active AST Firewalls\x1b[0m            \x1b[36m││\x1b[0m MCP Tool Latency:  \x1b[32m\x1b[1m{:4.1} ms\x1b[0m (Rust Speed) \x1b[36m│\x1b[0m\r\n", state.avg_latency_ms));
        screen.push_str("\x1b[36m└────────────────────────────────────────────┘└────────────────────────────────────────┘\x1b[0m\r\n\r\n");
        screen.push_str("\x1b[36m┌─ LIVE AGENT MCP ACTIVITY STREAM (Auto-refreshing) ───────────────────────────────────┐\x1b[0m\r\n");
        screen.push_str("│ TIME     TOOL                     TARGET FILE            STATUS          LATENCY     │\r\n");
        screen.push_str("├──────────────────────────────────────────────────────────────────────────────────────┤\r\n");
        if state.events.is_empty() {
            screen.push_str("│  \x1b[2mNo MCP tool calls logged yet. Waiting for Cursor / Claude Code tool requests...\x1b[0m     │\r\n");
            screen.push_str("│  \x1b[2mPress [S] to trigger a manual scan or [F] to sanitize AI comment bloat.\x1b[0m             │\r\n");
        } else {
            for e in state.events.iter().take(6) {
                let badge = if e.slop_found > 0 { "\x1b[41m\x1b[37m\x1b[1m INTERCEPT \x1b[0m" } else { "\x1b[42m\x1b[30m\x1b[1m   CLEAN   \x1b[0m" };
                let tool_str = if e.client_tool.len() > 22 { &e.client_tool[..22] } else { &e.client_tool };
                let file_str = if e.target_file.len() > 20 { &e.target_file[..20] } else { &e.target_file };
                screen.push_str(&format!("│ {:8} {:<22} {:<20} {}  {:3}ms     │\r\n", e.timestamp, tool_str, file_str, badge, e.latency_ms));
            }
        }
        screen.push_str("\x1b[36m└──────────────────────────────────────────────────────────────────────────────────────┘\x1b[0m\r\n\r\n");
        screen.push_str("  \x1b[1mCOMMANDS:\x1b[0m  \x1b[36m[S]\x1b[0m Audit Workspace   \x1b[36m[F]\x1b[0m Sanitize Bloat   \x1b[36m[R]\x1b[0m Refresh Feed   \x1b[36m[Q]\x1b[0m Quit\r\n");
        let _ = write!(out, "{}", screen);
        let _ = out.flush();
        if event::poll(Duration::from_millis(500)).unwrap_or(false) {
            if let Ok(Event::Key(key)) = event::read() {
                if key.kind == KeyEventKind::Press {
                    match key.code {
                        KeyCode::Char('q') | KeyCode::Char('Q') | KeyCode::Esc => break,
                        KeyCode::Char('s') | KeyCode::Char('S') => perform_scan_action(target_root),
                        KeyCode::Char('f') | KeyCode::Char('F') => perform_sanitize_action(target_root),
                        KeyCode::Char('r') | KeyCode::Char('R') => {},
                        _ => {}
                    }
                }
            }
        }
    }
    let _ = execute!(out, Show, LeaveAlternateScreen);
    let _ = disable_raw_mode();
}