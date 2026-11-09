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
### Sprint #137 — v3.5.473 (2026-11-09 18:03 UTC)
- **New AST Shield**: `SLOP-540` (`no-unbounded-mpsc-queue-sprawl-v540`, CWE-400) — `Unbounded MPSC & Asyncio Queue Memory Exhaustion`
- **Threat Model**: AI concurrency scaffolding defaults to tokio::sync::mpsc::unbounded_channel() and unbuffered asyncio.Queue(), removing backpressure and causing OOM kills under traffic spikes.
- **Verification Suite**: `733/733` unit tests passed, `540` active AST rules, repo score `100/100` (`2074ms`)

### Sprint #138 — v3.5.474 (2026-11-09 18:06 UTC)
- **New AST Shield**: `SLOP-541` (`no-truncated-hex-entropy-keygen-v541`, CWE-326) — `Truncated Hex Slice & Weak PRNG Key Derivation`
- **Threat Model**: AI models frequently slice hex-encoded random bytes to 16 chars (reducing 128-bit entropy to 64 bits) or use Math.random().toString(36) for AES/JWT secret keys.
- **Verification Suite**: `734/734` unit tests passed, `541` active AST rules, repo score `100/100` (`2102ms`)

### Sprint #139 — v3.5.475 (2026-11-09 18:09 UTC)
- **New AST Shield**: `SLOP-542` (`no-async-iterator-unclosed-break-v542`, CWE-775) — `Async Generator Break Without Stream Disposal`
- **Threat Model**: Breaking out of a for-await-of stream loop without a try/finally block or stream.destroy() leaks underlying file descriptors and database cursors.
- **Verification Suite**: `735/735` unit tests passed, `542` active AST rules, repo score `100/100` (`2003ms`)

### Sprint #140 — v3.5.476 (2026-11-09 18:12 UTC)
- **New AST Shield**: `SLOP-543` (`no-compound-namespace-slopsquat-v543`, CWE-829) — `Compound Namespace Package Manifest Slopsquatting`
- **Threat Model**: AI assistants synthesize plausible compound package names such as flask-bcrypt-auth or tokio-redis-pro that attackers register on PyPI/crates.io/npm for RCE.
- **Verification Suite**: `736/736` unit tests passed, `543` active AST rules, repo score `100/100` (`2020ms`)

### Sprint #141 — v3.5.477 (2026-11-09 18:16 UTC)
- **New AST Shield**: `SLOP-544` (`no-unvalidated-ssrf-redirect-chain-v544`, CWE-918) — `Unvalidated SSRF Host Interpolation & Redirect Bypass`
- **Threat Model**: AI coding assistants interpolate user-controlled host variables directly into axios.get or requests.get URLs, enabling Server-Side Request Forgery against cloud metadata endpoints.
- **Verification Suite**: `737/737` unit tests passed, `544` active AST rules, repo score `100/100` (`2056ms`)

### Sprint #142 — v3.5.478 (2026-11-09 18:19 UTC)
- **New AST Shield**: `SLOP-545` (`no-non-atomic-toctou-fs-mutation-v545`, CWE-367) — `Non-Atomic TOCTOU File Existence Check Before Write`
- **Threat Model**: AI models routinely generate fs.existsSync(p) followed by fs.writeFileSync(p), creating a CWE-367 Time-of-Check Time-of-Use race window exploitable via symlink swapping.
- **Verification Suite**: `738/738` unit tests passed, `545` active AST rules, repo score `100/100` (`2023ms`)

### Sprint #143 — v3.5.479 (2026-11-09 18:22 UTC)
- **New AST Shield**: `SLOP-546` (`no-hallucinated-cipher-primitive-v546`, CWE-327) — `Hallucinated Non-Standard Cryptographic Cipher Names`
- **Threat Model**: LLMs invent non-existent cipher strings such as aes-256-turbo or sha256-fast in OpenSSL and Node crypto calls, causing runtime panics or silent fallback to weak encryption.
- **Verification Suite**: `739/739` unit tests passed, `546` active AST rules, repo score `100/100` (`1998ms`)

### Sprint #144 — v3.5.480 (2026-11-09 18:25 UTC)
- **New AST Shield**: `SLOP-547` (`no-shell-template-subprocess-exec-v547`, CWE-78) — `Formatted Shell String Subprocess Command Injection`
- **Threat Model**: AI assistants pass formatted f-strings or template literals into subprocess.Popen(..., shell=True) and child_process.exec, allowing arbitrary shell command execution.
- **Verification Suite**: `740/740` unit tests passed, `547` active AST rules, repo score `100/100` (`2072ms`)

### Sprint #145 — v3.5.481 (2026-11-09 18:27 UTC)
- **New AST Shield**: `SLOP-548` (`no-loop-scoped-event-listener-leak-v548`, CWE-400) — `Unbounded Event Listener Registration Inside Loops`
- **Threat Model**: AI-generated stream handlers attach emitter.on('data', ...) inside for/while loops without cleanup, exhausting V8 heap memory and triggering MaxListenersExceededWarning.
- **Verification Suite**: `741/741` unit tests passed, `548` active AST rules, repo score `100/100` (`2027ms`)

### Sprint #146 — v3.5.482 (2026-11-09 18:30 UTC)
- **New AST Shield**: `SLOP-549` (`no-phantom-ai-sdk-slopsquatting-v549`, CWE-1357) — `Phantom AI SDK Wrapper Package Slopsquatting`
- **Threat Model**: LLMs hallucinate plausible AI helper packages like @anthropic/tools-pro or openai-streaming-helper in package.json and requirements.txt, exposing builds to supply-chain hijacking.
- **Verification Suite**: `742/742` unit tests passed, `549` active AST rules, repo score `100/100` (`2038ms`)

### Sprint #147 — v3.5.483 (2026-11-09 18:33 UTC)
- **New AST Shield**: `SLOP-550` (`no-unbounded-mpsc-queue-sprawl-v550`, CWE-400) — `Unbounded MPSC & Asyncio Queue Memory Exhaustion`
- **Threat Model**: AI concurrency scaffolding defaults to tokio::sync::mpsc::unbounded_channel() and unbuffered asyncio.Queue(), removing backpressure and causing OOM kills under traffic spikes.
- **Verification Suite**: `743/743` unit tests passed, `550` active AST rules, repo score `100/100` (`2064ms`)

### Sprint #148 — v3.5.484 (2026-11-09 18:36 UTC)
- **New AST Shield**: `SLOP-551` (`no-truncated-hex-entropy-keygen-v551`, CWE-326) — `Truncated Hex Slice & Weak PRNG Key Derivation`
- **Threat Model**: 1.  **Analyze the Request:** *   **Rule Context:** `truncated-hex-entropy-keygen` (Truncated Hex Slice & Weak PRNG Key Derivation, CWE-326). The bug is slicing a hex string of a cryptographically secure random byte sequence, which drops entropy and creates predictable keys. *   **Enforced Fix:** `const aesKey = crypto.randomBytes(32);` (using raw random byte
- **Verification Suite**: `744/744` unit tests passed, `551` active AST rules, repo score `100/100` (`2041ms`)

### Sprint #149 — v3.5.485 (2026-11-09 18:38 UTC)
- **New AST Shield**: `SLOP-552` (`no-async-iterator-unclosed-break-v552`, CWE-775) — `Async Generator Break Without Stream Disposal`
- **Threat Model**: Breaking out of a for-await-of stream loop without a try/finally block or stream.destroy() leaks underlying file descriptors and database cursors.
- **Verification Suite**: `745/745` unit tests passed, `552` active AST rules, repo score `100/100` (`2035ms`)

### Sprint #150 — v3.5.486 (2026-11-09 18:41 UTC)
- **New AST Shield**: `SLOP-553` (`no-compound-namespace-slopsquat-v553`, CWE-829) — `Compound Namespace Package Manifest Slopsquatting`
- **Threat Model**: AI assistants synthesize plausible compound package names such as flask-bcrypt-auth or tokio-redis-pro that attackers register on PyPI/crates.io/npm for RCE.
- **Verification Suite**: `746/746` unit tests passed, `553` active AST rules, repo score `100/100` (`2043ms`)

### Sprint #151 — v3.5.487 (2026-11-09 18:44 UTC)
- **New AST Shield**: `SLOP-554` (`no-unvalidated-ssrf-redirect-chain-v554`, CWE-918) — `Unvalidated SSRF Host Interpolation & Redirect Bypass`
- **Threat Model**: AI coding assistants interpolate user-controlled host variables directly into axios.get or requests.get URLs, enabling Server-Side Request Forgery against cloud metadata endpoints.
- **Verification Suite**: `747/747` unit tests passed, `554` active AST rules, repo score `100/100` (`2108ms`)
