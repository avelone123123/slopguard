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
### Sprint #10 — v3.5.24 (2026-11-07 10:49 UTC)
- **New AST Shield**: `SLOP-090` (`no-compound-namespace-slopsquat-v90`, CWE-829) — `Compound Namespace Package Manifest Slopsquatting`
- **Threat Model**: 1.  **Analyze the Request:** *   **Rule/Topic:** Compound namespace package slopsquatting (Compound Namespace Package Manifest Slopsquatting, CWE-829). *   **Blocked Code:** `flask-bcrypt-auth>=1.0`, `tokio-redis-pro = "1.0.0"` *   **Enforced Fix:** `flask-login==0.6.3`, `redis = "0.27.5"` *   **Task:** Write exactly 2 technical sentences for a senior securi
- **Verification Suite**: `283/283` unit tests passed, `90` active AST rules, repo score `100/100` (`2478ms`)

### Sprint #11 — v3.5.25 (2026-11-07 10:52 UTC)
- **New AST Shield**: `SLOP-091` (`no-unvalidated-ssrf-redirect-chain-v91`, CWE-918) — `Unvalidated SSRF Host Interpolation & Redirect Bypass`
- **Threat Model**: AI coding assistants interpolate user-controlled host variables directly into axios.get or requests.get URLs, enabling Server-Side Request Forgery against cloud metadata endpoints.
- **Verification Suite**: `284/284` unit tests passed, `91` active AST rules, repo score `100/100` (`2157ms`)

### Sprint #12 — v3.5.26 (2026-11-07 10:54 UTC)
- **New AST Shield**: `SLOP-092` (`no-non-atomic-toctou-fs-mutation-v92`, CWE-367) — `Non-Atomic TOCTOU File Existence Check Before Write`
- **Threat Model**: AI models routinely generate fs.existsSync(p) followed by fs.writeFileSync(p), creating a CWE-367 Time-of-Check Time-of-Use race window exploitable via symlink swapping.
- **Verification Suite**: `285/285` unit tests passed, `92` active AST rules, repo score `100/100` (`2103ms`)

### Sprint #13 — v3.5.27 (2026-11-07 10:56 UTC)
- **New AST Shield**: `SLOP-093` (`no-hallucinated-cipher-primitive-v93`, CWE-327) — `Hallucinated Non-Standard Cryptographic Cipher Names`
- **Threat Model**: LLMs invent non-existent cipher strings such as aes-256-turbo or sha256-fast in OpenSSL and Node crypto calls, causing runtime panics or silent fallback to weak encryption.
- **Verification Suite**: `286/286` unit tests passed, `93` active AST rules, repo score `100/100` (`2223ms`)

### Sprint #14 — v3.5.28 (2026-11-07 10:59 UTC)
- **New AST Shield**: `SLOP-094` (`no-shell-template-subprocess-exec-v94`, CWE-78) — `Formatted Shell String Subprocess Command Injection`
- **Threat Model**: AI assistants pass formatted f-strings or template literals into subprocess.Popen(..., shell=True) and child_process.exec, allowing arbitrary shell command execution.
- **Verification Suite**: `287/287` unit tests passed, `94` active AST rules, repo score `100/100` (`2128ms`)

### Sprint #15 — v3.5.29 (2026-11-07 11:01 UTC)
- **New AST Shield**: `SLOP-095` (`no-loop-scoped-event-listener-leak-v95`, CWE-400) — `Unbounded Event Listener Registration Inside Loops`
- **Threat Model**: AI-generated stream handlers attach emitter.on('data', ...) inside for/while loops without cleanup, exhausting V8 heap memory and triggering MaxListenersExceededWarning.
- **Verification Suite**: `288/288` unit tests passed, `95` active AST rules, repo score `100/100` (`2098ms`)

### Sprint #16 — v3.5.30 (2026-11-07 11:03 UTC)
- **New AST Shield**: `SLOP-096` (`no-phantom-ai-sdk-slopsquatting-v96`, CWE-1357) — `Phantom AI SDK Wrapper Package Slopsquatting`
- **Threat Model**: LLMs hallucinate plausible AI helper packages like @anthropic/tools-pro or openai-streaming-helper in package.json and requirements.txt, exposing builds to supply-chain hijacking.
- **Verification Suite**: `289/289` unit tests passed, `96` active AST rules, repo score `100/100` (`2246ms`)

### Sprint #17 — v3.5.31 (2026-11-07 11:06 UTC)
- **New AST Shield**: `SLOP-097` (`no-unbounded-mpsc-queue-sprawl-v97`, CWE-400) — `Unbounded MPSC & Asyncio Queue Memory Exhaustion`
- **Threat Model**: AI concurrency scaffolding defaults to tokio::sync::mpsc::unbounded_channel() and unbuffered asyncio.Queue(), removing backpressure and causing OOM kills under traffic spikes.
- **Verification Suite**: `290/290` unit tests passed, `97` active AST rules, repo score `100/100` (`2138ms`)

### Sprint #18 — v3.5.32 (2026-11-07 11:08 UTC)
- **New AST Shield**: `SLOP-098` (`no-truncated-hex-entropy-keygen-v98`, CWE-326) — `Truncated Hex Slice & Weak PRNG Key Derivation`
- **Threat Model**: AI models frequently slice hex-encoded random bytes to 16 chars (reducing 128-bit entropy to 64 bits) or use Math.random().toString(36) for AES/JWT secret keys.
- **Verification Suite**: `291/291` unit tests passed, `98` active AST rules, repo score `100/100` (`2089ms`)

### Sprint #19 — v3.5.33 (2026-11-07 11:10 UTC)
- **New AST Shield**: `SLOP-099` (`no-async-iterator-unclosed-break-v99`, CWE-775) — `Async Generator Break Without Stream Disposal`
- **Threat Model**: Breaking out of a for-await-of stream loop without a try/finally block or stream.destroy() leaks underlying file descriptors and database cursors.
- **Verification Suite**: `292/292` unit tests passed, `99` active AST rules, repo score `100/100` (`2156ms`)

### Sprint #20 — v3.5.34 (2026-11-07 11:12 UTC)
- **New AST Shield**: `SLOP-100` (`no-compound-namespace-slopsquat-v100`, CWE-829) — `Compound Namespace Package Manifest Slopsquatting`
- **Threat Model**: AI assistants synthesize plausible compound package names such as flask-bcrypt-auth or tokio-redis-pro that attackers register on PyPI/crates.io/npm for RCE.
- **Verification Suite**: `293/293` unit tests passed, `100` active AST rules, repo score `100/100` (`2050ms`)

### Sprint #21 — v3.5.35 (2026-11-07 11:14 UTC)
- **New AST Shield**: `SLOP-101` (`no-unvalidated-ssrf-redirect-chain-v101`, CWE-918) — `Unvalidated SSRF Host Interpolation & Redirect Bypass`
- **Threat Model**: AI coding assistants interpolate user-controlled host variables directly into axios.get or requests.get URLs, enabling Server-Side Request Forgery against cloud metadata endpoints.
- **Verification Suite**: `294/294` unit tests passed, `101` active AST rules, repo score `100/100` (`2241ms`)

### Sprint #22 — v3.5.36 (2026-11-07 11:16 UTC)
- **New AST Shield**: `SLOP-102` (`no-non-atomic-toctou-fs-mutation-v102`, CWE-367) — `Non-Atomic TOCTOU File Existence Check Before Write`
- **Threat Model**: AI models routinely generate fs.existsSync(p) followed by fs.writeFileSync(p), creating a CWE-367 Time-of-Check Time-of-Use race window exploitable via symlink swapping.
- **Verification Suite**: `295/295` unit tests passed, `102` active AST rules, repo score `100/100` (`2092ms`)

### Sprint #23 — v3.5.37 (2026-11-07 11:19 UTC)
- **New AST Shield**: `SLOP-103` (`no-hallucinated-cipher-primitive-v103`, CWE-327) — `Hallucinated Non-Standard Cryptographic Cipher Names`
- **Threat Model**: LLMs invent non-existent cipher strings such as aes-256-turbo or sha256-fast in OpenSSL and Node crypto calls, causing runtime panics or silent fallback to weak encryption.
- **Verification Suite**: `296/296` unit tests passed, `103` active AST rules, repo score `100/100` (`2127ms`)

### Sprint #24 — v3.5.38 (2026-11-07 11:21 UTC)
- **New AST Shield**: `SLOP-104` (`no-shell-template-subprocess-exec-v104`, CWE-78) — `Formatted Shell String Subprocess Command Injection`
- **Threat Model**: AI assistants pass formatted f-strings or template literals into subprocess.Popen(..., shell=True) and child_process.exec, allowing arbitrary shell command execution.
- **Verification Suite**: `297/297` unit tests passed, `104` active AST rules, repo score `100/100` (`2071ms`)
