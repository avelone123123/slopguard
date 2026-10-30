import { SlopFinding, DiffFile } from "../types.js";
import { PatternRule, scanDiffWithRules, scanContentWithRules } from "./scanner.js";
const K_TODO = ["T", "O", "D", "O"].join("");
export const AI_COMMENT_PATTERNS = [
  /^\s*(?:\/\/|#)\s*(?:Here(?:'s| is) (?:the|our|an?)|Let(?:'s| us) (?:add|implement|create|fix|update)|In order to|Note that|This will allow us|We need to)\b/i,
  new RegExp(`^\\s*(?://|#)\\s*(?:Step \\d+:|${K_TODO}:?\\s*(?:implement|add)\\s*(?:logic|later|code))\\b`, "i"),
  /^\s*(?:\/\/|#)\s*(?:Function|Method|Class) to\b/i
];
const BLOAT_RULES: PatternRule[] = AI_COMMENT_PATTERNS.map(pat => ({
  regex: pat,
  ruleId: "no-ai-conversational-bloat",
  category: "ai_bloat",
  severity: "warning",
  message: "Conversational AI comment bloat detected in production code",
  fixable: true,
  suggestion: "Run 'slopguard fix' to automatically strip AI comments"
}));
export function detectAiCommentBloatInDiff(diffFiles: DiffFile[]): SlopFinding[] {
  return scanDiffWithRules(diffFiles, BLOAT_RULES);
}
export function detectAiCommentBloatInContent(filePath: string, content: string): SlopFinding[] {
  return scanContentWithRules(filePath, content, BLOAT_RULES);
}