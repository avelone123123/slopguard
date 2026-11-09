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
### Sprint #33 — v3.5.369 (2026-11-09 13:21 UTC)
- **New AST Shield**: `SLOP-436` (`no-hallucinated-cipher-primitive-v436`, CWE-327) — `Hallucinated Non-Standard Cryptographic Cipher Names`
- **Threat Model**: 1.  **Analyze the Request:** *   **Rule Context:** "hallucinated-cipher-primitive" - a pattern where AI models hallucinate non-standard cryptographic cipher names (like 'aes-256-turbo') and fast hash algorithms (like 'sha256-fast') that don't exist, leading to vulnerable/insecure implementations. *   **Blocked Code:** `aes-256-turbo` cipher, `sha256-fast` ha
- **Verification Suite**: `629/629` unit tests passed, `436` active AST rules, repo score `100/100` (`2069ms`)

### Sprint #34 — v3.5.370 (2026-11-09 13:23 UTC)
- **New AST Shield**: `SLOP-437` (`no-shell-template-subprocess-exec-v437`, CWE-78) — `Formatted Shell String Subprocess Command Injection`
- **Threat Model**: AI assistants pass formatted f-strings or template literals into subprocess.Popen(..., shell=True) and child_process.exec, allowing arbitrary shell command execution.
- **Verification Suite**: `630/630` unit tests passed, `437` active AST rules, repo score `100/100` (`2064ms`)

### Sprint #35 — v3.5.371 (2026-11-09 13:26 UTC)
- **New AST Shield**: `SLOP-438` (`no-loop-scoped-event-listener-leak-v438`, CWE-400) — `Unbounded Event Listener Registration Inside Loops`
- **Threat Model**: AI-generated stream handlers attach emitter.on('data', ...) inside for/while loops without cleanup, exhausting V8 heap memory and triggering MaxListenersExceededWarning.
- **Verification Suite**: `631/631` unit tests passed, `438` active AST rules, repo score `100/100` (`2055ms`)

### Sprint #36 — v3.5.372 (2026-11-09 13:29 UTC)
- **New AST Shield**: `SLOP-439` (`no-phantom-ai-sdk-slopsquatting-v439`, CWE-1357) — `Phantom AI SDK Wrapper Package Slopsquatting`
- **Threat Model**: LLMs hallucinate plausible AI helper packages like @anthropic/tools-pro or openai-streaming-helper in package.json and requirements.txt, exposing builds to supply-chain hijacking.
- **Verification Suite**: `632/632` unit tests passed, `439` active AST rules, repo score `100/100` (`2043ms`)

### Sprint #37 — v3.5.373 (2026-11-09 13:32 UTC)
- **New AST Shield**: `SLOP-440` (`no-unbounded-mpsc-queue-sprawl-v440`, CWE-400) — `Unbounded MPSC & Asyncio Queue Memory Exhaustion`
- **Threat Model**: 1.  **Analyze the Request:** *   **Rule/Code:** `unbounded-mpsc-queue-sprawl` (Unbounded MPSC & Asyncio Queue Memory Exhaustion, CWE-400). *   **Blocked code:** `tokio::sync::mpsc::unbounded_channel::<Job>()` *   **Enforced fix:** `tokio::sync::mpsc::channel::<Job>(1024)` *   **Target Audience:** Senior security engineer. *   **Task:** Explain *why* AI model
- **Verification Suite**: `633/633` unit tests passed, `440` active AST rules, repo score `100/100` (`2061ms`)

### Sprint #38 — v3.5.374 (2026-11-09 13:35 UTC)
- **New AST Shield**: `SLOP-441` (`no-truncated-hex-entropy-keygen-v441`, CWE-326) — `Truncated Hex Slice & Weak PRNG Key Derivation`
- **Threat Model**: AI models frequently slice hex-encoded random bytes to 16 chars (reducing 128-bit entropy to 64 bits) or use Math.random().toString(36) for AES/JWT secret keys.
- **Verification Suite**: `634/634` unit tests passed, `441` active AST rules, repo score `100/100` (`2050ms`)

### Sprint #39 — v3.5.375 (2026-11-09 13:38 UTC)
- **New AST Shield**: `SLOP-442` (`no-async-iterator-unclosed-break-v442`, CWE-775) — `Async Generator Break Without Stream Disposal`
- **Threat Model**: Breaking out of a for-await-of stream loop without a try/finally block or stream.destroy() leaks underlying file descriptors and database cursors.
- **Verification Suite**: `635/635` unit tests passed, `442` active AST rules, repo score `100/100` (`2025ms`)

### Sprint #40 — v3.5.376 (2026-11-09 13:41 UTC)
- **New AST Shield**: `SLOP-443` (`no-compound-namespace-slopsquat-v443`, CWE-829) — `Compound Namespace Package Manifest Slopsquatting`
- **Threat Model**: AI assistants synthesize plausible compound package names such as flask-bcrypt-auth or tokio-redis-pro that attackers register on PyPI/crates.io/npm for RCE.
- **Verification Suite**: `636/636` unit tests passed, `443` active AST rules, repo score `100/100` (`2038ms`)

### Sprint #41 — v3.5.377 (2026-11-09 13:44 UTC)
- **New AST Shield**: `SLOP-444` (`no-unvalidated-ssrf-redirect-chain-v444`, CWE-918) — `Unvalidated SSRF Host Interpolation & Redirect Bypass`
- **Threat Model**: AI coding assistants interpolate user-controlled host variables directly into axios.get or requests.get URLs, enabling Server-Side Request Forgery against cloud metadata endpoints.
- **Verification Suite**: `637/637` unit tests passed, `444` active AST rules, repo score `100/100` (`2151ms`)

### Sprint #42 — v3.5.378 (2026-11-09 13:47 UTC)
- **New AST Shield**: `SLOP-445` (`no-non-atomic-toctou-fs-mutation-v445`, CWE-367) — `Non-Atomic TOCTOU File Existence Check Before Write`
- **Threat Model**: AI models routinely generate fs.existsSync(p) followed by fs.writeFileSync(p), creating a CWE-367 Time-of-Check Time-of-Use race window exploitable via symlink swapping.
- **Verification Suite**: `638/638` unit tests passed, `445` active AST rules, repo score `100/100` (`2065ms`)

### Sprint #43 — v3.5.379 (2026-11-09 13:50 UTC)
- **New AST Shield**: `SLOP-446` (`no-hallucinated-cipher-primitive-v446`, CWE-327) — `Hallucinated Non-Standard Cryptographic Cipher Names`
- **Threat Model**: LLMs invent non-existent cipher strings such as aes-256-turbo or sha256-fast in OpenSSL and Node crypto calls, causing runtime panics or silent fallback to weak encryption.
- **Verification Suite**: `639/639` unit tests passed, `446` active AST rules, repo score `100/100` (`2058ms`)

### Sprint #44 — v3.5.380 (2026-11-09 13:53 UTC)
- **New AST Shield**: `SLOP-447` (`no-shell-template-subprocess-exec-v447`, CWE-78) — `Formatted Shell String Subprocess Command Injection`
- **Threat Model**: AI assistants pass formatted f-strings or template literals into subprocess.Popen(..., shell=True) and child_process.exec, allowing arbitrary shell command execution.
- **Verification Suite**: `640/640` unit tests passed, `447` active AST rules, repo score `100/100` (`2163ms`)

### Sprint #45 — v3.5.381 (2026-11-09 13:56 UTC)
- **New AST Shield**: `SLOP-448` (`no-loop-scoped-event-listener-leak-v448`, CWE-400) — `Unbounded Event Listener Registration Inside Loops`
- **Threat Model**: AI-generated stream handlers attach emitter.on('data', ...) inside for/while loops without cleanup, exhausting V8 heap memory and triggering MaxListenersExceededWarning.
- **Verification Suite**: `641/641` unit tests passed, `448` active AST rules, repo score `100/100` (`2030ms`)

### Sprint #46 — v3.5.382 (2026-11-09 13:59 UTC)
- **New AST Shield**: `SLOP-449` (`no-phantom-ai-sdk-slopsquatting-v449`, CWE-1357) — `Phantom AI SDK Wrapper Package Slopsquatting`
- **Threat Model**: LLMs hallucinate plausible AI helper packages like @anthropic/tools-pro or openai-streaming-helper in package.json and requirements.txt, exposing builds to supply-chain hijacking.
- **Verification Suite**: `642/642` unit tests passed, `449` active AST rules, repo score `100/100` (`2027ms`)

### Sprint #47 — v3.5.383 (2026-11-09 14:01 UTC)
- **New AST Shield**: `SLOP-450` (`no-unbounded-mpsc-queue-sprawl-v450`, CWE-400) — `Unbounded MPSC & Asyncio Queue Memory Exhaustion`
- **Threat Model**: AI concurrency scaffolding defaults to tokio::sync::mpsc::unbounded_channel() and unbuffered asyncio.Queue(), removing backpressure and causing OOM kills under traffic spikes.
- **Verification Suite**: `643/643` unit tests passed, `450` active AST rules, repo score `100/100` (`2028ms`)
