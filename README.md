<div align="center">
  <img src="assets/logo.svg" alt="SlopGuard Logo" width="100%">
  <br>

  **English** | [Русский](README.ru.md) | [简体中文](README.zh.md) | [**Website &amp; Docs**](https://avelone123123.github.io/slopguard-site/) | [**Discord**](https://discord.gg/z7C9dprye) | [**Telegram**](https://t.me/SlopGuard)

  <br>

  <a href="https://discord.gg/z7C9dprye"><img alt="Discord" src="https://img.shields.io/badge/Discord-Join%20Chat-5865F2?style=flat-square&logo=discord&logoColor=white"></a>
  <a href="https://t.me/SlopGuard"><img alt="Telegram" src="https://img.shields.io/badge/Telegram-@SlopGuard-26A5E4?style=flat-square&logo=telegram&logoColor=white"></a>
  [![M8ven Score](https://m8ven.ai/badge/mcp/avelone123123-slopguard-ddb87e)](https://m8ven.ai/mcp/avelone123123-slopguard-ddb87e?s=readme)
  <img alt="100% Rust" src="https://img.shields.io/badge/100%25-Rust-b7410e?style=flat-square&logo=rust">
  <a href="https://github.com/avelone123123/slopguard/releases/tag/v3.5.382"><img alt="Release: v3.5.382" src="https://img.shields.io/badge/release-v3.5.382-10b981?style=flat-square"></a>
  <img alt="Tests: 642 Passed" src="https://img.shields.io/badge/tests-642%20passed%20(100%25)-10b981?style=flat-square">
  <img alt="License: MIT" src="https://img.shields.io/badge/license-MIT-263146?style=flat-square">
  <img alt="Latency: <0.35ms" src="https://img.shields.io/badge/latency-%3C0.35ms-2f6f4f?style=flat-square">
  <img alt="MCP Protocol: 2.0" src="https://img.shields.io/badge/MCP-JSON--RPC%202.0-7c3aed?style=flat-square">
  <img alt="Firewalls: 449 Active" src="https://img.shields.io/badge/firewalls-449%20active-blue?style=flat-square">
  <img alt="Platforms" src="https://img.shields.io/badge/platforms-Windows%20%C2%B7%20macOS%20%C2%B7%20Linux-2f7bf5?style=flat-square">

  <br>

  <h3>Stop AI agents from rotting your codebase with unverified slop.</h3>

  <p>
    <b>SlopGuard</b> is an ultra-fast, zero-overhead <b>pure Rust</b> firewall and <b>Model Context Protocol (MCP)</b> engine<br>
    that catches AI hallucinations, duplicate shadow utilities, type escapes, and dummy placeholders<br>
    <b>before</b> they pollute your git history.
  </p>

  <code>cargo install --git https://github.com/avelone123123/slopguard.git --bin slopguard</code>
  <br>
  <small style="color:#94a3b8">or from local clone: <code>cargo install --path .</code></small>

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

<p align="center">
  <img src="assets/interception-flow.svg" alt="Real-Time Slop Interception Architecture" width="100%">
</p>

- **100% Native Rust**: Single standalone binary without runtime bloat. Instant startup (<5ms).
- **66 Active AST Firewalls**: Blocks phantom imports, unawaited async loops, weak PRNGs, unsafe deserialization, leaked secrets, compound slopsquatting, async generator leaks, insecure direct keygen, unbounded channel sprawl, speculative TOCTOU file races, hallucinated crypto algorithms, and dynamic SSRF interpolation.
- **Git Diff & AST-Token Stream Analysis**: Scans only changed lines in working tree or staged commits using Jaccard token similarity.
- **Built-in Model Context Protocol (MCP)**: Plugs directly into Cursor, Claude Code, and Windsurf so the agent inspects and validates its own diffs before completing prompts.
- **One-Command Auto-Sanitizer**: `slopguard fix` automatically cleans conversational AI bloat.
- **Pre-commit Hook in 1-Second**: Installs a zero-friction git guardrail with `slopguard hook install`.
- **Dynamic AI Hygiene Score (0-100)**: Visual terminal reporting with severity breakdown, grade, and exportable SVG badges.

---

## Quick Start

### Installation

```bash
# Install latest release binary directly via Cargo Git:
cargo install --git https://github.com/avelone123123/slopguard.git --bin slopguard

# Or install from local workspace clone:
cargo install --path .
```

### Precompiled Release Binaries

| Target Binary | Platform | Format | SHA-256 Verification Hash |
| :--- | :--- | :--- | :--- |
| `slopguard-v3.5.382-windows-x64.exe` | Windows x86_64 | Native PE Standalone | `A6F0345EF99300C19653F6232BA134A2B71118D5B85265FA1B9A69E9F5101B06` |

Or build from source:

```bash
git clone https://github.com/avelone123123/slopguard.git
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

<p align="center">
  <img src="assets/terminal-evidence.svg" alt="Live Terminal TUI with Real-Time Oscilloscope" width="100%">
</p>

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
  - `[4] FIREWALLS`: Comprehensive 66-firewall active AST matrix with real-time enforcement status.
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

## Threat Matrix (THREAT-01 – THREAT-16)

SlopGuard maps all 66 AST firewalls across 16 strategic AI code rot and supply-chain threat vectors:

| Strategic Threat ID | Description | Primary Active Shields |
| :--- | :--- | :--- |
| `THREAT-01` | Ghost Package & Slopsquatting Manifest Filters | `SLOP-009`, `SLOP-011`, `SLOP-046`, `SLOP-049` |
| `THREAT-02` | Microtask Starvation & Unawaited Loop Anti-Patterns | `SLOP-014`, `SLOP-019`, `SLOP-020`, `SLOP-050` |
| `THREAT-03` | Insecure PRNG Entropy & Predictable Key Generation | `SLOP-004`, `SLOP-048`, `SLOP-051` |
| `THREAT-04` | Non-Canonical Deep Path Traversal & Zip-Slip Protection | `SLOP-030`, `SLOP-053`, `SLOP-054` |
| `THREAT-05` | Polymorphic Deserialization RCE Gadget Interception | `SLOP-034`, `SLOP-052`, `SLOP-056` |
| `THREAT-06` | Speculative Native FFI ABI & WASM Host Bindings | `SLOP-018` |
| `THREAT-07` | Compound Slopsquatting Namespaces & Speculative Manifests | `SLOP-057` |
| `THREAT-08` | Async Stream & Generator Iteration Unhandled Break Leaks | `SLOP-058` |
| `THREAT-09` | Direct Insecure Pseudo-Random Keygen & Truncated Entropy | `SLOP-059` |
| `THREAT-10` | Unbounded Channel Sprawl & Backpressure Hazards | `SLOP-060` |
| `THREAT-11` | Dynamic Subprocess Shell Interpolation Injections | `SLOP-061` |
| `THREAT-12` | Unbounded Event Listener Loops & Memory Leaks | `SLOP-062` |
| `THREAT-13` | Hallucinated Polyglot AI SDK Wrappers & Slopsquatting | `SLOP-063` |
| `THREAT-14` | Speculative TOCTOU File Race & Concurrency Invalidation | `SLOP-064` |
| `THREAT-15` | Hallucinated & Non-Standard Cryptographic Algorithms | `SLOP-065` |
| `THREAT-16` | Speculative Dynamic Host SSRF Template Interpolation | `SLOP-066` |

---

## Rules and Detections

| Threat ID | Rule Name | Severity | Category | What It Catches |
| :--- | :--- | :--- | :--- | :--- |
| `SLOP-001` | `no-hardcoded-secrets` | `ERROR` | Crypto & Auth | Leaked OpenAI/Anthropic keys, AWS credentials, RSA private keys |
| `SLOP-002` | `no-inlined-secrets` | `ERROR` | Crypto & Auth | Hardcoded client-side API secrets and private tokens in frontend bundles |
| `SLOP-003` | `no-ai-todo-placeholders` | `ERROR` | Code Quality | Unfinished TODO placeholders, stub returns, and throw not implemented |
| `SLOP-004` | `no-insecure-crypto` | `ERROR` | Crypto & Auth | Broken cryptographic hash functions MD5 or SHA1 in security paths |
| `SLOP-005` | `no-circular-dependencies` | `ERROR` | Async & Concurrency | Circular module import cycles (`A -> B -> A`) causing TDZ crashes |
| `SLOP-006` | `no-mock-hallucinations` | `ERROR` | Supply Chain | Public placeholder APIs (`jsonplaceholder`, mock JWTs, test emails) |
| `SLOP-007` | `no-hallucinated-imports` | `ERROR` | Supply Chain | Hallucinated subpath imports and package exports (`useLocalStorage`, etc.) |
| `SLOP-008` | `no-floating-promises` | `ERROR` | Async & Concurrency | Floating unhandled asynchronous promises leaking exceptions |
| `SLOP-009` | `no-ghost-dependencies` | `ERROR` | Supply Chain | Third-party dependencies imported without declaration in manifest |
| `SLOP-010` | `no-ai-conversational-bloat` | `WARN` | Code Quality | AI conversational commentary and prompt echoes in code |
| `SLOP-011` | `no-slopsquat-deps` | `ERROR` | Supply Chain | Slopsquatted package names mimicking canonical packages |
| `SLOP-012` | `no-ai-sycophancy-watermarks` | `WARN` | Code Quality | AI sycophancy watermarks and model apology comments |
| `SLOP-013` | `no-hallucinated-speculative-apis` | `ERROR` | Supply Chain | Speculative standard library methods hallucinated by AI |
| `SLOP-014` | `no-async-in-sync-callbacks` | `ERROR` | Async & Concurrency | Async closures passed into synchronous array iterator callbacks |
| `SLOP-015` | `no-speculative-orm-methods` | `ERROR` | Supply Chain | Hallucinated speculative ORM query helper methods |
| `SLOP-016` | `no-phantom-cloud-sdks` | `ERROR` | Supply Chain | Fabricated cloud provider wrapper SDK packages in manifests |
| `SLOP-017` | `no-ide-protocol-hallucinations` | `ERROR` | Supply Chain | Hallucinated IDE or editor extension APIs (`vscode.window.showAIMessage`, etc.) |
| `SLOP-018` | `no-speculative-ffi-wasm` | `ERROR` | Supply Chain | Speculative extern C symbols and fictional WASM imports |
| `SLOP-019` | `no-unbounded-concurrency` | `WARN` | Async & Concurrency | Unbounded concurrent operations without semaphore throttling |
| `SLOP-020` | `no-async-pipeline-races` | `WARN` | Async & Concurrency | Unshielded `Promise.race()` and unmonitored stream pipelines |
| `SLOP-021` | `no-async-array-predicates` | `ERROR` | Async & Concurrency | Async callback truthy trap in `.filter()`, `.some()`, `.every()`, `.find()` |
| `SLOP-022` | `no-redos-vulnerability` | `ERROR` | Async & Concurrency | Regular expressions vulnerable to catastrophic backtracking |
| `SLOP-023` | `no-deep-nesting-sprawl` | `WARN` | Async & Concurrency | Excessive block nesting sprawl exceeding cognitive thresholds (>= 5 levels) |
| `SLOP-024` | `no-hardcoded-jwt-secrets` | `ERROR` | Crypto & Auth | Hardcoded JWT secret keys or `alg: none` verification bypass |
| `SLOP-025` | `no-disabled-tls` | `ERROR` | Crypto & Auth | Disabled TLS certificate verification (`NODE_TLS_REJECT_UNAUTHORIZED='0'`) |
| `SLOP-026` | `no-insecure-cors` | `ERROR` | Crypto & Auth | Wildcard CORS origin (`*`) paired with credentials enabled |
| `SLOP-027` | `no-unscoped-idor-queries` | `ERROR` | Crypto & Auth | Unscoped database queries vulnerable to IDOR |
| `SLOP-028` | `no-dummy-secrets-mocks` | `ERROR` | Crypto & Auth | Dummy authentication tokens or mock credentials committed |
| `SLOP-029` | `no-ghost-env-vars` | `WARN` | Crypto & Auth | Environment variables referenced without documentation in template |
| `SLOP-030` | `no-path-traversal-risk` | `ERROR` | Path & Injection | Direct path traversal risk via unvalidated path joining |
| `SLOP-031` | `no-sql-injection-risk` | `ERROR` | Path & Injection | Raw SQL concatenation and template interpolation vulnerabilities |
| `SLOP-032` | `no-command-injection-risk` | `ERROR` | Path & Injection | Command injection via unescaped shell process execution |
| `SLOP-033` | `no-unsafe-xml-or-ssrf` | `ERROR` | Path & Injection | XXE entity injection or unvalidated internal SSRF access |
| `SLOP-034` | `no-unsafe-dynamic-eval` | `ERROR` | Path & Injection | Unsafe dynamic eval or code execution constructs (`eval`, `new Function`) |
| `SLOP-035` | `no-prototype-pollution` | `ERROR` | Path & Injection | Prototype pollution through recursive object merging (`__proto__`) |
| `SLOP-036` | `no-phantom-api-routes` | `ERROR` | Path & Injection | Frontend API calls invoking non-existent backend endpoints |
| `SLOP-037` | `no-silent-catch-pass` | `ERROR` | Path & Injection | Empty catch or except pass blocks suppressing failures |
| `SLOP-038` | `no-silent-error-swallowing` | `ERROR` | Path & Injection | Swallowed promise rejections without error logging |
| `SLOP-039` | `no-test-sabotage` | `ERROR` | Code Quality | Commented assertions, `.skip` abuse, or forced unconditional test passes |
| `SLOP-040` | `no-coverage-theater` | `ERROR` | Code Quality | Vacuous unit test assertions and coverage theater stubs |
| `SLOP-041` | `no-duplicate-shadow-utilities` | `WARN` | Code Quality | Reinvented shadow utilities duplicating existing helpers |
| `SLOP-042` | `no-type-safety-degradation` | `ERROR` | Code Quality | Mass downgrades to `any`, `@ts-ignore`, `as unknown as any` |
| `SLOP-043` | `no-excessive-scope-drift` | `WARN` | Code Quality | Autonomous agent edits touching files outside task scope (>12 files or `.env`) |
| `SLOP-044` | `no-unsafe-buffer-overflows` | `ERROR` | Code Quality | Unbounded buffer indexing and raw unchecked memory access |
| `SLOP-045` | `no-dangling-native-pointers` | `ERROR` | Code Quality | Dangling native raw pointers and use-after-free hazards |
| `SLOP-046` | `no-multi-registry-slopsquat` | `ERROR` | Supply Chain | Multi-registry slopsquatting targeting speculative packages |
| `SLOP-047` | `no-async-stream-leaks` | `WARN` | Async & Concurrency | Unconsumed asynchronous stream generators leaking file handles |
| `SLOP-048` | `no-static-entropy-seeds` | `ERROR` | Crypto & Auth | Static constant seeds passed to pseudo-random generators |
| `SLOP-049` | `no-hallucinated-package-manifest` | `ERROR` | Supply Chain | Autonomous slopsquatting hazard and unpinned manifest wildcards (`*`, `latest`) |
| `SLOP-050` | `no-unawaited-async-in-loops` | `ERROR` | Async & Concurrency | Unawaited async callback inside iterator causing event-loop starvation |
| `SLOP-051` | `no-insecure-prng-tokens` | `ERROR` | Crypto & Auth | Insecure PRNG (`Math.random()`, `random.choice()`) for security tokens & secrets |
| `SLOP-052` | `no-unsafe-deserialization` | `ERROR` | Path & Injection | Polyglot deserialization gadgets (`pickle.loads`, `yaml.load`, `XMLDecoder`) |
| `SLOP-053` | `no-deep-path-traversal` | `ERROR` | Path & Injection | Deep path traversal, multi-encoded sequences, and Windows UNC bypass |
| `SLOP-054` | `no-zip-slip-traversal` | `ERROR` | Path & Injection | ZipSlip archive extraction path traversal and boundary containment bypass |
| `SLOP-055` | `no-ide-protocol-hallucinations` | `ERROR` | Supply Chain | Hallucinated IDE or editor protocol extension APIs (`vscode.window.showAIMessage`, etc.) |
| `SLOP-056` | `no-polymorphic-deserialization` | `ERROR` | Path & Injection | Polyglot polymorphic deserialization gadgets enabling arbitrary code execution |
| `SLOP-057` | `no-slopsquat-compound-namespace` | `ERROR` | Supply Chain | Compound slopsquatting package namespaces targeting synthetic dependencies |
| `SLOP-058` | `no-async-generator-unhandled-break` | `ERROR` | Async & Concurrency | Async stream or generator iteration unhandled break leaking sockets and handles |
| `SLOP-059` | `no-insecure-direct-crypto-keygen` | `ERROR` | Crypto & Auth | Direct truncated pseudo-random key generation or insufficient bit entropy (<256-bit) |
| `SLOP-060` | `no-unbounded-channel-sprawl` | `ERROR` | Async & Concurrency | Unbounded channel allocation or task queue spawning without backpressure |
| `SLOP-061` | `no-speculative-subprocess-injection` | `ERROR` | Command & Injection | Unchecked dynamic subprocess or shell interpolation vulnerability (CWE-78) |
| `SLOP-062` | `no-unbounded-event-listener-loop` | `WARN` | Async & Concurrency | Unbounded event listener registration inside loop causing memory leak (CWE-400) |
| `SLOP-063` | `no-phantom-ai-wrappers` | `ERROR` | Supply Chain | Hallucinated polyglot AI SDK wrapper package prone to supply chain slopsquatting (CWE-1357) |
| `SLOP-064` | `no-speculative-file-race-toctou` | `ERROR` | Path & Injection | Check-then-write TOCTOU race condition vulnerability (CWE-367) |
| `SLOP-065` | `no-hallucinated-crypto-algorithm` | `ERROR` | Crypto & Auth | Fictional algorithm strings (`aes-256-turbo`, `sha256-fast`) (CWE-327) |
| `SLOP-066` | `no-speculative-ssrf-interpolation` | `ERROR` | Path & Injection | Unvalidated dynamic host template interpolations (CWE-918) |

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
| **SlopGuard (Rust v3.5.382)** | **< 2.5ms** | **< 0.35ms / 10k LOC** | **< 5.8 MB** |
| Biome | ~45ms | ~30ms | ~28 MB |
| ESLint | ~620ms | ~480ms | ~140 MB |
| SonarQube Scanner | ~4,200ms | ~3,100ms | ~600 MB |

---

## Community & Support

- **Discord Server**: Join our engineering community for support, showcase, and discussions: [discord.gg/z7C9dprye](https://discord.gg/z7C9dprye)
- **Telegram Channel**: Official real-time releases, CVE advisories, and benchmark proofs: [@SlopGuard](https://t.me/SlopGuard)

---

## License

MIT © [avelone123123](https://github.com/avelone123123)
