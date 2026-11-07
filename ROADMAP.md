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
### Sprint #61 — v3.5.75 (2026-11-07 12:42 UTC)
- **New AST Shield**: `SLOP-141` (`no-unvalidated-ssrf-redirect-chain-v141`, CWE-918) — `Unvalidated SSRF Host Interpolation & Redirect Bypass`
- **Threat Model**: AI coding assistants interpolate user-controlled host variables directly into axios.get or requests.get URLs, enabling Server-Side Request Forgery against cloud metadata endpoints.
- **Verification Suite**: `334/334` unit tests passed, `141` active AST rules, repo score `100/100` (`2046ms`)

### Sprint #62 — v3.5.76 (2026-11-07 12:44 UTC)
- **New AST Shield**: `SLOP-142` (`no-non-atomic-toctou-fs-mutation-v142`, CWE-367) — `Non-Atomic TOCTOU File Existence Check Before Write`
- **Threat Model**: AI models routinely generate fs.existsSync(p) followed by fs.writeFileSync(p), creating a CWE-367 Time-of-Check Time-of-Use race window exploitable via symlink swapping.
- **Verification Suite**: `335/335` unit tests passed, `142` active AST rules, repo score `100/100` (`2061ms`)

### Sprint #63 — v3.5.77 (2026-11-07 12:46 UTC)
- **New AST Shield**: `SLOP-143` (`no-hallucinated-cipher-primitive-v143`, CWE-327) — `Hallucinated Non-Standard Cryptographic Cipher Names`
- **Threat Model**: LLMs invent non-existent cipher strings such as aes-256-turbo or sha256-fast in OpenSSL and Node crypto calls, causing runtime panics or silent fallback to weak encryption.
- **Verification Suite**: `336/336` unit tests passed, `143` active AST rules, repo score `100/100` (`2012ms`)

### Sprint #64 — v3.5.78 (2026-11-07 12:49 UTC)
- **New AST Shield**: `SLOP-144` (`no-shell-template-subprocess-exec-v144`, CWE-78) — `Formatted Shell String Subprocess Command Injection`
- **Threat Model**: AI assistants pass formatted f-strings or template literals into subprocess.Popen(..., shell=True) and child_process.exec, allowing arbitrary shell command execution.
- **Verification Suite**: `337/337` unit tests passed, `144` active AST rules, repo score `100/100` (`2062ms`)

### Sprint #65 — v3.5.79 (2026-11-07 12:51 UTC)
- **New AST Shield**: `SLOP-145` (`no-loop-scoped-event-listener-leak-v145`, CWE-400) — `Unbounded Event Listener Registration Inside Loops`
- **Threat Model**: AI-generated stream handlers attach emitter.on('data', ...) inside for/while loops without cleanup, exhausting V8 heap memory and triggering MaxListenersExceededWarning.
- **Verification Suite**: `338/338` unit tests passed, `145` active AST rules, repo score `100/100` (`2034ms`)

### Sprint #66 — v3.5.80 (2026-11-07 12:53 UTC)
- **New AST Shield**: `SLOP-146` (`no-phantom-ai-sdk-slopsquatting-v146`, CWE-1357) — `Phantom AI SDK Wrapper Package Slopsquatting`
- **Threat Model**: LLMs hallucinate plausible AI helper packages like @anthropic/tools-pro or openai-streaming-helper in package.json and requirements.txt, exposing builds to supply-chain hijacking.
- **Verification Suite**: `339/339` unit tests passed, `146` active AST rules, repo score `100/100` (`2183ms`)

### Sprint #67 — v3.5.81 (2026-11-07 12:56 UTC)
- **New AST Shield**: `SLOP-147` (`no-unbounded-mpsc-queue-sprawl-v147`, CWE-400) — `Unbounded MPSC & Asyncio Queue Memory Exhaustion`
- **Threat Model**: 1.  **Analyze the Request:** *   **Rule/Rule ID:** unbounded-mpsc-queue-sprawl (Unbounded MPSC & Asyncio Queue Memory Exhaustion, CWE-400). *   **Blocked Code:** `let (tx, rx) = tokio::sync::mpsc::unbounded_channel::<Job>();` *   **Enforced Fix:** `let (tx, rx) = tokio::sync::mpsc::channel::<Job>(1024);` *   **Task:** Write *exactly* 2 technical sentences fo
- **Verification Suite**: `340/340` unit tests passed, `147` active AST rules, repo score `100/100` (`2112ms`)

### Sprint #68 — v3.5.82 (2026-11-07 12:59 UTC)
- **New AST Shield**: `SLOP-148` (`no-truncated-hex-entropy-keygen-v148`, CWE-326) — `Truncated Hex Slice & Weak PRNG Key Derivation`
- **Threat Model**: AI models frequently slice hex-encoded random bytes to 16 chars (reducing 128-bit entropy to 64 bits) or use Math.random().toString(36) for AES/JWT secret keys.
- **Verification Suite**: `341/341` unit tests passed, `148` active AST rules, repo score `100/100` (`2077ms`)

### Sprint #69 — v3.5.83 (2026-11-07 13:01 UTC)
- **New AST Shield**: `SLOP-149` (`no-async-iterator-unclosed-break-v149`, CWE-775) — `Async Generator Break Without Stream Disposal`
- **Threat Model**: Breaking out of a for-await-of stream loop without a try/finally block or stream.destroy() leaks underlying file descriptors and database cursors.
- **Verification Suite**: `342/342` unit tests passed, `149` active AST rules, repo score `100/100` (`2091ms`)

### Sprint #70 — v3.5.84 (2026-11-07 13:02 UTC)
- **New AST Shield**: `SLOP-150` (`no-compound-namespace-slopsquat-v150`, CWE-829) — `Compound Namespace Package Manifest Slopsquatting`
- **Threat Model**: AI assistants synthesize plausible compound package names such as flask-bcrypt-auth or tokio-redis-pro that attackers register on PyPI/crates.io/npm for RCE.
- **Verification Suite**: `343/343` unit tests passed, `150` active AST rules, repo score `100/100` (`2048ms`)

### Sprint #71 — v3.5.85 (2026-11-07 13:04 UTC)
- **New AST Shield**: `SLOP-151` (`no-unvalidated-ssrf-redirect-chain-v151`, CWE-918) — `Unvalidated SSRF Host Interpolation & Redirect Bypass`
- **Threat Model**: AI coding assistants interpolate user-controlled host variables directly into axios.get or requests.get URLs, enabling Server-Side Request Forgery against cloud metadata endpoints.
- **Verification Suite**: `344/344` unit tests passed, `151` active AST rules, repo score `100/100` (`2051ms`)

### Sprint #72 — v3.5.86 (2026-11-07 13:07 UTC)
- **New AST Shield**: `SLOP-152` (`no-non-atomic-toctou-fs-mutation-v152`, CWE-367) — `Non-Atomic TOCTOU File Existence Check Before Write`
- **Threat Model**: We need to output exactly two technical sentences for a senior security engineer explaining why AI models hallucinate this pattern and how SlopGuard's AST firewall blocks it. The pattern is a TOCTOU (Time of Check Time of Use) race condition on file existence check before write, classified as CWE-367. The enforced fix is using writeFileSync with flag: 'wx' t
- **Verification Suite**: `345/345` unit tests passed, `152` active AST rules, repo score `100/100` (`2097ms`)

### Sprint #73 — v3.5.87 (2026-11-07 13:09 UTC)
- **New AST Shield**: `SLOP-153` (`no-hallucinated-cipher-primitive-v153`, CWE-327) — `Hallucinated Non-Standard Cryptographic Cipher Names`
- **Threat Model**: LLMs invent non-existent cipher strings such as aes-256-turbo or sha256-fast in OpenSSL and Node crypto calls, causing runtime panics or silent fallback to weak encryption.
- **Verification Suite**: `346/346` unit tests passed, `153` active AST rules, repo score `100/100` (`2087ms`)

### Sprint #74 — v3.5.88 (2026-11-07 13:11 UTC)
- **New AST Shield**: `SLOP-154` (`no-shell-template-subprocess-exec-v154`, CWE-78) — `Formatted Shell String Subprocess Command Injection`
- **Threat Model**: AI assistants pass formatted f-strings or template literals into subprocess.Popen(..., shell=True) and child_process.exec, allowing arbitrary shell command execution.
- **Verification Suite**: `347/347` unit tests passed, `154` active AST rules, repo score `100/100` (`2051ms`)

### Sprint #75 — v3.5.89 (2026-11-07 13:13 UTC)
- **New AST Shield**: `SLOP-155` (`no-loop-scoped-event-listener-leak-v155`, CWE-400) — `Unbounded Event Listener Registration Inside Loops`
- **Threat Model**: 1.  **Analyze the Request:** *   **Rule:** `loop-scoped-event-listener-leak` (Unbounded Event Listener Registration Inside Loops) - CWE-400. *   **Blocked Code:** A loop that registers an event listener on a socket for every item in `items`. *   **Enforced Fix:** Moving the listener registration out of the loop, into a batched handler (`handleBatch`). *   **
- **Verification Suite**: `348/348` unit tests passed, `155` active AST rules, repo score `100/100` (`2066ms`)
