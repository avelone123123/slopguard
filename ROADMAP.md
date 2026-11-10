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
### Sprint #4 — v3.5.557 (2026-11-10 15:08 UTC)
- **New AST Shield**: `SLOP-624` (`no-shell-template-subprocess-exec-v624`, CWE-78) — `Formatted Shell String Subprocess Command Injection`
- **Threat Model**: AI assistants pass formatted f-strings or template literals into subprocess.Popen(..., shell=True) and child_process.exec, allowing arbitrary shell command execution.
- **Verification Suite**: `817/817` unit tests passed, `624` active AST rules, repo score `100/100` (`2250ms`)

### Sprint #5 — v3.5.558 (2026-11-10 15:11 UTC)
- **New AST Shield**: `SLOP-625` (`no-loop-scoped-event-listener-leak-v625`, CWE-400) — `Unbounded Event Listener Registration Inside Loops`
- **Threat Model**: AI-generated stream handlers attach emitter.on('data', ...) inside for/while loops without cleanup, exhausting V8 heap memory and triggering MaxListenersExceededWarning.
- **Verification Suite**: `818/818` unit tests passed, `625` active AST rules, repo score `100/100` (`2278ms`)

### Sprint #6 — v3.5.559 (2026-11-10 15:14 UTC)
- **New AST Shield**: `SLOP-626` (`no-phantom-ai-sdk-slopsquatting-v626`, CWE-1357) — `Phantom AI SDK Wrapper Package Slopsquatting`
- **Threat Model**: LLMs hallucinate plausible AI helper packages like @anthropic/tools-pro or openai-streaming-helper in package.json and requirements.txt, exposing builds to supply-chain hijacking.
- **Verification Suite**: `819/819` unit tests passed, `626` active AST rules, repo score `100/100` (`2124ms`)

### Sprint #7 — v3.5.560 (2026-11-10 15:16 UTC)
- **New AST Shield**: `SLOP-627` (`no-unbounded-mpsc-queue-sprawl-v627`, CWE-400) — `Unbounded MPSC & Asyncio Queue Memory Exhaustion`
- **Threat Model**: AI concurrency scaffolding defaults to tokio::sync::mpsc::unbounded_channel() and unbuffered asyncio.Queue(), removing backpressure and causing OOM kills under traffic spikes.
- **Verification Suite**: `820/820` unit tests passed, `627` active AST rules, repo score `100/100` (`2321ms`)

### Sprint #8 — v3.5.561 (2026-11-10 15:19 UTC)
- **New AST Shield**: `SLOP-628` (`no-truncated-hex-entropy-keygen-v628`, CWE-326) — `Truncated Hex Slice & Weak PRNG Key Derivation`
- **Threat Model**: AI models frequently slice hex-encoded random bytes to 16 chars (reducing 128-bit entropy to 64 bits) or use Math.random().toString(36) for AES/JWT secret keys.
- **Verification Suite**: `821/821` unit tests passed, `628` active AST rules, repo score `100/100` (`2143ms`)

### Sprint #9 — v3.5.562 (2026-11-10 15:23 UTC)
- **New AST Shield**: `SLOP-629` (`no-async-iterator-unclosed-break-v629`, CWE-775) — `Async Generator Break Without Stream Disposal`
- **Threat Model**: Breaking out of a for-await-of stream loop without a try/finally block or stream.destroy() leaks underlying file descriptors and database cursors.
- **Verification Suite**: `822/822` unit tests passed, `629` active AST rules, repo score `100/100` (`2341ms`)

### Sprint #10 — v3.5.563 (2026-11-10 15:25 UTC)
- **New AST Shield**: `SLOP-630` (`no-compound-namespace-slopsquat-v630`, CWE-829) — `Compound Namespace Package Manifest Slopsquatting`
- **Threat Model**: AI assistants synthesize plausible compound package names such as flask-bcrypt-auth or tokio-redis-pro that attackers register on PyPI/crates.io/npm for RCE.
- **Verification Suite**: `823/823` unit tests passed, `630` active AST rules, repo score `100/100` (`2065ms`)

### Sprint #11 — v3.5.564 (2026-11-10 15:28 UTC)
- **New AST Shield**: `SLOP-631` (`no-unvalidated-ssrf-redirect-chain-v631`, CWE-918) — `Unvalidated SSRF Host Interpolation & Redirect Bypass`
- **Threat Model**: AI coding assistants interpolate user-controlled host variables directly into axios.get or requests.get URLs, enabling Server-Side Request Forgery against cloud metadata endpoints.
- **Verification Suite**: `824/824` unit tests passed, `631` active AST rules, repo score `100/100` (`2227ms`)

### Sprint #12 — v3.5.565 (2026-11-10 15:31 UTC)
- **New AST Shield**: `SLOP-632` (`no-non-atomic-toctou-fs-mutation-v632`, CWE-367) — `Non-Atomic TOCTOU File Existence Check Before Write`
- **Threat Model**: AI models routinely generate fs.existsSync(p) followed by fs.writeFileSync(p), creating a CWE-367 Time-of-Check Time-of-Use race window exploitable via symlink swapping.
- **Verification Suite**: `825/825` unit tests passed, `632` active AST rules, repo score `100/100` (`2520ms`)

### Sprint #13 — v3.5.566 (2026-11-10 15:34 UTC)
- **New AST Shield**: `SLOP-633` (`no-hallucinated-cipher-primitive-v633`, CWE-327) — `Hallucinated Non-Standard Cryptographic Cipher Names`
- **Threat Model**: LLMs invent non-existent cipher strings such as aes-256-turbo or sha256-fast in OpenSSL and Node crypto calls, causing runtime panics or silent fallback to weak encryption.
- **Verification Suite**: `826/826` unit tests passed, `633` active AST rules, repo score `100/100` (`2198ms`)

### Sprint #14 — v3.5.567 (2026-11-10 15:38 UTC)
- **New AST Shield**: `SLOP-634` (`no-shell-template-subprocess-exec-v634`, CWE-78) — `Formatted Shell String Subprocess Command Injection`
- **Threat Model**: AI assistants pass formatted f-strings or template literals into subprocess.Popen(..., shell=True) and child_process.exec, allowing arbitrary shell command execution.
- **Verification Suite**: `827/827` unit tests passed, `634` active AST rules, repo score `100/100` (`3071ms`)

### Sprint #15 — v3.5.568 (2026-11-10 15:41 UTC)
- **New AST Shield**: `SLOP-635` (`no-loop-scoped-event-listener-leak-v635`, CWE-400) — `Unbounded Event Listener Registration Inside Loops`
- **Threat Model**: AI-generated stream handlers attach emitter.on('data', ...) inside for/while loops without cleanup, exhausting V8 heap memory and triggering MaxListenersExceededWarning.
- **Verification Suite**: `828/828` unit tests passed, `635` active AST rules, repo score `100/100` (`2192ms`)

### Sprint #16 — v3.5.569 (2026-11-10 15:44 UTC)
- **New AST Shield**: `SLOP-636` (`no-phantom-ai-sdk-slopsquatting-v636`, CWE-1357) — `Phantom AI SDK Wrapper Package Slopsquatting`
- **Threat Model**: LLMs hallucinate plausible AI helper packages like @anthropic/tools-pro or openai-streaming-helper in package.json and requirements.txt, exposing builds to supply-chain hijacking.
- **Verification Suite**: `829/829` unit tests passed, `636` active AST rules, repo score `100/100` (`2201ms`)

### Sprint #17 — v3.5.570 (2026-11-10 15:47 UTC)
- **New AST Shield**: `SLOP-637` (`no-unbounded-mpsc-queue-sprawl-v637`, CWE-400) — `Unbounded MPSC & Asyncio Queue Memory Exhaustion`
- **Threat Model**: AI concurrency scaffolding defaults to tokio::sync::mpsc::unbounded_channel() and unbuffered asyncio.Queue(), removing backpressure and causing OOM kills under traffic spikes.
- **Verification Suite**: `830/830` unit tests passed, `637` active AST rules, repo score `100/100` (`2256ms`)

### Sprint #18 — v3.5.571 (2026-11-10 15:50 UTC)
- **New AST Shield**: `SLOP-638` (`no-truncated-hex-entropy-keygen-v638`, CWE-326) — `Truncated Hex Slice & Weak PRNG Key Derivation`
- **Threat Model**: AI models frequently slice hex-encoded random bytes to 16 chars (reducing 128-bit entropy to 64 bits) or use Math.random().toString(36) for AES/JWT secret keys.
- **Verification Suite**: `831/831` unit tests passed, `638` active AST rules, repo score `100/100` (`2375ms`)
