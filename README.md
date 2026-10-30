<div align="center">
  <img src="assets/logo.svg" alt="SlopGuard Logo" width="100%">
  <br>

  **English** | [Русский](README.ru.md) | [简体中文](README.zh.md)

  <br>

  <img alt="100% Rust" src="https://img.shields.io/badge/100%25-Rust-b7410e?style=flat-square&logo=rust">
  <img alt="License: MIT" src="https://img.shields.io/badge/license-MIT-263146?style=flat-square">
  <img alt="Latency: <5ms" src="https://img.shields.io/badge/latency-%3C5ms-2f6f4f?style=flat-square">
  <img alt="MCP Protocol: 1.0" src="https://img.shields.io/badge/MCP-JSON--RPC%202.0-7c3aed?style=flat-square">
  <img alt="Platforms" src="https://img.shields.io/badge/platforms-Windows%20%C2%B7%20macOS%20%C2%B7%20Linux-2f7bf5?style=flat-square">

  <br>

  <h3>Stop AI agents from rotting your codebase with unverified slop.</h3>

  <p>
    <b>SlopGuard</b> is an ultra-fast, zero-overhead <b>pure Rust</b> firewall and <b>Model Context Protocol (MCP)</b> engine<br>
    that catches AI hallucinations, duplicate shadow utilities, type escapes, and dummy placeholders<br>
    <b>before</b> they pollute your git history.
  </p>

  <code>cargo install slopguard</code>

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

- **100% Native Rust**: Single standalone binary without runtime bloat. Instant startup (<5ms).
- **Git Diff & AST-Token Stream Analysis**: Scans only changed lines in working tree or staged commits using Jaccard token similarity.
- **Built-in Model Context Protocol (MCP)**: Plugs directly into Cursor, Claude Code, and Windsurf so the agent inspects and validates its own diffs before completing prompts.
- **One-Command Auto-Sanitizer**: `slopguard fix` automatically cleans conversational AI bloat.
- **Pre-commit Hook in 1-Second**: Installs a zero-friction git guardrail with `slopguard hook install`.
- **Dynamic AI Hygiene Score (0-100)**: Visual terminal reporting with severity breakdown, grade, and exportable SVG badges.

---

## Quick Start

### Installation

```bash
cargo install slopguard
```

Or build from source:

```bash
git clone https://github.com/avelo/slopguard.git
cd slopguard
cargo build --release
```

### Common Commands

```bash
# Launch interactive native terminal software monitor (or double-click slopguard.exe)
slopguard monitor

# Scan current uncommitted changes or working tree
slopguard check

# Launch live real-time web telemetry dashboard
slopguard dashboard

# Scan only staged git changes (ideal for git pre-commit hooks)
slopguard check --staged

# Live watch mode while vibe-coding (re-scans on every save)
slopguard watch

# Generate a recommended .slopguardrc.json config
slopguard init

# Output structured JSON for tooling / IDEs
slopguard check --json

# Emit GitHub Actions workflow annotations
slopguard check --github

# Generate GitHub Actions CI workflow in 1 second
slopguard ci

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

SlopGuard includes an official STDIO **Model Context Protocol (MCP)** server built right into the binary.

### For Cursor

Add the following to your `mcp.json` (`~/.cursor/mcp.json` or project `.cursor/mcp.json`):

```json
{
  "mcpServers": {
    "slopguard": {
      "command": "slopguard",
      "args": ["mcp"]
    }
  }
}
```

### For Claude Code

Run:

```bash
claude mcp add slopguard slopguard mcp
```

### Exposed MCP Tools

- `slopguard_scan_diff`: Allows the agent to inspect the current git diff for unfinished stubs and type degradations.
- `slopguard_check_snippet`: Validates an in-memory code block before writing it to a file.
- `slopguard_sanitize_file`: Automatically cleans AI comment artifacts from a target file.

---

## Native Terminal Software Monitor (`slopguard monitor`)

SlopGuard comes with an interactive, full-screen **native terminal TUI application** compiled directly into the binary. Simply double-click `slopguard.exe` in Windows Explorer or run:

```bash
slopguard monitor
# or run without arguments:
slopguard
```

- **5 Multi-Function Diagnostic Views**:
  - `[1] OVERVIEW`: Real-time moving telemetry wave oscilloscope, live MCP activity stream, hygiene gauge, and latency counters.
  - `[2] ISSUES`: Interactive findings inspector with `↑`/`↓` navigation, code snippets, and AI fix recommendations.
  - `[3] MCP STREAM`: Deep inspection of incoming MCP tool calls (`slopguard_scan_diff`, `slopguard_check_snippet`, `slopguard_sanitize_file`).
  - `[4] FIREWALLS`: 13 active AST rule firewalls matrix with real-time enforcement status.
  - `[5] REPOSITORY`: Git diagnostic status, active branch, latest commit, and working tree breakdown.
- **Real-Time Animated Wave Oscilloscope**: Moving multi-level ASCII/ANSI frequency spectrum visualizing MCP agent pulse and latency in real time.
- **Single-Key Controls**:
  - `[1..5]` or `[Tab]` Switch diagnostic views instantly
  - `[↑/↓]` or `[j/k]` Navigate issue and event logs
  - `[S]` Instant full repository audit
  - `[F]` One-key conversational AI bloat sanitizer
  - `[W]` Toggle live file change watcher
  - `[E]` Export audit diagnostics report (`slopguard-audit-report.json`)
  - `[R]` Force refresh telemetry log
  - `[Q]` Clean exit

---

## Live Visual Observer Dashboard (`slopguard ui`)

Observe what your AI agent (Cursor, Claude Code, Windsurf, Antigravity) is doing in real time.

```bash
slopguard dashboard
# or
slopguard ui
```

Opens a fast, embedded local web interface at `http://127.0.0.1:4545`:
- **Live Activity Feed**: Tracks all incoming MCP inspections with microsecond resolution.
- **AI Hygiene Meter**: Dynamic score and grade updated as code is generated.
- **Blocked Slop Counter**: Real-time counter of intercepted placeholders, dummy tokens, and type degradation.
- **Sub-millisecond Latency**: Telemetry sparklines verifying zero impact on developer flow.

---

## Rules and Detections

| Rule ID | Severity | What It Catches |
| :--- | :--- | :--- |
| `no-ai-todo-placeholders` | `ERROR` | Unfinished TODO stubs, `throw new Error("not implemented")` |
| `no-silent-catch-pass` | `ERROR` | Empty `catch(e) {}` blocks and Python `except: pass` |
| `no-dummy-secrets-mocks` | `ERROR` | Mock tokens (`sk-proj-dummy...`, `dummy_api_key`) |
| `no-mock-hallucinations` | `ERROR` | Public placeholder APIs (`jsonplaceholder`, mock JWTs, test emails) |
| `no-hallucinated-imports` | `ERROR` | Hallucinated package exports (`useLocalStorage`, `createClient`, etc.) |
| `no-phantom-api-routes` | `ERROR` | Client API calls with no matching backend endpoint |
| `no-ghost-env-vars` | `ERROR` | Environment variables missing from `.env.example` / `.env` template |
| `no-deep-nesting-sprawl` | `WARN` | Deep pyramid-of-doom control flow nesting (>= 5 levels) |
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

| Tool | Cold Startup | Scan Time (10k LOC Diff) | Memory Usage |
| :--- | :--- | :--- | :--- |
| **SlopGuard (Rust)** | **< 3ms** | **~8ms** | **~6 MB** |
| Biome | ~45ms | ~30ms | ~28 MB |
| ESLint | ~620ms | ~480ms | ~140 MB |
| SonarQube Scanner | ~4,200ms | ~3,100ms | ~600 MB |

---

## License

MIT © [avelo](https://github.com/avelo)
