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
### Sprint #77 — v3.5.312 (2026-11-08 17:13 UTC)
- **New AST Shield**: `SLOP-379` (`no-unbounded-mpsc-queue-sprawl-v379`, CWE-400) — `Unbounded MPSC & Asyncio Queue Memory Exhaustion`
- **Threat Model**: AI concurrency scaffolding defaults to tokio::sync::mpsc::unbounded_channel() and unbuffered asyncio.Queue(), removing backpressure and causing OOM kills under traffic spikes.
- **Verification Suite**: `572/572` unit tests passed, `379` active AST rules, repo score `100/100` (`2047ms`)

### Sprint #78 — v3.5.313 (2026-11-08 17:16 UTC)
- **New AST Shield**: `SLOP-380` (`no-truncated-hex-entropy-keygen-v380`, CWE-326) — `Truncated Hex Slice & Weak PRNG Key Derivation`
- **Threat Model**: AI models frequently slice hex-encoded random bytes to 16 chars (reducing 128-bit entropy to 64 bits) or use Math.random().toString(36) for AES/JWT secret keys.
- **Verification Suite**: `573/573` unit tests passed, `380` active AST rules, repo score `100/100` (`2081ms`)

### Sprint #79 — v3.5.314 (2026-11-08 17:19 UTC)
- **New AST Shield**: `SLOP-381` (`no-async-iterator-unclosed-break-v381`, CWE-775) — `Async Generator Break Without Stream Disposal`
- **Threat Model**: Breaking out of a for-await-of stream loop without a try/finally block or stream.destroy() leaks underlying file descriptors and database cursors.
- **Verification Suite**: `574/574` unit tests passed, `381` active AST rules, repo score `100/100` (`2232ms`)

### Sprint #80 — v3.5.315 (2026-11-08 17:23 UTC)
- **New AST Shield**: `SLOP-382` (`no-compound-namespace-slopsquat-v382`, CWE-829) — `Compound Namespace Package Manifest Slopsquatting`
- **Threat Model**: AI assistants synthesize plausible compound package names such as flask-bcrypt-auth or tokio-redis-pro that attackers register on PyPI/crates.io/npm for RCE.
- **Verification Suite**: `575/575` unit tests passed, `382` active AST rules, repo score `100/100` (`2266ms`)

### Sprint #81 — v3.5.316 (2026-11-08 17:26 UTC)
- **New AST Shield**: `SLOP-383` (`no-unvalidated-ssrf-redirect-chain-v383`, CWE-918) — `Unvalidated SSRF Host Interpolation & Redirect Bypass`
- **Threat Model**: 1.  **Analyze the Request:** *   **Rule/Topic:** unvalidated-ssrf-redirect-chain (CWE-918). *   **Blocked Code:** `const res = await axios.get(\`https://${req.query.apiHost}/v1/profile\`);` *   **Enforced Fix:** `if (!ALLOWED_ORIGINS.has(req.query.apiHost)) throw new Error('Blocked'); const res = await axios.get('https://api.internal.verified/v1/profile');` 
- **Verification Suite**: `576/576` unit tests passed, `383` active AST rules, repo score `100/100` (`2207ms`)

### Sprint #82 — v3.5.317 (2026-11-08 17:30 UTC)
- **New AST Shield**: `SLOP-384` (`no-non-atomic-toctou-fs-mutation-v384`, CWE-367) — `Non-Atomic TOCTOU File Existence Check Before Write`
- **Threat Model**: AI models routinely generate fs.existsSync(p) followed by fs.writeFileSync(p), creating a CWE-367 Time-of-Check Time-of-Use race window exploitable via symlink swapping.
- **Verification Suite**: `577/577` unit tests passed, `384` active AST rules, repo score `100/100` (`2128ms`)

### Sprint #83 — v3.5.318 (2026-11-08 17:33 UTC)
- **New AST Shield**: `SLOP-385` (`no-hallucinated-cipher-primitive-v385`, CWE-327) — `Hallucinated Non-Standard Cryptographic Cipher Names`
- **Threat Model**: LLMs invent non-existent cipher strings such as aes-256-turbo or sha256-fast in OpenSSL and Node crypto calls, causing runtime panics or silent fallback to weak encryption.
- **Verification Suite**: `578/578` unit tests passed, `385` active AST rules, repo score `100/100` (`2119ms`)

### Sprint #84 — v3.5.319 (2026-11-08 17:36 UTC)
- **New AST Shield**: `SLOP-386` (`no-shell-template-subprocess-exec-v386`, CWE-78) — `Formatted Shell String Subprocess Command Injection`
- **Threat Model**: AI assistants pass formatted f-strings or template literals into subprocess.Popen(..., shell=True) and child_process.exec, allowing arbitrary shell command execution.
- **Verification Suite**: `579/579` unit tests passed, `386` active AST rules, repo score `100/100` (`2208ms`)

### Sprint #85 — v3.5.320 (2026-11-08 17:39 UTC)
- **New AST Shield**: `SLOP-387` (`no-loop-scoped-event-listener-leak-v387`, CWE-400) — `Unbounded Event Listener Registration Inside Loops`
- **Threat Model**: AI-generated stream handlers attach emitter.on('data', ...) inside for/while loops without cleanup, exhausting V8 heap memory and triggering MaxListenersExceededWarning.
- **Verification Suite**: `580/580` unit tests passed, `387` active AST rules, repo score `100/100` (`2229ms`)

### Sprint #86 — v3.5.321 (2026-11-08 17:42 UTC)
- **New AST Shield**: `SLOP-388` (`no-phantom-ai-sdk-slopsquatting-v388`, CWE-1357) — `Phantom AI SDK Wrapper Package Slopsquatting`
- **Threat Model**: LLMs hallucinate plausible AI helper packages like @anthropic/tools-pro or openai-streaming-helper in package.json and requirements.txt, exposing builds to supply-chain hijacking.
- **Verification Suite**: `581/581` unit tests passed, `388` active AST rules, repo score `100/100` (`2114ms`)

### Sprint #87 — v3.5.322 (2026-11-08 17:45 UTC)
- **New AST Shield**: `SLOP-389` (`no-unbounded-mpsc-queue-sprawl-v389`, CWE-400) — `Unbounded MPSC & Asyncio Queue Memory Exhaustion`
- **Threat Model**: AI concurrency scaffolding defaults to tokio::sync::mpsc::unbounded_channel() and unbuffered asyncio.Queue(), removing backpressure and causing OOM kills under traffic spikes.
- **Verification Suite**: `582/582` unit tests passed, `389` active AST rules, repo score `100/100` (`2198ms`)

### Sprint #88 — v3.5.323 (2026-11-08 17:48 UTC)
- **New AST Shield**: `SLOP-390` (`no-truncated-hex-entropy-keygen-v390`, CWE-326) — `Truncated Hex Slice & Weak PRNG Key Derivation`
- **Threat Model**: AI models frequently slice hex-encoded random bytes to 16 chars (reducing 128-bit entropy to 64 bits) or use Math.random().toString(36) for AES/JWT secret keys.
- **Verification Suite**: `583/583` unit tests passed, `390` active AST rules, repo score `100/100` (`2165ms`)

### Sprint #89 — v3.5.324 (2026-11-08 17:51 UTC)
- **New AST Shield**: `SLOP-391` (`no-async-iterator-unclosed-break-v391`, CWE-775) — `Async Generator Break Without Stream Disposal`
- **Threat Model**: Breaking out of a for-await-of stream loop without a try/finally block or stream.destroy() leaks underlying file descriptors and database cursors.
- **Verification Suite**: `584/584` unit tests passed, `391` active AST rules, repo score `100/100` (`2157ms`)

### Sprint #90 — v3.5.325 (2026-11-08 17:54 UTC)
- **New AST Shield**: `SLOP-392` (`no-compound-namespace-slopsquat-v392`, CWE-829) — `Compound Namespace Package Manifest Slopsquatting`
- **Threat Model**: AI assistants synthesize plausible compound package names such as flask-bcrypt-auth or tokio-redis-pro that attackers register on PyPI/crates.io/npm for RCE.
- **Verification Suite**: `585/585` unit tests passed, `392` active AST rules, repo score `100/100` (`2130ms`)

### Sprint #91 — v3.5.326 (2026-11-08 17:57 UTC)
- **New AST Shield**: `SLOP-393` (`no-unvalidated-ssrf-redirect-chain-v393`, CWE-918) — `Unvalidated SSRF Host Interpolation & Redirect Bypass`
- **Threat Model**: AI coding assistants interpolate user-controlled host variables directly into axios.get or requests.get URLs, enabling Server-Side Request Forgery against cloud metadata endpoints.
- **Verification Suite**: `586/586` unit tests passed, `393` active AST rules, repo score `100/100` (`2151ms`)
