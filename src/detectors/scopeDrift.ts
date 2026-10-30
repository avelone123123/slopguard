import { SlopFinding, DiffFile } from "../types.js";
const SENSITIVE_PATTERNS = [
  /^\.env(?:\..+)?$/,
  /^\.github\/workflows\/.+$/,
  /^(?:package-lock\.json|yarn\.lock|pnpm-lock\.yaml)$/,
  /^(?:tsconfig\.json|jsconfig\.json)$/
];
export function detectScopeDrift(diffFiles: DiffFile[], maxDriftFiles: number = 12): SlopFinding[] {
  const findings: SlopFinding[] = [];
  const modifiedPaths = diffFiles.map(f => f.newPath || f.oldPath);
  if (modifiedPaths.length > maxDriftFiles) {
    findings.push({
      file: modifiedPaths[0] || "repository",
      line: 1,
      column: 1,
      ruleId: "no-excessive-scope-drift",
      category: "scope_drift",
      severity: "warning",
      message: `Excessive agent scope drift: ${modifiedPaths.length} files modified across single turn (threshold: ${maxDriftFiles})`,
      snippet: modifiedPaths.slice(0, 5).join(", ") + `... (+${modifiedPaths.length - 5} more)`,
      fixable: false,
      suggestion: "Split large agent refactorings into smaller, verified atomic steps"
    });
  }
  for (const fPath of modifiedPaths) {
    const fileName = fPath.split("/").pop() || fPath;
    for (const pattern of SENSITIVE_PATTERNS) {
      if (pattern.test(fileName) || pattern.test(fPath)) {
        findings.push({
          file: fPath,
          line: 1,
          column: 1,
          ruleId: "no-excessive-scope-drift",
          category: "scope_drift",
          severity: "warning",
          message: `Sensitive boundary file '${fPath}' modified unexpectedly by agent`,
          snippet: fPath,
          fixable: false,
          suggestion: "Confirm whether critical environment or configuration file was meant to be changed"
        });
      }
    }
  }
  return findings;
}