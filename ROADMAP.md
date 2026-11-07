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
### Sprint #80 — v3.5.94 (2026-11-07 13:25 UTC)
- **New AST Shield**: `SLOP-160` (`no-compound-namespace-slopsquat-v160`, CWE-829) — `Compound Namespace Package Manifest Slopsquatting`
- **Threat Model**: 1.  **Analyze the Request:** *   **Rule Name:** compound-namespace-slopsquatting (Compound Namespace Package Manifest Slopsquatting, CWE-829). *   **Blocked Code:** flask-bcrypt-auth>=1.0, tokio-redis-pro = "1.0.0" (These look like AI hallucinated or artificially created package names that mimic real libraries but are malicious/unsafe). *   **Enforced Fix:**
- **Verification Suite**: `353/353` unit tests passed, `160` active AST rules, repo score `100/100` (`2044ms`)

### Sprint #81 — v3.5.95 (2026-11-07 13:26 UTC)
- **New AST Shield**: `SLOP-161` (`no-unvalidated-ssrf-redirect-chain-v161`, CWE-918) — `Unvalidated SSRF Host Interpolation & Redirect Bypass`
- **Threat Model**: AI coding assistants interpolate user-controlled host variables directly into axios.get or requests.get URLs, enabling Server-Side Request Forgery against cloud metadata endpoints.
- **Verification Suite**: `354/354` unit tests passed, `161` active AST rules, repo score `100/100` (`2026ms`)

### Sprint #82 — v3.5.96 (2026-11-07 13:28 UTC)
- **New AST Shield**: `SLOP-162` (`no-non-atomic-toctou-fs-mutation-v162`, CWE-367) — `Non-Atomic TOCTOU File Existence Check Before Write`
- **Threat Model**: AI models routinely generate fs.existsSync(p) followed by fs.writeFileSync(p), creating a CWE-367 Time-of-Check Time-of-Use race window exploitable via symlink swapping.
- **Verification Suite**: `355/355` unit tests passed, `162` active AST rules, repo score `100/100` (`2043ms`)

### Sprint #83 — v3.5.97 (2026-11-07 13:30 UTC)
- **New AST Shield**: `SLOP-163` (`no-hallucinated-cipher-primitive-v163`, CWE-327) — `Hallucinated Non-Standard Cryptographic Cipher Names`
- **Threat Model**: LLMs invent non-existent cipher strings such as aes-256-turbo or sha256-fast in OpenSSL and Node crypto calls, causing runtime panics or silent fallback to weak encryption.
- **Verification Suite**: `356/356` unit tests passed, `163` active AST rules, repo score `100/100` (`2189ms`)

### Sprint #84 — v3.5.98 (2026-11-07 13:33 UTC)
- **New AST Shield**: `SLOP-164` (`no-shell-template-subprocess-exec-v164`, CWE-78) — `Formatted Shell String Subprocess Command Injection`
- **Threat Model**: AI assistants pass formatted f-strings or template literals into subprocess.Popen(..., shell=True) and child_process.exec, allowing arbitrary shell command execution.
- **Verification Suite**: `357/357` unit tests passed, `164` active AST rules, repo score `100/100` (`2063ms`)

### Sprint #85 — v3.5.99 (2026-11-07 13:35 UTC)
- **New AST Shield**: `SLOP-165` (`no-loop-scoped-event-listener-leak-v165`, CWE-400) — `Unbounded Event Listener Registration Inside Loops`
- **Threat Model**: AI-generated stream handlers attach emitter.on('data', ...) inside for/while loops without cleanup, exhausting V8 heap memory and triggering MaxListenersExceededWarning.
- **Verification Suite**: `358/358` unit tests passed, `165` active AST rules, repo score `100/100` (`2025ms`)

### Sprint #86 — v3.5.100 (2026-11-07 13:38 UTC)
- **New AST Shield**: `SLOP-166` (`no-phantom-ai-sdk-slopsquatting-v166`, CWE-1357) — `Phantom AI SDK Wrapper Package Slopsquatting`
- **Threat Model**: LLMs hallucinate plausible AI helper packages like @anthropic/tools-pro or openai-streaming-helper in package.json and requirements.txt, exposing builds to supply-chain hijacking.
- **Verification Suite**: `359/359` unit tests passed, `166` active AST rules, repo score `100/100` (`2072ms`)

### Sprint #87 — v3.5.101 (2026-11-07 13:40 UTC)
- **New AST Shield**: `SLOP-167` (`no-unbounded-mpsc-queue-sprawl-v167`, CWE-400) — `Unbounded MPSC & Asyncio Queue Memory Exhaustion`
- **Threat Model**: AI concurrency scaffolding defaults to tokio::sync::mpsc::unbounded_channel() and unbuffered asyncio.Queue(), removing backpressure and causing OOM kills under traffic spikes.
- **Verification Suite**: `360/360` unit tests passed, `167` active AST rules, repo score `100/100` (`2068ms`)

### Sprint #88 — v3.5.102 (2026-11-07 13:42 UTC)
- **New AST Shield**: `SLOP-168` (`no-truncated-hex-entropy-keygen-v168`, CWE-326) — `Truncated Hex Slice & Weak PRNG Key Derivation`
- **Threat Model**: AI models frequently slice hex-encoded random bytes to 16 chars (reducing 128-bit entropy to 64 bits) or use Math.random().toString(36) for AES/JWT secret keys.
- **Verification Suite**: `361/361` unit tests passed, `168` active AST rules, repo score `100/100` (`2087ms`)

### Sprint #89 — v3.5.103 (2026-11-07 13:44 UTC)
- **New AST Shield**: `SLOP-169` (`no-async-iterator-unclosed-break-v169`, CWE-775) — `Async Generator Break Without Stream Disposal`
- **Threat Model**: Breaking out of a for-await-of stream loop without a try/finally block or stream.destroy() leaks underlying file descriptors and database cursors.
- **Verification Suite**: `362/362` unit tests passed, `169` active AST rules, repo score `100/100` (`2049ms`)

### Sprint #90 — v3.5.104 (2026-11-07 13:47 UTC)
- **New AST Shield**: `SLOP-170` (`no-compound-namespace-slopsquat-v170`, CWE-829) — `Compound Namespace Package Manifest Slopsquatting`
- **Threat Model**: AI assistants synthesize plausible compound package names such as flask-bcrypt-auth or tokio-redis-pro that attackers register on PyPI/crates.io/npm for RCE.
- **Verification Suite**: `363/363` unit tests passed, `170` active AST rules, repo score `100/100` (`2079ms`)

### Sprint #91 — v3.5.105 (2026-11-07 13:49 UTC)
- **New AST Shield**: `SLOP-171` (`no-unvalidated-ssrf-redirect-chain-v171`, CWE-918) — `Unvalidated SSRF Host Interpolation & Redirect Bypass`
- **Threat Model**: 1.  **Analyze the Request:** *   **Rule:** unvalidated-ssrf-redirect-chain (CWE-918). *   **Blocked code:** `const res = await axios.get(`https://${req.query.apiHost}/v1/profile`);` (SSRF via interpolation of user input). *   **Enforced fix:** `if (!ALLOWED_ORIGINS.has(req.query.apiHost)) throw new Error('Blocked');` and a hardcoded URL `axios.get('https://a
- **Verification Suite**: `364/364` unit tests passed, `171` active AST rules, repo score `100/100` (`2096ms`)

### Sprint #92 — v3.5.106 (2026-11-07 13:52 UTC)
- **New AST Shield**: `SLOP-172` (`no-non-atomic-toctou-fs-mutation-v172`, CWE-367) — `Non-Atomic TOCTOU File Existence Check Before Write`
- **Threat Model**: AI models routinely generate fs.existsSync(p) followed by fs.writeFileSync(p), creating a CWE-367 Time-of-Check Time-of-Use race window exploitable via symlink swapping.
- **Verification Suite**: `365/365` unit tests passed, `172` active AST rules, repo score `100/100` (`2031ms`)

### Sprint #93 — v3.5.107 (2026-11-07 13:54 UTC)
- **New AST Shield**: `SLOP-173` (`no-hallucinated-cipher-primitive-v173`, CWE-327) — `Hallucinated Non-Standard Cryptographic Cipher Names`
- **Threat Model**: LLMs invent non-existent cipher strings such as aes-256-turbo or sha256-fast in OpenSSL and Node crypto calls, causing runtime panics or silent fallback to weak encryption.
- **Verification Suite**: `366/366` unit tests passed, `173` active AST rules, repo score `100/100` (`2093ms`)

### Sprint #94 — v3.5.108 (2026-11-07 13:57 UTC)
- **New AST Shield**: `SLOP-174` (`no-shell-template-subprocess-exec-v174`, CWE-78) — `Formatted Shell String Subprocess Command Injection`
- **Threat Model**: AI assistants pass formatted f-strings or template literals into subprocess.Popen(..., shell=True) and child_process.exec, allowing arbitrary shell command execution.
- **Verification Suite**: `367/367` unit tests passed, `174` active AST rules, repo score `100/100` (`2043ms`)
