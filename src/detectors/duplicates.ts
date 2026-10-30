import * as fs from "node:fs";
import * as path from "node:path";
import { SlopFinding, DiffFile, FunctionSignature } from "../types.js";
function tokenizeCode(code: string): Set<string> {
  const normalized = code.replace(/["'`][^"'`]*["'`]/g, "STR").replace(/\b\d+\b/g, "NUM");
  const words = normalized.match(/[a-zA-Z_$][a-zA-Z0-9_$]*/g) || [];
  return new Set(words.map(w => w.toLowerCase()));
}
function calculateJaccardSimilarity(setA: Set<string>, setB: Set<string>): number {
  if (setA.size === 0 || setB.size === 0) return 0;
  let intersectionCount = 0;
  for (const item of setA) {
    if (setB.has(item)) intersectionCount++;
  }
  const unionSize = setA.size + setB.size - intersectionCount;
  return unionSize === 0 ? 0 : intersectionCount / unionSize;
}
function normalizeIdentifier(name: string): string {
  return name.replace(/[-_]/g, "").toLowerCase();
}
export function extractFunctionsFromFile(filePath: string, content: string): FunctionSignature[] {
  const results: FunctionSignature[] = [];
  const lines = content.split(/\r?\n/);
  const fnRegex = /(?:export\s+)?(?:async\s+)?(?:function\s+([a-zA-Z0-9_$]+)|(?:const|let|var)\s+([a-zA-Z0-9_$]+)\s*=\s*(?:async\s*)?(?:\([^)]*\)|[a-zA-Z0-9_$]+)\s*=>|def\s+([a-zA-Z0-9_$]+)\s*\()/g;
  for (let idx = 0; idx < lines.length; idx++) {
    const line = lines[idx];
    let match: RegExpExecArray | null;
    fnRegex.lastIndex = 0;
    while ((match = fnRegex.exec(line)) !== null) {
      const name = match[1] || match[2] || match[3];
      if (!name || name.length < 3) continue;
      const snippetLines = lines.slice(idx, Math.min(idx + 12, lines.length)).join("\n");
      const tokens = tokenizeCode(snippetLines);
      if (tokens.size >= 8) {
        results.push({ name, file: filePath, line: idx + 1, paramCount: 0, bodySnippet: snippetLines, tokenSet: tokens });
      }
    }
  }
  return results;
}
export function indexExistingFunctions(projectRoot: string, ignorePatterns: string[]): FunctionSignature[] {
  const signatures: FunctionSignature[] = [];
  function walk(currentDir: string) {
    const entries = fs.readdirSync(currentDir, { withFileTypes: true });
    for (const entry of entries) {
      const fullPath = path.join(currentDir, entry.name);
      const relPath = path.relative(projectRoot, fullPath).replace(/\\/g, "/");
      const shouldIgnore = ignorePatterns.some(p => relPath.includes(p) || entry.name === p);
      if (shouldIgnore) continue;
      if (entry.isDirectory()) {
        walk(fullPath);
      } else if (/\.(ts|tsx|js|jsx|py)$/.test(entry.name)) {
        try {
          const content = fs.readFileSync(fullPath, "utf-8");
          signatures.push(...extractFunctionsFromFile(relPath, content));
        } catch {}
      }
    }
  }
  try {
    walk(projectRoot);
  } catch {}
  return signatures;
}
export function detectDuplicateUtilities(diffFiles: DiffFile[], existingSignatures: FunctionSignature[]): SlopFinding[] {
  const findings: SlopFinding[] = [];
  for (const file of diffFiles) {
    if (file.isDeleted) continue;
    const addedContent = file.chunks.flatMap(c => c.lines.filter(l => l.type === "add").map(l => l.content)).join("\n");
    const newSignatures = extractFunctionsFromFile(file.newPath || file.oldPath, addedContent);
    for (const newFn of newSignatures) {
      const normNewName = normalizeIdentifier(newFn.name);
      for (const oldFn of existingSignatures) {
        if (oldFn.file === (file.newPath || file.oldPath)) continue;
        const normOldName = normalizeIdentifier(oldFn.name);
        const nameMatches = normNewName === normOldName || (normNewName.includes(normOldName) && normOldName.length > 6) || (normOldName.includes(normNewName) && normNewName.length > 6);
        const tokenSim = calculateJaccardSimilarity(newFn.tokenSet, oldFn.tokenSet);
        if ((nameMatches && tokenSim >= 0.50) || tokenSim >= 0.85) {
          findings.push({
            file: file.newPath || file.oldPath,
            line: newFn.line,
            column: 1,
            ruleId: "no-duplicate-shadow-utilities",
            category: "duplicate",
            severity: "warning",
            message: `Shadow utility '${newFn.name}' duplicates existing '${oldFn.name}' in ${oldFn.file}:${oldFn.line} (${Math.round(tokenSim * 100)}% structural overlap)`,
            snippet: `function ${newFn.name}(...)`,
            fixable: false,
            suggestion: `Reuse and reference '${oldFn.name}' from '${oldFn.file}' instead of declaring a duplicate`
          });
          break;
        }
      }
    }
  }
  return findings;
}