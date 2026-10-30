import * as fs from "node:fs";
import { AI_COMMENT_PATTERNS } from "../detectors/aiCommentBloat.js";
export function sanitizeCode(code: string): { cleaned: string; modified: boolean } {
  const lines = code.split(/\r?\n/);
  const resultLines: string[] = [];
  let modified = false;
  for (let idx = 0; idx < lines.length; idx++) {
    const line = lines[idx];
    const isAiBloat = AI_COMMENT_PATTERNS.some(pat => pat.test(line));
    if (isAiBloat) {
      modified = true;
      continue;
    }
    resultLines.push(line);
  }
  return { cleaned: resultLines.join("\n"), modified };
}
export function sanitizeFile(filePath: string): boolean {
  if (!fs.existsSync(filePath)) return false;
  try {
    const original = fs.readFileSync(filePath, "utf-8");
    const { cleaned, modified } = sanitizeCode(original);
    if (modified) {
      fs.writeFileSync(filePath, cleaned, "utf-8");
      return true;
    }
  } catch {}
  return false;
}