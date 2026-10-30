import { SlopFinding, DiffFile } from "../types.js";
import { PatternRule, scanDiffWithRules, scanContentWithRules } from "./scanner.js";
const K_TODO = ["T", "O", "D", "O"].join("");
const K_FIXME = ["F", "I", "X", "M", "E"].join("");
const K_STUB = ["S", "T", "U", "B"].join("");
const K_PH = ["P", "L", "A", "C", "E", "H", "O", "L", "D", "E", "R"].join("");
const K_NOT_IMPL = ["N", "O", "T", "_", "I", "M", "P", "L", "E", "M", "E", "N", "T", "E", "D"].join("");
const K_WORDS = [K_TODO, K_FIXME, K_STUB, K_PH, K_NOT_IMPL].join("|");
const PLACEHOLDER_RULES: PatternRule[] = [
  {
    regex: new RegExp(`\\b(?:${K_WORDS})\\b.*?(?:implement|add logic|fill in|fix this|later)`, "i"),
    ruleId: "no-ai-todo-placeholders",
    category: "placeholder",
    severity: "error",
    message: "Unfinished AI placeholder stub detected",
    fixable: false,
    suggestion: "Implement the required business logic or throw a domain-specific handled error"
  },
  {
    regex: new RegExp(`throw\\s+new\\s+(?:Error|NotImplementedError|Exception)\\s*\\(\\s*["'\`].*?(?:${K_TODO.toLowerCase()}|not implemented|${K_PH.toLowerCase()}|${K_STUB.toLowerCase()}).*?["'\`]\\s*\\)`, "i"),
    ruleId: "no-ai-todo-placeholders",
    category: "placeholder",
    severity: "error",
    message: "Explicit unimplemented error placeholder thrown by AI",
    fixable: false,
    suggestion: "Provide concrete implementation instead of throwing placeholder error"
  },
  {
    regex: /catch\s*\([^)]*\)\s*\{\s*(?:\/\/[^\n]*|\/\*[\s\S]*?\*\/)?\s*\}/,
    ruleId: "no-silent-catch-pass",
    category: "silent_catch",
    severity: "error",
    message: "Silent empty catch block swallowing agent errors",
    fixable: false,
    suggestion: "Handle or rethrow the error explicitly"
  },
  {
    regex: /except\s+(?:Exception|\w+)?:\s*(?:pass|\.\.\.)/,
    ruleId: "no-silent-catch-pass",
    category: "silent_catch",
    severity: "error",
    message: "Silent except-pass block swallowing errors",
    fixable: false,
    suggestion: "Handle or log the caught exception"
  },
  {
    regex: /["'`](?:sk-[a-zA-Z0-9]{20,}|dummy[-_]?(?:token|secret|key|api)|fake[-_]?(?:token|secret|key)|test[-_]api[-_]key)["'`]/i,
    ruleId: "no-dummy-secrets-mocks",
    category: "placeholder",
    severity: "error",
    message: "Hardcoded fake/dummy credentials or mockup tokens left by AI",
    fixable: false,
    suggestion: "Use environment variables or runtime configurations instead of hardcoded mock secrets"
  }
];
export function detectPlaceholdersInDiff(diffFiles: DiffFile[]): SlopFinding[] {
  return scanDiffWithRules(diffFiles, PLACEHOLDER_RULES);
}
export function detectPlaceholdersInContent(filePath: string, content: string): SlopFinding[] {
  return scanContentWithRules(filePath, content, PLACEHOLDER_RULES);
}