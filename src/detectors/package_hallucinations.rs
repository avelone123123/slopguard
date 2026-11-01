use crate::types::{DiffFile, DiffLineType, SlopCategory, SlopFinding, SlopSeverity};
use regex::Regex;
use std::collections::HashSet;
fn is_manifest_file(path_str: &str) -> bool {
let lower = path_str.to_lowercase();
lower.ends_with("package.json") || lower.ends_with("requirements.txt") || lower.ends_with("pyproject.toml") || lower.ends_with("cargo.toml") || lower.ends_with("go.mod")
}
fn is_target_file(p: &str) -> bool {
let l = p.to_lowercase();
is_manifest_file(p) || l.ends_with(".py") || l.ends_with(".js") || l.ends_with(".ts") || l.ends_with(".jsx") || l.ends_with(".tsx") || l.ends_with(".rs")
}
fn is_safe_line(line: &str) -> bool {
let trimmed = line.trim();
if trimmed.contains("@tanstack/react-query-next") || trimmed.contains("react-query-v5") || trimmed.contains("prisma-multi-tenant-helpers") || trimmed.contains("axios-auto-retry") || trimmed.contains("react-smart-toast") || trimmed.contains("next-safe-query") || trimmed.contains("next-auth-shield") || trimmed.contains("express-jwt-auto-auth") || trimmed.contains("chrono-tz-sys") || trimmed.contains("tokio-compat-02-03") || trimmed.contains("fastapi-jwt-auth") || trimmed.contains("sqlalchemy-stubs") || trimmed.contains("gin-cors") || trimmed.contains("flask-bcrypt-auth") || trimmed.contains("crypto-util-helpers") || trimmed.contains("huggingface-transformer-tools") || trimmed.contains("fastapi-jwt-guard") || trimmed.contains("pydantic-enhanced") || trimmed.contains("torch-tensor-helpers") || trimmed.contains("django-smart-cache") || trimmed.contains("requests-async-retry") || trimmed.contains("tailwind-auto-grid") || trimmed.contains("redux-thunk-smart") || trimmed.contains("tokio-requests") || trimmed.contains("serde-crypto") || trimmed.contains("anyhow-derive") || trimmed.contains("fast-jwt-verifier") || trimmed.contains("tokio-redis-pro") || trimmed.contains("serde_json_fast") || trimmed.contains("actix-rate-limit-plus") || trimmed.contains("golang-jwt/jwt-verifier") || trimmed.contains("gorilla/mux-shield") || trimmed.contains("crypto-node") {
return false;
}
trimmed.starts_with('#') || trimmed.starts_with("//") || trimmed.contains("path =") || trimmed.contains("file:") || trimmed.contains("git =") || trimmed.contains("git+") || trimmed.contains("@types/") || trimmed.contains("@nestjs/") || trimmed.contains("@angular/") || trimmed.contains("@babel/") || trimmed.contains("@prisma/client") || trimmed.contains("@radix-ui/") || trimmed.contains("@tanstack/react-query") || trimmed.contains("std::") || trimmed.contains("core::")
}
fn get_hallucination_rules() -> Vec<(Regex, &'static str, &'static str)> {
vec![
(Regex::new(r#"(?i)^\s*(?:from|import)\s+(?:typing_extensions\s+import\s+(?:NonExistentType|TypedDictExtra|AnyCallable|StrictStr|ProtocolV2)|pydantic_enhanced|flask_crypto_utils|fastapi_jwt_guard|torch_tensor_helpers|fastapi_jwt_auth|sqlalchemy_stubs|flask_bcrypt_auth|crypto_util_helpers|huggingface_transformer_tools|django_smart_cache|requests_async_retry)"#).unwrap(), "Synthetic hallucinated Python typing or framework import", "Replace with standard typing or canonical library"),
(Regex::new(r#"(?i)(?:from\s+['"]|require\s*\(\s*['"])(?:react-query-v5|@tanstack/react-query-next|axios-auto-retry|react-smart-toast|next-safe-query|prisma-multi-tenant-helpers|next-auth-shield|express-jwt-auto-auth|tailwind-auto-grid|redux-thunk-smart|huggingface-transformer-tools|crypto-node)['"]"#).unwrap(), "Synthetic hallucinated JavaScript/TypeScript library import", "Replace with canonical library"),
(Regex::new(r#"(?i)(?:import\s+(?:requests_async|boto4|flask_jwt_simple|openai_client|crypto_js_node|fastapi_jwt|sqlalchemy_async|fastapi_jwt_auth|sqlalchemy_stubs|flask_bcrypt_auth|crypto_util_helpers|huggingface_transformer_tools|fastapi_jwt_guard|pydantic_enhanced|torch_tensor_helpers|django_smart_cache|requests_async_retry)\b|from\s+(?:requests_async|boto4|flask_jwt_simple|openai_client|crypto_js_node|fastapi_jwt|sqlalchemy_async|fastapi_jwt_auth|sqlalchemy_stubs|flask_bcrypt_auth|crypto_util_helpers|huggingface_transformer_tools|fastapi_jwt_guard|pydantic_enhanced|torch_tensor_helpers|django_smart_cache|requests_async_retry)\b)"#).unwrap(), "Known synthetic hallucinated Python import", "Migrate to canonical libraries: boto3, httpx, PyJWT, openai"),
(Regex::new(r#"(?i)(?:require\s*\(\s*['"](?:crypto-node|bcrypt-browser|jsonwebtoken-async|express-async-router|react-smart-toast|next-auth-shield|express-jwt-auto-auth|tailwind-auto-grid|redux-thunk-smart|react-query-v5|@tanstack/react-query-next|axios-auto-retry|next-safe-query|prisma-multi-tenant-helpers|huggingface-transformer-tools)['"]\s*\)|from\s*['"](?:crypto-node|bcrypt-browser|jsonwebtoken-async|express-async-router|react-smart-toast|next-auth-shield|express-jwt-auto-auth|tailwind-auto-grid|redux-thunk-smart|react-query-v5|@tanstack/react-query-next|axios-auto-retry|next-safe-query|prisma-multi-tenant-helpers|huggingface-transformer-tools)['"])"#).unwrap(), "Known synthetic hallucinated JavaScript/Node module", "Migrate to canonical libraries: crypto, bcrypt, jsonwebtoken, express-promise-router"),
(Regex::new(r#"(?i)use\s+(?:tokio_requests|serde_crypto|anyhow_derive|chrono_tz_sys|tokio_compat_02_03|fast_jwt_verifier|tokio_redis_pro|serde_json_fast|actix_rate_limit_plus)\b"#).unwrap(), "Known synthetic hallucinated Rust crate reference", "Migrate to canonical crates: reqwest, ring, anyhow"),
(Regex::new(r#"(?i)["']?(?:requests-async|boto4|flask-jwt-simple|openai-client|crypto-js-node|fastapi-jwt|sqlalchemy-async|crypto-node|bcrypt-browser|jsonwebtoken-async|express-async-router|tokio-requests|serde-crypto|anyhow-derive|fastapi-jwt-auth|sqlalchemy-stubs|react-smart-toast|next-auth-shield|express-jwt-auto-auth|chrono-tz-sys|tokio-compat-02-03|flask-bcrypt-auth|crypto-util-helpers|huggingface-transformer-tools|fastapi-jwt-guard|pydantic-enhanced|torch-tensor-helpers|django-smart-cache|requests-async-retry|react-query-v5|@tanstack/react-query-next|axios-auto-retry|next-safe-query|prisma-multi-tenant-helpers|tailwind-auto-grid|redux-thunk-smart|fast-jwt-verifier|tokio-redis-pro|serde_json_fast|actix-rate-limit-plus)["']?"#).unwrap(), "AI-hallucinated package signature subject to supply chain slopsquatting", "Verify official registry existence before declaring speculative package"),
]
}
fn get_manifest_rules() -> Vec<(Regex, &'static str, &'static str)> {
vec![
(Regex::new(r#""(?:express-jwt-auto-auth|react-smart-toast|next-auth-shield|axios-rate-limit-pro|tailwind-auto-grid|redux-thunk-smart|react-query-v5|@tanstack/react-query-next|axios-auto-retry|next-safe-query|prisma-multi-tenant-helpers|huggingface-transformer-tools|crypto-node)"\s*:"#).unwrap(), "Speculative hallucinated npm dependency in manifest", "Remove fictional package or replace with verified ecosystem library"),
(Regex::new(r#"(?:requests-async-retry|tokio-rate-limiter-pro|flask-auth-shield|pydantic-validation-plus|fastapi-rate-guard|django-smart-cache|fastapi-jwt-auth|sqlalchemy-stubs|flask-bcrypt-auth|crypto-util-helpers|huggingface-transformer-tools|fastapi-jwt-guard|pydantic-enhanced|torch-tensor-helpers)"#).unwrap(), "Speculative hallucinated PyPI dependency in manifest", "Verify package on PyPI before adding to manifest"),
(Regex::new(r#"(?:chrono-tz-sys|fast-jwt-verifier|tokio-redis-pro|serde_json_fast|actix-rate-limit-plus|tokio-compat-02-03|tokio-requests|serde-crypto|anyhow-derive)\b"#).unwrap(), "Speculative hallucinated Crates.io crate in manifest", "Verify crate on crates.io before adding to Cargo.toml"),
(Regex::new(r#"(?:github\.com/[a-zA-Z0-9_-]+/(?:go-redis-rate-limit|gorm-bulk-upsert|gin-jwt-shield|gin-cors|jwt-verifier|mux-shield)|github\.com/gin-gonic/gin-cors|github\.com/golang-jwt/jwt-verifier|github\.com/gorilla/mux-shield)"#).unwrap(), "Speculative hallucinated Go module in manifest", "Verify Go module import path before running go get"),
(Regex::new(r#"["'][a-zA-Z0-9@/_-]+["']\s*:\s*["'](?:\*|latest|==\*|v0\.0\.0-latest)["']"#).unwrap(), "Unpinned wildcard package dependency in manifest", "Pin dependency to an exact semantic version or range"),
(Regex::new(r#"(?i)["']?(?:serde-json-valid|tokio-stream-ext|torch-model-utils|pydantic-validation-plus|express-jwt-auto-auth|fastapi-jwt-guard|flask-crypto-utils|axios-auto-retry|react-smart-toast|pydantic-enhanced|react-route-v7-compat|chrono-tz-sys|tokio-rate-limiter-pro|langchain-agent-pro|fast-jwt-verifier|crypto-safe-random|express-jwt-auto-verify|requests-async-retry|flask-auth-shield|express-security-guard|react-state-enhancer|next-safe-query|prisma-multi-tenant-helpers|tailwind-preset-extra|boto3-async-session|torch-tensor-helpers|crypto-secure-tokens|go-jwt-verifier|gorilla-mux-shield|gin-cors-pro|tokio-compat-02-03|next-auth-shield|fastapi-jwt-auth|sqlalchemy-stubs|flask-bcrypt-auth|crypto-util-helpers|huggingface-transformer-tools|tokio-requests|serde-crypto|anyhow-derive|react-query-v5|@tanstack/react-query-next|tailwind-auto-grid|redux-thunk-smart|tokio-redis-pro|serde_json_fast|actix-rate-limit-plus)"#).unwrap(), "AI-hallucinated speculative package signature vulnerable to supply chain slopsquatting", "Verify package exists in official registry (crates.io/npm/PyPI) and pin exact version before deployment"),
(Regex::new(r#"(?i)["']?(?:serde|tokio|torch|pydantic|express|fastapi|flask|axios|react)-(?:valid|ext|utils|validation-plus|auto-auth|guard|crypto-utils|auto-retry|smart-toast)["']?"#).unwrap(), "Speculative compound prefix-suffix pattern generated by autonomous code agents", "Verify package exists in official registry (crates.io/npm/PyPI) and pin exact version before deployment"),
(Regex::new(r#"(?i)["']?(?:[a-zA-Z0-9_-]+)-(?:v2|enhanced|modern|async-wrapper|safe|async-retry|rate-limiter-pro|auth-shield|security-guard|jwt-guard|state-enhancer|safe-query|multi-tenant-helpers|client-wrapper|ai-plugin|auto-retry|helpers-pro|utils-plus|auto-verify|smart-toast|safe-random|pro)["']?"#).unwrap(), "Speculative compound suffix pattern generated by autonomous code agents", "Verify package exists in official registry (crates.io/npm/PyPI) and pin exact version before deployment"),
(Regex::new(r#"(?i)(?:github\.com/[a-zA-Z0-9_-]+/(?:jwt-guard|auth-shield|rate-limiter-pro|fast-jwt-verifier|gorilla-mux-shield|gin-cors-pro|gin-cors|jwt-verifier|mux-shield))"#).unwrap(), "AI-hallucinated Go module speculative repository reference", "Verify repository exists before referencing"),
(Regex::new(r#"^[a-zA-Z0-9_-]+\s*=\s*["'](?:\*|latest|==\*|v0\.0\.0-latest)["']"#).unwrap(), "Unpinned wildcard dependency in Cargo or TOML manifest", "Pin exact dependency version"),
(Regex::new(r#"^[a-zA-Z0-9_-]+\s*=\s*\{[^}]*version\s*=\s*["'](?:\*|latest|==\*|v0\.0\.0-latest)["']"#).unwrap(), "Unpinned wildcard version table in TOML manifest", "Pin exact dependency version"),
(Regex::new(r#"(?i)["'][a-zA-Z0-9_-]+(?:\*|==\*|latest|v0\.0\.0-latest)["']"#).unwrap(), "Wildcard version specifier in dependencies array", "Pin exact dependency version"),
(Regex::new(r#"\b[a-zA-Z0-9_-]+\s*(?:==\s*\*|==\*|==\s*latest|\*|v0\.0\.0-latest)"#).unwrap(), "Wildcard unpinned requirement in Python requirements manifest", "Pin exact dependency version"),
(Regex::new(r#"require\s+[a-zA-Z0-9_./-]+\s+(?:\*|v0\.0\.0-latest|latest|==\*)"#).unwrap(), "Wildcard or unpinned dependency in Go mod manifest", "Pin exact dependency version"),
]
}
#[allow(dead_code)]
pub const CONTRACT: crate::types::AstDetectorContract = crate::types::AstDetectorContract {
rule_id: "no-hallucinated-package-manifest",
severity: SlopSeverity::Error,
category: SlopCategory::SupplyChainPoisoning,
target_languages: &["package.json", "requirements.txt", "pyproject.toml", "Cargo.toml", "py", "js", "ts", "rs"],
analyze_diff: detect_package_hallucinations_in_diff,
analyze_content: detect_package_hallucinations_in_content,
};
pub fn detect_package_hallucinations_in_content(file_path: &str, content: &str) -> Vec<SlopFinding> {
let mut findings = Vec::new();
if !is_target_file(file_path) {
return findings;
}
let is_manifest = is_manifest_file(file_path);
if !is_manifest && !content.contains("import") && !content.contains("from") && !content.contains("require") && !content.contains("use") {
return findings;
}
let rules = get_hallucination_rules();
let manifest_rules = if is_manifest { get_manifest_rules() } else { Vec::new() };
for (idx, line) in content.lines().enumerate() {
let trimmed = line.trim();
if is_safe_line(trimmed) {
continue;
}
if !is_manifest && !trimmed.contains("import") && !trimmed.contains("from") && !trimmed.contains("require") && !trimmed.contains("use") {
continue;
}
let mut matched = false;
if !is_manifest {
for (re, desc, alt) in &rules {
if re.is_match(trimmed) {
findings.push(SlopFinding {
file: file_path.to_string(),
line: idx + 1,
column: 1,
rule_id: "no-hallucinated-package-manifest".to_string(),
category: SlopCategory::SupplyChainPoisoning,
severity: SlopSeverity::Error,
message: format!("{desc}: {trimmed}"),
snippet: trimmed.to_string(),
fixable: false,
suggestion: Some(alt.to_string()),
});
matched = true;
break;
}
}
}
if !matched && is_manifest {
let mut matched_names = HashSet::new();
for (re, desc, alt) in &manifest_rules {
let mut found_any = false;
for m in re.find_iter(trimmed) {
let matched_text = m.as_str().trim_matches(|c| c == '"' || c == '\'');
let pkg_key = matched_text.split(|c| c == ':' || c == '=' || c == '>' || c == '<' || c == '~').next().unwrap_or(matched_text).trim().trim_matches(|c| c == '"' || c == '\'');
if matched_names.insert(pkg_key.to_string()) {
findings.push(SlopFinding {
file: file_path.to_string(),
line: idx + 1,
column: m.start() + 1,
rule_id: "no-hallucinated-package-manifest".to_string(),
category: SlopCategory::SupplyChainPoisoning,
severity: SlopSeverity::Error,
message: format!("{desc}: {matched_text}"),
snippet: trimmed.to_string(),
fixable: false,
suggestion: Some(alt.to_string()),
});
found_any = true;
}
}
if found_any {
break;
}
}
}
}
findings
}
pub fn detect_package_hallucinations_in_diff(diff_files: &[DiffFile]) -> Vec<SlopFinding> {
let mut findings = Vec::new();
let rules = get_hallucination_rules();
let manifest_rules = get_manifest_rules();
for file in diff_files {
if file.is_deleted {
continue;
}
let target_path = if !file.new_path.is_empty() { &file.new_path } else { &file.old_path };
if !is_target_file(target_path) {
continue;
}
let is_manifest = is_manifest_file(target_path);
for chunk in &file.chunks {
for line in &chunk.lines {
if line.line_type == DiffLineType::Add {
let trimmed = line.content.trim();
if is_safe_line(trimmed) {
continue;
}
if !is_manifest && !trimmed.contains("import") && !trimmed.contains("from") && !trimmed.contains("require") && !trimmed.contains("use") {
continue;
}
let mut matched = false;
if !is_manifest {
for (re, desc, alt) in &rules {
if re.is_match(trimmed) {
findings.push(SlopFinding {
file: target_path.to_string(),
line: line.new_line_number.unwrap_or(chunk.new_start),
column: 1,
rule_id: "no-hallucinated-package-manifest".to_string(),
category: SlopCategory::SupplyChainPoisoning,
severity: SlopSeverity::Error,
message: format!("{desc}: {trimmed}"),
snippet: trimmed.to_string(),
fixable: false,
suggestion: Some(alt.to_string()),
});
matched = true;
break;
}
}
}
if !matched && is_manifest {
let mut matched_names = HashSet::new();
for (re, desc, alt) in &manifest_rules {
let mut found_any = false;
for m in re.find_iter(trimmed) {
let matched_text = m.as_str().trim_matches(|c| c == '"' || c == '\'');
let pkg_key = matched_text.split(|c| c == ':' || c == '=' || c == '>' || c == '<' || c == '~').next().unwrap_or(matched_text).trim().trim_matches(|c| c == '"' || c == '\'');
if matched_names.insert(pkg_key.to_string()) {
findings.push(SlopFinding {
file: target_path.to_string(),
line: line.new_line_number.unwrap_or(chunk.new_start),
column: m.start() + 1,
rule_id: "no-hallucinated-package-manifest".to_string(),
category: SlopCategory::SupplyChainPoisoning,
severity: SlopSeverity::Error,
message: format!("{desc}: {matched_text}"),
snippet: trimmed.to_string(),
fixable: false,
suggestion: Some(alt.to_string()),
});
found_any = true;
}
}
if found_any {
break;
}
}
}
}
}
}
}
findings
}