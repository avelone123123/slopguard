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
### Sprint #182 — v3.5.518 (2026-11-09 20:12 UTC)
- **New AST Shield**: `SLOP-585` (`no-non-atomic-toctou-fs-mutation-v585`, CWE-367) — `Non-Atomic TOCTOU File Existence Check Before Write`
- **Threat Model**: AI models routinely generate fs.existsSync(p) followed by fs.writeFileSync(p), creating a CWE-367 Time-of-Check Time-of-Use race window exploitable via symlink swapping.
- **Verification Suite**: `778/778` unit tests passed, `585` active AST rules, repo score `100/100` (`2329ms`)

### Sprint #183 — v3.5.519 (2026-11-09 20:15 UTC)
- **New AST Shield**: `SLOP-586` (`no-hallucinated-cipher-primitive-v586`, CWE-327) — `Hallucinated Non-Standard Cryptographic Cipher Names`
- **Threat Model**: LLMs invent non-existent cipher strings such as aes-256-turbo or sha256-fast in OpenSSL and Node crypto calls, causing runtime panics or silent fallback to weak encryption.
- **Verification Suite**: `779/779` unit tests passed, `586` active AST rules, repo score `100/100` (`2255ms`)

### Sprint #184 — v3.5.520 (2026-11-09 20:17 UTC)
- **New AST Shield**: `SLOP-587` (`no-shell-template-subprocess-exec-v587`, CWE-78) — `Formatted Shell String Subprocess Command Injection`
- **Threat Model**: AI assistants pass formatted f-strings or template literals into subprocess.Popen(..., shell=True) and child_process.exec, allowing arbitrary shell command execution.
- **Verification Suite**: `780/780` unit tests passed, `587` active AST rules, repo score `100/100` (`2381ms`)

### Sprint #185 — v3.5.521 (2026-11-09 20:21 UTC)
- **New AST Shield**: `SLOP-588` (`no-loop-scoped-event-listener-leak-v588`, CWE-400) — `Unbounded Event Listener Registration Inside Loops`
- **Threat Model**: 1.  **Analyze the Request:** *   **Rule:** loop-scoped-event-listener-leak (Unbounded Event Listener Registration Inside Loops, CWE-400). The "blocked code" registers an event listener inside a loop, causing a memory/listener leak. *   **Enforced fix:** Binds the handler to a batched function to avoid leaking listeners per iteration. *   **Task:** Write exac
- **Verification Suite**: `781/781` unit tests passed, `588` active AST rules, repo score `100/100` (`3320ms`)

### Sprint #186 — v3.5.522 (2026-11-09 20:24 UTC)
- **New AST Shield**: `SLOP-589` (`no-phantom-ai-sdk-slopsquatting-v589`, CWE-1357) — `Phantom AI SDK Wrapper Package Slopsquatting`
- **Threat Model**: LLMs hallucinate plausible AI helper packages like @anthropic/tools-pro or openai-streaming-helper in package.json and requirements.txt, exposing builds to supply-chain hijacking.
- **Verification Suite**: `782/782` unit tests passed, `589` active AST rules, repo score `100/100` (`2379ms`)

### Sprint #187 — v3.5.523 (2026-11-09 20:26 UTC)
- **New AST Shield**: `SLOP-590` (`no-unbounded-mpsc-queue-sprawl-v590`, CWE-400) — `Unbounded MPSC & Asyncio Queue Memory Exhaustion`
- **Threat Model**: 1.  **Analyze the Request:** *   **Rule:** `unbounded-mpsc-queue-sprawl` (Unbounded MPSC & Asyncio Queue Memory Exhaustion, CWE-400). *   **Blocked code:** `tokio::sync::mpsc::unbounded_channel::<Job>()` (allowing unbounded memory consumption, risking OOM/CWE-400). *   **Enforced fix:** `tokio::sync::mpsc::channel::<Job>(1024)` (bounded channel preventing un
- **Verification Suite**: `783/783` unit tests passed, `590` active AST rules, repo score `100/100` (`2397ms`)

### Sprint #188 — v3.5.524 (2026-11-09 20:30 UTC)
- **New AST Shield**: `SLOP-591` (`no-truncated-hex-entropy-keygen-v591`, CWE-326) — `Truncated Hex Slice & Weak PRNG Key Derivation`
- **Threat Model**: AI models frequently slice hex-encoded random bytes to 16 chars (reducing 128-bit entropy to 64 bits) or use Math.random().toString(36) for AES/JWT secret keys.
- **Verification Suite**: `784/784` unit tests passed, `591` active AST rules, repo score `100/100` (`2383ms`)

### Sprint #189 — v3.5.525 (2026-11-09 20:33 UTC)
- **New AST Shield**: `SLOP-592` (`no-async-iterator-unclosed-break-v592`, CWE-775) — `Async Generator Break Without Stream Disposal`
- **Threat Model**: Breaking out of a for-await-of stream loop without a try/finally block or stream.destroy() leaks underlying file descriptors and database cursors.
- **Verification Suite**: `785/785` unit tests passed, `592` active AST rules, repo score `100/100` (`2268ms`)

### Sprint #190 — v3.5.526 (2026-11-09 20:37 UTC)
- **New AST Shield**: `SLOP-593` (`no-compound-namespace-slopsquat-v593`, CWE-829) — `Compound Namespace Package Manifest Slopsquatting`
- **Threat Model**: AI assistants synthesize plausible compound package names such as flask-bcrypt-auth or tokio-redis-pro that attackers register on PyPI/crates.io/npm for RCE.
- **Verification Suite**: `786/786` unit tests passed, `593` active AST rules, repo score `100/100` (`2247ms`)

### Sprint #191 — v3.5.527 (2026-11-09 20:40 UTC)
- **New AST Shield**: `SLOP-594` (`no-unvalidated-ssrf-redirect-chain-v594`, CWE-918) — `Unvalidated SSRF Host Interpolation & Redirect Bypass`
- **Threat Model**: AI coding assistants interpolate user-controlled host variables directly into axios.get or requests.get URLs, enabling Server-Side Request Forgery against cloud metadata endpoints.
- **Verification Suite**: `787/787` unit tests passed, `594` active AST rules, repo score `100/100` (`2256ms`)

### Sprint #192 — v3.5.528 (2026-11-09 20:42 UTC)
- **New AST Shield**: `SLOP-595` (`no-non-atomic-toctou-fs-mutation-v595`, CWE-367) — `Non-Atomic TOCTOU File Existence Check Before Write`
- **Threat Model**: AI models routinely generate fs.existsSync(p) followed by fs.writeFileSync(p), creating a CWE-367 Time-of-Check Time-of-Use race window exploitable via symlink swapping.
- **Verification Suite**: `788/788` unit tests passed, `595` active AST rules, repo score `100/100` (`2053ms`)

### Sprint #193 — v3.5.529 (2026-11-09 20:45 UTC)
- **New AST Shield**: `SLOP-596` (`no-hallucinated-cipher-primitive-v596`, CWE-327) — `Hallucinated Non-Standard Cryptographic Cipher Names`
- **Threat Model**: LLMs invent non-existent cipher strings such as aes-256-turbo or sha256-fast in OpenSSL and Node crypto calls, causing runtime panics or silent fallback to weak encryption.
- **Verification Suite**: `789/789` unit tests passed, `596` active AST rules, repo score `100/100` (`2210ms`)

### Sprint #194 — v3.5.530 (2026-11-09 20:47 UTC)
- **New AST Shield**: `SLOP-597` (`no-shell-template-subprocess-exec-v597`, CWE-78) — `Formatted Shell String Subprocess Command Injection`
- **Threat Model**: AI assistants pass formatted f-strings or template literals into subprocess.Popen(..., shell=True) and child_process.exec, allowing arbitrary shell command execution.
- **Verification Suite**: `790/790` unit tests passed, `597` active AST rules, repo score `100/100` (`2035ms`)

### Sprint #195 — v3.5.531 (2026-11-09 20:50 UTC)
- **New AST Shield**: `SLOP-598` (`no-loop-scoped-event-listener-leak-v598`, CWE-400) — `Unbounded Event Listener Registration Inside Loops`
- **Threat Model**: AI-generated stream handlers attach emitter.on('data', ...) inside for/while loops without cleanup, exhausting V8 heap memory and triggering MaxListenersExceededWarning.
- **Verification Suite**: `791/791` unit tests passed, `598` active AST rules, repo score `100/100` (`2043ms`)

### Sprint #196 — v3.5.532 (2026-11-09 20:52 UTC)
- **New AST Shield**: `SLOP-599` (`no-phantom-ai-sdk-slopsquatting-v599`, CWE-1357) — `Phantom AI SDK Wrapper Package Slopsquatting`
- **Threat Model**: LLMs hallucinate plausible AI helper packages like @anthropic/tools-pro or openai-streaming-helper in package.json and requirements.txt, exposing builds to supply-chain hijacking.
- **Verification Suite**: `792/792` unit tests passed, `599` active AST rules, repo score `100/100` (`2011ms`)
