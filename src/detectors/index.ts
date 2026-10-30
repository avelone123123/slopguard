import { SlopFinding, SlopConfig, DiffFile, SlopSummary, SlopSeverity } from "../types.js";
import { detectPlaceholdersInDiff, detectPlaceholdersInContent } from "./placeholders.js";
import { detectTypeDegradationInDiff, detectTypeDegradationInContent } from "./typeDegradation.js";
import { detectGhostDependenciesInDiff, detectGhostDependenciesInContent } from "./ghostDependencies.js";
import { detectDuplicateUtilities, indexExistingFunctions } from "./duplicates.js";
import { detectScopeDrift } from "./scopeDrift.js";
import { detectAiCommentBloatInDiff, detectAiCommentBloatInContent } from "./aiCommentBloat.js";
export function runAllDiffDetectors(diffFiles: DiffFile[], projectRoot: string, config: SlopConfig): SlopFinding[] {
  const allFindings: SlopFinding[] = [];
  if (config.rules["no-ai-todo-placeholders"] !== "off" || config.rules["no-silent-catch-pass"] !== "off" || config.rules["no-dummy-secrets-mocks"] !== "off") {
    allFindings.push(...detectPlaceholdersInDiff(diffFiles));
  }
  if (config.rules["no-type-safety-degradation"] !== "off") {
    allFindings.push(...detectTypeDegradationInDiff(diffFiles));
  }
  if (config.rules["no-ghost-dependencies"] !== "off") {
    allFindings.push(...detectGhostDependenciesInDiff(diffFiles, projectRoot, config.allowedGhostDeps));
  }
  if (config.rules["no-excessive-scope-drift"] !== "off") {
    allFindings.push(...detectScopeDrift(diffFiles, config.maxDriftFiles));
  }
  if (config.rules["no-ai-conversational-bloat"] !== "off") {
    allFindings.push(...detectAiCommentBloatInDiff(diffFiles));
  }
  if (config.rules["no-duplicate-shadow-utilities"] !== "off") {
    const existingSignatures = indexExistingFunctions(projectRoot, config.ignore);
    allFindings.push(...detectDuplicateUtilities(diffFiles, existingSignatures));
  }
  return allFindings.filter(f => config.rules[f.ruleId] !== "off").map(f => ({
    ...f,
    severity: (config.rules[f.ruleId] === "error" || config.rules[f.ruleId] === "warning" ? config.rules[f.ruleId] : f.severity) as SlopSeverity
  }));
}
export function runAllContentDetectors(filePath: string, content: string, projectRoot: string, config: SlopConfig): SlopFinding[] {
  const allFindings: SlopFinding[] = [];
  allFindings.push(...detectPlaceholdersInContent(filePath, content));
  allFindings.push(...detectTypeDegradationInContent(filePath, content));
  allFindings.push(...detectGhostDependenciesInContent(filePath, content, projectRoot, config.allowedGhostDeps));
  allFindings.push(...detectAiCommentBloatInContent(filePath, content));
  return allFindings.filter(f => config.rules[f.ruleId] !== "off").map(f => ({
    ...f,
    severity: (config.rules[f.ruleId] === "error" || config.rules[f.ruleId] === "warning" ? config.rules[f.ruleId] : f.severity) as SlopSeverity
  }));
}
export function computeSummary(findings: SlopFinding[], scannedFiles: number, durationMs: number): SlopSummary {
  let errors = 0;
  let warnings = 0;
  for (const f of findings) {
    if (f.severity === "error") errors++;
    else if (f.severity === "warning") warnings++;
  }
  const penalty = (errors * 15) + (warnings * 5);
  const score = Math.max(0, 100 - penalty);
  return {
    totalIssues: findings.length,
    errors,
    warnings,
    score,
    findings,
    scannedFiles,
    durationMs
  };
}