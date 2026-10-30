import { SlopSummary, SlopFinding } from "../types.js";
const C = {
  reset: "\x1b[0m",
  bold: "\x1b[1m",
  dim: "\x1b[2m",
  red: "\x1b[31m",
  green: "\x1b[32m",
  yellow: "\x1b[33m",
  cyan: "\x1b[36m",
  gray: "\x1b[90m",
  white: "\x1b[37m",
  bgRed: "\x1b[41m",
  bgGreen: "\x1b[42m",
  bgYellow: "\x1b[43m"
};
function getSeverityBadge(sev: string): string {
  if (sev === "error") return `${C.bgRed}${C.white}${C.bold} ERROR ${C.reset}`;
  if (sev === "warning") return `${C.bgYellow}${C.white}${C.bold} WARN ${C.reset}`;
  return `${C.cyan}${C.bold} INFO ${C.reset}`;
}
export function formatTerminalReport(summary: SlopSummary): string {
  const out: string[] = [];
  const width = 78;
  const border = "─".repeat(width);
  out.push(`${C.cyan}┌${border}┐${C.reset}`);
  out.push(`${C.cyan}│${C.reset}  ${C.bold}SLOPGUARD${C.reset} ${C.dim}v1.0.0${C.reset} - AI Slop & Hallucination Firewall${" ".repeat(Math.max(0, width - 48))}${C.cyan}│${C.reset}`);
  out.push(`${C.cyan}└${border}┘${C.reset}`);
  if (summary.findings.length === 0) {
    out.push(`${C.green}${C.bold}[PASS]${C.reset} Zero AI slop detected! Repository is clean and deterministic.`);
    out.push(`${C.dim}Scanned ${summary.scannedFiles} file(s) in ${summary.durationMs}ms | AI Hygiene Score: 100/100${C.reset}`);
    return out.join("\n");
  }
  out.push(`${C.bold}Findings (${summary.findings.length} issues in ${summary.scannedFiles} files):${C.reset}\n`);
  const grouped = new Map<string, SlopFinding[]>();
  for (const f of summary.findings) {
    const list = grouped.get(f.file) || [];
    list.push(f);
    grouped.set(f.file, list);
  }
  for (const [file, items] of grouped.entries()) {
    out.push(`${C.bold}${C.white}${file}${C.reset}`);
    for (const item of items) {
      out.push(`  ${getSeverityBadge(item.severity)} ${C.dim}L${item.line}:${item.column}${C.reset} ${item.message}`);
      if (item.snippet) {
        out.push(`    ${C.gray}│${C.reset} ${C.dim}${item.snippet}${C.reset}`);
      }
      if (item.suggestion) {
        out.push(`    ${C.cyan}→ Fix:${C.reset} ${item.suggestion}`);
      }
    }
    out.push("");
  }
  const scoreBadge = summary.score >= 80 ? `${C.green}${C.bold}${summary.score}/100 [GRADE A]${C.reset}` : summary.score >= 50 ? `${C.yellow}${C.bold}${summary.score}/100 [GRADE C]${C.reset}` : `${C.red}${C.bold}${summary.score}/100 [GRADE F - REJECT]${C.reset}`;
  out.push(`${C.dim}${border}${C.reset}`);
  out.push(`AI Hygiene Score: ${scoreBadge} | Errors: ${C.red}${summary.errors}${C.reset} | Warnings: ${C.yellow}${summary.warnings}${C.reset} | ${summary.durationMs}ms`);
  out.push(`${C.dim}${border}${C.reset}`);
  return out.join("\n");
}