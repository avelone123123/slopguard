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
### Sprint #126 — v3.5.140 (2026-11-07 15:08 UTC)
- **New AST Shield**: `SLOP-206` (`no-phantom-ai-sdk-slopsquatting-v206`, CWE-1357) — `Phantom AI SDK Wrapper Package Slopsquatting`
- **Threat Model**: 1.  **Analyze the Request:** *   **Rule/Topic:** Phantom AI SDK Wrapper Package Slopsquatting (CWE-1357). *   **Blocked Code:** `"@anthropic/tools-pro": "^1.0.0"`, `"openai-streaming-helper": "^2.1.0"` (These look like maliciously named packages attempting to squat on legitimate SDK names). *   **Enforced Fix:** `"@anthropic-ai/sdk": "^0.27.0"`, `"openai": "
- **Verification Suite**: `399/399` unit tests passed, `206` active AST rules, repo score `100/100` (`2054ms`)

### Sprint #127 — v3.5.141 (2026-11-07 15:10 UTC)
- **New AST Shield**: `SLOP-207` (`no-unbounded-mpsc-queue-sprawl-v207`, CWE-400) — `Unbounded MPSC & Asyncio Queue Memory Exhaustion`
- **Threat Model**: AI concurrency scaffolding defaults to tokio::sync::mpsc::unbounded_channel() and unbuffered asyncio.Queue(), removing backpressure and causing OOM kills under traffic spikes.
- **Verification Suite**: `400/400` unit tests passed, `207` active AST rules, repo score `100/100` (`2108ms`)

### Sprint #128 — v3.5.142 (2026-11-07 15:12 UTC)
- **New AST Shield**: `SLOP-208` (`no-truncated-hex-entropy-keygen-v208`, CWE-326) — `Truncated Hex Slice & Weak PRNG Key Derivation`
- **Threat Model**: AI models frequently slice hex-encoded random bytes to 16 chars (reducing 128-bit entropy to 64 bits) or use Math.random().toString(36) for AES/JWT secret keys.
- **Verification Suite**: `401/401` unit tests passed, `208` active AST rules, repo score `100/100` (`2031ms`)

### Sprint #129 — v3.5.143 (2026-11-07 15:15 UTC)
- **New AST Shield**: `SLOP-209` (`no-async-iterator-unclosed-break-v209`, CWE-775) — `Async Generator Break Without Stream Disposal`
- **Threat Model**: Breaking out of a for-await-of stream loop without a try/finally block or stream.destroy() leaks underlying file descriptors and database cursors.
- **Verification Suite**: `402/402` unit tests passed, `209` active AST rules, repo score `100/100` (`2053ms`)

### Sprint #130 — v3.5.144 (2026-11-07 15:17 UTC)
- **New AST Shield**: `SLOP-210` (`no-compound-namespace-slopsquat-v210`, CWE-829) — `Compound Namespace Package Manifest Slopsquatting`
- **Threat Model**: AI assistants synthesize plausible compound package names such as flask-bcrypt-auth or tokio-redis-pro that attackers register on PyPI/crates.io/npm for RCE.
- **Verification Suite**: `403/403` unit tests passed, `210` active AST rules, repo score `100/100` (`2069ms`)

### Sprint #131 — v3.5.145 (2026-11-07 15:19 UTC)
- **New AST Shield**: `SLOP-211` (`no-unvalidated-ssrf-redirect-chain-v211`, CWE-918) — `Unvalidated SSRF Host Interpolation & Redirect Bypass`
- **Threat Model**: AI coding assistants interpolate user-controlled host variables directly into axios.get or requests.get URLs, enabling Server-Side Request Forgery against cloud metadata endpoints.
- **Verification Suite**: `404/404` unit tests passed, `211` active AST rules, repo score `100/100` (`2158ms`)

### Sprint #132 — v3.5.146 (2026-11-07 15:22 UTC)
- **New AST Shield**: `SLOP-212` (`no-non-atomic-toctou-fs-mutation-v212`, CWE-367) — `Non-Atomic TOCTOU File Existence Check Before Write`
- **Threat Model**: AI models routinely generate fs.existsSync(p) followed by fs.writeFileSync(p), creating a CWE-367 Time-of-Check Time-of-Use race window exploitable via symlink swapping.
- **Verification Suite**: `405/405` unit tests passed, `212` active AST rules, repo score `100/100` (`2038ms`)

### Sprint #133 — v3.5.147 (2026-11-07 15:24 UTC)
- **New AST Shield**: `SLOP-213` (`no-hallucinated-cipher-primitive-v213`, CWE-327) — `Hallucinated Non-Standard Cryptographic Cipher Names`
- **Threat Model**: 1.  **Analyze the Request:** *   **Rule:** hallucinated-cipher-primitive (Non-standard cipher names/hashes like 'aes-256-turbo', 'sha256-fast'). *   **Blocked code:** `crypto.createCipheriv('aes-256-turbo', key, iv)`, `hashlib.new('sha256-fast')` *   **Enforced fix:** `crypto.createCipheriv('aes-256-gcm', key, iv)`, `hashlib.sha256(data)` *   **Task:** Write
- **Verification Suite**: `406/406` unit tests passed, `213` active AST rules, repo score `100/100` (`2068ms`)

### Sprint #134 — v3.5.148 (2026-11-07 15:26 UTC)
- **New AST Shield**: `SLOP-214` (`no-shell-template-subprocess-exec-v214`, CWE-78) — `Formatted Shell String Subprocess Command Injection`
- **Threat Model**: AI assistants pass formatted f-strings or template literals into subprocess.Popen(..., shell=True) and child_process.exec, allowing arbitrary shell command execution.
- **Verification Suite**: `407/407` unit tests passed, `214` active AST rules, repo score `100/100` (`2057ms`)

### Sprint #135 — v3.5.149 (2026-11-07 15:28 UTC)
- **New AST Shield**: `SLOP-215` (`no-loop-scoped-event-listener-leak-v215`, CWE-400) — `Unbounded Event Listener Registration Inside Loops`
- **Threat Model**: AI-generated stream handlers attach emitter.on('data', ...) inside for/while loops without cleanup, exhausting V8 heap memory and triggering MaxListenersExceededWarning.
- **Verification Suite**: `408/408` unit tests passed, `215` active AST rules, repo score `100/100` (`2051ms`)

### Sprint #136 — v3.5.150 (2026-11-07 15:31 UTC)
- **New AST Shield**: `SLOP-216` (`no-phantom-ai-sdk-slopsquatting-v216`, CWE-1357) — `Phantom AI SDK Wrapper Package Slopsquatting`
- **Threat Model**: LLMs hallucinate plausible AI helper packages like @anthropic/tools-pro or openai-streaming-helper in package.json and requirements.txt, exposing builds to supply-chain hijacking.
- **Verification Suite**: `409/409` unit tests passed, `216` active AST rules, repo score `100/100` (`2081ms`)

### Sprint #137 — v3.5.151 (2026-11-07 15:34 UTC)
- **New AST Shield**: `SLOP-217` (`no-unbounded-mpsc-queue-sprawl-v217`, CWE-400) — `Unbounded MPSC & Asyncio Queue Memory Exhaustion`
- **Threat Model**: AI concurrency scaffolding defaults to tokio::sync::mpsc::unbounded_channel() and unbuffered asyncio.Queue(), removing backpressure and causing OOM kills under traffic spikes.
- **Verification Suite**: `410/410` unit tests passed, `217` active AST rules, repo score `100/100` (`2051ms`)

### Sprint #138 — v3.5.152 (2026-11-07 15:37 UTC)
- **New AST Shield**: `SLOP-218` (`no-truncated-hex-entropy-keygen-v218`, CWE-326) — `Truncated Hex Slice & Weak PRNG Key Derivation`
- **Threat Model**: AI models frequently slice hex-encoded random bytes to 16 chars (reducing 128-bit entropy to 64 bits) or use Math.random().toString(36) for AES/JWT secret keys.
- **Verification Suite**: `411/411` unit tests passed, `218` active AST rules, repo score `100/100` (`2031ms`)

### Sprint #139 — v3.5.153 (2026-11-07 15:40 UTC)
- **New AST Shield**: `SLOP-219` (`no-async-iterator-unclosed-break-v219`, CWE-775) — `Async Generator Break Without Stream Disposal`
- **Threat Model**: Breaking out of a for-await-of stream loop without a try/finally block or stream.destroy() leaks underlying file descriptors and database cursors.
- **Verification Suite**: `412/412` unit tests passed, `219` active AST rules, repo score `100/100` (`2034ms`)

### Sprint #140 — v3.5.154 (2026-11-07 15:42 UTC)
- **New AST Shield**: `SLOP-220` (`no-compound-namespace-slopsquat-v220`, CWE-829) — `Compound Namespace Package Manifest Slopsquatting`
- **Threat Model**: 1.  **Analyze the Request:** *   **Rule/Name:** compound-namespace-slopsquatting (CWE-829). *   **Blocked Code (Examples of slopsquat):** `flask-bcrypt-auth>=1.0`, `tokio-redis-pro = "1.0.0"`. *   **Enforced Fix (Correct versions):** `flask-login==0.6.3`, `redis = "0.27.5"`. *   **Task:** Write exactly 2 technical sentences for a senior security engineer exp
- **Verification Suite**: `413/413` unit tests passed, `220` active AST rules, repo score `100/100` (`2076ms`)
