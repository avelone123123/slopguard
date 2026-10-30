import * as fs from "node:fs";
import * as path from "node:path";
import { SlopConfig } from "./types.js";
export const DEFAULT_CONFIG: SlopConfig = {
  ignore: ["node_modules", ".git", "dist", "build", "coverage", ".next", ".turbo", "vendor", "*.min.js", "*.lock", "package-lock.json", "*.md", "*.svg", "test/**", "tests/**", "*.test.*", "*.spec.*"],
  rules: {
    "no-ai-todo-placeholders": "error",
    "no-silent-catch-pass": "error",
    "no-dummy-secrets-mocks": "error",
    "no-type-safety-degradation": "error",
    "no-ghost-dependencies": "error",
    "no-duplicate-shadow-utilities": "warning",
    "no-ai-conversational-bloat": "warning",
    "no-excessive-scope-drift": "warning"
  },
  maxDriftFiles: 12,
  allowedGhostDeps: []
};
export function loadConfig(projectRoot: string): SlopConfig {
  const configPaths = [path.join(projectRoot, ".slopguardrc.json"), path.join(projectRoot, ".slopguardrc"), path.join(projectRoot, "slopguard.config.json")];
  for (const cp of configPaths) {
    if (fs.existsSync(cp)) {
      try {
        const raw = fs.readFileSync(cp, "utf-8");
        const parsed = JSON.parse(raw);
        return {
          ignore: Array.isArray(parsed.ignore) ? [...DEFAULT_CONFIG.ignore, ...parsed.ignore] : DEFAULT_CONFIG.ignore,
          rules: { ...DEFAULT_CONFIG.rules, ...(parsed.rules || {}) },
          maxDriftFiles: typeof parsed.maxDriftFiles === "number" ? parsed.maxDriftFiles : DEFAULT_CONFIG.maxDriftFiles,
          allowedGhostDeps: Array.isArray(parsed.allowedGhostDeps) ? parsed.allowedGhostDeps : DEFAULT_CONFIG.allowedGhostDeps
        };
      } catch {
        return DEFAULT_CONFIG;
      }
    }
  }
  return DEFAULT_CONFIG;
}
export function shouldIgnorePath(filePath: string, ignorePatterns: string[]): boolean {
  const normalized = filePath.replace(/\\/g, "/");
  return ignorePatterns.some(pattern => {
    const cleanPattern = pattern.replace(/^\.\//, "").replace(/\\/g, "/");
    if (cleanPattern.endsWith("/**")) {
      const baseDir = cleanPattern.slice(0, -3);
      return normalized === baseDir || normalized.startsWith(`${baseDir}/`) || normalized.includes(`/${baseDir}/`);
    }
    if (cleanPattern.startsWith("*.")) {
      const ext = cleanPattern.slice(1);
      return normalized.endsWith(ext) || normalized.includes(ext);
    }
    return normalized.includes(`/${cleanPattern}/`) || normalized.startsWith(`${cleanPattern}/`) || normalized === cleanPattern;
  });
}