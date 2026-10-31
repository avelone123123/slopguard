pub mod ai_bloat;
pub mod circular_deps;
pub mod command_injection;
pub mod complexity;
pub mod coverage_theater;
pub mod duplicates;
pub mod env_vars;
pub mod floating_promises;
pub mod ghost_dependencies;
pub mod hallucinated_imports;
pub mod insecure_transport;
pub mod mocks;
pub mod path_traversal;
pub mod phantom_routes;
pub mod placeholders;
pub mod prototype_pollution;
pub mod redos;
pub mod scanner;
pub mod scope_drift;
pub mod secrets;
pub mod silent_errors;
pub mod sql_injection;
pub mod type_degradation;
pub mod unsafe_eval;
pub mod unsafe_deserialization;
pub mod insecure_cors;
pub mod watermarks;
pub mod inlined_secrets;
pub mod slopsquat;
pub mod insecure_crypto;
pub mod test_sabotage;
pub mod unbounded_concurrency;
pub mod async_callback_traps;
use crate::detectors::ai_bloat::{detect_ai_comment_bloat_in_content, detect_ai_comment_bloat_in_diff};
use crate::detectors::circular_deps::{detect_circular_dependencies_in_content, detect_circular_dependencies_in_diff};
use crate::detectors::command_injection::{detect_command_injection_in_content, detect_command_injection_in_diff};
use crate::detectors::complexity::{detect_nesting_sprawl_in_content, detect_nesting_sprawl_in_diff};
use crate::detectors::coverage_theater::{detect_coverage_theater_in_content, detect_coverage_theater_in_diff};
use crate::detectors::duplicates::{detect_duplicate_utilities, index_existing_functions};
use crate::detectors::env_vars::{detect_ghost_env_vars_in_content, detect_ghost_env_vars_in_diff};
use crate::detectors::floating_promises::{detect_floating_promises_in_content, detect_floating_promises_in_diff};
use crate::detectors::ghost_dependencies::{detect_ghost_dependencies_in_content, detect_ghost_dependencies_in_diff};
use crate::detectors::hallucinated_imports::{detect_hallucinated_imports_in_content, detect_hallucinated_imports_in_diff};
use crate::detectors::insecure_transport::{detect_insecure_transport_in_content, detect_insecure_transport_in_diff};
use crate::detectors::mocks::{detect_mocks_in_content, detect_mocks_in_diff};
use crate::detectors::path_traversal::{detect_path_traversal_in_content, detect_path_traversal_in_diff};
use crate::detectors::phantom_routes::{detect_phantom_routes_in_content, detect_phantom_routes_in_diff};
use crate::detectors::placeholders::{detect_placeholders_in_content, detect_placeholders_in_diff};
use crate::detectors::prototype_pollution::{detect_prototype_pollution_in_content, detect_prototype_pollution_in_diff};
use crate::detectors::redos::{detect_redos_in_content, detect_redos_in_diff};
use crate::detectors::scope_drift::detect_scope_drift;
use crate::detectors::secrets::{detect_secrets_in_content, detect_secrets_in_diff};
use crate::detectors::silent_errors::{detect_silent_errors_in_content, detect_silent_errors_in_diff};
use crate::detectors::sql_injection::{detect_sql_injection_in_content, detect_sql_injection_in_diff};
use crate::detectors::type_degradation::{detect_type_degradation_in_content, detect_type_degradation_in_diff};
use crate::detectors::unsafe_eval::{detect_unsafe_eval_in_content, detect_unsafe_eval_in_diff};
use crate::detectors::unsafe_deserialization::{detect_unsafe_deserialization_in_content, detect_unsafe_deserialization_in_diff};
use crate::detectors::insecure_cors::{detect_insecure_cors_in_content, detect_insecure_cors_in_diff};
use crate::detectors::watermarks::{detect_watermarks_in_content, detect_watermarks_in_diff};
use crate::detectors::inlined_secrets::{detect_inlined_secrets_in_content, detect_inlined_secrets_in_diff};
use crate::detectors::slopsquat::{detect_slopsquat_in_content, detect_slopsquat_in_diff};
use crate::detectors::insecure_crypto::{detect_insecure_crypto_in_content, detect_insecure_crypto_in_diff};
use crate::detectors::test_sabotage::{detect_test_sabotage_in_content, detect_test_sabotage_in_diff};
use crate::detectors::unbounded_concurrency::{detect_unbounded_concurrency_in_content, detect_unbounded_concurrency_in_diff};
use crate::detectors::async_callback_traps::{detect_async_callback_traps_in_content, detect_async_callback_traps_in_diff};
use crate::types::{DiffFile, SlopConfig, SlopFinding, SlopSeverity, SlopSummary};
use std::path::Path;
pub fn run_all_diff_detectors(diff_files: &[DiffFile], project_root: &Path, config: &SlopConfig) -> Vec<SlopFinding> {
    let mut findings = Vec::new();
    if config.rules.get("no-ai-todo-placeholders").map(|s| s != "off").unwrap_or(true) || config.rules.get("no-silent-catch-pass").map(|s| s != "off").unwrap_or(true) || config.rules.get("no-dummy-secrets-mocks").map(|s| s != "off").unwrap_or(true) {
        findings.extend(detect_placeholders_in_diff(diff_files));
    }
    if config.rules.get("no-mock-hallucinations").map(|s| s != "off").unwrap_or(true) {
        findings.extend(detect_mocks_in_diff(diff_files));
    }
    if config.rules.get("no-hallucinated-imports").map(|s| s != "off").unwrap_or(true) {
        findings.extend(detect_hallucinated_imports_in_diff(diff_files));
    }
    if config.rules.get("no-phantom-api-routes").map(|s| s != "off").unwrap_or(true) {
        findings.extend(detect_phantom_routes_in_diff(diff_files, project_root));
    }
    if config.rules.get("no-ghost-env-vars").map(|s| s != "off").unwrap_or(true) {
        findings.extend(detect_ghost_env_vars_in_diff(diff_files, project_root));
    }
    if config.rules.get("no-deep-nesting-sprawl").map(|s| s != "off").unwrap_or(true) {
        findings.extend(detect_nesting_sprawl_in_diff(diff_files, 7));
    }
    if config.rules.get("no-type-safety-degradation").map(|s| s != "off").unwrap_or(true) {
        findings.extend(detect_type_degradation_in_diff(diff_files));
    }
    if config.rules.get("no-ghost-dependencies").map(|s| s != "off").unwrap_or(true) {
        findings.extend(detect_ghost_dependencies_in_diff(diff_files, project_root, &config.allowed_ghost_deps));
    }
    if config.rules.get("no-circular-dependencies").map(|s| s != "off").unwrap_or(true) {
        findings.extend(detect_circular_dependencies_in_diff(diff_files, project_root));
    }
    if config.rules.get("no-ai-sycophancy-watermarks").map(|s| s != "off").unwrap_or(true) {
        findings.extend(detect_watermarks_in_diff(diff_files));
    }
    if config.rules.get("no-excessive-scope-drift").map(|s| s != "off").unwrap_or(true) {
        findings.extend(detect_scope_drift(diff_files, config.max_drift_files));
    }
    if config.rules.get("no-ai-conversational-bloat").map(|s| s != "off").unwrap_or(true) {
        findings.extend(detect_ai_comment_bloat_in_diff(diff_files));
    }
    if config.rules.get("no-duplicate-shadow-utilities").map(|s| s != "off").unwrap_or(true) {
        let existing = index_existing_functions(project_root, &config.ignore);
        findings.extend(detect_duplicate_utilities(diff_files, &existing));
    }
    if config.rules.get("no-hardcoded-secrets").map(|s| s != "off").unwrap_or(true) {
        findings.extend(detect_secrets_in_diff(diff_files));
    }
    if config.rules.get("no-unsafe-dynamic-eval").map(|s| s != "off").unwrap_or(true) {
        findings.extend(detect_unsafe_eval_in_diff(diff_files));
    }
    if config.rules.get("no-sql-injection-risk").map(|s| s != "off").unwrap_or(true) {
        findings.extend(detect_sql_injection_in_diff(diff_files));
    }
    if config.rules.get("no-floating-promises").map(|s| s != "off").unwrap_or(true) || config.rules.get("no-async-array-predicates").map(|s| s != "off").unwrap_or(true) {
        findings.extend(detect_floating_promises_in_diff(diff_files));
    }
    if config.rules.get("no-prototype-pollution").map(|s| s != "off").unwrap_or(true) {
        findings.extend(detect_prototype_pollution_in_diff(diff_files));
    }
    if config.rules.get("no-command-injection-risk").map(|s| s != "off").unwrap_or(true) {
        findings.extend(detect_command_injection_in_diff(diff_files));
    }
    if config.rules.get("no-path-traversal-risk").map(|s| s != "off").unwrap_or(true) {
        findings.extend(detect_path_traversal_in_diff(diff_files));
    }
    if config.rules.get("no-silent-error-swallowing").map(|s| s != "off").unwrap_or(true) {
        findings.extend(detect_silent_errors_in_diff(diff_files));
    }
    if config.rules.get("no-redos-vulnerability").map(|s| s != "off").unwrap_or(true) {
        findings.extend(detect_redos_in_diff(diff_files));
    }
    if config.rules.get("no-coverage-theater").map(|s| s != "off").unwrap_or(true) {
        findings.extend(detect_coverage_theater_in_diff(diff_files));
    }
    if config.rules.get("no-disabled-tls").map(|s| s != "off").unwrap_or(true) {
        findings.extend(detect_insecure_transport_in_diff(diff_files));
    }
    if config.rules.get("no-unsafe-deserialization").map(|s| s != "off").unwrap_or(true) {
        findings.extend(detect_unsafe_deserialization_in_diff(diff_files));
    }
    if config.rules.get("no-insecure-cors").map(|s| s != "off").unwrap_or(true) {
        findings.extend(detect_insecure_cors_in_diff(diff_files));
    }
    if config.rules.get("no-inlined-secrets").map(|s| s != "off").unwrap_or(true) {
        findings.extend(detect_inlined_secrets_in_diff(diff_files));
    }
    if config.rules.get("no-slopsquat-deps").map(|s| s != "off").unwrap_or(true) {
        findings.extend(detect_slopsquat_in_diff(diff_files));
    }
    if config.rules.get("no-insecure-crypto").map(|s| s != "off").unwrap_or(true) {
        findings.extend(detect_insecure_crypto_in_diff(diff_files));
    }
    if config.rules.get("no-test-sabotage").map(|s| s != "off").unwrap_or(true) {
        findings.extend(detect_test_sabotage_in_diff(diff_files));
    }
    if config.rules.get("no-unbounded-concurrency").map(|s| s != "off").unwrap_or(true) {
        findings.extend(detect_unbounded_concurrency_in_diff(diff_files));
    }
    if config.rules.get("no-async-in-sync-callbacks").map(|s| s != "off").unwrap_or(true) {
        findings.extend(detect_async_callback_traps_in_diff(diff_files));
    }
    findings.into_iter().filter(|f| config.rules.get(&f.rule_id).map(|s| s != "off").unwrap_or(true)).map(|mut f| {
        if let Some(rule_setting) = config.rules.get(&f.rule_id) {
            if rule_setting == "error" {
                f.severity = SlopSeverity::Error;
            } else if rule_setting == "warning" {
                f.severity = SlopSeverity::Warning;
            }
        }
        f
    }).collect()
}
pub fn run_all_content_detectors(file_path: &str, content: &str, project_root: &Path, config: &SlopConfig) -> Vec<SlopFinding> {
    let mut findings = Vec::new();
    findings.extend(detect_placeholders_in_content(file_path, content));
    findings.extend(detect_mocks_in_content(file_path, content));
    findings.extend(detect_hallucinated_imports_in_content(file_path, content));
    findings.extend(detect_phantom_routes_in_content(file_path, content, project_root));
    findings.extend(detect_ghost_env_vars_in_content(file_path, content, project_root));
    findings.extend(detect_nesting_sprawl_in_content(file_path, content, 7));
    findings.extend(detect_type_degradation_in_content(file_path, content));
    findings.extend(detect_ghost_dependencies_in_content(file_path, content, project_root, &config.allowed_ghost_deps));
    findings.extend(detect_circular_dependencies_in_content(file_path, content, project_root));
    findings.extend(detect_watermarks_in_content(file_path, content));
    findings.extend(detect_ai_comment_bloat_in_content(file_path, content));
    findings.extend(detect_secrets_in_content(file_path, content));
    findings.extend(detect_unsafe_eval_in_content(file_path, content));
    findings.extend(detect_sql_injection_in_content(file_path, content));
    findings.extend(detect_floating_promises_in_content(file_path, content));
    findings.extend(detect_prototype_pollution_in_content(file_path, content));
    findings.extend(detect_command_injection_in_content(file_path, content));
    findings.extend(detect_path_traversal_in_content(file_path, content));
    findings.extend(detect_silent_errors_in_content(file_path, content));
    findings.extend(detect_redos_in_content(file_path, content));
    findings.extend(detect_coverage_theater_in_content(file_path, content));
    findings.extend(detect_insecure_transport_in_content(file_path, content));
    findings.extend(detect_unsafe_deserialization_in_content(file_path, content));
    findings.extend(detect_insecure_cors_in_content(file_path, content));
    findings.extend(detect_inlined_secrets_in_content(file_path, content));
    findings.extend(detect_slopsquat_in_content(file_path, content));
    findings.extend(detect_insecure_crypto_in_content(file_path, content));
    findings.extend(detect_test_sabotage_in_content(file_path, content));
    findings.extend(detect_unbounded_concurrency_in_content(file_path, content));
    findings.extend(detect_async_callback_traps_in_content(file_path, content));
    findings.into_iter().filter(|f| config.rules.get(&f.rule_id).map(|s| s != "off").unwrap_or(true)).map(|mut f| {
        if let Some(rule_setting) = config.rules.get(&f.rule_id) {
            if rule_setting == "error" {
                f.severity = SlopSeverity::Error;
            } else if rule_setting == "warning" {
                f.severity = SlopSeverity::Warning;
            }
        }
        f
    }).collect()
}
pub fn compute_summary(findings: Vec<SlopFinding>, scanned_files: usize, duration_ms: u128) -> SlopSummary {
    let mut errors = 0;
    let mut warnings = 0;
    for f in &findings {
        match f.severity {
            SlopSeverity::Error => errors += 1,
            SlopSeverity::Warning => warnings += 1,
            SlopSeverity::Info => {}
        }
    }
    let penalty = (errors * 15) + (warnings * 5);
    let score = 100usize.saturating_sub(penalty);
    SlopSummary {
        total_issues: findings.len(),
        errors,
        warnings,
        score,
        findings,
        scanned_files,
        duration_ms,
    }
}