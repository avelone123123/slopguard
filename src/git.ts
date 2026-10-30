import { execSync } from "node:child_process";
import { DiffFile, DiffChunk } from "./types.js";
export function isGitRepo(cwd: string): boolean {
  try {
    execSync("git rev-parse --is-inside-work-tree", { cwd, stdio: "ignore" });
    return true;
  } catch {
    return false;
  }
}
export function hasCommits(cwd: string): boolean {
  try {
    execSync("git rev-parse --verify HEAD", { cwd, stdio: "ignore" });
    return true;
  } catch {
    return false;
  }
}
export function getGitRoot(cwd: string): string {
  try {
    return execSync("git rev-parse --show-toplevel", { cwd, encoding: "utf-8", stdio: ["ignore", "pipe", "ignore"] }).trim();
  } catch {
    return cwd;
  }
}
export function getDiffOutput(cwd: string, stagedOnly: boolean = false): string {
  if (!hasCommits(cwd)) {
    try {
      return execSync("git diff --cached --unified=3", { cwd, encoding: "utf-8", stdio: ["ignore", "pipe", "ignore"], maxBuffer: 10 * 1024 * 1024 });
    } catch {
      return "";
    }
  }
  try {
    const cmd = stagedOnly ? "git diff --cached --unified=3" : "git diff HEAD --unified=3";
    return execSync(cmd, { cwd, encoding: "utf-8", stdio: ["ignore", "pipe", "ignore"], maxBuffer: 10 * 1024 * 1024 });
  } catch {
    try {
      return execSync("git diff --unified=3", { cwd, encoding: "utf-8", stdio: ["ignore", "pipe", "ignore"], maxBuffer: 10 * 1024 * 1024 });
    } catch {
      return "";
    }
  }
}
export function getUntrackedFiles(cwd: string): string[] {
  try {
    const out = execSync("git ls-files --others --exclude-standard", { cwd, encoding: "utf-8", stdio: ["ignore", "pipe", "ignore"] });
    return out.split(/\r?\n/).filter(f => f.trim().length > 0);
  } catch {
    return [];
  }
}
export function parseUnifiedDiff(diffText: string): DiffFile[] {
  const files: DiffFile[] = [];
  const lines = diffText.split(/\r?\n/);
  let currentFile: DiffFile | null = null;
  let currentChunk: DiffChunk | null = null;
  let oldLineCounter = 0;
  let newLineCounter = 0;
  for (let i = 0; i < lines.length; i++) {
    const line = lines[i];
    if (line.startsWith("diff --git ")) {
      if (currentChunk && currentFile) {
        currentFile.chunks.push(currentChunk);
        currentChunk = null;
      }
      if (currentFile) {
        files.push(currentFile);
      }
      const parts = line.split(" ");
      const oldPath = parts[2]?.replace(/^[ab]\//, "") || "";
      const newPath = parts[3]?.replace(/^[ab]\//, "") || "";
      currentFile = { oldPath, newPath, chunks: [], isNew: false, isDeleted: false };
    } else if (line.startsWith("new file mode ") && currentFile) {
      currentFile.isNew = true;
    } else if (line.startsWith("deleted file mode ") && currentFile) {
      currentFile.isDeleted = true;
    } else if (line.startsWith("@@ ") && currentFile) {
      if (currentChunk) {
        currentFile.chunks.push(currentChunk);
      }
      const match = line.match(/^@@ -(\d+)(?:,(\d+))? \+(\d+)(?:,(\d+))? @@/);
      if (match) {
        oldLineCounter = parseInt(match[1], 10);
        newLineCounter = parseInt(match[3], 10);
        currentChunk = {
          oldStart: oldLineCounter,
          oldLines: match[2] ? parseInt(match[2], 10) : 1,
          newStart: newLineCounter,
          newLines: match[4] ? parseInt(match[4], 10) : 1,
          lines: []
        };
      }
    } else if (currentChunk && line.startsWith("+") && !line.startsWith("+++")) {
      currentChunk.lines.push({ type: "add", content: line.slice(1), newLineNumber: newLineCounter++ });
    } else if (currentChunk && line.startsWith("-") && !line.startsWith("---")) {
      currentChunk.lines.push({ type: "del", content: line.slice(1), oldLineNumber: oldLineCounter++ });
    } else if (currentChunk && line.startsWith(" ")) {
      currentChunk.lines.push({ type: "normal", content: line.slice(1), oldLineNumber: oldLineCounter++, newLineNumber: newLineCounter++ });
    }
  }
  if (currentChunk && currentFile) {
    currentFile.chunks.push(currentChunk);
  }
  if (currentFile) {
    files.push(currentFile);
  }
  return files;
}