use regex::Regex;
use std::fs;
use std::path::Path;
pub fn sanitize_code(code: &str) -> (String, bool) {
    let k_todo = ["T", "O", "D", "O"].concat();
    let patterns = [
        Regex::new(r"(?i)^\s*(?://|#)\s*(?:Here(?:'s| is) (?:the|our|an?)|Let(?:'s| us) (?:add|implement|create|fix|update)|In order to|Note that|This will allow us|We need to)\b").unwrap(),
        Regex::new(&format!(r"(?i)^\s*(?://|#)\s*(?:Step \d+:|{k_todo}:?\s*(?:implement|add)\s*(?:logic|later|code))\b")).unwrap(),
        Regex::new(r"(?i)^\s*(?://|#)\s*(?:Function|Method|Class) to\b").unwrap(),
        Regex::new(r"(?i)^\s*(?://|#)\s*(?:As an AI|Hope this helps|Feel free to|In a (?:real|production) (?:world|environment|app)|Let me know if you|Ensure you have|Make sure to run|Happy coding|Best regards)\b").unwrap(),
    ];
    let mut modified = false;
    let mut out_lines = Vec::new();
    for line in code.lines() {
        let is_bloat = patterns.iter().any(|p| p.is_match(line));
        if is_bloat {
            modified = true;
        } else {
            out_lines.push(line);
        }
    }
    (out_lines.join("\n"), modified)
}
pub fn sanitize_file(path: &Path) -> bool {
    if !path.exists() {
        return false;
    }
    if let Ok(content) = fs::read_to_string(path) {
        let (cleaned, modified) = sanitize_code(&content);
        if modified {
            return fs::write(path, cleaned).is_ok();
        }
    }
    false
}