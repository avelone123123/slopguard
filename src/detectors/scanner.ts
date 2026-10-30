import { SlopFinding, DiffFile } from "../types.js";
export interface PatternRule {
  regex: RegExp;
  ruleId: string;
  category: SlopFinding["category"];
  severity: SlopFinding["severity"];
  message: string;
  fixable: boolean;
  suggestion: string;
}
export function scanDiffWithRules(diffFiles: DiffFile[], rules: PatternRule[], fileFilter?: (path: string) => boolean): SlopFinding[] {
  const findings: SlopFinding[] = [];
  for (const file of diffFiles) {
    if (file.isDeleted) continue;
    const targetPath = file.newPath || file.oldPath;
    if (fileFilter && !fileFilter(targetPath)) continue;
    for (const chunk of file.chunks) {
      for (const line of chunk.lines) {
        if (line.type !== "add") continue;
        const lineContent = line.content;
        const lineNum = line.newLineNumber || 1;
        for (const rule of rules) {
          const match = lineContent.match(rule.regex);
          if (match) {
            findings.push({
              file: targetPath,
              line: lineNum,
              column: (match.index || 0) + 1,
              ruleId: rule.ruleId,
              category: rule.category,
              severity: rule.severity,
              message: rule.message,
              snippet: lineContent.trim(),
              fixable: rule.fixable,
              suggestion: rule.suggestion
            });
          }
        }
      }
    }
  }
  return findings;
}
export function scanContentWithRules(filePath: string, content: string, rules: PatternRule[], fileFilter?: (path: string) => boolean): SlopFinding[] {
  const findings: SlopFinding[] = [];
  if (fileFilter && !fileFilter(filePath)) return findings;
  const lines = content.split(/\r?\n/);
  for (let idx = 0; idx < lines.length; idx++) {
    const lineContent = lines[idx];
    const lineNum = idx + 1;
    for (const rule of rules) {
      const match = lineContent.match(rule.regex);
      if (match) {
        findings.push({
          file: filePath,
          line: lineNum,
          column: (match.index || 0) + 1,
          ruleId: rule.ruleId,
          category: rule.category,
          severity: rule.severity,
          message: rule.message,
          snippet: lineContent.trim(),
          fixable: rule.fixable,
          suggestion: rule.suggestion
        });
      }
    }
  }
  return findings;
}