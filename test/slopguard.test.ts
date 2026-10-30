import test from "node:test";
import assert from "node:assert/strict";
import { detectPlaceholdersInContent } from "../dist/detectors/placeholders.js";
import { detectTypeDegradationInContent } from "../dist/detectors/typeDegradation.js";
import { detectGhostDependenciesInContent } from "../dist/detectors/ghostDependencies.js";
import { detectAiCommentBloatInContent } from "../dist/detectors/aiCommentBloat.js";
import { sanitizeCode } from "../dist/fixers/sanitizer.js";
import { computeSummary } from "../dist/detectors/index.js";
import { generateBadgeSvg } from "../dist/reporters/svg.js";
test("detects unfinished AI placeholders and dummy secrets", () => {
  const badCode = ["const apiKey = \"sk-1234567890123456789012\";", "function execute() {", "  " + ["/", "/", " ", "T", "O", "D", "O", ": implement later"].join(""), "  throw new Error(\"not implemented yet\");", "}", "try { run(); } catch (err) {}"].join("\n");
  const findings = detectPlaceholdersInContent("test.ts", badCode);
  assert.ok(findings.length >= 3);
  assert.ok(findings.some(f => f.ruleId === "no-ai-todo-placeholders"));
  assert.ok(findings.some(f => f.ruleId === "no-silent-catch-pass"));
  assert.ok(findings.some(f => f.ruleId === "no-dummy-secrets-mocks"));
});
test("detects type safety degradations and escape hatches", () => {
  const badTypes = ["  " + ["/", "/", " ", "@", "ts-", "ignore"].join(""), "  const user: any = fetchUser();", "  const data = raw as unknown as any;"].join("\n");
  const findings = detectTypeDegradationInContent("test.ts", badTypes);
  assert.ok(findings.length >= 2);
  assert.ok(findings.every(f => f.ruleId === "no-type-safety-degradation"));
});
test("detects ghost dependencies not in package.json", () => {
  const code = "import axios from \"axios\";\nimport { merge } from \"lodash-es\";";
  const findings = detectGhostDependenciesInContent("test.ts", code, process.cwd(), []);
  assert.ok(findings.length >= 2);
  assert.ok(findings.some(f => f.message.includes("axios")));
  assert.ok(findings.some(f => f.message.includes("lodash-es")));
});
test("detects and sanitizes conversational AI comment bloat", () => {
  const bloatCode = ["// Here is the updated code:", "function sum(a: number, b: number) {", "// In order to calculate", "return a + b;", "}"].join("\n");
  const findings = detectAiCommentBloatInContent("test.ts", bloatCode);
  assert.ok(findings.length >= 2);
  const { cleaned, modified } = sanitizeCode(bloatCode);
  assert.equal(modified, true);
  assert.ok(!cleaned.includes("Here is the updated code"));
  assert.ok(cleaned.includes("return a + b"));
});
test("computes accurate AI hygiene scores and SVG badges", () => {
  const summary = computeSummary([], 5, 42);
  assert.equal(summary.score, 100);
  assert.equal(summary.totalIssues, 0);
  const badge = generateBadgeSvg(summary.score, summary.errors);
  assert.ok(badge.includes("100% clean"));
  assert.ok(badge.includes("<svg"));
});