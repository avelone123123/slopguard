use crate::types::{DiffChunk, DiffFile, DiffLine, DiffLineType};
use std::path::{Path, PathBuf};
use std::process::Command;
pub fn is_git_repo(cwd: &Path) -> bool {
    Command::new("git")
        .args(["rev-parse", "--is-inside-work-tree"])
        .current_dir(cwd)
        .output()
        .map(|o| o.status.success())
        .unwrap_or(false)
}
pub fn has_commits(cwd: &Path) -> bool {
    Command::new("git")
        .args(["rev-parse", "--verify", "HEAD"])
        .current_dir(cwd)
        .output()
        .map(|o| o.status.success())
        .unwrap_or(false)
}
pub fn get_git_root(cwd: &Path) -> PathBuf {
    if let Ok(output) = Command::new("git").args(["rev-parse", "--show-toplevel"]).current_dir(cwd).output() {
        if output.status.success() {
            let path_str = String::from_utf8_lossy(&output.stdout).trim().to_string();
            if !path_str.is_empty() {
                return PathBuf::from(path_str);
            }
        }
    }
    cwd.to_path_buf()
}
pub fn get_diff_output(cwd: &Path, staged_only: bool) -> String {
    let mut cmd = Command::new("git");
    cmd.current_dir(cwd);
    if staged_only || !has_commits(cwd) {
        cmd.args(["diff", "--cached", "--unified=3"]);
    } else {
        cmd.args(["diff", "HEAD", "--unified=3"]);
    }
    if let Ok(output) = cmd.output() {
        if output.status.success() {
            return String::from_utf8_lossy(&output.stdout).to_string();
        }
    }
    let mut fallback = Command::new("git");
    fallback.current_dir(cwd).args(["diff", "--unified=3"]);
    fallback.output().map(|o| String::from_utf8_lossy(&o.stdout).to_string()).unwrap_or_default()
}
pub fn get_untracked_files(cwd: &Path) -> Vec<String> {
    if let Ok(output) = Command::new("git").args(["ls-files", "--others", "--exclude-standard"]).current_dir(cwd).output() {
        if output.status.success() {
            return String::from_utf8_lossy(&output.stdout).lines().map(|s| s.trim().to_string()).filter(|s| !s.is_empty()).collect();
        }
    }
    Vec::new()
}
pub fn parse_unified_diff(diff_text: &str) -> Vec<DiffFile> {
    let mut files = Vec::new();
    let mut current_file: Option<DiffFile> = None;
    let mut current_chunk: Option<DiffChunk> = None;
    let mut old_line_counter = 0;
    let mut new_line_counter = 0;
    for line in diff_text.lines() {
        if line.starts_with("diff --git ") {
            if let Some(chunk) = current_chunk.take() {
                if let Some(ref mut f) = current_file {
                    f.chunks.push(chunk);
                }
            }
            if let Some(f) = current_file.take() {
                files.push(f);
            }
            let parts: Vec<&str> = line.split_whitespace().collect();
            let old_path = parts.get(2).map(|s| s.trim_start_matches("a/")).unwrap_or("").to_string();
            let new_path = parts.get(3).map(|s| s.trim_start_matches("b/")).unwrap_or("").to_string();
            current_file = Some(DiffFile { old_path, new_path, chunks: Vec::new(), is_new: false, is_deleted: false });
        } else if line.starts_with("new file mode ") {
            if let Some(ref mut f) = current_file {
                f.is_new = true;
            }
        } else if line.starts_with("deleted file mode ") {
            if let Some(ref mut f) = current_file {
                f.is_deleted = true;
            }
        } else if line.starts_with("@@ ") {
            if let Some(chunk) = current_chunk.take() {
                if let Some(ref mut f) = current_file {
                    f.chunks.push(chunk);
                }
            }
            let re = regex::Regex::new(r"^@@ -(\d+)(?:,\d+)? \+(\d+)(?:,\d+)? @@").unwrap();
            if let Some(caps) = re.captures(line) {
                old_line_counter = caps.get(1).and_then(|m| m.as_str().parse::<usize>().ok()).unwrap_or(1);
                new_line_counter = caps.get(2).and_then(|m| m.as_str().parse::<usize>().ok()).unwrap_or(1);
                current_chunk = Some(DiffChunk { old_start: old_line_counter, old_lines: 1, new_start: new_line_counter, new_lines: 1, lines: Vec::new() });
            }
        } else if let Some(ref mut chunk) = current_chunk {
            if line.starts_with('+') && !line.starts_with("+++") {
                chunk.lines.push(DiffLine { line_type: DiffLineType::Add, content: line[1..].to_string(), old_line_number: None, new_line_number: Some(new_line_counter) });
                new_line_counter += 1;
            } else if line.starts_with('-') && !line.starts_with("---") {
                chunk.lines.push(DiffLine { line_type: DiffLineType::Del, content: line[1..].to_string(), old_line_number: Some(old_line_counter), new_line_number: None });
                old_line_counter += 1;
            } else if line.starts_with(' ') {
                chunk.lines.push(DiffLine { line_type: DiffLineType::Normal, content: line[1..].to_string(), old_line_number: Some(old_line_counter), new_line_number: Some(new_line_counter) });
                old_line_counter += 1;
                new_line_counter += 1;
            }
        }
    }
    if let Some(chunk) = current_chunk.take() {
        if let Some(ref mut f) = current_file {
            f.chunks.push(chunk);
        }
    }
    if let Some(f) = current_file.take() {
        files.push(f);
    }
    files
}
pub fn get_current_branch(cwd: &Path) -> String {
    if let Ok(output) = Command::new("git").args(["branch", "--show-current"]).current_dir(cwd).output() {
        if output.status.success() {
            let b = String::from_utf8_lossy(&output.stdout).trim().to_string();
            if !b.is_empty() {
                return b;
            }
        }
    }
    "main".to_string()
}
pub fn get_latest_commit(cwd: &Path) -> String {
    if let Ok(output) = Command::new("git").args(["log", "-1", "--oneline"]).current_dir(cwd).output() {
        if output.status.success() {
            let c = String::from_utf8_lossy(&output.stdout).trim().to_string();
            if !c.is_empty() {
                return c;
            }
        }
    }
    "initial".to_string()
}