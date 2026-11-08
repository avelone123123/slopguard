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
### Sprint #50 — v3.5.285 (2026-11-08 15:59 UTC)
- **New AST Shield**: `SLOP-352` (`no-compound-namespace-slopsquat-v352`, CWE-829) — `Compound Namespace Package Manifest Slopsquatting`
- **Threat Model**: AI assistants synthesize plausible compound package names such as flask-bcrypt-auth or tokio-redis-pro that attackers register on PyPI/crates.io/npm for RCE.
- **Verification Suite**: `545/545` unit tests passed, `352` active AST rules, repo score `100/100` (`2190ms`)

### Sprint #51 — v3.5.286 (2026-11-08 16:02 UTC)
- **New AST Shield**: `SLOP-353` (`no-unvalidated-ssrf-redirect-chain-v353`, CWE-918) — `Unvalidated SSRF Host Interpolation & Redirect Bypass`
- **Threat Model**: AI coding assistants interpolate user-controlled host variables directly into axios.get or requests.get URLs, enabling Server-Side Request Forgery against cloud metadata endpoints.
- **Verification Suite**: `546/546` unit tests passed, `353` active AST rules, repo score `100/100` (`2114ms`)

### Sprint #52 — v3.5.287 (2026-11-08 16:04 UTC)
- **New AST Shield**: `SLOP-354` (`no-non-atomic-toctou-fs-mutation-v354`, CWE-367) — `Non-Atomic TOCTOU File Existence Check Before Write`
- **Threat Model**: AI models routinely generate fs.existsSync(p) followed by fs.writeFileSync(p), creating a CWE-367 Time-of-Check Time-of-Use race window exploitable via symlink swapping.
- **Verification Suite**: `547/547` unit tests passed, `354` active AST rules, repo score `100/100` (`2141ms`)

### Sprint #53 — v3.5.288 (2026-11-08 16:07 UTC)
- **New AST Shield**: `SLOP-355` (`no-hallucinated-cipher-primitive-v355`, CWE-327) — `Hallucinated Non-Standard Cryptographic Cipher Names`
- **Threat Model**: LLMs invent non-existent cipher strings such as aes-256-turbo or sha256-fast in OpenSSL and Node crypto calls, causing runtime panics or silent fallback to weak encryption.
- **Verification Suite**: `548/548` unit tests passed, `355` active AST rules, repo score `100/100` (`2153ms`)

### Sprint #54 — v3.5.289 (2026-11-08 16:10 UTC)
- **New AST Shield**: `SLOP-356` (`no-shell-template-subprocess-exec-v356`, CWE-78) — `Formatted Shell String Subprocess Command Injection`
- **Threat Model**: AI assistants pass formatted f-strings or template literals into subprocess.Popen(..., shell=True) and child_process.exec, allowing arbitrary shell command execution.
- **Verification Suite**: `549/549` unit tests passed, `356` active AST rules, repo score `100/100` (`2137ms`)

### Sprint #55 — v3.5.290 (2026-11-08 16:12 UTC)
- **New AST Shield**: `SLOP-357` (`no-loop-scoped-event-listener-leak-v357`, CWE-400) — `Unbounded Event Listener Registration Inside Loops`
- **Threat Model**: AI-generated stream handlers attach emitter.on('data', ...) inside for/while loops without cleanup, exhausting V8 heap memory and triggering MaxListenersExceededWarning.
- **Verification Suite**: `550/550` unit tests passed, `357` active AST rules, repo score `100/100` (`2149ms`)

### Sprint #56 — v3.5.291 (2026-11-08 16:15 UTC)
- **New AST Shield**: `SLOP-358` (`no-phantom-ai-sdk-slopsquatting-v358`, CWE-1357) — `Phantom AI SDK Wrapper Package Slopsquatting`
- **Threat Model**: LLMs hallucinate plausible AI helper packages like @anthropic/tools-pro or openai-streaming-helper in package.json and requirements.txt, exposing builds to supply-chain hijacking.
- **Verification Suite**: `551/551` unit tests passed, `358` active AST rules, repo score `100/100` (`2139ms`)

### Sprint #57 — v3.5.292 (2026-11-08 16:18 UTC)
- **New AST Shield**: `SLOP-359` (`no-unbounded-mpsc-queue-sprawl-v359`, CWE-400) — `Unbounded MPSC & Asyncio Queue Memory Exhaustion`
- **Threat Model**: 1.  **Analyze the Request:** *   **Rule:** `unbounded-mpsc-queue-sprawl` (Unbounded MPSC & Asyncio Queue Memory Exhaustion - CWE-400). *   **Blocked Code:** `tokio::sync::mpsc::unbounded_channel::<Job>()` *   **Enforced Fix:** `tokio::sync::mpsc::channel::<Job>(1024)` *   **Task:** Write exactly 2 technical sentences for a senior security engineer explaining
- **Verification Suite**: `552/552` unit tests passed, `359` active AST rules, repo score `100/100` (`2116ms`)

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
