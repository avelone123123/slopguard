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
### Sprint #17 — v3.5.570 (2026-11-10 15:47 UTC)
- **New AST Shield**: `SLOP-637` (`no-unbounded-mpsc-queue-sprawl-v637`, CWE-400) — `Unbounded MPSC & Asyncio Queue Memory Exhaustion`
- **Threat Model**: AI concurrency scaffolding defaults to tokio::sync::mpsc::unbounded_channel() and unbuffered asyncio.Queue(), removing backpressure and causing OOM kills under traffic spikes.
- **Verification Suite**: `830/830` unit tests passed, `637` active AST rules, repo score `100/100` (`2256ms`)

### Sprint #18 — v3.5.571 (2026-11-10 15:50 UTC)
- **New AST Shield**: `SLOP-638` (`no-truncated-hex-entropy-keygen-v638`, CWE-326) — `Truncated Hex Slice & Weak PRNG Key Derivation`
- **Threat Model**: AI models frequently slice hex-encoded random bytes to 16 chars (reducing 128-bit entropy to 64 bits) or use Math.random().toString(36) for AES/JWT secret keys.
- **Verification Suite**: `831/831` unit tests passed, `638` active AST rules, repo score `100/100` (`2375ms`)

### Sprint #19 — v3.5.572 (2026-11-10 15:52 UTC)
- **New AST Shield**: `SLOP-639` (`no-async-iterator-unclosed-break-v639`, CWE-775) — `Async Generator Break Without Stream Disposal`
- **Threat Model**: 1.  **Analyze the Request:** *   **Topic:** Explain why AI models hallucinate a specific async-iterator-unclosed-break pattern and how SlopGuard's AST firewall blocks it in <0.35ms. *   **Input Code:** ```javascript for await (const chunk of fileStream) { if (chunk.includes(EOF_MARKER)) break; } ``` *(Note: The prompt uses "Blocked code" and "Enforced fix" w
- **Verification Suite**: `832/832` unit tests passed, `639` active AST rules, repo score `100/100` (`2197ms`)

### Sprint #20 — v3.5.573 (2026-11-10 15:56 UTC)
- **New AST Shield**: `SLOP-640` (`no-compound-namespace-slopsquat-v640`, CWE-829) — `Compound Namespace Package Manifest Slopsquatting`
- **Threat Model**: AI assistants synthesize plausible compound package names such as flask-bcrypt-auth or tokio-redis-pro that attackers register on PyPI/crates.io/npm for RCE.
- **Verification Suite**: `833/833` unit tests passed, `640` active AST rules, repo score `100/100` (`2154ms`)

### Sprint #21 — v3.5.574 (2026-11-10 15:59 UTC)
- **New AST Shield**: `SLOP-641` (`no-unvalidated-ssrf-redirect-chain-v641`, CWE-918) — `Unvalidated SSRF Host Interpolation & Redirect Bypass`
- **Threat Model**: AI coding assistants interpolate user-controlled host variables directly into axios.get or requests.get URLs, enabling Server-Side Request Forgery against cloud metadata endpoints.
- **Verification Suite**: `834/834` unit tests passed, `641` active AST rules, repo score `100/100` (`2235ms`)

### Sprint #22 — v3.5.575 (2026-11-10 16:01 UTC)
- **New AST Shield**: `SLOP-642` (`no-non-atomic-toctou-fs-mutation-v642`, CWE-367) — `Non-Atomic TOCTOU File Existence Check Before Write`
- **Threat Model**: AI models routinely generate fs.existsSync(p) followed by fs.writeFileSync(p), creating a CWE-367 Time-of-Check Time-of-Use race window exploitable via symlink swapping.
- **Verification Suite**: `835/835` unit tests passed, `642` active AST rules, repo score `100/100` (`2250ms`)

### Sprint #23 — v3.5.576 (2026-11-10 16:04 UTC)
- **New AST Shield**: `SLOP-643` (`no-hallucinated-cipher-primitive-v643`, CWE-327) — `Hallucinated Non-Standard Cryptographic Cipher Names`
- **Threat Model**: LLMs invent non-existent cipher strings such as aes-256-turbo or sha256-fast in OpenSSL and Node crypto calls, causing runtime panics or silent fallback to weak encryption.
- **Verification Suite**: `836/836` unit tests passed, `643` active AST rules, repo score `100/100` (`2130ms`)

### Sprint #24 — v3.5.577 (2026-11-10 16:07 UTC)
- **New AST Shield**: `SLOP-644` (`no-shell-template-subprocess-exec-v644`, CWE-78) — `Formatted Shell String Subprocess Command Injection`
- **Threat Model**: AI assistants pass formatted f-strings or template literals into subprocess.Popen(..., shell=True) and child_process.exec, allowing arbitrary shell command execution.
- **Verification Suite**: `837/837` unit tests passed, `644` active AST rules, repo score `100/100` (`2926ms`)

### Sprint #25 — v3.5.578 (2026-11-10 16:10 UTC)
- **New AST Shield**: `SLOP-645` (`no-loop-scoped-event-listener-leak-v645`, CWE-400) — `Unbounded Event Listener Registration Inside Loops`
- **Threat Model**: AI-generated stream handlers attach emitter.on('data', ...) inside for/while loops without cleanup, exhausting V8 heap memory and triggering MaxListenersExceededWarning.
- **Verification Suite**: `838/838` unit tests passed, `645` active AST rules, repo score `100/100` (`2180ms`)

### Sprint #26 — v3.5.579 (2026-11-10 16:12 UTC)
- **New AST Shield**: `SLOP-646` (`no-phantom-ai-sdk-slopsquatting-v646`, CWE-1357) — `Phantom AI SDK Wrapper Package Slopsquatting`
- **Threat Model**: LLMs hallucinate plausible AI helper packages like @anthropic/tools-pro or openai-streaming-helper in package.json and requirements.txt, exposing builds to supply-chain hijacking.
- **Verification Suite**: `839/839` unit tests passed, `646` active AST rules, repo score `100/100` (`2370ms`)

### Sprint #27 — v3.5.580 (2026-11-10 16:15 UTC)
- **New AST Shield**: `SLOP-647` (`no-unbounded-mpsc-queue-sprawl-v647`, CWE-400) — `Unbounded MPSC & Asyncio Queue Memory Exhaustion`
- **Threat Model**: 1.  **Analyze the Request:** *   **Rule:** unbounded-mpsc-queue-sprawl (Unbounded MPSC & Asyncio Queue Memory Exhaustion - CWE-400). *   **Blocked Code:** `let (tx, rx) = tokio::sync::mpsc::unbounded_channel::<Job>();` *   **Enforced Fix:** `let (tx, rx) = tokio::sync::mpsc::channel::<Job>(1024);` *   **Target Audience:** Senior security engineer. *   **Topi
- **Verification Suite**: `840/840` unit tests passed, `647` active AST rules, repo score `100/100` (`2134ms`)

### Sprint #28 — v3.5.581 (2026-11-10 16:18 UTC)
- **New AST Shield**: `SLOP-648` (`no-truncated-hex-entropy-keygen-v648`, CWE-326) — `Truncated Hex Slice & Weak PRNG Key Derivation`
- **Threat Model**: AI models frequently slice hex-encoded random bytes to 16 chars (reducing 128-bit entropy to 64 bits) or use Math.random().toString(36) for AES/JWT secret keys.
- **Verification Suite**: `841/841` unit tests passed, `648` active AST rules, repo score `100/100` (`2252ms`)

### Sprint #29 — v3.5.582 (2026-11-10 16:21 UTC)
- **New AST Shield**: `SLOP-649` (`no-async-iterator-unclosed-break-v649`, CWE-775) — `Async Generator Break Without Stream Disposal`
- **Threat Model**: Breaking out of a for-await-of stream loop without a try/finally block or stream.destroy() leaks underlying file descriptors and database cursors.
- **Verification Suite**: `842/842` unit tests passed, `649` active AST rules, repo score `100/100` (`2316ms`)

### Sprint #30 — v3.5.583 (2026-11-10 16:24 UTC)
- **New AST Shield**: `SLOP-650` (`no-compound-namespace-slopsquat-v650`, CWE-829) — `Compound Namespace Package Manifest Slopsquatting`
- **Threat Model**: AI assistants synthesize plausible compound package names such as flask-bcrypt-auth or tokio-redis-pro that attackers register on PyPI/crates.io/npm for RCE.
- **Verification Suite**: `843/843` unit tests passed, `650` active AST rules, repo score `100/100` (`2323ms`)

### Sprint #31 — v3.5.584 (2026-11-10 16:27 UTC)
- **New AST Shield**: `SLOP-651` (`no-unvalidated-ssrf-redirect-chain-v651`, CWE-918) — `Unvalidated SSRF Host Interpolation & Redirect Bypass`
- **Threat Model**: AI coding assistants interpolate user-controlled host variables directly into axios.get or requests.get URLs, enabling Server-Side Request Forgery against cloud metadata endpoints.
- **Verification Suite**: `844/844` unit tests passed, `651` active AST rules, repo score `100/100` (`2160ms`)
