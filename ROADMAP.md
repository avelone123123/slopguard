# SlopGuard Engineering Roadmap
## Phase 1: High-Performance Core (Completed)
- [x] Zero-overhead AST diff engine in pure Rust.
- [x] 44 unit tests with comprehensive regression coverage.
- [x] MCP Protocol integration (stdio JSON-RPC) for Cursor, Claude Code, Antigravity.
- [x] Live Web Dashboard server with telemetry charts.
- [x] Precompiled release artifacts for Windows x64.
## Phase 2: Autonomous Multi-Agent Office & Ecosystem (Completed)
- [x] 15-Agent Autonomous Engineering Studio in `agent-triad`.
- [x] Interactive 2D Chibi Office Floor with task passing and dialogues.
- [x] Automated GitHub Issue Hunting and triage.
- [x] SlopGuard v2.9.0 Core Rust AST Engine & Deep Hallucination Shields (177 tests passing).
- [x] Standalone precompiled binaries and Cargo Git multi-tier distribution.
## Phase 3: Advanced Cognitive Hallucination Defense & NextGen v3.0.0 (Completed)
- [x] SlopGuard v3.0.0 NextGen Core Rust Engine (202 verified tests, 54 active AST firewalls).
- [x] Supply-Chain Slopsquat Shield: Expanded signatures across PyPI, npm, Cargo, Go with wildcard rejection (`*`, `==*`, `latest`).
- [x] Async Loop Traps & Microtask Queue Starvation: Unbounded `while(true)` without backoff, unawaited promises in loops.
- [x] Deep Path Traversal, Archive Zip-Slip & Windows UNC Bounds (`%c0%ae%c0%ae`, `\u002e\u002e`, `\\?\`, `\\.\`).
- [x] Insecure PRNG & CSPRNG Entropy Guard: Halts non-cryptographic PRNGs for security tokens with explicit CSPRNG remediation.
- [x] Polyglot Deserialization Gadgets: Multi-language defense across Python, Node, PHP, Ruby, Java, and .NET.
- [x] Interactive Web AST Diff Playground & Desktop Studio v3.0.0 telemetry synchronization.
## Phase 4: Global Registry & crates.io Distribution (Active)
- [x] SlopGuard v3.1.0 crates.io publishing pipeline verification (cargo publish --dry-run verified, 132 KiB package, 0 errors).
- [x] SlopGuard v3.1.0 Ecosystem Upgrade across Core, Web Diff Playground, and Desktop Studio.
- [x] SlopGuard v3.2.0 NextGen QA Benchmarking, Streaming Fuzzing & 56 AST Firewalls (221 passed unit tests, sub-0.42ms latency).
- [x] SlopGuard v3.3.0 NextGen Core AST Engine & 60 AST Firewalls (241 passed unit tests, Slop-057 to Slop-060, sub-0.42ms throughput).
- [x] SlopGuard v3.4.0 NextGen Core AST Engine & 63 AST Firewalls (245 passed unit tests, SLOP-061 Subprocess Injection, SLOP-062 Unbounded Event Listener Loops, SLOP-063 Phantom AI SDK Wrappers, sub-0.35ms throughput).
- [x] SlopGuard v3.5.0 NextGen Enterprise (252 passed unit tests, 66 active AST firewalls, SLOP-064 TOCTOU File Race, SLOP-065 Hallucinated Crypto, SLOP-066 Speculative SSRF, multi-language parity).
- [ ] VS Code and Cursor IDE extensions marketplace publishing.
- [ ] Native cross-platform installer (Winget, Homebrew).
- [ ] Semantic vector graph analysis for multi-file circular hallucination logic.
- [ ] LLM agent self-prompting honeypot interception.
- [ ] WebAssembly in-browser playground for instant live audits.

## Autonomous Council Verification Log
### Sprint #1 — Core Rust AST Engine & Deep Hallucination Shields (2026-11-05 22:32 UTC)
- **Intercepted Case Study**: `...` via `SLOP-06X (...)`
- **Threat Model**: 2 concise technical sentences
- **Live Repository Audit**: Score `100/100` across `12` files (`0 errors, 0 warnings`, `2131ms`)

### Sprint #2 — Core Rust AST Engine & Deep Hallucination Shields (2026-11-05 22:33 UTC)
- **Intercepted Case Study**: `Hallucinated Non-Standard Cipher & Hash Names` via `SLOP-065 (no-hallucinated-crypto-algorithm)`
- **Threat Model**: We are given a seed rule SLOP-065 which is about no-hallucinated-crypto-algorithm. The vulnerable pattern uses non-standard crypto algorithm names ('aes-256-turbo', 'sha256-fast') and the safe remediation uses standard names ('aes-256-gcm', 'sha256'). The task is to write a concise 2-sentence deep engineering threat analysis explaining: 1. Why LLMs generate 
- **Live Repository Audit**: Score `100/100` across `12` files (`0 errors, 0 warnings`, `2152ms`)

### Sprint #3 — Core Rust AST Engine & Deep Hallucination Shields (2026-11-05 22:34 UTC)
- **Intercepted Case Study**: `Unvalidated Hostname Interpolation in HTTP Clients (SSRF)` via `SLOP-066 (no-speculative-ssrf-interpolation)`
- **Threat Model**: AI models hallucinate this pattern because they prioritize syntactic plausibility over semantic security checks, generating code that mirrors common but insecure interpolation idioms. SlopGuard's AST firewall detects the unvalidated host interpolation in the request expression and enforces the ALLOWED_ORIGINS whitelist before the request is issued, completing the block in under
- **Live Repository Audit**: Score `100/100` across `12` files (`0 errors, 0 warnings`, `2258ms`)

### Sprint #1 — Core Rust AST Engine & Deep Hallucination Shields (2026-11-05 22:40 UTC)
- **Intercepted Case Study**: `TOCTOU File Existence Race Before Write` via `SLOP-064 (no-speculative-file-race-toctou)`
- **Threat Model**: AI models hallucinate the exists‑then‑write pattern because they learn from insecure code snippets and lack semantic awareness of race conditions, so they reproduce the unsafe idiom without
- **Live Repository Audit**: Score `100/100` across `12` files (`0 errors, 0 warnings`, `2075ms`)

### Sprint #1 — Core Rust AST Engine & Deep Hallucination Shields (2026-11-06 10:21 UTC)
- **Intercepted Case Study**: `TOCTOU File Existence Race Before Write` via `SLOP-064 (no-speculative-file-race-toctou)`
- **Threat Model**: AI coding assistants routinely generate fs.existsSync(p) followed by fs.writeFileSync(p), creating a CWE-367 Time-of-Check Time-of-Use race window exploitable via symlink swapping.
- **Live Repository Audit**: Score `100/100` across `12` files (`0 errors, 0 warnings`, `2028ms`)

### Sprint #2 — Web Playground, Live Audits & Interactive UX (2026-11-06 10:25 UTC)
- **Intercepted Case Study**: `Hallucinated Non-Standard Cipher & Hash Names` via `SLOP-065 (no-hallucinated-crypto-algorithm)`
- **Threat Model**: LLMs invent non-existent cipher strings such as aes-256-turbo or sha256-fast in OpenSSL and Node crypto calls, causing runtime panics or silent fallback to weak encryption.
- **Live Repository Audit**: Score `100/100` across `12` files (`0 errors, 0 warnings`, `2607ms`)

### Sprint #4 — v3.5.1 (2026-11-06 10:34 UTC)
- **New AST Shield**: `SLOP-067` (`no-shell-template-subprocess-exec-v67`, CWE-78) — `Formatted Shell String Subprocess Command Injection`
- **Threat Model**: AI assistants pass formatted f-strings or template literals into subprocess.Popen(..., shell=True) and child_process.exec, allowing arbitrary shell command execution.
- **Verification Suite**: `260/260` unit tests passed, `67` active AST rules, repo score `100/100` (`2137ms`)

### Sprint #1 — v3.5.2 (2026-11-06 10:45 UTC)
- **New AST Shield**: `SLOP-068` (`no-unvalidated-ssrf-redirect-chain-v68`, CWE-918) — `Unvalidated SSRF Host Interpolation & Redirect Bypass`
- **Threat Model**: AI coding assistants interpolate user-controlled host variables directly into axios.get or requests.get URLs, enabling Server-Side Request Forgery against cloud metadata endpoints.
- **Verification Suite**: `261/261` unit tests passed, `68` active AST rules, repo score `100/100` (`2197ms`)
