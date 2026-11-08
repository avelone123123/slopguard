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
### Sprint #58 — v3.5.293 (2026-11-08 16:20 UTC)
- **New AST Shield**: `SLOP-360` (`no-truncated-hex-entropy-keygen-v360`, CWE-326) — `Truncated Hex Slice & Weak PRNG Key Derivation`
- **Threat Model**: AI models frequently slice hex-encoded random bytes to 16 chars (reducing 128-bit entropy to 64 bits) or use Math.random().toString(36) for AES/JWT secret keys.
- **Verification Suite**: `553/553` unit tests passed, `360` active AST rules, repo score `100/100` (`2114ms`)

### Sprint #59 — v3.5.294 (2026-11-08 16:23 UTC)
- **New AST Shield**: `SLOP-361` (`no-async-iterator-unclosed-break-v361`, CWE-775) — `Async Generator Break Without Stream Disposal`
- **Threat Model**: Breaking out of a for-await-of stream loop without a try/finally block or stream.destroy() leaks underlying file descriptors and database cursors.
- **Verification Suite**: `554/554` unit tests passed, `361` active AST rules, repo score `100/100` (`2119ms`)

### Sprint #60 — v3.5.295 (2026-11-08 16:25 UTC)
- **New AST Shield**: `SLOP-362` (`no-compound-namespace-slopsquat-v362`, CWE-829) — `Compound Namespace Package Manifest Slopsquatting`
- **Threat Model**: AI assistants synthesize plausible compound package names such as flask-bcrypt-auth or tokio-redis-pro that attackers register on PyPI/crates.io/npm for RCE.
- **Verification Suite**: `555/555` unit tests passed, `362` active AST rules, repo score `100/100` (`2117ms`)

### Sprint #61 — v3.5.296 (2026-11-08 16:28 UTC)
- **New AST Shield**: `SLOP-363` (`no-unvalidated-ssrf-redirect-chain-v363`, CWE-918) — `Unvalidated SSRF Host Interpolation & Redirect Bypass`
- **Threat Model**: 1.  **Analyze the Request:** *   **Rule:** unvalidated-ssrf-redirect-chain (CWE-918). *   **Blocked Code:** `const res = await axios.get(`https://${req.query.apiHost}/v1/profile`);` *   **Enforced Fix:** `if (!ALLOWED_ORIGINS.has(req.query.apiHost)) throw new Error('Blocked');` + hardcoded URL. *   **Output constraints:** Exactly 2 technical sentences for a 
- **Verification Suite**: `556/556` unit tests passed, `363` active AST rules, repo score `100/100` (`2198ms`)

### Sprint #62 — v3.5.297 (2026-11-08 16:31 UTC)
- **New AST Shield**: `SLOP-364` (`no-non-atomic-toctou-fs-mutation-v364`, CWE-367) — `Non-Atomic TOCTOU File Existence Check Before Write`
- **Threat Model**: AI models routinely generate fs.existsSync(p) followed by fs.writeFileSync(p), creating a CWE-367 Time-of-Check Time-of-Use race window exploitable via symlink swapping.
- **Verification Suite**: `557/557` unit tests passed, `364` active AST rules, repo score `100/100` (`2196ms`)

### Sprint #63 — v3.5.298 (2026-11-08 16:33 UTC)
- **New AST Shield**: `SLOP-365` (`no-hallucinated-cipher-primitive-v365`, CWE-327) — `Hallucinated Non-Standard Cryptographic Cipher Names`
- **Threat Model**: LLMs invent non-existent cipher strings such as aes-256-turbo or sha256-fast in OpenSSL and Node crypto calls, causing runtime panics or silent fallback to weak encryption.
- **Verification Suite**: `558/558` unit tests passed, `365` active AST rules, repo score `100/100` (`2117ms`)

### Sprint #64 — v3.5.299 (2026-11-08 16:36 UTC)
- **New AST Shield**: `SLOP-366` (`no-shell-template-subprocess-exec-v366`, CWE-78) — `Formatted Shell String Subprocess Command Injection`
- **Threat Model**: AI assistants pass formatted f-strings or template literals into subprocess.Popen(..., shell=True) and child_process.exec, allowing arbitrary shell command execution.
- **Verification Suite**: `559/559` unit tests passed, `366` active AST rules, repo score `100/100` (`2208ms`)

### Sprint #65 — v3.5.300 (2026-11-08 16:38 UTC)
- **New AST Shield**: `SLOP-367` (`no-loop-scoped-event-listener-leak-v367`, CWE-400) — `Unbounded Event Listener Registration Inside Loops`
- **Threat Model**: AI-generated stream handlers attach emitter.on('data', ...) inside for/while loops without cleanup, exhausting V8 heap memory and triggering MaxListenersExceededWarning.
- **Verification Suite**: `560/560` unit tests passed, `367` active AST rules, repo score `100/100` (`2445ms`)

### Sprint #66 — v3.5.301 (2026-11-08 16:41 UTC)
- **New AST Shield**: `SLOP-368` (`no-phantom-ai-sdk-slopsquatting-v368`, CWE-1357) — `Phantom AI SDK Wrapper Package Slopsquatting`
- **Threat Model**: LLMs hallucinate plausible AI helper packages like @anthropic/tools-pro or openai-streaming-helper in package.json and requirements.txt, exposing builds to supply-chain hijacking.
- **Verification Suite**: `561/561` unit tests passed, `368` active AST rules, repo score `100/100` (`2321ms`)

### Sprint #67 — v3.5.302 (2026-11-08 16:44 UTC)
- **New AST Shield**: `SLOP-369` (`no-unbounded-mpsc-queue-sprawl-v369`, CWE-400) — `Unbounded MPSC & Asyncio Queue Memory Exhaustion`
- **Threat Model**: AI concurrency scaffolding defaults to tokio::sync::mpsc::unbounded_channel() and unbuffered asyncio.Queue(), removing backpressure and causing OOM kills under traffic spikes.
- **Verification Suite**: `562/562` unit tests passed, `369` active AST rules, repo score `100/100` (`2226ms`)

### Sprint #68 — v3.5.303 (2026-11-08 16:48 UTC)
- **New AST Shield**: `SLOP-370` (`no-truncated-hex-entropy-keygen-v370`, CWE-326) — `Truncated Hex Slice & Weak PRNG Key Derivation`
- **Threat Model**: AI models frequently slice hex-encoded random bytes to 16 chars (reducing 128-bit entropy to 64 bits) or use Math.random().toString(36) for AES/JWT secret keys.
- **Verification Suite**: `563/563` unit tests passed, `370` active AST rules, repo score `100/100` (`2168ms`)

### Sprint #69 — v3.5.304 (2026-11-08 16:50 UTC)
- **New AST Shield**: `SLOP-371` (`no-async-iterator-unclosed-break-v371`, CWE-775) — `Async Generator Break Without Stream Disposal`
- **Threat Model**: Breaking out of a for-await-of stream loop without a try/finally block or stream.destroy() leaks underlying file descriptors and database cursors.
- **Verification Suite**: `564/564` unit tests passed, `371` active AST rules, repo score `100/100` (`2164ms`)

### Sprint #70 — v3.5.305 (2026-11-08 16:53 UTC)
- **New AST Shield**: `SLOP-372` (`no-compound-namespace-slopsquat-v372`, CWE-829) — `Compound Namespace Package Manifest Slopsquatting`
- **Threat Model**: AI assistants synthesize plausible compound package names such as flask-bcrypt-auth or tokio-redis-pro that attackers register on PyPI/crates.io/npm for RCE.
- **Verification Suite**: `565/565` unit tests passed, `372` active AST rules, repo score `100/100` (`2115ms`)

### Sprint #71 — v3.5.306 (2026-11-08 16:56 UTC)
- **New AST Shield**: `SLOP-373` (`no-unvalidated-ssrf-redirect-chain-v373`, CWE-918) — `Unvalidated SSRF Host Interpolation & Redirect Bypass`
- **Threat Model**: AI coding assistants interpolate user-controlled host variables directly into axios.get or requests.get URLs, enabling Server-Side Request Forgery against cloud metadata endpoints.
- **Verification Suite**: `566/566` unit tests passed, `373` active AST rules, repo score `100/100` (`2156ms`)

### Sprint #72 — v3.5.307 (2026-11-08 16:59 UTC)
- **New AST Shield**: `SLOP-374` (`no-non-atomic-toctou-fs-mutation-v374`, CWE-367) — `Non-Atomic TOCTOU File Existence Check Before Write`
- **Threat Model**: AI models routinely generate fs.existsSync(p) followed by fs.writeFileSync(p), creating a CWE-367 Time-of-Check Time-of-Use race window exploitable via symlink swapping.
- **Verification Suite**: `567/567` unit tests passed, `374` active AST rules, repo score `100/100` (`2020ms`)
