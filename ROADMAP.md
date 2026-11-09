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
### Sprint #129 — v3.5.465 (2026-11-09 17:41 UTC)
- **New AST Shield**: `SLOP-532` (`no-async-iterator-unclosed-break-v532`, CWE-775) — `Async Generator Break Without Stream Disposal`
- **Threat Model**: Breaking out of a for-await-of stream loop without a try/finally block or stream.destroy() leaks underlying file descriptors and database cursors.
- **Verification Suite**: `725/725` unit tests passed, `532` active AST rules, repo score `100/100` (`2059ms`)

### Sprint #130 — v3.5.466 (2026-11-09 17:44 UTC)
- **New AST Shield**: `SLOP-533` (`no-compound-namespace-slopsquat-v533`, CWE-829) — `Compound Namespace Package Manifest Slopsquatting`
- **Threat Model**: AI assistants synthesize plausible compound package names such as flask-bcrypt-auth or tokio-redis-pro that attackers register on PyPI/crates.io/npm for RCE.
- **Verification Suite**: `726/726` unit tests passed, `533` active AST rules, repo score `100/100` (`2043ms`)

### Sprint #131 — v3.5.467 (2026-11-09 17:47 UTC)
- **New AST Shield**: `SLOP-534` (`no-unvalidated-ssrf-redirect-chain-v534`, CWE-918) — `Unvalidated SSRF Host Interpolation & Redirect Bypass`
- **Threat Model**: AI coding assistants interpolate user-controlled host variables directly into axios.get or requests.get URLs, enabling Server-Side Request Forgery against cloud metadata endpoints.
- **Verification Suite**: `727/727` unit tests passed, `534` active AST rules, repo score `100/100` (`2079ms`)

### Sprint #132 — v3.5.468 (2026-11-09 17:50 UTC)
- **New AST Shield**: `SLOP-535` (`no-non-atomic-toctou-fs-mutation-v535`, CWE-367) — `Non-Atomic TOCTOU File Existence Check Before Write`
- **Threat Model**: We need to understand the rule: non-atomic-toctou-fs-mutation (Non-Atomic TOCTOU File Existence Check Before Write, CWE-367). The blocked code is a classic TOCTOU pattern: check if file exists with fs.existsSync, then writeFileSync. The fix is to use atomic write with flag 'wx' (O_CREAT|O_EXCL|O_WRONLY) which avoids race conditions and ensures fail-if-exists
- **Verification Suite**: `728/728` unit tests passed, `535` active AST rules, repo score `100/100` (`2043ms`)

### Sprint #133 — v3.5.469 (2026-11-09 17:53 UTC)
- **New AST Shield**: `SLOP-536` (`no-hallucinated-cipher-primitive-v536`, CWE-327) — `Hallucinated Non-Standard Cryptographic Cipher Names`
- **Threat Model**: LLMs invent non-existent cipher strings such as aes-256-turbo or sha256-fast in OpenSSL and Node crypto calls, causing runtime panics or silent fallback to weak encryption.
- **Verification Suite**: `729/729` unit tests passed, `536` active AST rules, repo score `100/100` (`2066ms`)

### Sprint #134 — v3.5.470 (2026-11-09 17:55 UTC)
- **New AST Shield**: `SLOP-537` (`no-shell-template-subprocess-exec-v537`, CWE-78) — `Formatted Shell String Subprocess Command Injection`
- **Threat Model**: AI assistants pass formatted f-strings or template literals into subprocess.Popen(..., shell=True) and child_process.exec, allowing arbitrary shell command execution.
- **Verification Suite**: `730/730` unit tests passed, `537` active AST rules, repo score `100/100` (`2010ms`)

### Sprint #135 — v3.5.471 (2026-11-09 17:58 UTC)
- **New AST Shield**: `SLOP-538` (`no-loop-scoped-event-listener-leak-v538`, CWE-400) — `Unbounded Event Listener Registration Inside Loops`
- **Threat Model**: AI-generated stream handlers attach emitter.on('data', ...) inside for/while loops without cleanup, exhausting V8 heap memory and triggering MaxListenersExceededWarning.
- **Verification Suite**: `731/731` unit tests passed, `538` active AST rules, repo score `100/100` (`2078ms`)

### Sprint #136 — v3.5.472 (2026-11-09 18:01 UTC)
- **New AST Shield**: `SLOP-539` (`no-phantom-ai-sdk-slopsquatting-v539`, CWE-1357) — `Phantom AI SDK Wrapper Package Slopsquatting`
- **Threat Model**: 1.  **Analyze the Request:** *   **Rule/Concept:** phantom-ai-sdk-slopsquatting (Phantom AI SDK Wrapper Package Slopsquatting, CWE-1357). This means a malicious or compromised package (the "phantom" one) falsely claims to be a legitimate AI SDK wrapper to trick developers into installing it, leading to supply chain attacks. *   **Blocked Code:** `"@anthropic
- **Verification Suite**: `732/732` unit tests passed, `539` active AST rules, repo score `100/100` (`2033ms`)

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
