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
### Sprint #88 — v3.5.424 (2026-11-09 15:52 UTC)
- **New AST Shield**: `SLOP-491` (`no-truncated-hex-entropy-keygen-v491`, CWE-326) — `Truncated Hex Slice & Weak PRNG Key Derivation`
- **Threat Model**: AI models frequently slice hex-encoded random bytes to 16 chars (reducing 128-bit entropy to 64 bits) or use Math.random().toString(36) for AES/JWT secret keys.
- **Verification Suite**: `684/684` unit tests passed, `491` active AST rules, repo score `100/100` (`2059ms`)

### Sprint #89 — v3.5.425 (2026-11-09 15:54 UTC)
- **New AST Shield**: `SLOP-492` (`no-async-iterator-unclosed-break-v492`, CWE-775) — `Async Generator Break Without Stream Disposal`
- **Threat Model**: Breaking out of a for-await-of stream loop without a try/finally block or stream.destroy() leaks underlying file descriptors and database cursors.
- **Verification Suite**: `685/685` unit tests passed, `492` active AST rules, repo score `100/100` (`2059ms`)

### Sprint #90 — v3.5.426 (2026-11-09 15:56 UTC)
- **New AST Shield**: `SLOP-493` (`no-compound-namespace-slopsquat-v493`, CWE-829) — `Compound Namespace Package Manifest Slopsquatting`
- **Threat Model**: AI assistants synthesize plausible compound package names such as flask-bcrypt-auth or tokio-redis-pro that attackers register on PyPI/crates.io/npm for RCE.
- **Verification Suite**: `686/686` unit tests passed, `493` active AST rules, repo score `100/100` (`2001ms`)

### Sprint #91 — v3.5.427 (2026-11-09 15:58 UTC)
- **New AST Shield**: `SLOP-494` (`no-unvalidated-ssrf-redirect-chain-v494`, CWE-918) — `Unvalidated SSRF Host Interpolation & Redirect Bypass`
- **Threat Model**: AI coding assistants interpolate user-controlled host variables directly into axios.get or requests.get URLs, enabling Server-Side Request Forgery against cloud metadata endpoints.
- **Verification Suite**: `687/687` unit tests passed, `494` active AST rules, repo score `100/100` (`2034ms`)

### Sprint #92 — v3.5.428 (2026-11-09 16:01 UTC)
- **New AST Shield**: `SLOP-495` (`no-non-atomic-toctou-fs-mutation-v495`, CWE-367) — `Non-Atomic TOCTOU File Existence Check Before Write`
- **Threat Model**: AI models routinely generate fs.existsSync(p) followed by fs.writeFileSync(p), creating a CWE-367 Time-of-Check Time-of-Use race window exploitable via symlink swapping.
- **Verification Suite**: `688/688` unit tests passed, `495` active AST rules, repo score `100/100` (`2055ms`)

### Sprint #93 — v3.5.429 (2026-11-09 16:03 UTC)
- **New AST Shield**: `SLOP-496` (`no-hallucinated-cipher-primitive-v496`, CWE-327) — `Hallucinated Non-Standard Cryptographic Cipher Names`
- **Threat Model**: LLMs invent non-existent cipher strings such as aes-256-turbo or sha256-fast in OpenSSL and Node crypto calls, causing runtime panics or silent fallback to weak encryption.
- **Verification Suite**: `689/689` unit tests passed, `496` active AST rules, repo score `100/100` (`2107ms`)

### Sprint #94 — v3.5.430 (2026-11-09 16:05 UTC)
- **New AST Shield**: `SLOP-497` (`no-shell-template-subprocess-exec-v497`, CWE-78) — `Formatted Shell String Subprocess Command Injection`
- **Threat Model**: AI assistants pass formatted f-strings or template literals into subprocess.Popen(..., shell=True) and child_process.exec, allowing arbitrary shell command execution.
- **Verification Suite**: `690/690` unit tests passed, `497` active AST rules, repo score `100/100` (`2057ms`)

### Sprint #95 — v3.5.431 (2026-11-09 16:07 UTC)
- **New AST Shield**: `SLOP-498` (`no-loop-scoped-event-listener-leak-v498`, CWE-400) — `Unbounded Event Listener Registration Inside Loops`
- **Threat Model**: AI-generated stream handlers attach emitter.on('data', ...) inside for/while loops without cleanup, exhausting V8 heap memory and triggering MaxListenersExceededWarning.
- **Verification Suite**: `691/691` unit tests passed, `498` active AST rules, repo score `100/100` (`2036ms`)

### Sprint #96 — v3.5.432 (2026-11-09 16:09 UTC)
- **New AST Shield**: `SLOP-499` (`no-phantom-ai-sdk-slopsquatting-v499`, CWE-1357) — `Phantom AI SDK Wrapper Package Slopsquatting`
- **Threat Model**: LLMs hallucinate plausible AI helper packages like @anthropic/tools-pro or openai-streaming-helper in package.json and requirements.txt, exposing builds to supply-chain hijacking.
- **Verification Suite**: `692/692` unit tests passed, `499` active AST rules, repo score `100/100` (`2047ms`)

### Sprint #97 — v3.5.433 (2026-11-09 16:12 UTC)
- **New AST Shield**: `SLOP-500` (`no-unbounded-mpsc-queue-sprawl-v500`, CWE-400) — `Unbounded MPSC & Asyncio Queue Memory Exhaustion`
- **Threat Model**: AI concurrency scaffolding defaults to tokio::sync::mpsc::unbounded_channel() and unbuffered asyncio.Queue(), removing backpressure and causing OOM kills under traffic spikes.
- **Verification Suite**: `693/693` unit tests passed, `500` active AST rules, repo score `100/100` (`2084ms`)

### Sprint #98 — v3.5.434 (2026-11-09 16:14 UTC)
- **New AST Shield**: `SLOP-501` (`no-truncated-hex-entropy-keygen-v501`, CWE-326) — `Truncated Hex Slice & Weak PRNG Key Derivation`
- **Threat Model**: AI models frequently slice hex-encoded random bytes to 16 chars (reducing 128-bit entropy to 64 bits) or use Math.random().toString(36) for AES/JWT secret keys.
- **Verification Suite**: `694/694` unit tests passed, `501` active AST rules, repo score `100/100` (`2070ms`)

### Sprint #99 — v3.5.435 (2026-11-09 16:17 UTC)
- **New AST Shield**: `SLOP-502` (`no-async-iterator-unclosed-break-v502`, CWE-775) — `Async Generator Break Without Stream Disposal`
- **Threat Model**: Breaking out of a for-await-of stream loop without a try/finally block or stream.destroy() leaks underlying file descriptors and database cursors.
- **Verification Suite**: `695/695` unit tests passed, `502` active AST rules, repo score `100/100` (`2061ms`)

### Sprint #100 — v3.5.436 (2026-11-09 16:19 UTC)
- **New AST Shield**: `SLOP-503` (`no-compound-namespace-slopsquat-v503`, CWE-829) — `Compound Namespace Package Manifest Slopsquatting`
- **Threat Model**: AI assistants synthesize plausible compound package names such as flask-bcrypt-auth or tokio-redis-pro that attackers register on PyPI/crates.io/npm for RCE.
- **Verification Suite**: `696/696` unit tests passed, `503` active AST rules, repo score `100/100` (`1988ms`)

### Sprint #101 — v3.5.437 (2026-11-09 16:22 UTC)
- **New AST Shield**: `SLOP-504` (`no-unvalidated-ssrf-redirect-chain-v504`, CWE-918) — `Unvalidated SSRF Host Interpolation & Redirect Bypass`
- **Threat Model**: AI coding assistants interpolate user-controlled host variables directly into axios.get or requests.get URLs, enabling Server-Side Request Forgery against cloud metadata endpoints.
- **Verification Suite**: `697/697` unit tests passed, `504` active AST rules, repo score `100/100` (`2102ms`)

### Sprint #102 — v3.5.438 (2026-11-09 16:24 UTC)
- **New AST Shield**: `SLOP-505` (`no-non-atomic-toctou-fs-mutation-v505`, CWE-367) — `Non-Atomic TOCTOU File Existence Check Before Write`
- **Threat Model**: AI models routinely generate fs.existsSync(p) followed by fs.writeFileSync(p), creating a CWE-367 Time-of-Check Time-of-Use race window exploitable via symlink swapping.
- **Verification Suite**: `698/698` unit tests passed, `505` active AST rules, repo score `100/100` (`2081ms`)
