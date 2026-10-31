# Changelog
All notable changes to SlopGuard are documented in this file.
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