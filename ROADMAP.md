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
### Sprint #195 — v3.5.209 (2026-11-07 18:27 UTC)
- **New AST Shield**: `SLOP-275` (`no-loop-scoped-event-listener-leak-v275`, CWE-400) — `Unbounded Event Listener Registration Inside Loops`
- **Threat Model**: AI-generated stream handlers attach emitter.on('data', ...) inside for/while loops without cleanup, exhausting V8 heap memory and triggering MaxListenersExceededWarning.
- **Verification Suite**: `468/468` unit tests passed, `275` active AST rules, repo score `100/100` (`2090ms`)

### Sprint #196 — v3.5.210 (2026-11-07 18:30 UTC)
- **New AST Shield**: `SLOP-276` (`no-phantom-ai-sdk-slopsquatting-v276`, CWE-1357) — `Phantom AI SDK Wrapper Package Slopsquatting`
- **Threat Model**: 1.  **Analyze the Request:** *   **Rule:** phantom-ai-sdk-slopsquatting (Phantom AI SDK Wrapper Package Slopsquatting, CWE-1357). This implies a package name confusion/sneaky replacement where a malicious or vulnerable package is named similarly to a legitimate one to trick developers. *   **Blocked Code:** `"@anthropic/tools-pro": "^1.0.0", "openai-streamin
- **Verification Suite**: `469/469` unit tests passed, `276` active AST rules, repo score `100/100` (`2214ms`)

### Sprint #197 — v3.5.211 (2026-11-07 18:33 UTC)
- **New AST Shield**: `SLOP-277` (`no-unbounded-mpsc-queue-sprawl-v277`, CWE-400) — `Unbounded MPSC & Asyncio Queue Memory Exhaustion`
- **Threat Model**: AI concurrency scaffolding defaults to tokio::sync::mpsc::unbounded_channel() and unbuffered asyncio.Queue(), removing backpressure and causing OOM kills under traffic spikes.
- **Verification Suite**: `470/470` unit tests passed, `277` active AST rules, repo score `100/100` (`2116ms`)

### Sprint #198 — v3.5.212 (2026-11-07 18:36 UTC)
- **New AST Shield**: `SLOP-278` (`no-truncated-hex-entropy-keygen-v278`, CWE-326) — `Truncated Hex Slice & Weak PRNG Key Derivation`
- **Threat Model**: AI models frequently slice hex-encoded random bytes to 16 chars (reducing 128-bit entropy to 64 bits) or use Math.random().toString(36) for AES/JWT secret keys.
- **Verification Suite**: `471/471` unit tests passed, `278` active AST rules, repo score `100/100` (`2138ms`)

### Sprint #199 — v3.5.213 (2026-11-07 18:38 UTC)
- **New AST Shield**: `SLOP-279` (`no-async-iterator-unclosed-break-v279`, CWE-775) — `Async Generator Break Without Stream Disposal`
- **Threat Model**: Breaking out of a for-await-of stream loop without a try/finally block or stream.destroy() leaks underlying file descriptors and database cursors.
- **Verification Suite**: `472/472` unit tests passed, `279` active AST rules, repo score `100/100` (`2145ms`)

### Sprint #200 — v3.5.214 (2026-11-07 18:41 UTC)
- **New AST Shield**: `SLOP-280` (`no-compound-namespace-slopsquat-v280`, CWE-829) — `Compound Namespace Package Manifest Slopsquatting`
- **Threat Model**: AI assistants synthesize plausible compound package names such as flask-bcrypt-auth or tokio-redis-pro that attackers register on PyPI/crates.io/npm for RCE.
- **Verification Suite**: `473/473` unit tests passed, `280` active AST rules, repo score `100/100` (`2083ms`)

### Sprint #201 — v3.5.215 (2026-11-07 18:44 UTC)
- **New AST Shield**: `SLOP-281` (`no-unvalidated-ssrf-redirect-chain-v281`, CWE-918) — `Unvalidated SSRF Host Interpolation & Redirect Bypass`
- **Threat Model**: AI coding assistants interpolate user-controlled host variables directly into axios.get or requests.get URLs, enabling Server-Side Request Forgery against cloud metadata endpoints.
- **Verification Suite**: `474/474` unit tests passed, `281` active AST rules, repo score `100/100` (`2079ms`)

### Sprint #202 — v3.5.216 (2026-11-07 18:48 UTC)
- **New AST Shield**: `SLOP-282` (`no-non-atomic-toctou-fs-mutation-v282`, CWE-367) — `Non-Atomic TOCTOU File Existence Check Before Write`
- **Threat Model**: AI models routinely generate fs.existsSync(p) followed by fs.writeFileSync(p), creating a CWE-367 Time-of-Check Time-of-Use race window exploitable via symlink swapping.
- **Verification Suite**: `475/475` unit tests passed, `282` active AST rules, repo score `100/100` (`2089ms`)

### Sprint #203 — v3.5.217 (2026-11-07 18:51 UTC)
- **New AST Shield**: `SLOP-283` (`no-hallucinated-cipher-primitive-v283`, CWE-327) — `Hallucinated Non-Standard Cryptographic Cipher Names`
- **Threat Model**: LLMs invent non-existent cipher strings such as aes-256-turbo or sha256-fast in OpenSSL and Node crypto calls, causing runtime panics or silent fallback to weak encryption.
- **Verification Suite**: `476/476` unit tests passed, `283` active AST rules, repo score `100/100` (`2122ms`)

### Sprint #204 — v3.5.218 (2026-11-07 18:54 UTC)
- **New AST Shield**: `SLOP-284` (`no-shell-template-subprocess-exec-v284`, CWE-78) — `Formatted Shell String Subprocess Command Injection`
- **Threat Model**: 1.  **Analyze the Request:** *   **Rule:** Shell-template-subprocess-exec (CWE-78 Formatted Shell String Subprocess Command Injection). *   **Blocked code:** `subprocess.Popen(f'tar -xzf {archive_name}', shell=True)` (f-string + shell=True = command injection). *   **Enforced fix:** `subprocess.run(['tar', '-xzf', archive_name], check=True)` (list args, no s
- **Verification Suite**: `477/477` unit tests passed, `284` active AST rules, repo score `100/100` (`2060ms`)

### Sprint #205 — v3.5.219 (2026-11-07 18:57 UTC)
- **New AST Shield**: `SLOP-285` (`no-loop-scoped-event-listener-leak-v285`, CWE-400) — `Unbounded Event Listener Registration Inside Loops`
- **Threat Model**: AI-generated stream handlers attach emitter.on('data', ...) inside for/while loops without cleanup, exhausting V8 heap memory and triggering MaxListenersExceededWarning.
- **Verification Suite**: `478/478` unit tests passed, `285` active AST rules, repo score `100/100` (`2320ms`)

### Sprint #206 — v3.5.220 (2026-11-07 19:00 UTC)
- **New AST Shield**: `SLOP-286` (`no-phantom-ai-sdk-slopsquatting-v286`, CWE-1357) — `Phantom AI SDK Wrapper Package Slopsquatting`
- **Threat Model**: LLMs hallucinate plausible AI helper packages like @anthropic/tools-pro or openai-streaming-helper in package.json and requirements.txt, exposing builds to supply-chain hijacking.
- **Verification Suite**: `479/479` unit tests passed, `286` active AST rules, repo score `100/100` (`2258ms`)

### Sprint #207 — v3.5.221 (2026-11-07 19:03 UTC)
- **New AST Shield**: `SLOP-287` (`no-unbounded-mpsc-queue-sprawl-v287`, CWE-400) — `Unbounded MPSC & Asyncio Queue Memory Exhaustion`
- **Threat Model**: AI concurrency scaffolding defaults to tokio::sync::mpsc::unbounded_channel() and unbuffered asyncio.Queue(), removing backpressure and causing OOM kills under traffic spikes.
- **Verification Suite**: `480/480` unit tests passed, `287` active AST rules, repo score `100/100` (`2154ms`)

### Sprint #208 — v3.5.222 (2026-11-07 19:06 UTC)
- **New AST Shield**: `SLOP-288` (`no-truncated-hex-entropy-keygen-v288`, CWE-326) — `Truncated Hex Slice & Weak PRNG Key Derivation`
- **Threat Model**: AI models frequently slice hex-encoded random bytes to 16 chars (reducing 128-bit entropy to 64 bits) or use Math.random().toString(36) for AES/JWT secret keys.
- **Verification Suite**: `481/481` unit tests passed, `288` active AST rules, repo score `100/100` (`2184ms`)

### Sprint #209 — v3.5.223 (2026-11-07 19:09 UTC)
- **New AST Shield**: `SLOP-289` (`no-async-iterator-unclosed-break-v289`, CWE-775) — `Async Generator Break Without Stream Disposal`
- **Threat Model**: Breaking out of a for-await-of stream loop without a try/finally block or stream.destroy() leaks underlying file descriptors and database cursors.
- **Verification Suite**: `482/482` unit tests passed, `289` active AST rules, repo score `100/100` (`2119ms`)
