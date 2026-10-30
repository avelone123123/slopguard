import * as fs from "node:fs";
import * as path from "node:path";
import { SlopFinding, DiffFile } from "../types.js";
const NODE_BUILTINS = new Set(["assert", "async_hooks", "buffer", "child_process", "cluster", "console", "constants", "crypto", "dgram", "diagnostics_channel", "dns", "domain", "events", "fs", "fs/promises", "http", "http2", "https", "inspector", "module", "net", "os", "path", "path/posix", "path/win32", "perf_hooks", "process", "punycode", "querystring", "readline", "repl", "stream", "stream/consumers", "stream/promises", "stream/web", "string_decoder", "timers", "timers/promises", "tls", "trace_events", "tty", "url", "util", "util/types", "v8", "vm", "wasi", "worker_threads", "zlib"]);
export function getDeclaredDependencies(projectRoot: string): Set<string> {
  const deps = new Set<string>();
  const pkgPath = path.join(projectRoot, "package.json");
  if (fs.existsSync(pkgPath)) {
    try {
      const pkg = JSON.parse(fs.readFileSync(pkgPath, "utf-8"));
      const sections = ["dependencies", "devDependencies", "peerDependencies", "optionalDependencies"];
      for (const sec of sections) {
        if (pkg[sec] && typeof pkg[sec] === "object") {
          for (const key of Object.keys(pkg[sec])) {
            deps.add(key);
          }
        }
      }
    } catch {}
  }
  return deps;
}
function extractPackageName(importPath: string): string {
  if (importPath.startsWith("node:")) return importPath;
  if (importPath.startsWith("@")) {
    const parts = importPath.split("/");
    return parts.length >= 2 ? `${parts[0]}/${parts[1]}` : importPath;
  }
  return importPath.split("/")[0];
}
const IMPORT_STATEMENT_REGEX = /^\s*(?:import\s+(?:[\w*\s{},]*\s+from\s+)?|export\s+[\w*\s{},]*\s+from\s+|(?:const|let|var)\s+[\w*\s{},]+\s*=\s*require\s*\()\s*["']([^"']+)["']/;
export function detectGhostDependenciesInDiff(diffFiles: DiffFile[], projectRoot: string, allowedGhostDeps: string[] = []): SlopFinding[] {
  const findings: SlopFinding[] = [];
  const declared = getDeclaredDependencies(projectRoot);
  const allowed = new Set(allowedGhostDeps);
  for (const file of diffFiles) {
    if (file.isDeleted) continue;
    const isJsTs = /\.(ts|tsx|js|jsx|mjs|cjs)$/.test(file.newPath || file.oldPath);
    if (!isJsTs) continue;
    for (const chunk of file.chunks) {
      for (const line of chunk.lines) {
        if (line.type !== "add") continue;
        const lineContent = line.content;
        const lineNum = line.newLineNumber || 1;
        const match = lineContent.match(IMPORT_STATEMENT_REGEX);
        if (match) {
          const rawImport = match[1];
          if (rawImport.startsWith(".") || rawImport.startsWith("/") || rawImport.startsWith("~") || rawImport.startsWith("@/")) continue;
          if (rawImport.startsWith("node:")) continue;
          const pkgName = extractPackageName(rawImport);
          if (NODE_BUILTINS.has(pkgName)) continue;
          if (!declared.has(pkgName) && !allowed.has(pkgName)) {
            findings.push({
              file: file.newPath || file.oldPath,
              line: lineNum,
              column: (match.index || 0) + 1,
              ruleId: "no-ghost-dependencies",
              category: "ghost_dependency",
              severity: "error",
              message: `Ghost dependency '${pkgName}' imported by AI but not declared in package.json`,
              snippet: lineContent.trim(),
              fixable: false,
              suggestion: `Run 'npm install ${pkgName}' or remove hallucinated import`
            });
          }
        }
      }
    }
  }
  return findings;
}
export function detectGhostDependenciesInContent(filePath: string, content: string, projectRoot: string, allowedGhostDeps: string[] = []): SlopFinding[] {
  const findings: SlopFinding[] = [];
  const isJsTs = /\.(ts|tsx|js|jsx|mjs|cjs)$/.test(filePath);
  if (!isJsTs) return findings;
  const declared = getDeclaredDependencies(projectRoot);
  const allowed = new Set(allowedGhostDeps);
  const lines = content.split(/\r?\n/);
  for (let idx = 0; idx < lines.length; idx++) {
    const lineContent = lines[idx];
    const lineNum = idx + 1;
    const match = lineContent.match(IMPORT_STATEMENT_REGEX);
    if (match) {
      const rawImport = match[1];
      if (rawImport.startsWith(".") || rawImport.startsWith("/") || rawImport.startsWith("~") || rawImport.startsWith("@/")) continue;
      if (rawImport.startsWith("node:")) continue;
      const pkgName = extractPackageName(rawImport);
      if (NODE_BUILTINS.has(pkgName)) continue;
      if (!declared.has(pkgName) && !allowed.has(pkgName)) {
        findings.push({
          file: filePath,
          line: lineNum,
          column: (match.index || 0) + 1,
          ruleId: "no-ghost-dependencies",
          category: "ghost_dependency",
          severity: "error",
          message: `Ghost dependency '${pkgName}' imported by AI but not declared in package.json`,
          snippet: lineContent.trim(),
          fixable: false,
          suggestion: `Run 'npm install ${pkgName}' or remove hallucinated import`
        });
      }
    }
  }
  return findings;
}