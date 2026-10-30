use crate::types::{SlopSeverity, SlopSummary};
use std::collections::HashMap;
pub fn format_terminal_report(summary: &SlopSummary) -> String {
    let mut out = Vec::new();
    let border = "─".repeat(78);
    out.push(format!("\x1b[36m┌{}┐\x1b[0m", border));
    out.push(format!("\x1b[36m│\x1b[0m  \x1b[1mSLOPGUARD (RUST)\x1b[0m \x1b[2mv1.0.0\x1b[0m - Native AI Slop & Hallucination Firewall          \x1b[36m│\x1b[0m"));
    out.push(format!("\x1b[36m└{}┘\x1b[0m", border));
    if summary.findings.is_empty() {
        out.push(format!("\x1b[32m\x1b[1m[PASS]\x1b[0m Zero AI slop detected! Repository is clean and deterministic."));
        out.push(format!("\x1b[2mScanned {} file(s) in {}ms | AI Hygiene Score: 100/100\x1b[0m", summary.scanned_files, summary.duration_ms));
        return out.join("\n");
    }
    out.push(format!("\x1b[1mFindings ({} issues in {} files):\x1b[0m\n", summary.findings.len(), summary.scanned_files));
    let mut grouped: HashMap<String, Vec<&crate::types::SlopFinding>> = HashMap::new();
    for f in &summary.findings {
        grouped.entry(f.file.clone()).or_default().push(f);
    }
    for (file, items) in grouped {
        out.push(format!("\x1b[1m\x1b[37m{}\x1b[0m", file));
        for item in items {
            let badge = match item.severity {
                SlopSeverity::Error => "\x1b[41m\x1b[37m\x1b[1m ERROR \x1b[0m",
                SlopSeverity::Warning => "\x1b[43m\x1b[37m\x1b[1m WARN \x1b[0m",
                SlopSeverity::Info => "\x1b[36m\x1b[1m INFO \x1b[0m",
            };
            out.push(format!("  {} \x1b[2mL{}:{}\x1b[0m {}", badge, item.line, item.column, item.message));
            if !item.snippet.is_empty() {
                out.push(format!("    \x1b[90m│\x1b[0m \x1b[2m{}\x1b[0m", item.snippet));
            }
            if let Some(ref sugg) = item.suggestion {
                out.push(format!("    \x1b[36m→ Fix:\x1b[0m {}", sugg));
            }
        }
        out.push(String::new());
    }
    let score_badge = if summary.score >= 80 {
        format!("\x1b[32m\x1b[1m{}/100 [GRADE A]\x1b[0m", summary.score)
    } else if summary.score >= 50 {
        format!("\x1b[33m\x1b[1m{}/100 [GRADE C]\x1b[0m", summary.score)
    } else {
        format!("\x1b[31m\x1b[1m{}/100 [GRADE F - REJECT]\x1b[0m", summary.score)
    };
    out.push(format!("\x1b[2m{}\x1b[0m", border));
    out.push(format!("AI Hygiene Score: {} | Errors: \x1b[31m{}\x1b[0m | Warnings: \x1b[33m{}\x1b[0m | {}ms", score_badge, summary.errors, summary.warnings, summary.duration_ms));
    out.push(format!("\x1b[2m{}\x1b[0m", border));
    out.join("\n")
}
pub fn format_json_report(summary: &SlopSummary) -> String {
    serde_json::to_string_pretty(summary).unwrap_or_else(|_| "{}".to_string())
}
pub fn format_github_actions_report(summary: &SlopSummary) -> String {
    let mut out = Vec::new();
    for f in &summary.findings {
        let level = match f.severity {
            SlopSeverity::Error => "error",
            SlopSeverity::Warning => "warning",
            SlopSeverity::Info => "notice",
        };
        out.push(format!("::{level} file={},line={},col={}::[SlopGuard] {} ({})", f.file, f.line, f.column, f.message, f.rule_id));
    }
    out.join("\n")
}
pub fn generate_badge_svg(score: usize, errors: usize) -> String {
    let status_color = if errors > 0 { "#e5484d" } else if score >= 80 { "#30a46c" } else { "#f5a623" };
    let status_text = if errors > 0 { format!("{} slop errors", errors) } else { format!("{}% clean", score) };
    let label = "slopguard";
    let label_width = 68;
    let status_width = if errors > 0 { 84 } else { 70 };
    let total_width = label_width + status_width;
    format!("<svg xmlns=\"http://www.w3.org/2000/svg\" width=\"{}\" height=\"20\" role=\"img\" aria-label=\"{}: {}\"><linearGradient id=\"s\" x2=\"0\" y2=\"100%\"><stop offset=\"0\" stop-color=\"#bbb\" stop-opacity=\".1\"/><stop offset=\"1\" stop-opacity=\".1\"/></linearGradient><clipPath id=\"r\"><rect width=\"{}\" height=\"20\" rx=\"3\" fill=\"#fff\"/></clipPath><g clip-path=\"url(#r)\"><rect width=\"{}\" height=\"20\" fill=\"#20232a\"/><rect x=\"{}\" width=\"{}\" height=\"20\" fill=\"{}\"/><rect width=\"{}\" height=\"20\" fill=\"url(#s)\"/></g><g fill=\"#fff\" text-anchor=\"middle\" font-family=\"Verdana,Geneva,DejaVu Sans,sans-serif\" text-rendering=\"geometricPrecision\" font-size=\"110\"><text aria-hidden=\"true\" x=\"{}\" y=\"150\" fill=\"#010101\" fill-opacity=\".3\" transform=\"scale(.1)\">{}</text><text x=\"{}\" y=\"140\" transform=\"scale(.1)\" fill=\"#fff\">{}</text><text aria-hidden=\"true\" x=\"{}\" y=\"150\" fill=\"#010101\" fill-opacity=\".3\" transform=\"scale(.1)\">{}</text><text x=\"{}\" y=\"140\" transform=\"scale(.1)\" fill=\"#fff\">{}</text></g></svg>", total_width, label, status_text, total_width, label_width, label_width, status_width, status_color, total_width, (label_width * 10) / 2, label, (label_width * 10) / 2, label, label_width * 10 + (status_width * 10) / 2, status_text, label_width * 10 + (status_width * 10) / 2, status_text)
}