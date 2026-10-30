import * as readline from "node:readline";
import { getGitRoot, getDiffOutput, parseUnifiedDiff } from "./git.js";
import { loadConfig } from "./config.js";
import { runAllDiffDetectors, runAllContentDetectors, computeSummary } from "./detectors/index.js";
import { sanitizeFile, sanitizeCode } from "./fixers/sanitizer.js";
export function startMcpServer(): void {
  const rl = readline.createInterface({ input: process.stdin, output: process.stdout, terminal: false });
  rl.on("line", (line: string) => {
    const trimmed = line.trim();
    if (!trimmed) return;
    try {
      const request = JSON.parse(trimmed);
      handleMcpMessage(request);
    } catch {}
  });
}
function sendMcpResponse(response: Record<string, unknown>): void {
  process.stdout.write(JSON.stringify(response) + "\n");
}
function handleMcpMessage(msg: Record<string, unknown>): void {
  const id = msg.id;
  const method = msg.method as string;
  if (method === "initialize") {
    sendMcpResponse({
      jsonrpc: "2.0",
      id,
      result: {
        protocolVersion: "2024-11-05",
        capabilities: { tools: {} },
        serverInfo: { name: "slopguard", version: "1.0.0" }
      }
    });
    return;
  }
  if (method === "notifications/initialized") {
    return;
  }
  if (method === "ping") {
    sendMcpResponse({ jsonrpc: "2.0", id, result: {} });
    return;
  }
  if (method === "tools/list") {
    sendMcpResponse({
      jsonrpc: "2.0",
      id,
      result: {
        tools: [
          {
            name: "slopguard_scan_diff",
            description: "Scan active working tree or staged git diff for AI-generated code slop, type bypasses, dummy tokens, and duplicate utilities",
            inputSchema: {
              type: "object",
              properties: {
                stagedOnly: { type: "boolean", description: "Only inspect staged git changes" }
              }
            }
          },
          {
            name: "slopguard_check_snippet",
            description: "Inspect an in-memory code snippet for AI placeholders, type degradation, and conversational comment bloat before writing it to disk",
            inputSchema: {
              type: "object",
              properties: {
                filePath: { type: "string", description: "Target relative file path (e.g. src/auth.ts)" },
                code: { type: "string", description: "Raw code snippet to analyze" }
              },
              required: ["filePath", "code"]
            }
          },
          {
            name: "slopguard_sanitize_file",
            description: "Automatically remove AI conversational comment bloat and redundant placeholders from a file",
            inputSchema: {
              type: "object",
              properties: {
                filePath: { type: "string", description: "File path to clean" }
              },
              required: ["filePath"]
            }
          }
        ]
      }
    });
    return;
  }
  if (method === "tools/call") {
    const params = (msg.params || {}) as { name: string; arguments?: Record<string, unknown> };
    const toolName = params.name;
    const args = params.arguments || {};
    const cwd = process.cwd();
    const config = loadConfig(cwd);
    if (toolName === "slopguard_scan_diff") {
      const startTime = Date.now();
      const diffOutput = getDiffOutput(cwd, Boolean(args.stagedOnly));
      const diffFiles = parseUnifiedDiff(diffOutput);
      const findings = runAllDiffDetectors(diffFiles, cwd, config);
      const summary = computeSummary(findings, diffFiles.length, Date.now() - startTime);
      sendMcpResponse({
        jsonrpc: "2.0",
        id,
        result: {
          content: [{ type: "text", text: JSON.stringify(summary, null, 2) }]
        }
      });
      return;
    }
    if (toolName === "slopguard_check_snippet") {
      const startTime = Date.now();
      const filePath = String(args.filePath || "snippet.ts");
      const code = String(args.code || "");
      const findings = runAllContentDetectors(filePath, code, cwd, config);
      const summary = computeSummary(findings, 1, Date.now() - startTime);
      sendMcpResponse({
        jsonrpc: "2.0",
        id,
        result: {
          content: [{ type: "text", text: JSON.stringify(summary, null, 2) }]
        }
      });
      return;
    }
    if (toolName === "slopguard_sanitize_file") {
      const filePath = String(args.filePath || "");
      const success = sanitizeFile(filePath);
      sendMcpResponse({
        jsonrpc: "2.0",
        id,
        result: {
          content: [{ type: "text", text: JSON.stringify({ success, filePath }) }]
        }
      });
      return;
    }
    sendMcpResponse({
      jsonrpc: "2.0",
      id,
      error: { code: -32601, message: `Unknown tool '${toolName}'` }
    });
    return;
  }
  if (id !== undefined) {
    sendMcpResponse({ jsonrpc: "2.0", id, error: { code: -32601, message: "Method not found" } });
  }
}