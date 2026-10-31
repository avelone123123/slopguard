use crate::types::SlopConfig;
use std::collections::HashMap;
use std::fs;
use std::path::Path;
pub fn default_config() -> SlopConfig {
    let mut rules = HashMap::new();
    rules.insert("no-ai-todo-placeholders".to_string(), "error".to_string());
    rules.insert("no-silent-catch-pass".to_string(), "error".to_string());
    rules.insert("no-dummy-secrets-mocks".to_string(), "error".to_string());
    rules.insert("no-mock-hallucinations".to_string(), "error".to_string());
    rules.insert("no-type-safety-degradation".to_string(), "error".to_string());
    rules.insert("no-ghost-dependencies".to_string(), "error".to_string());
    rules.insert("no-duplicate-shadow-utilities".to_string(), "warning".to_string());
    rules.insert("no-ai-conversational-bloat".to_string(), "warning".to_string());
    rules.insert("no-excessive-scope-drift".to_string(), "warning".to_string());
    rules.insert("no-sql-injection-risk".to_string(), "error".to_string());
    rules.insert("no-floating-promises".to_string(), "error".to_string());
    rules.insert("no-prototype-pollution".to_string(), "error".to_string());
    rules.insert("no-command-injection-risk".to_string(), "error".to_string());
    rules.insert("no-path-traversal-risk".to_string(), "error".to_string());
    rules.insert("no-silent-error-swallowing".to_string(), "error".to_string());
    rules.insert("no-redos-vulnerability".to_string(), "error".to_string());
    rules.insert("no-coverage-theater".to_string(), "warning".to_string());
    rules.insert("no-disabled-tls".to_string(), "error".to_string());
    rules.insert("no-unsafe-deserialization".to_string(), "error".to_string());
    rules.insert("no-insecure-cors".to_string(), "error".to_string());
    rules.insert("no-async-array-predicates".to_string(), "error".to_string());
    rules.insert("no-inlined-secrets".to_string(), "error".to_string());
    rules.insert("no-slopsquat-deps".to_string(), "warning".to_string());
    rules.insert("no-insecure-crypto".to_string(), "error".to_string());
    rules.insert("no-test-sabotage".to_string(), "error".to_string());
    rules.insert("no-unbounded-concurrency".to_string(), "warning".to_string());
    rules.insert("no-async-in-sync-callbacks".to_string(), "error".to_string());
    rules.insert("no-hallucinated-speculative-apis".to_string(), "error".to_string());
    rules.insert("no-insecure-prng-tokens".to_string(), "error".to_string());
    rules.insert("no-unawaited-async-in-loops".to_string(), "error".to_string());
    rules.insert("no-hallucinated-package-manifest".to_string(), "error".to_string());
    rules.insert("no-hardcoded-jwt-secrets".to_string(), "error".to_string());
    rules.insert("no-unscoped-idor-queries".to_string(), "error".to_string());
    rules.insert("no-unsafe-xml-or-ssrf".to_string(), "error".to_string());
    rules.insert("no-speculative-orm-methods".to_string(), "error".to_string());
    rules.insert("no-deep-path-traversal".to_string(), "error".to_string());
    rules.insert("no-async-pipeline-races".to_string(), "warning".to_string());
    rules.insert("no-phantom-cloud-sdks".to_string(), "error".to_string());
    SlopConfig {
        ignore: vec![
            "target".into(),
            ".git".into(),
            "node_modules".into(),
            "dist".into(),
            "build".into(),
            "coverage".into(),
            ".next".into(),
            "vendor".into(),
            "*.lock".into(),
            "*.md".into(),
            "*.svg".into(),
            "test/**".into(),
            "tests/**".into(),
        ],
        rules,
        max_drift_files: 12,
        allowed_ghost_deps: Vec::new(),
    }
}
pub fn load_config(project_root: &Path) -> SlopConfig {
    let candidates = [
        project_root.join(".slopguardrc.json"),
        project_root.join(".slopguardrc"),
        project_root.join("slopguard.config.json"),
    ];
    for path in &candidates {
        if path.exists() {
            if let Ok(content) = fs::read_to_string(path) {
                if let Ok(mut cfg) = serde_json::from_str::<SlopConfig>(&content) {
                    let mut def = default_config();
                    def.ignore.append(&mut cfg.ignore);
                    def.rules.extend(cfg.rules);
                    def.max_drift_files = cfg.max_drift_files;
                    def.allowed_ghost_deps = cfg.allowed_ghost_deps;
                    return def;
                }
            }
        }
    }
    default_config()
}
pub fn is_supported_code_file(path_str: &str) -> bool {
    let lower = path_str.to_lowercase();
    lower.ends_with(".rs")
        || lower.ends_with(".ts")
        || lower.ends_with(".tsx")
        || lower.ends_with(".js")
        || lower.ends_with(".jsx")
        || lower.ends_with(".mjs")
        || lower.ends_with(".cjs")
        || lower.ends_with(".py")
        || lower.ends_with(".go")
        || lower.ends_with(".php")
        || lower.ends_with(".java")
        || lower.ends_with(".cpp")
        || lower.ends_with(".c")
        || lower.ends_with(".cs")
        || lower.ends_with(".rb")
        || lower.ends_with(".env")
        || lower.contains(".env.")
        || lower.ends_with("package.json")
        || lower.ends_with("requirements.txt")
        || lower.ends_with("pyproject.toml")
        || lower.ends_with("cargo.toml")
}
pub fn should_ignore_path(file_path: &str, ignore_patterns: &[String]) -> bool {
    let normalized = file_path.replace('\\', "/");
    for pattern in ignore_patterns {
        let clean = pattern.trim_start_matches("./").replace('\\', "/");
        if clean.ends_with("/**") {
            let base = &clean[..clean.len() - 3];
            if normalized == base || normalized.starts_with(&format!("{}/", base)) || normalized.contains(&format!("/{}/", base)) {
                return true;
            }
        } else if clean.starts_with("*.") {
            let ext = &clean[1..];
            if normalized.ends_with(ext) {
                return true;
            }
        } else if normalized.contains(&format!("/{}/", clean)) || normalized.starts_with(&format!("{}/", clean)) || normalized == clean {
            return true;
        }
    }
    false
}