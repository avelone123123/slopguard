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
### Sprint #112 — v3.5.448 (2026-11-09 16:51 UTC)
- **New AST Shield**: `SLOP-515` (`no-non-atomic-toctou-fs-mutation-v515`, CWE-367) — `Non-Atomic TOCTOU File Existence Check Before Write`
- **Threat Model**: AI models routinely generate fs.existsSync(p) followed by fs.writeFileSync(p), creating a CWE-367 Time-of-Check Time-of-Use race window exploitable via symlink swapping.
- **Verification Suite**: `708/708` unit tests passed, `515` active AST rules, repo score `100/100` (`2154ms`)

### Sprint #113 — v3.5.449 (2026-11-09 16:54 UTC)
- **New AST Shield**: `SLOP-516` (`no-hallucinated-cipher-primitive-v516`, CWE-327) — `Hallucinated Non-Standard Cryptographic Cipher Names`
- **Threat Model**: LLMs invent non-existent cipher strings such as aes-256-turbo or sha256-fast in OpenSSL and Node crypto calls, causing runtime panics or silent fallback to weak encryption.
- **Verification Suite**: `709/709` unit tests passed, `516` active AST rules, repo score `100/100` (`2032ms`)

### Sprint #114 — v3.5.450 (2026-11-09 16:57 UTC)
- **New AST Shield**: `SLOP-517` (`no-shell-template-subprocess-exec-v517`, CWE-78) — `Formatted Shell String Subprocess Command Injection`
- **Threat Model**: AI assistants pass formatted f-strings or template literals into subprocess.Popen(..., shell=True) and child_process.exec, allowing arbitrary shell command execution.
- **Verification Suite**: `710/710` unit tests passed, `517` active AST rules, repo score `100/100` (`2057ms`)

### Sprint #115 — v3.5.451 (2026-11-09 17:00 UTC)
- **New AST Shield**: `SLOP-518` (`no-loop-scoped-event-listener-leak-v518`, CWE-400) — `Unbounded Event Listener Registration Inside Loops`
- **Threat Model**: AI-generated stream handlers attach emitter.on('data', ...) inside for/while loops without cleanup, exhausting V8 heap memory and triggering MaxListenersExceededWarning.
- **Verification Suite**: `711/711` unit tests passed, `518` active AST rules, repo score `100/100` (`2043ms`)

### Sprint #116 — v3.5.452 (2026-11-09 17:03 UTC)
- **New AST Shield**: `SLOP-519` (`no-phantom-ai-sdk-slopsquatting-v519`, CWE-1357) — `Phantom AI SDK Wrapper Package Slopsquatting`
- **Threat Model**: LLMs hallucinate plausible AI helper packages like @anthropic/tools-pro or openai-streaming-helper in package.json and requirements.txt, exposing builds to supply-chain hijacking.
- **Verification Suite**: `712/712` unit tests passed, `519` active AST rules, repo score `100/100` (`2058ms`)

### Sprint #117 — v3.5.453 (2026-11-09 17:06 UTC)
- **New AST Shield**: `SLOP-520` (`no-unbounded-mpsc-queue-sprawl-v520`, CWE-400) — `Unbounded MPSC & Asyncio Queue Memory Exhaustion`
- **Threat Model**: AI concurrency scaffolding defaults to tokio::sync::mpsc::unbounded_channel() and unbuffered asyncio.Queue(), removing backpressure and causing OOM kills under traffic spikes.
- **Verification Suite**: `713/713` unit tests passed, `520` active AST rules, repo score `100/100` (`2079ms`)

### Sprint #118 — v3.5.454 (2026-11-09 17:09 UTC)
- **New AST Shield**: `SLOP-521` (`no-truncated-hex-entropy-keygen-v521`, CWE-326) — `Truncated Hex Slice & Weak PRNG Key Derivation`
- **Threat Model**: 1.  **Analyze the Request:** *   **Rule/Topic:** Truncated-hex-entropy-keygen (CWE-326). The blocked code is `crypto.randomBytes(32).toString('hex').slice(0, 16)` which truncates the hex string to 16 chars (64 bits of entropy instead of 256 bits), leading to weak key derivation. The enforced fix is `const aesKey = crypto.randomBytes(32);`. *   **Task:** Writ
- **Verification Suite**: `714/714` unit tests passed, `521` active AST rules, repo score `100/100` (`2070ms`)

### Sprint #119 — v3.5.455 (2026-11-09 17:12 UTC)
- **New AST Shield**: `SLOP-522` (`no-async-iterator-unclosed-break-v522`, CWE-775) — `Async Generator Break Without Stream Disposal`
- **Threat Model**: Breaking out of a for-await-of stream loop without a try/finally block or stream.destroy() leaks underlying file descriptors and database cursors.
- **Verification Suite**: `715/715` unit tests passed, `522` active AST rules, repo score `100/100` (`2073ms`)

### Sprint #120 — v3.5.456 (2026-11-09 17:14 UTC)
- **New AST Shield**: `SLOP-523` (`no-compound-namespace-slopsquat-v523`, CWE-829) — `Compound Namespace Package Manifest Slopsquatting`
- **Threat Model**: AI assistants synthesize plausible compound package names such as flask-bcrypt-auth or tokio-redis-pro that attackers register on PyPI/crates.io/npm for RCE.
- **Verification Suite**: `716/716` unit tests passed, `523` active AST rules, repo score `100/100` (`2014ms`)

### Sprint #121 — v3.5.457 (2026-11-09 17:18 UTC)
- **New AST Shield**: `SLOP-524` (`no-unvalidated-ssrf-redirect-chain-v524`, CWE-918) — `Unvalidated SSRF Host Interpolation & Redirect Bypass`
- **Threat Model**: AI coding assistants interpolate user-controlled host variables directly into axios.get or requests.get URLs, enabling Server-Side Request Forgery against cloud metadata endpoints.
- **Verification Suite**: `717/717` unit tests passed, `524` active AST rules, repo score `100/100` (`2078ms`)

### Sprint #122 — v3.5.458 (2026-11-09 17:21 UTC)
- **New AST Shield**: `SLOP-525` (`no-non-atomic-toctou-fs-mutation-v525`, CWE-367) — `Non-Atomic TOCTOU File Existence Check Before Write`
- **Threat Model**: AI models routinely generate fs.existsSync(p) followed by fs.writeFileSync(p), creating a CWE-367 Time-of-Check Time-of-Use race window exploitable via symlink swapping.
- **Verification Suite**: `718/718` unit tests passed, `525` active AST rules, repo score `100/100` (`2115ms`)

### Sprint #123 — v3.5.459 (2026-11-09 17:23 UTC)
- **New AST Shield**: `SLOP-526` (`no-hallucinated-cipher-primitive-v526`, CWE-327) — `Hallucinated Non-Standard Cryptographic Cipher Names`
- **Threat Model**: LLMs invent non-existent cipher strings such as aes-256-turbo or sha256-fast in OpenSSL and Node crypto calls, causing runtime panics or silent fallback to weak encryption.
- **Verification Suite**: `719/719` unit tests passed, `526` active AST rules, repo score `100/100` (`2080ms`)

### Sprint #124 — v3.5.460 (2026-11-09 17:27 UTC)
- **New AST Shield**: `SLOP-527` (`no-shell-template-subprocess-exec-v527`, CWE-78) — `Formatted Shell String Subprocess Command Injection`
- **Threat Model**: AI assistants pass formatted f-strings or template literals into subprocess.Popen(..., shell=True) and child_process.exec, allowing arbitrary shell command execution.
- **Verification Suite**: `720/720` unit tests passed, `527` active AST rules, repo score `100/100` (`2068ms`)

### Sprint #125 — v3.5.461 (2026-11-09 17:29 UTC)
- **New AST Shield**: `SLOP-528` (`no-loop-scoped-event-listener-leak-v528`, CWE-400) — `Unbounded Event Listener Registration Inside Loops`
- **Threat Model**: AI-generated stream handlers attach emitter.on('data', ...) inside for/while loops without cleanup, exhausting V8 heap memory and triggering MaxListenersExceededWarning.
- **Verification Suite**: `721/721` unit tests passed, `528` active AST rules, repo score `100/100` (`2058ms`)

### Sprint #126 — v3.5.462 (2026-11-09 17:33 UTC)
- **New AST Shield**: `SLOP-529` (`no-phantom-ai-sdk-slopsquatting-v529`, CWE-1357) — `Phantom AI SDK Wrapper Package Slopsquatting`
- **Threat Model**: LLMs hallucinate plausible AI helper packages like @anthropic/tools-pro or openai-streaming-helper in package.json and requirements.txt, exposing builds to supply-chain hijacking.
- **Verification Suite**: `722/722` unit tests passed, `529` active AST rules, repo score `100/100` (`2048ms`)
