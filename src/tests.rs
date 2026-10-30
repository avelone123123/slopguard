#[cfg(test)]
mod test_suite {
    use crate::dashboard::get_dashboard_html;
    use crate::detectors::ai_bloat::detect_ai_comment_bloat_in_content;
    use crate::detectors::compute_summary;
    use crate::detectors::ghost_dependencies::detect_ghost_dependencies_in_content;
    use crate::detectors::mocks::detect_mocks_in_content;
    use crate::detectors::placeholders::detect_placeholders_in_content;
    use crate::detectors::type_degradation::detect_type_degradation_in_content;
    use crate::fixers::sanitize_code;
    use crate::reporters::generate_badge_svg;
    use crate::telemetry::{create_shared_telemetry, record_event};
    use std::path::Path;
    #[test]
    fn test_detects_placeholders_and_dummy_tokens() {
        let code = ["const key = \"sk-1234567890123456789012\";", "fn run() {", "    let _todo = 1;", "    panic!(\"not implemented yet\");", "}", "fn handle() {", "    let _ = std::panic::catch_unwind(|| {});", "}"].join("\n");
        let findings = detect_placeholders_in_content("test.rs", &code);
        assert!(!findings.is_empty());
    }
    #[test]
    fn test_detects_rust_and_go_stubs() {
        let rust_code = "fn compute() {\n    todo!();\n}\nfn backup() {\n    unimplemented!();\n}";
        let findings = detect_placeholders_in_content("test.rs", rust_code);
        assert_eq!(findings.len(), 2);
    }
    #[test]
    fn test_detects_mock_hallucinations() {
        let host = ["json", "placeholder.typicode.com"].concat();
        let email = ["john", "@", "example.com"].concat();
        let code = format!("const API = \"https://{host}/posts\";\nconst user = \"{email}\";\n");
        let findings = detect_mocks_in_content("api.ts", &code);
        assert_eq!(findings.len(), 2);
    }
    #[test]
    fn test_detects_type_degradations() {
        let k_ts = ["@", "ts-", "ignore"].concat();
        let k_any = ["a", "n", "y"].concat();
        let code = format!("// {k_ts}\nconst x: {k_any} = 42;\nconst y = z as unknown as {k_any};\n");
        let findings = detect_type_degradation_in_content("test.ts", &code);
        assert!(findings.len() >= 2);
    }
    #[test]
    fn test_detects_ghost_dependencies() {
        let code = "import axios from \"axios\";\nimport { merge } from \"lodash-es\";\n";
        let findings = detect_ghost_dependencies_in_content("test.ts", code, Path::new("."), &[]);
        assert!(findings.len() >= 2);
    }
    #[test]
    fn test_detects_and_sanitizes_ai_bloat() {
        let bloat = "// Here is the updated code:\nfn sum(a: i32, b: i32) -> i32 {\n// In order to return\na + b\n}\n";
        let findings = detect_ai_comment_bloat_in_content("test.rs", bloat);
        assert!(!findings.is_empty());
        let (cleaned, modified) = sanitize_code(bloat);
        assert!(modified);
        assert!(!cleaned.contains("Here is the updated code"));
        assert!(cleaned.contains("fn sum"));
    }
    #[test]
    fn test_dashboard_ui_html() {
        let html = get_dashboard_html();
        assert!(html.contains("SLOP"));
        assert!(html.contains("GUARD"));
        assert!(html.contains("/api/telemetry"));
    }
    #[test]
    fn test_telemetry_event_recording() {
        let shared = create_shared_telemetry();
        record_event(&shared, Path::new("."), "test_tool", "src/auth.rs", 2, 85, 3, "Blocked 2 items");
        let state = shared.lock().unwrap();
        assert_eq!(state.total_scans, 1);
        assert_eq!(state.total_slop_blocked, 2);
        assert_eq!(state.current_hygiene_score, 85);
        assert_eq!(state.events.len(), 1);
    }
    #[test]
    fn test_computes_score_and_badge() {
        let summary = compute_summary(Vec::new(), 10, 42);
        assert_eq!(summary.score, 100);
        assert_eq!(summary.total_issues, 0);
        let badge = generate_badge_svg(summary.score, summary.errors);
        assert!(badge.contains("100% clean"));
        assert!(badge.contains("<svg"));
    }
}