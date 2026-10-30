mod config;
mod detectors;
mod fixers;
mod git;
mod mcp;
mod reporters;
#[cfg(test)]
mod tests;
mod types;
use crate::config::{default_config, is_supported_code_file, load_config, should_ignore_path};
use crate::detectors::{compute_summary, run_all_content_detectors, run_all_diff_detectors};
use crate::fixers::sanitize_file;
use crate::git::{get_diff_output, get_git_root, get_untracked_files, is_git_repo, parse_unified_diff};
use crate::mcp::start_mcp_server;
use crate::reporters::{format_github_actions_report, format_json_report, format_terminal_report, generate_badge_svg};
use crate::types::{DiffChunk, DiffFile, DiffLine, DiffLineType, SlopFinding};
use std::env;
use std::fs;
use std::io::{self, BufRead};
use std::path::Path;
use std::process::exit;
use std::thread::sleep;
use std::time::{Duration, Instant};
use walkdir::WalkDir;
fn scan_directory_recursively(dir: &Path, root_dir: &Path, ignore_patterns: &[String]) -> Vec<std::path::PathBuf> {
    let mut files = Vec::new();
    let walker = WalkDir::new(dir).into_iter().filter_entry(|e| {
        let name = e.file_name().to_string_lossy();
        if e.file_type().is_dir() && (name == "target" || name == ".git" || name == "node_modules" || name == "dist" || name == "build" || name == ".next") {
            return false;
        }
        let rel = e.path().strip_prefix(root_dir).unwrap_or(e.path());
        let rel_str = rel.to_string_lossy().replace('\\', "/");
        !should_ignore_path(&rel_str, ignore_patterns)
    });
    for entry in walker.filter_map(|e| e.ok()) {
        if entry.file_type().is_file() {
            let rel = entry.path().strip_prefix(root_dir).unwrap_or(entry.path());
            let rel_str = rel.to_string_lossy().replace('\\', "/");
            if is_supported_code_file(&rel_str) {
                files.push(entry.path().to_path_buf());
            }
        }
    }
    files
}
fn print_summary_output(summary: &crate::types::SlopSummary, is_json: bool, is_github: bool) {
    if is_json {
        println!("{}", format_json_report(summary));
    } else if is_github {
        println!("{}", format_github_actions_report(summary));
    } else {
        println!("{}", format_terminal_report(summary));
    }
}
fn run_check_command(target_dir: &Path, is_staged: bool, is_json: bool, is_github: bool) -> usize {
    let config = load_config(target_dir);
    let start = Instant::now();
    if is_git_repo(target_dir) {
        let diff_text = get_diff_output(target_dir, is_staged);
        let mut diff_files: Vec<DiffFile> = parse_unified_diff(&diff_text).into_iter().filter(|f| {
            let target = if !f.new_path.is_empty() { &f.new_path } else { &f.old_path };
            !should_ignore_path(target, &config.ignore) && is_supported_code_file(target)
        }).collect();
        if !is_staged {
            let untracked = get_untracked_files(target_dir);
            for u in untracked {
                let rel_str = u.replace('\\', "/");
                if should_ignore_path(&rel_str, &config.ignore) || !is_supported_code_file(&rel_str) {
                    continue;
                }
                let full_path = target_dir.join(&u);
                if full_path.is_file() {
                    if let Ok(content) = fs::read_to_string(&full_path) {
                        let lines: Vec<DiffLine> = content.lines().enumerate().map(|(i, c)| DiffLine { line_type: DiffLineType::Add, content: c.to_string(), old_line_number: None, new_line_number: Some(i + 1) }).collect();
                        let lines_len = lines.len();
                        diff_files.push(DiffFile { old_path: rel_str.clone(), new_path: rel_str, chunks: vec![DiffChunk { old_start: 1, old_lines: 0, new_start: 1, new_lines: lines_len, lines }], is_new: true, is_deleted: false });
                    }
                }
            }
        }
        if !diff_files.is_empty() {
            let findings = run_all_diff_detectors(&diff_files, target_dir, &config);
            let summary = compute_summary(findings, diff_files.len(), start.elapsed().as_millis());
            print_summary_output(&summary, is_json, is_github);
            return summary.errors;
        }
    }
    let all_files = scan_directory_recursively(target_dir, target_dir, &config.ignore);
    let mut all_findings: Vec<SlopFinding> = Vec::new();
    for file_path in &all_files {
        if let Ok(content) = fs::read_to_string(file_path) {
            let rel = file_path.strip_prefix(target_dir).unwrap_or(file_path);
            let rel_str = rel.to_string_lossy().replace('\\', "/");
            all_findings.extend(run_all_content_detectors(&rel_str, &content, target_dir, &config));
        }
    }
    let summary = compute_summary(all_findings, all_files.len(), start.elapsed().as_millis());
    print_summary_output(&summary, is_json, is_github);
    summary.errors
}
fn run_watch_loop(target_root: &Path) {
    println!("\x1b[36m[SlopGuard Watcher]\x1b[0m Watching for live code changes... (Press Ctrl+C to stop)");
    let mut last_diff = String::new();
    loop {
        let current_diff = get_diff_output(target_root, false);
        if current_diff != last_diff {
            last_diff = current_diff;
            print!("\x1b[2J\x1b[1;1H");
            let _ = run_check_command(target_root, false, false, false);
        }
        sleep(Duration::from_millis(800));
    }
}
fn main() {
    let args: Vec<String> = env::args().collect();
    let is_interactive = args.len() <= 1;
    let command = args.get(1).map(|s| s.as_str()).unwrap_or("check");
    let is_json = args.iter().any(|a| a == "--json");
    let is_github = args.iter().any(|a| a == "--github");
    let cwd = env::current_dir().unwrap_or_else(|_| Path::new(".").to_path_buf());
    let target_root = if is_git_repo(&cwd) { get_git_root(&cwd) } else { cwd.clone() };
    if command == "mcp" {
        start_mcp_server();
        return;
    }
    if command == "watch" {
        run_watch_loop(&target_root);
        return;
    }
    if command == "init" {
        let cfg_path = target_root.join(".slopguardrc.json");
        let def = default_config();
        let json_str = serde_json::to_string_pretty(&def).unwrap_or_default();
        let _ = fs::write(&cfg_path, json_str);
        println!("[SlopGuard] Configuration file created at: {}", cfg_path.display());
        return;
    }
    if command == "ci" {
        let wf_dir = target_root.join(".github").join("workflows");
        let _ = fs::create_dir_all(&wf_dir);
        let wf_path = wf_dir.join("slopguard.yml");
        let yml = "name: SlopGuard AI Hygiene\non: [push, pull_request]\njobs:\n  audit:\n    runs-on: ubuntu-latest\n    steps:\n      - uses: actions/checkout@v4\n      - uses: dtolnay/rust-toolchain@stable\n      - run: cargo install --path .\n      - run: slopguard check --github\n";
        let _ = fs::write(&wf_path, yml);
        println!("[SlopGuard] GitHub Actions CI workflow created at: {}", wf_path.display());
        return;
    }
    if command == "hook" && args.get(2).map(|s| s.as_str()) == Some("install") {
        let git_dir = target_root.join(".git");
        if !git_dir.exists() {
            eprintln!("Error: Not a git repository");
            exit(1);
        }
        let hooks_dir = git_dir.join("hooks");
        let _ = fs::create_dir_all(&hooks_dir);
        let hook_path = hooks_dir.join("pre-commit");
        let script = "#!/bin/sh\nslopguard check --staged\n";
        let _ = fs::write(&hook_path, script);
        println!("[SlopGuard] Pre-commit hook successfully installed at .git/hooks/pre-commit");
        return;
    }
    if command == "badge" {
        let config = load_config(&target_root);
        let start = Instant::now();
        let diff_text = get_diff_output(&target_root, false);
        let diff_files = parse_unified_diff(&diff_text);
        let findings = run_all_diff_detectors(&diff_files, &target_root, &config);
        let summary = compute_summary(findings, diff_files.len(), start.elapsed().as_millis());
        let badge_svg = generate_badge_svg(summary.score, summary.errors);
        let target_name = args.get(2).map(|s| s.as_str()).unwrap_or("slopguard-badge.svg");
        let target_path = target_root.join(target_name);
        let _ = fs::write(&target_path, badge_svg);
        println!("[SlopGuard] SVG Badge generated at: {}", target_path.display());
        return;
    }
    if command == "fix" {
        let config = load_config(&target_root);
        let target = if let Some(p) = args.get(2) { target_root.join(p) } else { target_root.clone() };
        let mut fixed_count = 0;
        if target.is_file() {
            if sanitize_file(&target) {
                fixed_count += 1;
            }
        } else {
            let all_files = scan_directory_recursively(&target, &target_root, &config.ignore);
            for f in all_files {
                if sanitize_file(&f) {
                    fixed_count += 1;
                }
            }
        }
        println!("[SlopGuard] Sanitized AI bloat from {fixed_count} file(s)");
        return;
    }
    if command == "all" {
        let config = load_config(&target_root);
        let start = Instant::now();
        let target = if let Some(p) = args.get(2) { target_root.join(p) } else { target_root.clone() };
        let all_files = scan_directory_recursively(&target, &target_root, &config.ignore);
        let mut all_findings: Vec<SlopFinding> = Vec::new();
        for file_path in &all_files {
            if let Ok(content) = fs::read_to_string(file_path) {
                let rel = file_path.strip_prefix(&target_root).unwrap_or(file_path);
                let rel_str = rel.to_string_lossy().replace('\\', "/");
                all_findings.extend(run_all_content_detectors(&rel_str, &content, &target_root, &config));
            }
        }
        let summary = compute_summary(all_findings, all_files.len(), start.elapsed().as_millis());
        print_summary_output(&summary, is_json, is_github);
        if summary.errors > 0 {
            exit(1);
        }
        return;
    }
    if command == "check" || is_interactive {
        let is_staged = args.iter().any(|a| a == "--staged");
        let errors = run_check_command(&target_root, is_staged, is_json, is_github);
        if is_interactive {
            println!("\nPress Enter to exit or type a command (all, fix, badge, hook, ci, init, watch):");
            let mut line = String::new();
            let _ = io::stdin().lock().read_line(&mut line);
            let trimmed = line.trim();
            if trimmed == "all" {
                let _ = run_check_command(&target_root, false, false, false);
            } else if trimmed == "fix" {
                let config = load_config(&target_root);
                let all_files = scan_directory_recursively(&target_root, &target_root, &config.ignore);
                let mut count = 0;
                for f in all_files {
                    if sanitize_file(&f) { count += 1; }
                }
                println!("[SlopGuard] Sanitized {count} file(s)");
            } else if trimmed == "watch" {
                run_watch_loop(&target_root);
            }
            return;
        }
        if errors > 0 {
            exit(1);
        }
        return;
    }
    println!("Usage: slopguard [check|all|fix|watch|init|badge|ci|hook install|mcp] [--json] [--github]");
}