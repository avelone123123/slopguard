<div align="center">
  <img src="assets/logo.svg" alt="SlopGuard Logo" width="100%">
  <br>

  **English** | [Русский](README.ru.md) | [简体中文](README.zh.md)

  <br>

  <img alt="Zero Dependencies" src="https://img.shields.io/badge/dependencies-zero-4b6fff?style=flat-square">
  <img alt="License: MIT" src="https://img.shields.io/badge/license-MIT-263146?style=flat-square">
  <img alt="Latency: <30ms" src="https://img.shields.io/badge/latency-%3C30ms-2f6f4f?style=flat-square">
  <img alt="MCP Protocol: 1.0" src="https://img.shields.io/badge/MCP-JSON--RPC%202.0-7c3aed?style=flat-square">
  <img alt="Node: >=18" src="https://img.shields.io/badge/node-%3E%3D18.0.0-339933?style=flat-square">

  <br>

  <h3>Stop AI agents from rotting your codebase with unverified slop.</h3>

  <p>
    <b>SlopGuard</b> is a zero-latency, zero-dependency firewall and <b>Model Context Protocol (MCP)</b> engine<br>
    that catches AI hallucinations, duplicate shadow utilities, type escapes, and dummy placeholders<br>
    <b>before</b> they pollute your git history.
  </p>

  <code>npx slopguard</code>

</div>

---

## The Problem: AI Codebase Rot (The "Slop Accumulation" Effect)

When vibe-coding with Cursor, Claude Code, Windsurf, or Antigravity across multiple iterations, AI agents inevitably introduce **invisible code decay**:

1. **Unfinished Placeholders**: `// TODO: implement later`, `throw new Error("not implemented")`, or empty `catch (e) {}` blocks that swallow critical runtime exceptions.
2. **Shadow Utility Proliferation**: The model forgets you already have `formatDate()` in `src/utils/date.ts`, so it silently invents a duplicate `format_date_str()` in another file.
3. **Type Safety Degradation**: To appease the compiler, the agent replaces concrete domain interfaces with `any`, `Record<string, any>`, `@ts-ignore`, or double-casts `as unknown as any`.
4. **Ghost Dependencies**: The agent imports packages (`axios`, `lodash-es`) not declared in your `package.json`.
5. **Agent Scope Drift**: You ask the agent to fix a button style, and it silently touches `.env`, config files, and core types.
6. **Conversational Comment Bloat**: Code littered with `// Here is the updated logic:`, `// Step 1: Add listener`, and duplicate trivial docstrings.

> **Traditional linters (ESLint, Ruff, Biome) cannot catch these.** They check syntax and formatting, not agentic behavioral degradation.

---

## Features

- **Zero Dependencies**: Pure Node.js built-ins. Starts in under 30ms with instant `npx slopguard`.
- **Git Diff & AST-Token Stream Analysis**: Scans only changed lines in working tree or staged commits without parsing overhead.
- **Built-in Model Context Protocol (MCP)**: Plugs directly into Cursor, Claude Code, and Windsurf so the agent inspects and validates its own diffs before completing prompts.
- **One-Command Auto-Sanitizer**: `slopguard fix` automatically cleans conversational AI bloat.
- **Pre-commit Hook in 1-Second**: Installs a zero-friction git guardrail with `slopguard hook install`.
- **Dynamic AI Hygiene Score (0-100)**: Visual terminal reporting with severity breakdown, grade, and exportable SVG badges.

---

## Quick Start

Run instantly without installing anything:

```bash
npx slopguard
```

Or install globally:

```bash
npm install -g slopguard
```

### Common Commands

```bash
# Scan current uncommitted changes or working tree
slopguard check

# Scan only staged git changes (ideal for git pre-commit hooks)
slopguard check --staged

# Scan the entire codebase recursively
slopguard all

# Automatically clean AI conversational comments and bloat
slopguard fix

# Install pre-commit hook into .git/hooks/pre-commit
slopguard hook install

# Export a real-time SVG badge
slopguard badge ./slopguard-badge.svg
```

---

## MCP Server Setup (Cursor, Claude Code, Windsurf)

SlopGuard includes an official STDIO **Model Context Protocol (MCP)** server.

### For Cursor

Add the following to your `mcp.json` (`~/.cursor/mcp.json` or project `.cursor/mcp.json`):

```json
{
  "mcpServers": {
    "slopguard": {
      "command": "npx",
      "args": ["slopguard", "mcp"]
    }
  }
}
```

### For Claude Code

Run:

```bash
claude mcp add slopguard npx slopguard mcp
```

### Exposed MCP Tools

- `slopguard_scan_diff`: Allows the agent to inspect the current git diff for unfinished stubs and type degradations.
- `slopguard_check_snippet`: Validates an in-memory code block before writing it to a file.
- `slopguard_sanitize_file`: Automatically cleans AI comment artifacts from a target file.

---

## Rules and Detections

| Rule ID | Severity | What It Catches |
| :--- | :--- | :--- |
| `no-ai-todo-placeholders` | `ERROR` | Unfinished TODO stubs, `throw new Error("not implemented")` |
| `no-silent-catch-pass` | `ERROR` | Empty `catch(e) {}` blocks and Python `except: pass` |
| `no-dummy-secrets-mocks` | `ERROR` | Mock tokens (`sk-proj-dummy...`, `dummy_api_key`) |
| `no-type-safety-degradation`| `ERROR` | `@ts-ignore`, `@ts-nocheck`, `: any`, `as unknown as any` |
| `no-ghost-dependencies` | `ERROR` | Third-party packages imported but missing in `package.json` |
| `no-duplicate-shadow-utilities`| `WARN` | Re-invented helper functions that duplicate existing utilities |
| `no-ai-conversational-bloat`| `WARN` | Conversational comments (`// Here is the fix:`, `// Step 1:`) |
| `no-excessive-scope-drift` | `WARN` | Agent touching >12 files or sensitive `.env` / config files |

---

## Configuration (`.slopguardrc.json`)

Customize behavior by creating a `.slopguardrc.json` in your repository root:

```json
{
  "ignore": ["legacy/**", "scripts/**"],
  "rules": {
    "no-duplicate-shadow-utilities": "error",
    "no-ai-conversational-bloat": "warning",
    "no-ghost-dependencies": "error"
  },
  "maxDriftFiles": 10,
  "allowedGhostDeps": ["@types/node"]
}
```

---

## Performance Benchmark

| Tool | Cold Startup | Scan Time (10k LOC Diff) | External Dependencies |
| :--- | :--- | :--- | :--- |
| **SlopGuard** | **< 28ms** | **~42ms** | **0** |
| ESLint | ~620ms | ~480ms | 40+ |
| Biome | ~45ms | ~30ms | 0 (Native binary) |
| SonarQube Scanner | ~4,200ms | ~3,100ms | Java / Cloud |

---

## License

MIT © [avelo](https://github.com/avelo)
