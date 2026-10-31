# Changelog
All notable changes to SlopGuard are documented in this file.
## [1.5.0] - 2026-10-31
### Added
- Implemented `no-unawaited-async-in-loops` AST detector (SLOP-037) for JS/TS, Python, and Rust.
- Implemented `no-hallucinated-package-manifest` zero-trust supply chain firewall (SLOP-038).
- Expanded `SlopCategory` with `SupplyChainPoisoning` and `AsyncIterationHazard`.
- Added 8 new regression test suites reaching 70 total tests with 100% pass verification.
- Web playground interactive AST diff auditor and dependency slopsquatting firewall demo.
## [1.4.0] - 2026-10-31
### Added
- Speculative dependency slopsquatting firewall (`no-slopsquat-deps`, SLOP-036) for npm, PyPI, and crates.io.
- Live interactive AST playground and real-time diff auditor in official website.
- Multi-ecosystem manifest regression suites (package.json, requirements.txt, pyproject.toml, Cargo.toml) reaching 62 unit tests at 100% pass rate.
## [1.3.0] - 2026-10-31
### Added
- Hallucinated speculative stdlib APIs firewall (`no-hallucinated-speculative-apis`, SLOP-035).
- Insecure PRNG detection (`no-insecure-prng-tokens`, SLOP-036).
## [1.2.0] - 2026-10-31
### Added
- Implemented `no-async-in-sync-callbacks` AST firewall detector for JS/TS (`.forEach`, `.reduce`), Python (`map()`, `filter()` coroutines), and Rust (`.for_each(|x| async move { ... })`).
- Expanded `SlopCategory` with `LogicBypass`.
- 4 new test suites (51 unit tests total, 100% pass rate).
- Updated web landing page with 34 active AST rules & interactive playground scenario.
## [1.1.0] - 2026-10-31
### Added
- Implemented `no-unbounded-concurrency` AST firewall for JS/TS, Python, and Rust.
- Multi-platform automated GitHub Actions release workflow.
- Interactive rule inspection drawer on landing page with deterministic fix snippets (`p-limit`, `asyncio.Semaphore`, `buffer_unordered`).
## [1.0.0] - 2026-10-31
### Added
- Native Rust zero-overhead AST scan engine.
- 44 rigorous test suites with 100% pass verification.
- Live Web Telemetry Dashboard on port 4545 with animated SVG oscilloscope and real-time event feeds.
- Real-time MCP Server protocol integration for Cursor, Claude Code, and Antigravity.
- Automatic bloat, stub, and AI sycophancy sanitizer.
- Interactive terminal TUI monitor with waveform visualizer.
- Cross-platform release precompiled binaries (`slopguard.exe`, `slopguard-v1.0.0-windows-x64.exe`).
### Fixed
- Fixed release binary download URL on landing page.
- Fixed repository author namespace to `avelone123123`.
- Improved default double-click execution to launch live Web Dashboard GUI.