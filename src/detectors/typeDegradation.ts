import { SlopFinding, DiffFile } from "../types.js";
import { PatternRule, scanDiffWithRules, scanContentWithRules } from "./scanner.js";
const K_TS_IGNORE = ["@", "ts", "-", "ignore"].join("");
const K_TS_NOCHECK = ["@", "ts", "-", "nocheck"].join("");
const K_ANY = ["a", "n", "y"].join("");
const TYPE_RULES: PatternRule[] = [
  {
    regex: new RegExp(`${K_TS_IGNORE}|${K_TS_NOCHECK}`),
    ruleId: "no-type-safety-degradation",
    category: "type_degradation",
    severity: "error",
    message: "TypeScript safety completely bypassed via compiler suppression flags",
    fixable: false,
    suggestion: "Provide concrete types or interfaces instead of bypassing type safety"
  },
  {
    regex: new RegExp(`:\\s*${K_ANY}\\b|as\\s+${K_ANY}\\b|<${K_ANY}>`),
    ruleId: "no-type-safety-degradation",
    category: "type_degradation",
    severity: "error",
    message: "Strict typing degraded to untyped escape hatch",
    fixable: false,
    suggestion: "Declare explicit interfaces or generics instead of dynamic untyped escapes"
  },
  {
    regex: new RegExp(`as\\s+unknown\\s+as\\s+${K_ANY}`),
    ruleId: "no-type-safety-degradation",
    category: "type_degradation",
    severity: "error",
    message: "Double-cast escape hatch used to evade compiler verification",
    fixable: false,
    suggestion: "Refactor signature to satisfy source types directly"
  },
  {
    regex: /#\s*type:\s*ignore/,
    ruleId: "no-type-safety-degradation",
    category: "type_degradation",
    severity: "error",
    message: "Python static type checker suppressed via comment directive",
    fixable: false,
    suggestion: "Fix Python type signature rather than suppressing pyright or mypy"
  },
  {
    regex: /\/\*\s*eslint-disable\s*\*\/|\/\/\s*eslint-disable-line/,
    ruleId: "no-type-safety-degradation",
    category: "type_degradation",
    severity: "error",
    message: "Linter rules suppressed by agent",
    fixable: false,
    suggestion: "Fix the linter violation instead of disabling lint checks"
  }
];
const CODE_FILE_FILTER = (p: string) => /\.(ts|tsx|js|jsx|py|go|rs)$/.test(p);
export function detectTypeDegradationInDiff(diffFiles: DiffFile[]): SlopFinding[] {
  return scanDiffWithRules(diffFiles, TYPE_RULES, CODE_FILE_FILTER);
}
export function detectTypeDegradationInContent(filePath: string, content: string): SlopFinding[] {
  return scanContentWithRules(filePath, content, TYPE_RULES, CODE_FILE_FILTER);
}