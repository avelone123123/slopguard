import * as fs from "node:fs";
import * as path from "node:path";
import { isGitRepo, getGitRoot, getDiffOutput, parseUnifiedDiff, getUntrackedFiles } from "./git.js";
import { loadConfig, shouldIgnorePath } from "./config.js";
import { runAllDiffDetectors, runAllContentDetectors, computeSummary } from "./detectors/index.js";
import { formatTerminalReport } from "./reporters/terminal.js";
import { generateBadgeSvg, generateAuditCardSvg } from "./reporters/svg.js";
import { sanitizeFile } from "./fixers/sanitizer.js";
import { startMcpServer } from "./mcp.js";
import { SlopFinding } from "./types.js";
function isSupportedCodeFile(filePath: string): boolean {
  return /\.(ts|tsx|js|jsx|mjs|cjs|py|go|rs|php)$/.test(filePath);
}
function scanDirectoryRecursively(dir: string, rootDir: string, ignorePatterns: string[]): string[] {
  const filePaths: string[] = [];
  const entries = fs.readdirSync(dir, { withFileTypes: true });
  for (const entry of entries) {
    const fullPath = path.join(dir, entry.name);
    const relPath = path.relative(rootDir, fullPath).replace(/\\/g, "/");
    if (shouldIgnorePath(relPath, ignorePatterns)) continue;
    if (entry.isDirectory()) {
      filePaths.push(...scanDirectoryRecursively(fullPath, rootDir, ignorePatterns));
    } else if (isSupportedCodeFile(entry.name)) {
      filePaths.push(fullPath);
    }
  }
  return filePaths;
}
export async function runCli(): Promise<void> {
  const args = process.argv.slice(2);
  const command = args[0] || "check";
  const cwd = process.cwd();
  if (command === "mcp") {
    startMcpServer();
    return;
  }
  if (command === "hook" && args[1] === "install") {
    const gitDir = path.join(cwd, ".git");
    if (!fs.existsSync(gitDir)) {
      process.stderr.write("Error: Not a git repository\n");
      process.exit(1);
    }
    const hooksDir = path.join(gitDir, "hooks");
    if (!fs.existsSync(hooksDir)) fs.mkdirSync(hooksDir, { recursive: true });
    const hookPath = path.join(hooksDir, "pre-commit");
    const script = "#!/bin/sh\nnode node_modules/slopguard/bin/slopguard.js check --staged || npx slopguard check --staged\n";
    fs.writeFileSync(hookPath, script, { mode: 0o755 });
    process.stdout.write("[SlopGuard] Pre-commit hook successfully installed at .git/hooks/pre-commit\n");
    return;
  }
  if (command === "badge") {
    const config = loadConfig(cwd);
    const startTime = Date.now();
    const diffText = getDiffOutput(cwd, false);
    const diffFiles = parseUnifiedDiff(diffText).filter(f => !shouldIgnorePath(f.newPath || f.oldPath, config.ignore) && isSupportedCodeFile(f.newPath || f.oldPath));
    const findings = runAllDiffDetectors(diffFiles, cwd, config);
    const summary = computeSummary(findings, diffFiles.length, Date.now() - startTime);
    const badgeSvg = generateBadgeSvg(summary.score, summary.errors);
    const targetPath = path.resolve(cwd, args[1] || "slopguard-badge.svg");
    fs.writeFileSync(targetPath, badgeSvg, "utf-8");
    process.stdout.write(`[SlopGuard] SVG Badge generated at: ${targetPath}\n`);
    return;
  }
  if (command === "fix") {
    const config = loadConfig(cwd);
    const target = args[1] ? path.resolve(cwd, args[1]) : cwd;
    let fixedCount = 0;
    if (fs.existsSync(target) && fs.statSync(target).isFile()) {
      if (sanitizeFile(target)) fixedCount++;
    } else {
      const allFiles = scanDirectoryRecursively(target, cwd, config.ignore);
      for (const f of allFiles) {
        if (sanitizeFile(f)) fixedCount++;
      }
    }
    process.stdout.write(`[SlopGuard] Sanitized AI bloat from ${fixedCount} file(s)\n`);
    return;
  }
  if (command === "all") {
    const config = loadConfig(cwd);
    const startTime = Date.now();
    const targetDir = args[1] ? path.resolve(cwd, args[1]) : cwd;
    const allFiles = scanDirectoryRecursively(targetDir, cwd, config.ignore);
    const allFindings: SlopFinding[] = [];
    for (const filePath of allFiles) {
      try {
        const content = fs.readFileSync(filePath, "utf-8");
        const relPath = path.relative(cwd, filePath).replace(/\\/g, "/");
        allFindings.push(...runAllContentDetectors(relPath, content, cwd, config));
      } catch {}
    }
    const summary = computeSummary(allFindings, allFiles.length, Date.now() - startTime);
    process.stdout.write(formatTerminalReport(summary) + "\n");
    if (summary.errors > 0) process.exit(1);
    return;
  }
  if (command === "check" || !command.startsWith("-")) {
    const isStaged = args.includes("--staged");
    const config = loadConfig(cwd);
    const startTime = Date.now();
    if (isGitRepo(cwd)) {
      const diffText = getDiffOutput(cwd, isStaged);
      const rawDiffFiles = parseUnifiedDiff(diffText);
      const diffFiles = rawDiffFiles.filter(f => !shouldIgnorePath(f.newPath || f.oldPath, config.ignore) && isSupportedCodeFile(f.newPath || f.oldPath));
      if (!isStaged) {
        const untracked = getUntrackedFiles(cwd);
        for (const u of untracked) {
          const relPath = u.replace(/\\/g, "/");
          if (shouldIgnorePath(relPath, config.ignore) || !isSupportedCodeFile(relPath)) continue;
          const fullPath = path.join(cwd, u);
          if (fs.existsSync(fullPath) && fs.statSync(fullPath).isFile()) {
            try {
              const content = fs.readFileSync(fullPath, "utf-8");
              const lines = content.split(/\r?\n/).map((c, i) => ({ type: "add" as const, content: c, newLineNumber: i + 1 }));
              diffFiles.push({ oldPath: relPath, newPath: relPath, isNew: true, isDeleted: false, chunks: [{ oldStart: 1, oldLines: 0, newStart: 1, newLines: lines.length, lines }] });
            } catch {}
          }
        }
      }
      if (diffFiles.length > 0) {
        const findings = runAllDiffDetectors(diffFiles, cwd, config);
        const summary = computeSummary(findings, diffFiles.length, Date.now() - startTime);
        process.stdout.write(formatTerminalReport(summary) + "\n");
        if (summary.errors > 0) process.exit(1);
        return;
      }
    }
    const allFiles = scanDirectoryRecursively(cwd, cwd, config.ignore);
    const allFindings: SlopFinding[] = [];
    for (const filePath of allFiles) {
      try {
        const content = fs.readFileSync(filePath, "utf-8");
        const relPath = path.relative(cwd, filePath).replace(/\\/g, "/");
        allFindings.push(...runAllContentDetectors(relPath, content, cwd, config));
      } catch {}
    }
    const summary = computeSummary(allFindings, allFiles.length, Date.now() - startTime);
    process.stdout.write(formatTerminalReport(summary) + "\n");
    if (summary.errors > 0) process.exit(1);
    return;
  }
  process.stdout.write("Usage: slopguard [check|all|fix|badge|hook install|mcp]\n");
}