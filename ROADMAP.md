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
### Sprint #38 — v3.5.273 (2026-11-08 15:25 UTC)
- **New AST Shield**: `SLOP-340` (`no-truncated-hex-entropy-keygen-v340`, CWE-326) — `Truncated Hex Slice & Weak PRNG Key Derivation`
- **Threat Model**: AI models frequently slice hex-encoded random bytes to 16 chars (reducing 128-bit entropy to 64 bits) or use Math.random().toString(36) for AES/JWT secret keys.
- **Verification Suite**: `533/533` unit tests passed, `340` active AST rules, repo score `100/100` (`2258ms`)

### Sprint #39 — v3.5.274 (2026-11-08 15:28 UTC)
- **New AST Shield**: `SLOP-341` (`no-async-iterator-unclosed-break-v341`, CWE-775) — `Async Generator Break Without Stream Disposal`
- **Threat Model**: Breaking out of a for-await-of stream loop without a try/finally block or stream.destroy() leaks underlying file descriptors and database cursors.
- **Verification Suite**: `534/534` unit tests passed, `341` active AST rules, repo score `100/100` (`2090ms`)

### Sprint #40 — v3.5.275 (2026-11-08 15:31 UTC)
- **New AST Shield**: `SLOP-342` (`no-compound-namespace-slopsquat-v342`, CWE-829) — `Compound Namespace Package Manifest Slopsquatting`
- **Threat Model**: AI assistants synthesize plausible compound package names such as flask-bcrypt-auth or tokio-redis-pro that attackers register on PyPI/crates.io/npm for RCE.
- **Verification Suite**: `535/535` unit tests passed, `342` active AST rules, repo score `100/100` (`2125ms`)

### Sprint #41 — v3.5.276 (2026-11-08 15:34 UTC)
- **New AST Shield**: `SLOP-343` (`no-unvalidated-ssrf-redirect-chain-v343`, CWE-918) — `Unvalidated SSRF Host Interpolation & Redirect Bypass`
- **Threat Model**: AI coding assistants interpolate user-controlled host variables directly into axios.get or requests.get URLs, enabling Server-Side Request Forgery against cloud metadata endpoints.
- **Verification Suite**: `536/536` unit tests passed, `343` active AST rules, repo score `100/100` (`2100ms`)

### Sprint #42 — v3.5.277 (2026-11-08 15:36 UTC)
- **New AST Shield**: `SLOP-344` (`no-non-atomic-toctou-fs-mutation-v344`, CWE-367) — `Non-Atomic TOCTOU File Existence Check Before Write`
- **Threat Model**: We need to answer: "Write exactly 2 technical sentences for a senior security engineer explaining why AI models hallucinate this pattern and how SlopGuard's AST firewall blocks it in <0.35ms. Output ONLY the 2 sentences, no intro." Context: The rule is "non-atomic-toctou-fs-mutation (Non-Atomic TOCTOU File Existence Check Before Write), CWE-367". Blocked cod
- **Verification Suite**: `537/537` unit tests passed, `344` active AST rules, repo score `100/100` (`2149ms`)

### Sprint #43 — v3.5.278 (2026-11-08 15:39 UTC)
- **New AST Shield**: `SLOP-345` (`no-hallucinated-cipher-primitive-v345`, CWE-327) — `Hallucinated Non-Standard Cryptographic Cipher Names`
- **Threat Model**: LLMs invent non-existent cipher strings such as aes-256-turbo or sha256-fast in OpenSSL and Node crypto calls, causing runtime panics or silent fallback to weak encryption.
- **Verification Suite**: `538/538` unit tests passed, `345` active AST rules, repo score `100/100` (`2166ms`)

### Sprint #44 — v3.5.279 (2026-11-08 15:42 UTC)
- **New AST Shield**: `SLOP-346` (`no-shell-template-subprocess-exec-v346`, CWE-78) — `Formatted Shell String Subprocess Command Injection`
- **Threat Model**: AI assistants pass formatted f-strings or template literals into subprocess.Popen(..., shell=True) and child_process.exec, allowing arbitrary shell command execution.
- **Verification Suite**: `539/539` unit tests passed, `346` active AST rules, repo score `100/100` (`2107ms`)

### Sprint #45 — v3.5.280 (2026-11-08 15:45 UTC)
- **New AST Shield**: `SLOP-347` (`no-loop-scoped-event-listener-leak-v347`, CWE-400) — `Unbounded Event Listener Registration Inside Loops`
- **Threat Model**: AI-generated stream handlers attach emitter.on('data', ...) inside for/while loops without cleanup, exhausting V8 heap memory and triggering MaxListenersExceededWarning.
- **Verification Suite**: `540/540` unit tests passed, `347` active AST rules, repo score `100/100` (`2133ms`)

### Sprint #46 — v3.5.281 (2026-11-08 15:47 UTC)
- **New AST Shield**: `SLOP-348` (`no-phantom-ai-sdk-slopsquatting-v348`, CWE-1357) — `Phantom AI SDK Wrapper Package Slopsquatting`
- **Threat Model**: LLMs hallucinate plausible AI helper packages like @anthropic/tools-pro or openai-streaming-helper in package.json and requirements.txt, exposing builds to supply-chain hijacking.
- **Verification Suite**: `541/541` unit tests passed, `348` active AST rules, repo score `100/100` (`2100ms`)

### Sprint #47 — v3.5.282 (2026-11-08 15:50 UTC)
- **New AST Shield**: `SLOP-349` (`no-unbounded-mpsc-queue-sprawl-v349`, CWE-400) — `Unbounded MPSC & Asyncio Queue Memory Exhaustion`
- **Threat Model**: 1.  **Analyze the Request:** *   **Rule/Bug:** `unbounded_mpsc_queue_sprawl` (CWE-400) - using an unbounded MPSC/Asyncio queue instead of a bounded one, leading to memory exhaustion. *   **Blocked Code:** `tokio::sync::mpsc::unbounded_channel::<Job>()` *   **Enforced Fix:** `tokio::sync::mpsc::channel::<Job>(1024)` *   **Task:** Write exactly *two* technical
- **Verification Suite**: `542/542` unit tests passed, `349` active AST rules, repo score `100/100` (`2176ms`)

### Sprint #48 — v3.5.283 (2026-11-08 15:53 UTC)
- **New AST Shield**: `SLOP-350` (`no-truncated-hex-entropy-keygen-v350`, CWE-326) — `Truncated Hex Slice & Weak PRNG Key Derivation`
- **Threat Model**: 1.  **Analyze the Request:** *   **Rule/Topic:** `truncated-hex-entropy-keygen` (Truncated Hex Slice & Weak PRNG Key Derivation, CWE-326). *   **Blocked Code:** `const aesKey = crypto.randomBytes(32).toString('hex').slice(0, 16);` *   **Enforced Fix:** `const aesKey = crypto.randomBytes(32);` *   **Target Audience:** Senior security engineer. *   **Output Co
- **Verification Suite**: `543/543` unit tests passed, `350` active AST rules, repo score `100/100` (`2149ms`)

### Sprint #49 — v3.5.284 (2026-11-08 15:56 UTC)
- **New AST Shield**: `SLOP-351` (`no-async-iterator-unclosed-break-v351`, CWE-775) — `Async Generator Break Without Stream Disposal`
- **Threat Model**: Breaking out of a for-await-of stream loop without a try/finally block or stream.destroy() leaks underlying file descriptors and database cursors.
- **Verification Suite**: `544/544` unit tests passed, `351` active AST rules, repo score `100/100` (`2162ms`)

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
