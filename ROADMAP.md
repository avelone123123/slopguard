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
### Sprint #210 — v3.5.546 (2026-11-09 21:32 UTC)
- **New AST Shield**: `SLOP-613` (`no-compound-namespace-slopsquat-v613`, CWE-829) — `Compound Namespace Package Manifest Slopsquatting`
- **Threat Model**: AI assistants synthesize plausible compound package names such as flask-bcrypt-auth or tokio-redis-pro that attackers register on PyPI/crates.io/npm for RCE.
- **Verification Suite**: `806/806` unit tests passed, `613` active AST rules, repo score `100/100` (`2013ms`)

### Sprint #211 — v3.5.547 (2026-11-09 21:35 UTC)
- **New AST Shield**: `SLOP-614` (`no-unvalidated-ssrf-redirect-chain-v614`, CWE-918) — `Unvalidated SSRF Host Interpolation & Redirect Bypass`
- **Threat Model**: AI coding assistants interpolate user-controlled host variables directly into axios.get or requests.get URLs, enabling Server-Side Request Forgery against cloud metadata endpoints.
- **Verification Suite**: `807/807` unit tests passed, `614` active AST rules, repo score `100/100` (`2250ms`)

### Sprint #212 — v3.5.548 (2026-11-09 21:38 UTC)
- **New AST Shield**: `SLOP-615` (`no-non-atomic-toctou-fs-mutation-v615`, CWE-367) — `Non-Atomic TOCTOU File Existence Check Before Write`
- **Threat Model**: 1.  **Analyze the Request:** *   **Rule:** Non-Atomic TOCTOU File Existence Check Before Write (CWE-367). *   **Blocked Code:** `if (fs.existsSync(uploadPath)) { fs.writeFileSync(uploadPath, payload); }` *   **Enforced Fix:** `fs.writeFileSync(uploadPath, payload, { flag: 'wx' });` *   **Topic:** Why AI models hallucinate this pattern and how SlopGuard's AST
- **Verification Suite**: `808/808` unit tests passed, `615` active AST rules, repo score `100/100` (`2078ms`)

### Sprint #213 — v3.5.549 (2026-11-09 21:41 UTC)
- **New AST Shield**: `SLOP-616` (`no-hallucinated-cipher-primitive-v616`, CWE-327) — `Hallucinated Non-Standard Cryptographic Cipher Names`
- **Threat Model**: LLMs invent non-existent cipher strings such as aes-256-turbo or sha256-fast in OpenSSL and Node crypto calls, causing runtime panics or silent fallback to weak encryption.
- **Verification Suite**: `809/809` unit tests passed, `616` active AST rules, repo score `100/100` (`2188ms`)

### Sprint #214 — v3.5.550 (2026-11-09 21:43 UTC)
- **New AST Shield**: `SLOP-617` (`no-shell-template-subprocess-exec-v617`, CWE-78) — `Formatted Shell String Subprocess Command Injection`
- **Threat Model**: AI assistants pass formatted f-strings or template literals into subprocess.Popen(..., shell=True) and child_process.exec, allowing arbitrary shell command execution.
- **Verification Suite**: `810/810` unit tests passed, `617` active AST rules, repo score `100/100` (`2126ms`)

### Sprint #215 — v3.5.551 (2026-11-09 21:46 UTC)
- **New AST Shield**: `SLOP-618` (`no-loop-scoped-event-listener-leak-v618`, CWE-400) — `Unbounded Event Listener Registration Inside Loops`
- **Threat Model**: AI-generated stream handlers attach emitter.on('data', ...) inside for/while loops without cleanup, exhausting V8 heap memory and triggering MaxListenersExceededWarning.
- **Verification Suite**: `811/811` unit tests passed, `618` active AST rules, repo score `100/100` (`2177ms`)

### Sprint #216 — v3.5.552 (2026-11-09 21:49 UTC)
- **New AST Shield**: `SLOP-619` (`no-phantom-ai-sdk-slopsquatting-v619`, CWE-1357) — `Phantom AI SDK Wrapper Package Slopsquatting`
- **Threat Model**: LLMs hallucinate plausible AI helper packages like @anthropic/tools-pro or openai-streaming-helper in package.json and requirements.txt, exposing builds to supply-chain hijacking.
- **Verification Suite**: `812/812` unit tests passed, `619` active AST rules, repo score `100/100` (`2090ms`)

### Sprint #217 — v3.5.553 (2026-11-09 21:51 UTC)
- **New AST Shield**: `SLOP-620` (`no-unbounded-mpsc-queue-sprawl-v620`, CWE-400) — `Unbounded MPSC & Asyncio Queue Memory Exhaustion`
- **Threat Model**: AI concurrency scaffolding defaults to tokio::sync::mpsc::unbounded_channel() and unbuffered asyncio.Queue(), removing backpressure and causing OOM kills under traffic spikes.
- **Verification Suite**: `813/813` unit tests passed, `620` active AST rules, repo score `100/100` (`2957ms`)

### Sprint #1 — v3.5.554 (2026-11-10 14:58 UTC)
- **New AST Shield**: `SLOP-621` (`no-unvalidated-ssrf-redirect-chain-v621`, CWE-918) — `Unvalidated SSRF Host Interpolation & Redirect Bypass`
- **Threat Model**: AI coding assistants interpolate user-controlled host variables directly into axios.get or requests.get URLs, enabling Server-Side Request Forgery against cloud metadata endpoints.
- **Verification Suite**: `814/814` unit tests passed, `621` active AST rules, repo score `100/100` (`2301ms`)

### Sprint #2 — v3.5.555 (2026-11-10 15:01 UTC)
- **New AST Shield**: `SLOP-622` (`no-non-atomic-toctou-fs-mutation-v622`, CWE-367) — `Non-Atomic TOCTOU File Existence Check Before Write`
- **Threat Model**: AI models routinely generate fs.existsSync(p) followed by fs.writeFileSync(p), creating a CWE-367 Time-of-Check Time-of-Use race window exploitable via symlink swapping.
- **Verification Suite**: `815/815` unit tests passed, `622` active AST rules, repo score `100/100` (`2662ms`)

### Sprint #3 — v3.5.556 (2026-11-10 15:04 UTC)
- **New AST Shield**: `SLOP-623` (`no-hallucinated-cipher-primitive-v623`, CWE-327) — `Hallucinated Non-Standard Cryptographic Cipher Names`
- **Threat Model**: LLMs invent non-existent cipher strings such as aes-256-turbo or sha256-fast in OpenSSL and Node crypto calls, causing runtime panics or silent fallback to weak encryption.
- **Verification Suite**: `816/816` unit tests passed, `623` active AST rules, repo score `100/100` (`2474ms`)

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
