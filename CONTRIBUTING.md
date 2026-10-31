# Contributing to SlopGuard
Thank you for your interest in making AI coding cleaner, safer, and hallucination-free!
## How to Contribute
1. Fork the repository `https://github.com/avelone123123/slopguard`.
2. Create your feature branch (`git checkout -b feature/amazing-shield`).
3. Ensure your code passes all checks: `cargo test` and `cargo run -- all`.
4. Follow code guidelines: 100% Rust, zero AI-written comments, compact high-efficiency AST logic, no emojis (use SVG).
5. Commit your changes with descriptive messages: `git commit -m "feat(ast): add prototype pollution detector"`.
6. Push to your branch and open a Pull Request.
## New AST Detector Checklist
- Implement detector in `src/detectors/`.
- Register detector in `src/detectors/mod.rs`.
- Add test cases in `src/tests.rs` (both malicious snippet and valid clean snippet).
- Verify 100% test pass rate with `cargo test`.
## Code Review
Pull requests are automatically audited by our autonomous multi-agent engineering triad before lead architect approval.