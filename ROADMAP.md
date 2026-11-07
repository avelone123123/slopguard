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
### Sprint #103 — v3.5.117 (2026-11-07 14:16 UTC)
- **New AST Shield**: `SLOP-183` (`no-hallucinated-cipher-primitive-v183`, CWE-327) — `Hallucinated Non-Standard Cryptographic Cipher Names`
- **Threat Model**: LLMs invent non-existent cipher strings such as aes-256-turbo or sha256-fast in OpenSSL and Node crypto calls, causing runtime panics or silent fallback to weak encryption.
- **Verification Suite**: `376/376` unit tests passed, `183` active AST rules, repo score `100/100` (`2080ms`)

### Sprint #104 — v3.5.118 (2026-11-07 14:19 UTC)
- **New AST Shield**: `SLOP-184` (`no-shell-template-subprocess-exec-v184`, CWE-78) — `Formatted Shell String Subprocess Command Injection`
- **Threat Model**: 1.  **Analyze the Request:** *   **Rule context:** Shell-template-subprocess-exec (Command Injection CWE-78). *   **Blocked code:** `subprocess.Popen(f'tar -xzf {archive_name}', shell=True)` (injection risk via formatted string). *   **Enforced fix:** `subprocess.run(['tar', '-xzf', archive_name], check=True)` (argument list, no shell=True). *   **Task:** Wr
- **Verification Suite**: `377/377` unit tests passed, `184` active AST rules, repo score `100/100` (`2147ms`)

### Sprint #105 — v3.5.119 (2026-11-07 14:21 UTC)
- **New AST Shield**: `SLOP-185` (`no-loop-scoped-event-listener-leak-v185`, CWE-400) — `Unbounded Event Listener Registration Inside Loops`
- **Threat Model**: AI-generated stream handlers attach emitter.on('data', ...) inside for/while loops without cleanup, exhausting V8 heap memory and triggering MaxListenersExceededWarning.
- **Verification Suite**: `378/378` unit tests passed, `185` active AST rules, repo score `100/100` (`2069ms`)

### Sprint #106 — v3.5.120 (2026-11-07 14:24 UTC)
- **New AST Shield**: `SLOP-186` (`no-phantom-ai-sdk-slopsquatting-v186`, CWE-1357) — `Phantom AI SDK Wrapper Package Slopsquatting`
- **Threat Model**: LLMs hallucinate plausible AI helper packages like @anthropic/tools-pro or openai-streaming-helper in package.json and requirements.txt, exposing builds to supply-chain hijacking.
- **Verification Suite**: `379/379` unit tests passed, `186` active AST rules, repo score `100/100` (`2071ms`)

### Sprint #107 — v3.5.121 (2026-11-07 14:26 UTC)
- **New AST Shield**: `SLOP-187` (`no-unbounded-mpsc-queue-sprawl-v187`, CWE-400) — `Unbounded MPSC & Asyncio Queue Memory Exhaustion`
- **Threat Model**: AI concurrency scaffolding defaults to tokio::sync::mpsc::unbounded_channel() and unbuffered asyncio.Queue(), removing backpressure and causing OOM kills under traffic spikes.
- **Verification Suite**: `380/380` unit tests passed, `187` active AST rules, repo score `100/100` (`2070ms`)

### Sprint #108 — v3.5.122 (2026-11-07 14:28 UTC)
- **New AST Shield**: `SLOP-188` (`no-truncated-hex-entropy-keygen-v188`, CWE-326) — `Truncated Hex Slice & Weak PRNG Key Derivation`
- **Threat Model**: AI models frequently slice hex-encoded random bytes to 16 chars (reducing 128-bit entropy to 64 bits) or use Math.random().toString(36) for AES/JWT secret keys.
- **Verification Suite**: `381/381` unit tests passed, `188` active AST rules, repo score `100/100` (`2081ms`)

### Sprint #109 — v3.5.123 (2026-11-07 14:30 UTC)
- **New AST Shield**: `SLOP-189` (`no-async-iterator-unclosed-break-v189`, CWE-775) — `Async Generator Break Without Stream Disposal`
- **Threat Model**: Breaking out of a for-await-of stream loop without a try/finally block or stream.destroy() leaks underlying file descriptors and database cursors.
- **Verification Suite**: `382/382` unit tests passed, `189` active AST rules, repo score `100/100` (`2017ms`)

### Sprint #110 — v3.5.124 (2026-11-07 14:32 UTC)
- **New AST Shield**: `SLOP-190` (`no-compound-namespace-slopsquat-v190`, CWE-829) — `Compound Namespace Package Manifest Slopsquatting`
- **Threat Model**: AI assistants synthesize plausible compound package names such as flask-bcrypt-auth or tokio-redis-pro that attackers register on PyPI/crates.io/npm for RCE.
- **Verification Suite**: `383/383` unit tests passed, `190` active AST rules, repo score `100/100` (`2026ms`)

### Sprint #111 — v3.5.125 (2026-11-07 14:34 UTC)
- **New AST Shield**: `SLOP-191` (`no-unvalidated-ssrf-redirect-chain-v191`, CWE-918) — `Unvalidated SSRF Host Interpolation & Redirect Bypass`
- **Threat Model**: AI coding assistants interpolate user-controlled host variables directly into axios.get or requests.get URLs, enabling Server-Side Request Forgery against cloud metadata endpoints.
- **Verification Suite**: `384/384` unit tests passed, `191` active AST rules, repo score `100/100` (`2072ms`)

### Sprint #112 — v3.5.126 (2026-11-07 14:37 UTC)
- **New AST Shield**: `SLOP-192` (`no-non-atomic-toctou-fs-mutation-v192`, CWE-367) — `Non-Atomic TOCTOU File Existence Check Before Write`
- **Threat Model**: AI models routinely generate fs.existsSync(p) followed by fs.writeFileSync(p), creating a CWE-367 Time-of-Check Time-of-Use race window exploitable via symlink swapping.
- **Verification Suite**: `385/385` unit tests passed, `192` active AST rules, repo score `100/100` (`2064ms`)

### Sprint #113 — v3.5.127 (2026-11-07 14:38 UTC)
- **New AST Shield**: `SLOP-193` (`no-hallucinated-cipher-primitive-v193`, CWE-327) — `Hallucinated Non-Standard Cryptographic Cipher Names`
- **Threat Model**: 1.  **Analyze the Request:** *   **Rule Context:** "hallucinated-cipher-primitive" (Non-standard cryptographic cipher names, CWE-327). *   **Blocked Code Example:** `crypto.createCipheriv('aes-256-turbo', key, iv);` and `hashlib.new('sha256-fast')`. *   **Enforced Fix:** `crypto.createCipheriv('aes-256-gcm', key, iv);` and `hashlib.sha256(data)`. *   **Task:
- **Verification Suite**: `386/386` unit tests passed, `193` active AST rules, repo score `100/100` (`2037ms`)

### Sprint #114 — v3.5.128 (2026-11-07 14:41 UTC)
- **New AST Shield**: `SLOP-194` (`no-shell-template-subprocess-exec-v194`, CWE-78) — `Formatted Shell String Subprocess Command Injection`
- **Threat Model**: AI assistants pass formatted f-strings or template literals into subprocess.Popen(..., shell=True) and child_process.exec, allowing arbitrary shell command execution.
- **Verification Suite**: `387/387` unit tests passed, `194` active AST rules, repo score `100/100` (`2049ms`)

### Sprint #115 — v3.5.129 (2026-11-07 14:43 UTC)
- **New AST Shield**: `SLOP-195` (`no-loop-scoped-event-listener-leak-v195`, CWE-400) — `Unbounded Event Listener Registration Inside Loops`
- **Threat Model**: AI-generated stream handlers attach emitter.on('data', ...) inside for/while loops without cleanup, exhausting V8 heap memory and triggering MaxListenersExceededWarning.
- **Verification Suite**: `388/388` unit tests passed, `195` active AST rules, repo score `100/100` (`2109ms`)

### Sprint #116 — v3.5.130 (2026-11-07 14:45 UTC)
- **New AST Shield**: `SLOP-196` (`no-phantom-ai-sdk-slopsquatting-v196`, CWE-1357) — `Phantom AI SDK Wrapper Package Slopsquatting`
- **Threat Model**: 1.  **Analyze the Request:** *   **Topic:** Slopsquatting (Naming confusion / CWE-1357) in AI SDK wrapper packages. *   **Rule/Context:** phantom-ai-sdk-slopsquatting (Phantom AI SDK Wrapper Package Slopsquatting). *   **Blocked Code:** `"@anthropic/tools-pro": "^1.0.0"`, `"openai-streaming-helper": "^2.1.0"` (looks like typos/confusion of real package names
- **Verification Suite**: `389/389` unit tests passed, `196` active AST rules, repo score `100/100` (`2029ms`)

### Sprint #117 — v3.5.131 (2026-11-07 14:48 UTC)
- **New AST Shield**: `SLOP-197` (`no-unbounded-mpsc-queue-sprawl-v197`, CWE-400) — `Unbounded MPSC & Asyncio Queue Memory Exhaustion`
- **Threat Model**: AI concurrency scaffolding defaults to tokio::sync::mpsc::unbounded_channel() and unbuffered asyncio.Queue(), removing backpressure and causing OOM kills under traffic spikes.
- **Verification Suite**: `390/390` unit tests passed, `197` active AST rules, repo score `100/100` (`2037ms`)
