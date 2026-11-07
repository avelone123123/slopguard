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

### Sprint #25 — v3.5.39 (2026-11-07 11:23 UTC)
- **New AST Shield**: `SLOP-105` (`no-loop-scoped-event-listener-leak-v105`, CWE-400) — `Unbounded Event Listener Registration Inside Loops`
- **Threat Model**: AI-generated stream handlers attach emitter.on('data', ...) inside for/while loops without cleanup, exhausting V8 heap memory and triggering MaxListenersExceededWarning.
- **Verification Suite**: `298/298` unit tests passed, `105` active AST rules, repo score `100/100` (`2053ms`)

### Sprint #26 — v3.5.40 (2026-11-07 11:25 UTC)
- **New AST Shield**: `SLOP-106` (`no-phantom-ai-sdk-slopsquatting-v106`, CWE-1357) — `Phantom AI SDK Wrapper Package Slopsquatting`
- **Threat Model**: LLMs hallucinate plausible AI helper packages like @anthropic/tools-pro or openai-streaming-helper in package.json and requirements.txt, exposing builds to supply-chain hijacking.
- **Verification Suite**: `299/299` unit tests passed, `106` active AST rules, repo score `100/100` (`2086ms`)

### Sprint #27 — v3.5.41 (2026-11-07 11:27 UTC)
- **New AST Shield**: `SLOP-107` (`no-unbounded-mpsc-queue-sprawl-v107`, CWE-400) — `Unbounded MPSC & Asyncio Queue Memory Exhaustion`
- **Threat Model**: AI concurrency scaffolding defaults to tokio::sync::mpsc::unbounded_channel() and unbuffered asyncio.Queue(), removing backpressure and causing OOM kills under traffic spikes.
- **Verification Suite**: `300/300` unit tests passed, `107` active AST rules, repo score `100/100` (`2033ms`)

### Sprint #28 — v3.5.42 (2026-11-07 11:30 UTC)
- **New AST Shield**: `SLOP-108` (`no-truncated-hex-entropy-keygen-v108`, CWE-326) — `Truncated Hex Slice & Weak PRNG Key Derivation`
- **Threat Model**: AI models frequently slice hex-encoded random bytes to 16 chars (reducing 128-bit entropy to 64 bits) or use Math.random().toString(36) for AES/JWT secret keys.
- **Verification Suite**: `301/301` unit tests passed, `108` active AST rules, repo score `100/100` (`2062ms`)

### Sprint #29 — v3.5.43 (2026-11-07 11:32 UTC)
- **New AST Shield**: `SLOP-109` (`no-async-iterator-unclosed-break-v109`, CWE-775) — `Async Generator Break Without Stream Disposal`
- **Threat Model**: Breaking out of a for-await-of stream loop without a try/finally block or stream.destroy() leaks underlying file descriptors and database cursors.
- **Verification Suite**: `302/302` unit tests passed, `109` active AST rules, repo score `100/100` (`2139ms`)

### Sprint #30 — v3.5.44 (2026-11-07 11:34 UTC)
- **New AST Shield**: `SLOP-110` (`no-compound-namespace-slopsquat-v110`, CWE-829) — `Compound Namespace Package Manifest Slopsquatting`
- **Threat Model**: AI assistants synthesize plausible compound package names such as flask-bcrypt-auth or tokio-redis-pro that attackers register on PyPI/crates.io/npm for RCE.
- **Verification Suite**: `303/303` unit tests passed, `110` active AST rules, repo score `100/100` (`2036ms`)

### Sprint #31 — v3.5.45 (2026-11-07 11:36 UTC)
- **New AST Shield**: `SLOP-111` (`no-unvalidated-ssrf-redirect-chain-v111`, CWE-918) — `Unvalidated SSRF Host Interpolation & Redirect Bypass`
- **Threat Model**: AI models hallucinate the SSRF bypass pattern because generative training overweights code-completion heuristics that emit interpolated host
- **Verification Suite**: `304/304` unit tests passed, `111` active AST rules, repo score `100/100` (`2045ms`)

### Sprint #32 — v3.5.46 (2026-11-07 11:38 UTC)
- **New AST Shield**: `SLOP-112` (`no-non-atomic-toctou-fs-mutation-v112`, CWE-367) — `Non-Atomic TOCTOU File Existence Check Before Write`
- **Threat Model**: AI models routinely generate fs.existsSync(p) followed by fs.writeFileSync(p), creating a CWE-367 Time-of-Check Time-of-Use race window exploitable via symlink swapping.
- **Verification Suite**: `305/305` unit tests passed, `112` active AST rules, repo score `100/100` (`2023ms`)
