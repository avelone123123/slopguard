export type SlopCategory = "placeholder" | "duplicate" | "type_degradation" | "ghost_dependency" | "scope_drift" | "ai_bloat" | "silent_catch";
export type SlopSeverity = "error" | "warning" | "info";
export interface SlopFinding {
  file: string;
  line: number;
  column: number;
  ruleId: string;
  category: SlopCategory;
  severity: SlopSeverity;
  message: string;
  snippet: string;
  fixable: boolean;
  suggestion?: string;
}
export interface SlopSummary {
  totalIssues: number;
  errors: number;
  warnings: number;
  score: number;
  findings: SlopFinding[];
  scannedFiles: number;
  durationMs: number;
}
export interface SlopConfig {
  ignore: string[];
  rules: Record<string, "error" | "warning" | "off">;
  maxDriftFiles: number;
  allowedGhostDeps: string[];
}
export interface DiffLine {
  type: "add" | "del" | "normal";
  content: string;
  oldLineNumber?: number;
  newLineNumber?: number;
}
export interface DiffChunk {
  oldStart: number;
  oldLines: number;
  newStart: number;
  newLines: number;
  lines: DiffLine[];
}
export interface DiffFile {
  oldPath: string;
  newPath: string;
  chunks: DiffChunk[];
  isNew: boolean;
  isDeleted: boolean;
}
export interface FunctionSignature {
  name: string;
  file: string;
  line: number;
  paramCount: number;
  bodySnippet: string;
  tokenSet: Set<string>;
}