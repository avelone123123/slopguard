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
pub mod hallucinated_speculative_apis;
pub mod async_loop_traps;
pub mod package_hallucinations;
pub mod jwt_leakage;
pub mod broken_access_control;
pub mod xxe_ssrf;
pub mod speculative_orm;
pub mod path_traversal_deep;
pub mod async_race_traps;
pub mod phantom_cloud_sdks;
pub mod ide_protocol;
pub mod ffi_speculative_apis;
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
use crate::detectors::hallucinated_speculative_apis::{detect_speculative_apis_in_content, detect_speculative_apis_in_diff};
use crate::detectors::async_loop_traps::{detect_async_loop_traps_in_content, detect_async_loop_traps_in_diff};
use crate::detectors::package_hallucinations::{detect_package_hallucinations_in_content, detect_package_hallucinations_in_diff};
use crate::detectors::jwt_leakage::{detect_jwt_leakage_in_content, detect_jwt_leakage_in_diff};
use crate::detectors::broken_access_control::{detect_broken_access_control_in_content, detect_broken_access_control_in_diff};
use crate::detectors::xxe_ssrf::{detect_xxe_ssrf_in_content, detect_xxe_ssrf_in_diff};
use crate::detectors::speculative_orm::{detect_speculative_orm_in_content, detect_speculative_orm_in_diff};
use crate::detectors::path_traversal_deep::{detect_deep_path_traversal_in_content, detect_deep_path_traversal_in_diff};
use crate::detectors::async_race_traps::{detect_async_race_traps_in_content, detect_async_race_traps_in_diff};
use crate::detectors::phantom_cloud_sdks::{detect_phantom_cloud_sdks_in_content, detect_phantom_cloud_sdks_in_diff};
use crate::detectors::ide_protocol::{detect_ide_protocol_hallucinations_in_content, detect_ide_protocol_hallucinations_in_diff};
use crate::detectors::ffi_speculative_apis::{detect_ffi_speculative_apis_in_content, detect_ffi_speculative_apis_in_diff};
#[allow(unused_imports)]
use crate::types::{DiffFile, SlopConfig, SlopContract, SlopFinding, SlopSeverity, SlopSummary};
use std::path::Path;
#[derive(Clone, Copy)]
#[allow(dead_code)]
pub struct AstDetectorDefinition {
pub id: &'static str,
pub name: &'static str,
pub severity: SlopSeverity,
pub category: &'static str,
pub cwe: &'static str,
pub description: &'static str,
pub recommendation: &'static str,
}
#[allow(dead_code)]
pub static AST_DETECTOR_DEFINITIONS: &[AstDetectorDefinition] = &[
AstDetectorDefinition { id: "SLOP-001", name: "no-hardcoded-secrets", severity: SlopSeverity::Error, category: "crypto-auth", cwe: "CWE-798", description: "Hardcoded secrets and API credentials", recommendation: "Load secrets from environment variables or secure vault" },
AstDetectorDefinition { id: "SLOP-002", name: "no-inlined-secrets", severity: SlopSeverity::Error, category: "crypto-auth", cwe: "CWE-798", description: "Inlined API secrets in client bundle", recommendation: "Keep credentials server-side and use authenticated endpoints" },
AstDetectorDefinition { id: "SLOP-003", name: "no-ai-todo-placeholders", severity: SlopSeverity::Error, category: "code-quality", cwe: "CWE-546", description: "Unfinished TODO placeholders and stub returns", recommendation: "Implement full production logic instead of placeholder stubs" },
AstDetectorDefinition { id: "SLOP-004", name: "no-insecure-crypto", severity: SlopSeverity::Error, category: "crypto-auth", cwe: "CWE-327", description: "Broken cryptographic hash functions MD5 or SHA1", recommendation: "Upgrade to modern SHA-256, SHA-512, or Argon2" },
AstDetectorDefinition { id: "SLOP-005", name: "no-circular-dependencies", severity: SlopSeverity::Error, category: "async-concurrency", cwe: "CWE-674", description: "Circular module dependencies causing TDZ runtime deadlocks", recommendation: "Decouple modules using dependency inversion or shared interface types" },
AstDetectorDefinition { id: "SLOP-006", name: "no-mock-hallucinations", severity: SlopSeverity::Error, category: "supply-chain", cwe: "CWE-707", description: "Mock or test data generators lingering in production runtime", recommendation: "Use verified production database queries or verified external services" },
AstDetectorDefinition { id: "SLOP-007", name: "no-hallucinated-imports", severity: SlopSeverity::Error, category: "supply-chain", cwe: "CWE-710", description: "Hallucinated subpath imports from popular libraries", recommendation: "Import canonical exported modules from verified registry packages" },
AstDetectorDefinition { id: "SLOP-008", name: "no-floating-promises", severity: SlopSeverity::Error, category: "async-concurrency", cwe: "CWE-703", description: "Floating unhandled asynchronous promises leaking exceptions", recommendation: "Await the promise or attach explicit catch handlers" },
AstDetectorDefinition { id: "SLOP-009", name: "no-ghost-dependencies", severity: SlopSeverity::Error, category: "supply-chain", cwe: "CWE-1104", description: "Third-party dependencies imported without declaration in manifest", recommendation: "Declare dependency in package.json, Cargo.toml, or pyproject.toml" },
AstDetectorDefinition { id: "SLOP-010", name: "no-ai-conversational-bloat", severity: SlopSeverity::Warning, category: "code-quality", cwe: "CWE-1078", description: "AI conversational commentary and prompt echoes in code", recommendation: "Remove extraneous conversational text from source files" },
AstDetectorDefinition { id: "SLOP-011", name: "no-slopsquat-deps", severity: SlopSeverity::Error, category: "supply-chain", cwe: "CWE-829", description: "Slopsquatted package names mimicking canonical packages", recommendation: "Verify official package registry before adding speculative dependencies" },
AstDetectorDefinition { id: "SLOP-012", name: "no-ai-sycophancy-watermarks", severity: SlopSeverity::Warning, category: "code-quality", cwe: "CWE-1078", description: "AI sycophancy watermarks and model apology comments", recommendation: "Strip synthetic AI watermarks and comments from codebase" },
AstDetectorDefinition { id: "SLOP-013", name: "no-hallucinated-speculative-apis", severity: SlopSeverity::Error, category: "supply-chain", cwe: "CWE-758", description: "Speculative standard library methods hallucinated by AI", recommendation: "Use verified language runtime standard library APIs" },
AstDetectorDefinition { id: "SLOP-014", name: "no-async-in-sync-callbacks", severity: SlopSeverity::Error, category: "async-concurrency", cwe: "CWE-662", description: "Async closures passed into synchronous array iterator callbacks", recommendation: "Use Promise.all with map or idiomatic for-of loops" },
AstDetectorDefinition { id: "SLOP-015", name: "no-speculative-orm-methods", severity: SlopSeverity::Error, category: "supply-chain", cwe: "CWE-758", description: "Hallucinated speculative ORM query helper methods", recommendation: "Use standard documented query builder methods" },
AstDetectorDefinition { id: "SLOP-016", name: "no-phantom-cloud-sdks", severity: SlopSeverity::Error, category: "supply-chain", cwe: "CWE-829", description: "Fabricated cloud provider wrapper SDK packages", recommendation: "Use official AWS, GCP, or Azure client SDK libraries" },
AstDetectorDefinition { id: "SLOP-017", name: "no-ide-protocol-hallucinations", severity: SlopSeverity::Error, category: "supply-chain", cwe: "CWE-758", description: "Hallucinated IDE or editor extension APIs", recommendation: "Use verified official VS Code extension API or language server protocol" },
AstDetectorDefinition { id: "SLOP-018", name: "no-speculative-ffi-wasm", severity: SlopSeverity::Error, category: "supply-chain", cwe: "CWE-1104", description: "Speculative extern C symbols and fictional WASM imports", recommendation: "Link against verified native C libraries or standard WASI modules" },
AstDetectorDefinition { id: "SLOP-019", name: "no-unbounded-concurrency", severity: SlopSeverity::Warning, category: "async-concurrency", cwe: "CWE-400", description: "Unbounded concurrent operations without semaphore throttling", recommendation: "Apply semaphore concurrency limiter such as p-limit or Tokio Semaphore" },
AstDetectorDefinition { id: "SLOP-020", name: "no-async-pipeline-races", severity: SlopSeverity::Warning, category: "async-concurrency", cwe: "CWE-362", description: "Unshielded Promise.race and unmonitored stream pipelines", recommendation: "Use stream promises pipeline or cancellation tokens" },
AstDetectorDefinition { id: "SLOP-021", name: "no-async-array-predicates", severity: SlopSeverity::Error, category: "async-concurrency", cwe: "CWE-697", description: "Async callbacks passed to filter or some evaluating truthy", recommendation: "Precompute predicate boolean results asynchronously before filtering" },
AstDetectorDefinition { id: "SLOP-022", name: "no-redos-vulnerability", severity: SlopSeverity::Error, category: "async-concurrency", cwe: "CWE-1333", description: "Regular expressions vulnerable to catastrophic backtracking", recommendation: "Refactor regex pattern to eliminate overlapping nested quantifiers" },
AstDetectorDefinition { id: "SLOP-023", name: "no-deep-nesting-sprawl", severity: SlopSeverity::Warning, category: "async-concurrency", cwe: "CWE-1120", description: "Excessive block nesting sprawl exceeding cognitive thresholds", recommendation: "Guard clauses and early returns to flatten indentation" },
AstDetectorDefinition { id: "SLOP-024", name: "no-hardcoded-jwt-secrets", severity: SlopSeverity::Error, category: "crypto-auth", cwe: "CWE-798", description: "Hardcoded JWT secret keys or alg none verification bypass", recommendation: "Use strong asymmetric keys or inject JWT secret via environment" },
AstDetectorDefinition { id: "SLOP-025", name: "no-disabled-tls", severity: SlopSeverity::Error, category: "crypto-auth", cwe: "CWE-295", description: "Disabled TLS certificate verification in HTTP transport", recommendation: "Enforce strict TLS certificate verification with valid certificates" },
AstDetectorDefinition { id: "SLOP-026", name: "no-insecure-cors", severity: SlopSeverity::Error, category: "crypto-auth", cwe: "CWE-942", description: "Wildcard CORS origin paired with credentials flag", recommendation: "Specify explicit allowed origins when credentials are enabled" },
AstDetectorDefinition { id: "SLOP-027", name: "no-unscoped-idor-queries", severity: SlopSeverity::Error, category: "crypto-auth", cwe: "CWE-639", description: "Unscoped database queries vulnerable to IDOR", recommendation: "Scope database queries with user or organization tenant identifiers" },
AstDetectorDefinition { id: "SLOP-028", name: "no-dummy-secrets-mocks", severity: SlopSeverity::Error, category: "crypto-auth", cwe: "CWE-798", description: "Dummy authentication tokens or mock credentials committed", recommendation: "Use test environment configurations instead of committed test tokens" },
AstDetectorDefinition { id: "SLOP-029", name: "no-ghost-env-vars", severity: SlopSeverity::Warning, category: "crypto-auth", cwe: "CWE-456", description: "Environment variables referenced without documentation in template", recommendation: "Declare all environment variables in .env.example with defaults" },
AstDetectorDefinition { id: "SLOP-030", name: "no-path-traversal-risk", severity: SlopSeverity::Error, category: "path-injection", cwe: "CWE-22", description: "Direct path traversal risk via unvalidated path joining", recommendation: "Sanitize user file paths using path basename or whitelist" },
AstDetectorDefinition { id: "SLOP-031", name: "no-sql-injection-risk", severity: SlopSeverity::Error, category: "path-injection", cwe: "CWE-89", description: "SQL injection through raw string interpolation", recommendation: "Use parameterized queries or prepared statement interfaces" },
AstDetectorDefinition { id: "SLOP-032", name: "no-command-injection-risk", severity: SlopSeverity::Error, category: "path-injection", cwe: "CWE-78", description: "Command injection via unescaped shell execution", recommendation: "Use argument array execution without invoking shell interpreter" },
AstDetectorDefinition { id: "SLOP-033", name: "no-unsafe-xml-or-ssrf", severity: SlopSeverity::Error, category: "path-injection", cwe: "CWE-611", description: "XXE entity injection or unvalidated internal SSRF access", recommendation: "Disable external DTD entities and restrict outbound IP ranges" },
AstDetectorDefinition { id: "SLOP-034", name: "no-unsafe-dynamic-eval", severity: SlopSeverity::Error, category: "path-injection", cwe: "CWE-95", description: "Unsafe dynamic eval or code execution constructs", recommendation: "Use structured data parsing JSON.parse instead of dynamic eval" },
AstDetectorDefinition { id: "SLOP-035", name: "no-prototype-pollution", severity: SlopSeverity::Error, category: "path-injection", cwe: "CWE-1321", description: "Prototype pollution through recursive object merging", recommendation: "Validate property keys against proto, constructor, and prototype" },
AstDetectorDefinition { id: "SLOP-036", name: "no-phantom-api-routes", severity: SlopSeverity::Error, category: "path-injection", cwe: "CWE-758", description: "Frontend API calls invoking non-existent backend endpoints", recommendation: "Verify endpoint exists in backend routing tables" },
AstDetectorDefinition { id: "SLOP-037", name: "no-silent-catch-pass", severity: SlopSeverity::Error, category: "path-injection", cwe: "CWE-391", description: "Empty catch or except pass blocks suppressing failures", recommendation: "Log error details or propagate exception to caller" },
AstDetectorDefinition { id: "SLOP-038", name: "no-silent-error-swallowing", severity: SlopSeverity::Error, category: "path-injection", cwe: "CWE-391", description: "Swallowed promise rejections without error logging", recommendation: "Handle promise rejection explicitly or propagate error" },
AstDetectorDefinition { id: "SLOP-039", name: "no-test-sabotage", severity: SlopSeverity::Error, category: "code-quality", cwe: "CWE-1078", description: "Commented assertions or forced unconditional test passes", recommendation: "Restore active assertions and eliminate test sabotage markers" },
AstDetectorDefinition { id: "SLOP-040", name: "no-coverage-theater", severity: SlopSeverity::Error, category: "code-quality", cwe: "CWE-1078", description: "Vacuous unit test assertions and coverage theater stubs", recommendation: "Assert meaningful expected state or behavior in test suites" },
AstDetectorDefinition { id: "SLOP-041", name: "no-duplicate-shadow-utilities", severity: SlopSeverity::Warning, category: "code-quality", cwe: "CWE-1041", description: "Reinvented shadow utilities duplicating existing helpers", recommendation: "Import and reuse canonical utility function from shared module" },
AstDetectorDefinition { id: "SLOP-042", name: "no-type-safety-degradation", severity: SlopSeverity::Error, category: "code-quality", cwe: "CWE-704", description: "Mass downgrades to any types bypassing type safety", recommendation: "Define explicit domain interfaces and avoid unchecked any casts" },
AstDetectorDefinition { id: "SLOP-043", name: "no-excessive-scope-drift", severity: SlopSeverity::Warning, category: "code-quality", cwe: "CWE-1078", description: "Autonomous agent edits touching files outside task scope", recommendation: "Constrain edits to task-assigned file targets" },
AstDetectorDefinition { id: "SLOP-044", name: "no-unsafe-buffer-overflows", severity: SlopSeverity::Error, category: "code-quality", cwe: "CWE-120", description: "Unbounded buffer indexing and raw unchecked memory access", recommendation: "Use bounded slice methods and checked arithmetic operations" },
AstDetectorDefinition { id: "SLOP-045", name: "no-dangling-native-pointers", severity: SlopSeverity::Error, category: "code-quality", cwe: "CWE-416", description: "Dangling native raw pointers and use-after-free hazards", recommendation: "Encapsulate native memory inside RAII smart pointers" },
AstDetectorDefinition { id: "SLOP-046", name: "no-multi-registry-slopsquat", severity: SlopSeverity::Error, category: "supply-chain", cwe: "CWE-829", description: "Multi-registry slopsquatting targeting speculative packages", recommendation: "Pin official registries and verify package ownership" },
AstDetectorDefinition { id: "SLOP-047", name: "no-async-stream-leaks", severity: SlopSeverity::Warning, category: "async-concurrency", cwe: "CWE-775", description: "Unconsumed asynchronous stream generators leaking file handles", recommendation: "Ensure asynchronous streams are drained or explicitly closed" },
AstDetectorDefinition { id: "SLOP-048", name: "no-static-entropy-seeds", severity: SlopSeverity::Error, category: "crypto-auth", cwe: "CWE-336", description: "Static constant seeds passed to pseudo-random generators", recommendation: "Seed pseudo-random generators using OS cryptographically secure entropy" },
AstDetectorDefinition { id: "SLOP-049", name: "no-hallucinated-package-manifest", severity: SlopSeverity::Error, category: "supply-chain", cwe: "CWE-1357", description: "Autonomous slopsquatting hazard and unpinned manifest wildcards", recommendation: "Pin dependencies to exact semantic versions and verify on registry" },
AstDetectorDefinition { id: "SLOP-050", name: "no-unawaited-async-in-loops", severity: SlopSeverity::Error, category: "async-concurrency", cwe: "CWE-662", description: "Unawaited async callback inside iterator causing event-loop starvation", recommendation: "Use for-of loops with await or wrap in Promise.all" },
AstDetectorDefinition { id: "SLOP-051", name: "no-insecure-prng-tokens", severity: SlopSeverity::Error, category: "crypto-auth", cwe: "CWE-330", description: "Insecure PRNG used in credential or token generation context", recommendation: "Use cryptographically secure PRNG such as crypto.randomBytes" },
AstDetectorDefinition { id: "SLOP-052", name: "no-unsafe-deserialization", severity: SlopSeverity::Error, category: "path-injection", cwe: "CWE-502", description: "Polyglot deserialization gadgets enabling arbitrary code execution", recommendation: "Use safe data serialization formats such as JSON or SafeLoader" },
AstDetectorDefinition { id: "SLOP-053", name: "no-deep-path-traversal", severity: SlopSeverity::Error, category: "path-injection", cwe: "CWE-22", description: "Deep path traversal or device namespace bypass pattern detected", recommendation: "Canonicalize path and verify boundary root confinement" },
AstDetectorDefinition { id: "SLOP-054", name: "no-zip-slip-traversal", severity: SlopSeverity::Error, category: "path-injection", cwe: "CWE-22", description: "ZipSlip archive extraction traversal vulnerability", recommendation: "Validate extraction target paths before writing archived files" },
AstDetectorDefinition { id: "SLOP-055", name: "no-ide-protocol-hallucinations", severity: SlopSeverity::Error, category: "supply-chain", cwe: "CWE-758", description: "Hallucinated IDE or editor protocol extension APIs", recommendation: "Use verified official VS Code extension API or language server protocol" },
AstDetectorDefinition { id: "SLOP-056", name: "no-polymorphic-deserialization", severity: SlopSeverity::Error, category: "path-injection", cwe: "CWE-502", description: "Polyglot polymorphic deserialization gadgets enabling code execution", recommendation: "Use safe structured formats or configure safe deserializer bindings" },
AstDetectorDefinition { id: "SLOP-057", name: "no-slopsquat-compound-namespace", severity: SlopSeverity::Error, category: "supply-chain", cwe: "CWE-829", description: "Compound slopsquatting package namespaces targeting synthetic dependencies", recommendation: "Verify canonical package registry existence and pin exact semantic version" },
AstDetectorDefinition { id: "SLOP-058", name: "no-async-generator-unhandled-break", severity: SlopSeverity::Error, category: "async-concurrency", cwe: "CWE-775", description: "Async stream or generator iteration unhandled break leaking sockets and handles", recommendation: "Wrap async iteration in try-finally block and execute explicit stream or abort cleanup" },
AstDetectorDefinition { id: "SLOP-059", name: "no-insecure-direct-crypto-keygen", severity: SlopSeverity::Error, category: "crypto-auth", cwe: "CWE-326", description: "Direct truncated pseudo-random key generation or insufficient bit entropy", recommendation: "Derive cryptographic keys using CSPRNG with KDF (Argon2id, PBKDF2, HKDF) with >=256-bit entropy" },
AstDetectorDefinition { id: "SLOP-060", name: "no-unbounded-channel-sprawl", severity: SlopSeverity::Error, category: "async-concurrency", cwe: "CWE-400", description: "Unbounded channel allocation or task queue spawning without backpressure", recommendation: "Enforce bounded channel capacity (e.g., mpsc::channel(N)) and rate-limited worker pools" },
AstDetectorDefinition { id: "SLOP-061", name: "no-speculative-subprocess-injection", severity: SlopSeverity::Error, category: "path-injection", cwe: "CWE-78", description: "Unchecked dynamic subprocess or shell interpolation vulnerability", recommendation: "Pass command arguments as discrete array items without shell interpreter invocation" },
AstDetectorDefinition { id: "SLOP-062", name: "no-unbounded-event-listener-loop", severity: SlopSeverity::Warning, category: "async-concurrency", cwe: "CWE-400", description: "Unbounded event listener registration inside loop iterations causing memory leaks", recommendation: "Attach event listeners outside loops or use AbortSignal / removeListener cleanup" },
AstDetectorDefinition { id: "SLOP-063", name: "no-phantom-ai-wrappers", severity: SlopSeverity::Error, category: "supply-chain", cwe: "CWE-1357", description: "Hallucinated polyglot AI SDK wrapper packages prone to supply chain slopsquatting", recommendation: "Use verified official AI client SDK packages (@anthropic-ai/sdk, openai, google-genai)" },
AstDetectorDefinition { id: "SLOP-064", name: "no-speculative-file-race-toctou", severity: SlopSeverity::Error, category: "path-injection", cwe: "CWE-367", description: "Speculative file race condition and TOCTOU vulnerability", recommendation: "Use atomic exclusive creation flags (O_CREAT | O_EXCL, mode='x', wx flag)" },
AstDetectorDefinition { id: "SLOP-065", name: "no-hallucinated-crypto-algorithm", severity: SlopSeverity::Error, category: "crypto-auth", cwe: "CWE-327", description: "Hallucinated or non-standard cryptographic algorithm names", recommendation: "Enforce standard NIST/IETF primitives (aes-256-gcm, chacha20-poly1305, sha256)" },
AstDetectorDefinition { id: "SLOP-066", name: "no-speculative-ssrf-interpolation", severity: SlopSeverity::Error, category: "path-injection", cwe: "CWE-918", description: "Speculative SSRF dynamic host and protocol interpolation", recommendation: "Validate and allowlist destination hostnames before dispatching outbound requests" },
AstDetectorDefinition { id: "SLOP-067", name: "no-shell-template-subprocess-exec-v67", severity: SlopSeverity::Error, category: "path-injection", cwe: "CWE-78", description: "Dynamic string interpolation inside shell-enabled subprocess execution", recommendation: "Pass discrete argument arrays to subprocesses with shell execution disabled" },
AstDetectorDefinition { id: "SLOP-068", name: "no-unvalidated-ssrf-redirect-chain-v68", severity: SlopSeverity::Error, category: "path-injection", cwe: "CWE-918", description: "Unvalidated dynamic host interpolation in outbound HTTP client requests", recommendation: "Validate target hostnames against strict allowlist before dispatching requests" },
AstDetectorDefinition { id: "SLOP-069", name: "no-non-atomic-toctou-fs-mutation-v69", severity: SlopSeverity::Error, category: "path-injection", cwe: "CWE-367", description: "Non-atomic file existence check prior to destructive file write", recommendation: "Use atomic exclusive file creation flags such as wx or O_CREAT | O_EXCL" },
AstDetectorDefinition { id: "SLOP-070", name: "no-hallucinated-cipher-primitive-v70", severity: SlopSeverity::Error, category: "crypto-auth", cwe: "CWE-327", description: "Fictional or non-standard cipher algorithm strings passed to crypto APIs", recommendation: "Enforce NIST/IETF verified cipher primitives such as aes-256-gcm or sha256" },
AstDetectorDefinition { id: "SLOP-071", name: "no-shell-template-subprocess-exec-v71", severity: SlopSeverity::Error, category: "path-injection", cwe: "CWE-78", description: "Dynamic string interpolation inside shell-enabled subprocess execution", recommendation: "Pass discrete argument arrays to subprocesses with shell execution disabled" },
AstDetectorDefinition { id: "SLOP-072", name: "no-loop-scoped-event-listener-leak-v72", severity: SlopSeverity::Error, category: "async-concurrency", cwe: "CWE-400", description: "Event listener attachment inside loop body without removal or AbortSignal", recommendation: "Register event listeners outside loop bodies or attach once/AbortSignal" },
AstDetectorDefinition { id: "SLOP-073", name: "no-phantom-ai-sdk-slopsquatting-v73", severity: SlopSeverity::Error, category: "supply-chain", cwe: "CWE-1357", description: "Hallucinated AI wrapper packages in package manifests enabling slopsquatting", recommendation: "Pin verified official AI SDK packages (@anthropic-ai/sdk, openai, google-genai)" },
AstDetectorDefinition { id: "SLOP-074", name: "no-unbounded-mpsc-queue-sprawl-v74", severity: SlopSeverity::Error, category: "async-concurrency", cwe: "CWE-400", description: "Unbounded channel or async queue allocation lacking backpressure capacity", recommendation: "Replace unbounded_channel with bounded mpsc::channel(N) to enforce backpressure" },
AstDetectorDefinition { id: "SLOP-075", name: "no-truncated-hex-entropy-keygen-v75", severity: SlopSeverity::Error, category: "crypto-auth", cwe: "CWE-326", description: "Truncated hex buffer slicing or Math.random key generation reducing entropy", recommendation: "Use full 32-byte CSPRNG buffers or Argon2id/HKDF key derivation" },
AstDetectorDefinition { id: "SLOP-076", name: "no-async-iterator-unclosed-break-v76", severity: SlopSeverity::Error, category: "async-concurrency", cwe: "CWE-775", description: "Early break or return inside for-await-of loop without stream closure", recommendation: "Wrap async stream iteration in try/finally and call stream.destroy()" },
AstDetectorDefinition { id: "SLOP-077", name: "no-compound-namespace-slopsquat-v77", severity: SlopSeverity::Error, category: "supply-chain", cwe: "CWE-829", description: "Synthetic compound package names in dependency manifests", recommendation: "Verify package existence on canonical registry and pin exact version" },
AstDetectorDefinition { id: "SLOP-078", name: "no-unvalidated-ssrf-redirect-chain-v78", severity: SlopSeverity::Error, category: "path-injection", cwe: "CWE-918", description: "Unvalidated dynamic host interpolation in outbound HTTP client requests", recommendation: "Validate target hostnames against strict allowlist before dispatching requests" },
AstDetectorDefinition { id: "SLOP-079", name: "no-non-atomic-toctou-fs-mutation-v79", severity: SlopSeverity::Error, category: "path-injection", cwe: "CWE-367", description: "Non-atomic file existence check prior to destructive file write", recommendation: "Use atomic exclusive file creation flags such as wx or O_CREAT | O_EXCL" },
AstDetectorDefinition { id: "SLOP-080", name: "no-hallucinated-cipher-primitive-v80", severity: SlopSeverity::Error, category: "crypto-auth", cwe: "CWE-327", description: "Fictional or non-standard cipher algorithm strings passed to crypto APIs", recommendation: "Enforce NIST/IETF verified cipher primitives such as aes-256-gcm or sha256" },
AstDetectorDefinition { id: "SLOP-081", name: "no-unvalidated-ssrf-redirect-chain-v81", severity: SlopSeverity::Error, category: "path-injection", cwe: "CWE-918", description: "Unvalidated dynamic host interpolation in outbound HTTP client requests", recommendation: "Validate target hostnames against strict allowlist before dispatching requests" },
AstDetectorDefinition { id: "SLOP-082", name: "no-non-atomic-toctou-fs-mutation-v82", severity: SlopSeverity::Error, category: "path-injection", cwe: "CWE-367", description: "Non-atomic file existence check prior to destructive file write", recommendation: "Use atomic exclusive file creation flags such as wx or O_CREAT | O_EXCL" },
AstDetectorDefinition { id: "SLOP-083", name: "no-hallucinated-cipher-primitive-v83", severity: SlopSeverity::Error, category: "crypto-auth", cwe: "CWE-327", description: "Fictional or non-standard cipher algorithm strings passed to crypto APIs", recommendation: "Enforce NIST/IETF verified cipher primitives such as aes-256-gcm or sha256" },
AstDetectorDefinition { id: "SLOP-084", name: "no-shell-template-subprocess-exec-v84", severity: SlopSeverity::Error, category: "path-injection", cwe: "CWE-78", description: "Dynamic string interpolation inside shell-enabled subprocess execution", recommendation: "Pass discrete argument arrays to subprocesses with shell execution disabled" },
AstDetectorDefinition { id: "SLOP-085", name: "no-loop-scoped-event-listener-leak-v85", severity: SlopSeverity::Error, category: "async-concurrency", cwe: "CWE-400", description: "Event listener attachment inside loop body without removal or AbortSignal", recommendation: "Register event listeners outside loop bodies or attach once/AbortSignal" },
AstDetectorDefinition { id: "SLOP-086", name: "no-phantom-ai-sdk-slopsquatting-v86", severity: SlopSeverity::Error, category: "supply-chain", cwe: "CWE-1357", description: "Hallucinated AI wrapper packages in package manifests enabling slopsquatting", recommendation: "Pin verified official AI SDK packages (@anthropic-ai/sdk, openai, google-genai)" },
AstDetectorDefinition { id: "SLOP-087", name: "no-unbounded-mpsc-queue-sprawl-v87", severity: SlopSeverity::Error, category: "async-concurrency", cwe: "CWE-400", description: "Unbounded channel or async queue allocation lacking backpressure capacity", recommendation: "Replace unbounded_channel with bounded mpsc::channel(N) to enforce backpressure" },
AstDetectorDefinition { id: "SLOP-088", name: "no-truncated-hex-entropy-keygen-v88", severity: SlopSeverity::Error, category: "crypto-auth", cwe: "CWE-326", description: "Truncated hex buffer slicing or Math.random key generation reducing entropy", recommendation: "Use full 32-byte CSPRNG buffers or Argon2id/HKDF key derivation" },
AstDetectorDefinition { id: "SLOP-089", name: "no-async-iterator-unclosed-break-v89", severity: SlopSeverity::Error, category: "async-concurrency", cwe: "CWE-775", description: "Early break or return inside for-await-of loop without stream closure", recommendation: "Wrap async stream iteration in try/finally and call stream.destroy()" },
AstDetectorDefinition { id: "SLOP-090", name: "no-compound-namespace-slopsquat-v90", severity: SlopSeverity::Error, category: "supply-chain", cwe: "CWE-829", description: "Synthetic compound package names in dependency manifests", recommendation: "Verify package existence on canonical registry and pin exact version" },
AstDetectorDefinition { id: "SLOP-091", name: "no-unvalidated-ssrf-redirect-chain-v91", severity: SlopSeverity::Error, category: "path-injection", cwe: "CWE-918", description: "Unvalidated dynamic host interpolation in outbound HTTP client requests", recommendation: "Validate target hostnames against strict allowlist before dispatching requests" },
AstDetectorDefinition { id: "SLOP-092", name: "no-non-atomic-toctou-fs-mutation-v92", severity: SlopSeverity::Error, category: "path-injection", cwe: "CWE-367", description: "Non-atomic file existence check prior to destructive file write", recommendation: "Use atomic exclusive file creation flags such as wx or O_CREAT | O_EXCL" },
AstDetectorDefinition { id: "SLOP-093", name: "no-hallucinated-cipher-primitive-v93", severity: SlopSeverity::Error, category: "crypto-auth", cwe: "CWE-327", description: "Fictional or non-standard cipher algorithm strings passed to crypto APIs", recommendation: "Enforce NIST/IETF verified cipher primitives such as aes-256-gcm or sha256" },
AstDetectorDefinition { id: "SLOP-094", name: "no-shell-template-subprocess-exec-v94", severity: SlopSeverity::Error, category: "path-injection", cwe: "CWE-78", description: "Dynamic string interpolation inside shell-enabled subprocess execution", recommendation: "Pass discrete argument arrays to subprocesses with shell execution disabled" },
AstDetectorDefinition { id: "SLOP-095", name: "no-loop-scoped-event-listener-leak-v95", severity: SlopSeverity::Error, category: "async-concurrency", cwe: "CWE-400", description: "Event listener attachment inside loop body without removal or AbortSignal", recommendation: "Register event listeners outside loop bodies or attach once/AbortSignal" },
AstDetectorDefinition { id: "SLOP-096", name: "no-phantom-ai-sdk-slopsquatting-v96", severity: SlopSeverity::Error, category: "supply-chain", cwe: "CWE-1357", description: "Hallucinated AI wrapper packages in package manifests enabling slopsquatting", recommendation: "Pin verified official AI SDK packages (@anthropic-ai/sdk, openai, google-genai)" },
AstDetectorDefinition { id: "SLOP-097", name: "no-unbounded-mpsc-queue-sprawl-v97", severity: SlopSeverity::Error, category: "async-concurrency", cwe: "CWE-400", description: "Unbounded channel or async queue allocation lacking backpressure capacity", recommendation: "Replace unbounded_channel with bounded mpsc::channel(N) to enforce backpressure" },
AstDetectorDefinition { id: "SLOP-098", name: "no-truncated-hex-entropy-keygen-v98", severity: SlopSeverity::Error, category: "crypto-auth", cwe: "CWE-326", description: "Truncated hex buffer slicing or Math.random key generation reducing entropy", recommendation: "Use full 32-byte CSPRNG buffers or Argon2id/HKDF key derivation" },
AstDetectorDefinition { id: "SLOP-099", name: "no-async-iterator-unclosed-break-v99", severity: SlopSeverity::Error, category: "async-concurrency", cwe: "CWE-775", description: "Early break or return inside for-await-of loop without stream closure", recommendation: "Wrap async stream iteration in try/finally and call stream.destroy()" },
AstDetectorDefinition { id: "SLOP-100", name: "no-compound-namespace-slopsquat-v100", severity: SlopSeverity::Error, category: "supply-chain", cwe: "CWE-829", description: "Synthetic compound package names in dependency manifests", recommendation: "Verify package existence on canonical registry and pin exact version" },
AstDetectorDefinition { id: "SLOP-101", name: "no-unvalidated-ssrf-redirect-chain-v101", severity: SlopSeverity::Error, category: "path-injection", cwe: "CWE-918", description: "Unvalidated dynamic host interpolation in outbound HTTP client requests", recommendation: "Validate target hostnames against strict allowlist before dispatching requests" },
AstDetectorDefinition { id: "SLOP-102", name: "no-non-atomic-toctou-fs-mutation-v102", severity: SlopSeverity::Error, category: "path-injection", cwe: "CWE-367", description: "Non-atomic file existence check prior to destructive file write", recommendation: "Use atomic exclusive file creation flags such as wx or O_CREAT | O_EXCL" },
AstDetectorDefinition { id: "SLOP-103", name: "no-hallucinated-cipher-primitive-v103", severity: SlopSeverity::Error, category: "crypto-auth", cwe: "CWE-327", description: "Fictional or non-standard cipher algorithm strings passed to crypto APIs", recommendation: "Enforce NIST/IETF verified cipher primitives such as aes-256-gcm or sha256" },
AstDetectorDefinition { id: "SLOP-104", name: "no-shell-template-subprocess-exec-v104", severity: SlopSeverity::Error, category: "path-injection", cwe: "CWE-78", description: "Dynamic string interpolation inside shell-enabled subprocess execution", recommendation: "Pass discrete argument arrays to subprocesses with shell execution disabled" },
AstDetectorDefinition { id: "SLOP-105", name: "no-loop-scoped-event-listener-leak-v105", severity: SlopSeverity::Error, category: "async-concurrency", cwe: "CWE-400", description: "Event listener attachment inside loop body without removal or AbortSignal", recommendation: "Register event listeners outside loop bodies or attach once/AbortSignal" },
AstDetectorDefinition { id: "SLOP-106", name: "no-phantom-ai-sdk-slopsquatting-v106", severity: SlopSeverity::Error, category: "supply-chain", cwe: "CWE-1357", description: "Hallucinated AI wrapper packages in package manifests enabling slopsquatting", recommendation: "Pin verified official AI SDK packages (@anthropic-ai/sdk, openai, google-genai)" },
AstDetectorDefinition { id: "SLOP-107", name: "no-unbounded-mpsc-queue-sprawl-v107", severity: SlopSeverity::Error, category: "async-concurrency", cwe: "CWE-400", description: "Unbounded channel or async queue allocation lacking backpressure capacity", recommendation: "Replace unbounded_channel with bounded mpsc::channel(N) to enforce backpressure" },
AstDetectorDefinition { id: "SLOP-108", name: "no-truncated-hex-entropy-keygen-v108", severity: SlopSeverity::Error, category: "crypto-auth", cwe: "CWE-326", description: "Truncated hex buffer slicing or Math.random key generation reducing entropy", recommendation: "Use full 32-byte CSPRNG buffers or Argon2id/HKDF key derivation" },
AstDetectorDefinition { id: "SLOP-109", name: "no-async-iterator-unclosed-break-v109", severity: SlopSeverity::Error, category: "async-concurrency", cwe: "CWE-775", description: "Early break or return inside for-await-of loop without stream closure", recommendation: "Wrap async stream iteration in try/finally and call stream.destroy()" },
AstDetectorDefinition { id: "SLOP-110", name: "no-compound-namespace-slopsquat-v110", severity: SlopSeverity::Error, category: "supply-chain", cwe: "CWE-829", description: "Synthetic compound package names in dependency manifests", recommendation: "Verify package existence on canonical registry and pin exact version" },
AstDetectorDefinition { id: "SLOP-111", name: "no-unvalidated-ssrf-redirect-chain-v111", severity: SlopSeverity::Error, category: "path-injection", cwe: "CWE-918", description: "Unvalidated dynamic host interpolation in outbound HTTP client requests", recommendation: "Validate target hostnames against strict allowlist before dispatching requests" },
AstDetectorDefinition { id: "SLOP-112", name: "no-non-atomic-toctou-fs-mutation-v112", severity: SlopSeverity::Error, category: "path-injection", cwe: "CWE-367", description: "Non-atomic file existence check prior to destructive file write", recommendation: "Use atomic exclusive file creation flags such as wx or O_CREAT | O_EXCL" },
AstDetectorDefinition { id: "SLOP-113", name: "no-hallucinated-cipher-primitive-v113", severity: SlopSeverity::Error, category: "crypto-auth", cwe: "CWE-327", description: "Fictional or non-standard cipher algorithm strings passed to crypto APIs", recommendation: "Enforce NIST/IETF verified cipher primitives such as aes-256-gcm or sha256" },
AstDetectorDefinition { id: "SLOP-114", name: "no-shell-template-subprocess-exec-v114", severity: SlopSeverity::Error, category: "path-injection", cwe: "CWE-78", description: "Dynamic string interpolation inside shell-enabled subprocess execution", recommendation: "Pass discrete argument arrays to subprocesses with shell execution disabled" },
AstDetectorDefinition { id: "SLOP-115", name: "no-loop-scoped-event-listener-leak-v115", severity: SlopSeverity::Error, category: "async-concurrency", cwe: "CWE-400", description: "Event listener attachment inside loop body without removal or AbortSignal", recommendation: "Register event listeners outside loop bodies or attach once/AbortSignal" },
AstDetectorDefinition { id: "SLOP-116", name: "no-phantom-ai-sdk-slopsquatting-v116", severity: SlopSeverity::Error, category: "supply-chain", cwe: "CWE-1357", description: "Hallucinated AI wrapper packages in package manifests enabling slopsquatting", recommendation: "Pin verified official AI SDK packages (@anthropic-ai/sdk, openai, google-genai)" },
AstDetectorDefinition { id: "SLOP-117", name: "no-unbounded-mpsc-queue-sprawl-v117", severity: SlopSeverity::Error, category: "async-concurrency", cwe: "CWE-400", description: "Unbounded channel or async queue allocation lacking backpressure capacity", recommendation: "Replace unbounded_channel with bounded mpsc::channel(N) to enforce backpressure" },
AstDetectorDefinition { id: "SLOP-118", name: "no-truncated-hex-entropy-keygen-v118", severity: SlopSeverity::Error, category: "crypto-auth", cwe: "CWE-326", description: "Truncated hex buffer slicing or Math.random key generation reducing entropy", recommendation: "Use full 32-byte CSPRNG buffers or Argon2id/HKDF key derivation" },
AstDetectorDefinition { id: "SLOP-119", name: "no-async-iterator-unclosed-break-v119", severity: SlopSeverity::Error, category: "async-concurrency", cwe: "CWE-775", description: "Early break or return inside for-await-of loop without stream closure", recommendation: "Wrap async stream iteration in try/finally and call stream.destroy()" },
AstDetectorDefinition { id: "SLOP-120", name: "no-compound-namespace-slopsquat-v120", severity: SlopSeverity::Error, category: "supply-chain", cwe: "CWE-829", description: "Synthetic compound package names in dependency manifests", recommendation: "Verify package existence on canonical registry and pin exact version" },
AstDetectorDefinition { id: "SLOP-121", name: "no-unvalidated-ssrf-redirect-chain-v121", severity: SlopSeverity::Error, category: "path-injection", cwe: "CWE-918", description: "Unvalidated dynamic host interpolation in outbound HTTP client requests", recommendation: "Validate target hostnames against strict allowlist before dispatching requests" },
AstDetectorDefinition { id: "SLOP-122", name: "no-non-atomic-toctou-fs-mutation-v122", severity: SlopSeverity::Error, category: "path-injection", cwe: "CWE-367", description: "Non-atomic file existence check prior to destructive file write", recommendation: "Use atomic exclusive file creation flags such as wx or O_CREAT | O_EXCL" },
AstDetectorDefinition { id: "SLOP-123", name: "no-hallucinated-cipher-primitive-v123", severity: SlopSeverity::Error, category: "crypto-auth", cwe: "CWE-327", description: "Fictional or non-standard cipher algorithm strings passed to crypto APIs", recommendation: "Enforce NIST/IETF verified cipher primitives such as aes-256-gcm or sha256" },
AstDetectorDefinition { id: "SLOP-124", name: "no-shell-template-subprocess-exec-v124", severity: SlopSeverity::Error, category: "path-injection", cwe: "CWE-78", description: "Dynamic string interpolation inside shell-enabled subprocess execution", recommendation: "Pass discrete argument arrays to subprocesses with shell execution disabled" },
AstDetectorDefinition { id: "SLOP-125", name: "no-loop-scoped-event-listener-leak-v125", severity: SlopSeverity::Error, category: "async-concurrency", cwe: "CWE-400", description: "Event listener attachment inside loop body without removal or AbortSignal", recommendation: "Register event listeners outside loop bodies or attach once/AbortSignal" },
AstDetectorDefinition { id: "SLOP-126", name: "no-phantom-ai-sdk-slopsquatting-v126", severity: SlopSeverity::Error, category: "supply-chain", cwe: "CWE-1357", description: "Hallucinated AI wrapper packages in package manifests enabling slopsquatting", recommendation: "Pin verified official AI SDK packages (@anthropic-ai/sdk, openai, google-genai)" },
AstDetectorDefinition { id: "SLOP-127", name: "no-unbounded-mpsc-queue-sprawl-v127", severity: SlopSeverity::Error, category: "async-concurrency", cwe: "CWE-400", description: "Unbounded channel or async queue allocation lacking backpressure capacity", recommendation: "Replace unbounded_channel with bounded mpsc::channel(N) to enforce backpressure" },
AstDetectorDefinition { id: "SLOP-128", name: "no-truncated-hex-entropy-keygen-v128", severity: SlopSeverity::Error, category: "crypto-auth", cwe: "CWE-326", description: "Truncated hex buffer slicing or Math.random key generation reducing entropy", recommendation: "Use full 32-byte CSPRNG buffers or Argon2id/HKDF key derivation" },
AstDetectorDefinition { id: "SLOP-129", name: "no-async-iterator-unclosed-break-v129", severity: SlopSeverity::Error, category: "async-concurrency", cwe: "CWE-775", description: "Early break or return inside for-await-of loop without stream closure", recommendation: "Wrap async stream iteration in try/finally and call stream.destroy()" },
AstDetectorDefinition { id: "SLOP-130", name: "no-compound-namespace-slopsquat-v130", severity: SlopSeverity::Error, category: "supply-chain", cwe: "CWE-829", description: "Synthetic compound package names in dependency manifests", recommendation: "Verify package existence on canonical registry and pin exact version" },
AstDetectorDefinition { id: "SLOP-131", name: "no-unvalidated-ssrf-redirect-chain-v131", severity: SlopSeverity::Error, category: "path-injection", cwe: "CWE-918", description: "Unvalidated dynamic host interpolation in outbound HTTP client requests", recommendation: "Validate target hostnames against strict allowlist before dispatching requests" },
AstDetectorDefinition { id: "SLOP-132", name: "no-non-atomic-toctou-fs-mutation-v132", severity: SlopSeverity::Error, category: "path-injection", cwe: "CWE-367", description: "Non-atomic file existence check prior to destructive file write", recommendation: "Use atomic exclusive file creation flags such as wx or O_CREAT | O_EXCL" },
AstDetectorDefinition { id: "SLOP-133", name: "no-hallucinated-cipher-primitive-v133", severity: SlopSeverity::Error, category: "crypto-auth", cwe: "CWE-327", description: "Fictional or non-standard cipher algorithm strings passed to crypto APIs", recommendation: "Enforce NIST/IETF verified cipher primitives such as aes-256-gcm or sha256" },
AstDetectorDefinition { id: "SLOP-134", name: "no-shell-template-subprocess-exec-v134", severity: SlopSeverity::Error, category: "path-injection", cwe: "CWE-78", description: "Dynamic string interpolation inside shell-enabled subprocess execution", recommendation: "Pass discrete argument arrays to subprocesses with shell execution disabled" },
AstDetectorDefinition { id: "SLOP-135", name: "no-loop-scoped-event-listener-leak-v135", severity: SlopSeverity::Error, category: "async-concurrency", cwe: "CWE-400", description: "Event listener attachment inside loop body without removal or AbortSignal", recommendation: "Register event listeners outside loop bodies or attach once/AbortSignal" },
AstDetectorDefinition { id: "SLOP-136", name: "no-phantom-ai-sdk-slopsquatting-v136", severity: SlopSeverity::Error, category: "supply-chain", cwe: "CWE-1357", description: "Hallucinated AI wrapper packages in package manifests enabling slopsquatting", recommendation: "Pin verified official AI SDK packages (@anthropic-ai/sdk, openai, google-genai)" },
AstDetectorDefinition { id: "SLOP-137", name: "no-unbounded-mpsc-queue-sprawl-v137", severity: SlopSeverity::Error, category: "async-concurrency", cwe: "CWE-400", description: "Unbounded channel or async queue allocation lacking backpressure capacity", recommendation: "Replace unbounded_channel with bounded mpsc::channel(N) to enforce backpressure" },
AstDetectorDefinition { id: "SLOP-138", name: "no-truncated-hex-entropy-keygen-v138", severity: SlopSeverity::Error, category: "crypto-auth", cwe: "CWE-326", description: "Truncated hex buffer slicing or Math.random key generation reducing entropy", recommendation: "Use full 32-byte CSPRNG buffers or Argon2id/HKDF key derivation" },
AstDetectorDefinition { id: "SLOP-139", name: "no-async-iterator-unclosed-break-v139", severity: SlopSeverity::Error, category: "async-concurrency", cwe: "CWE-775", description: "Early break or return inside for-await-of loop without stream closure", recommendation: "Wrap async stream iteration in try/finally and call stream.destroy()" },
AstDetectorDefinition { id: "SLOP-140", name: "no-compound-namespace-slopsquat-v140", severity: SlopSeverity::Error, category: "supply-chain", cwe: "CWE-829", description: "Synthetic compound package names in dependency manifests", recommendation: "Verify package existence on canonical registry and pin exact version" },
AstDetectorDefinition { id: "SLOP-141", name: "no-unvalidated-ssrf-redirect-chain-v141", severity: SlopSeverity::Error, category: "path-injection", cwe: "CWE-918", description: "Unvalidated dynamic host interpolation in outbound HTTP client requests", recommendation: "Validate target hostnames against strict allowlist before dispatching requests" },
AstDetectorDefinition { id: "SLOP-142", name: "no-non-atomic-toctou-fs-mutation-v142", severity: SlopSeverity::Error, category: "path-injection", cwe: "CWE-367", description: "Non-atomic file existence check prior to destructive file write", recommendation: "Use atomic exclusive file creation flags such as wx or O_CREAT | O_EXCL" },
AstDetectorDefinition { id: "SLOP-143", name: "no-hallucinated-cipher-primitive-v143", severity: SlopSeverity::Error, category: "crypto-auth", cwe: "CWE-327", description: "Fictional or non-standard cipher algorithm strings passed to crypto APIs", recommendation: "Enforce NIST/IETF verified cipher primitives such as aes-256-gcm or sha256" },
AstDetectorDefinition { id: "SLOP-144", name: "no-shell-template-subprocess-exec-v144", severity: SlopSeverity::Error, category: "path-injection", cwe: "CWE-78", description: "Dynamic string interpolation inside shell-enabled subprocess execution", recommendation: "Pass discrete argument arrays to subprocesses with shell execution disabled" },
AstDetectorDefinition { id: "SLOP-145", name: "no-loop-scoped-event-listener-leak-v145", severity: SlopSeverity::Error, category: "async-concurrency", cwe: "CWE-400", description: "Event listener attachment inside loop body without removal or AbortSignal", recommendation: "Register event listeners outside loop bodies or attach once/AbortSignal" },
AstDetectorDefinition { id: "SLOP-146", name: "no-phantom-ai-sdk-slopsquatting-v146", severity: SlopSeverity::Error, category: "supply-chain", cwe: "CWE-1357", description: "Hallucinated AI wrapper packages in package manifests enabling slopsquatting", recommendation: "Pin verified official AI SDK packages (@anthropic-ai/sdk, openai, google-genai)" },
AstDetectorDefinition { id: "SLOP-147", name: "no-unbounded-mpsc-queue-sprawl-v147", severity: SlopSeverity::Error, category: "async-concurrency", cwe: "CWE-400", description: "Unbounded channel or async queue allocation lacking backpressure capacity", recommendation: "Replace unbounded_channel with bounded mpsc::channel(N) to enforce backpressure" },
AstDetectorDefinition { id: "SLOP-148", name: "no-truncated-hex-entropy-keygen-v148", severity: SlopSeverity::Error, category: "crypto-auth", cwe: "CWE-326", description: "Truncated hex buffer slicing or Math.random key generation reducing entropy", recommendation: "Use full 32-byte CSPRNG buffers or Argon2id/HKDF key derivation" },
AstDetectorDefinition { id: "SLOP-149", name: "no-async-iterator-unclosed-break-v149", severity: SlopSeverity::Error, category: "async-concurrency", cwe: "CWE-775", description: "Early break or return inside for-await-of loop without stream closure", recommendation: "Wrap async stream iteration in try/finally and call stream.destroy()" },
AstDetectorDefinition { id: "SLOP-150", name: "no-compound-namespace-slopsquat-v150", severity: SlopSeverity::Error, category: "supply-chain", cwe: "CWE-829", description: "Synthetic compound package names in dependency manifests", recommendation: "Verify package existence on canonical registry and pin exact version" },
AstDetectorDefinition { id: "SLOP-151", name: "no-unvalidated-ssrf-redirect-chain-v151", severity: SlopSeverity::Error, category: "path-injection", cwe: "CWE-918", description: "Unvalidated dynamic host interpolation in outbound HTTP client requests", recommendation: "Validate target hostnames against strict allowlist before dispatching requests" },
AstDetectorDefinition { id: "SLOP-152", name: "no-non-atomic-toctou-fs-mutation-v152", severity: SlopSeverity::Error, category: "path-injection", cwe: "CWE-367", description: "Non-atomic file existence check prior to destructive file write", recommendation: "Use atomic exclusive file creation flags such as wx or O_CREAT | O_EXCL" },
AstDetectorDefinition { id: "SLOP-153", name: "no-hallucinated-cipher-primitive-v153", severity: SlopSeverity::Error, category: "crypto-auth", cwe: "CWE-327", description: "Fictional or non-standard cipher algorithm strings passed to crypto APIs", recommendation: "Enforce NIST/IETF verified cipher primitives such as aes-256-gcm or sha256" },
AstDetectorDefinition { id: "SLOP-154", name: "no-shell-template-subprocess-exec-v154", severity: SlopSeverity::Error, category: "path-injection", cwe: "CWE-78", description: "Dynamic string interpolation inside shell-enabled subprocess execution", recommendation: "Pass discrete argument arrays to subprocesses with shell execution disabled" },
AstDetectorDefinition { id: "SLOP-155", name: "no-loop-scoped-event-listener-leak-v155", severity: SlopSeverity::Error, category: "async-concurrency", cwe: "CWE-400", description: "Event listener attachment inside loop body without removal or AbortSignal", recommendation: "Register event listeners outside loop bodies or attach once/AbortSignal" },
AstDetectorDefinition { id: "SLOP-156", name: "no-phantom-ai-sdk-slopsquatting-v156", severity: SlopSeverity::Error, category: "supply-chain", cwe: "CWE-1357", description: "Hallucinated AI wrapper packages in package manifests enabling slopsquatting", recommendation: "Pin verified official AI SDK packages (@anthropic-ai/sdk, openai, google-genai)" },
AstDetectorDefinition { id: "SLOP-157", name: "no-unbounded-mpsc-queue-sprawl-v157", severity: SlopSeverity::Error, category: "async-concurrency", cwe: "CWE-400", description: "Unbounded channel or async queue allocation lacking backpressure capacity", recommendation: "Replace unbounded_channel with bounded mpsc::channel(N) to enforce backpressure" },
AstDetectorDefinition { id: "SLOP-158", name: "no-truncated-hex-entropy-keygen-v158", severity: SlopSeverity::Error, category: "crypto-auth", cwe: "CWE-326", description: "Truncated hex buffer slicing or Math.random key generation reducing entropy", recommendation: "Use full 32-byte CSPRNG buffers or Argon2id/HKDF key derivation" },
AstDetectorDefinition { id: "SLOP-159", name: "no-async-iterator-unclosed-break-v159", severity: SlopSeverity::Error, category: "async-concurrency", cwe: "CWE-775", description: "Early break or return inside for-await-of loop without stream closure", recommendation: "Wrap async stream iteration in try/finally and call stream.destroy()" },
AstDetectorDefinition { id: "SLOP-160", name: "no-compound-namespace-slopsquat-v160", severity: SlopSeverity::Error, category: "supply-chain", cwe: "CWE-829", description: "Synthetic compound package names in dependency manifests", recommendation: "Verify package existence on canonical registry and pin exact version" },
AstDetectorDefinition { id: "SLOP-161", name: "no-unvalidated-ssrf-redirect-chain-v161", severity: SlopSeverity::Error, category: "path-injection", cwe: "CWE-918", description: "Unvalidated dynamic host interpolation in outbound HTTP client requests", recommendation: "Validate target hostnames against strict allowlist before dispatching requests" },
AstDetectorDefinition { id: "SLOP-162", name: "no-non-atomic-toctou-fs-mutation-v162", severity: SlopSeverity::Error, category: "path-injection", cwe: "CWE-367", description: "Non-atomic file existence check prior to destructive file write", recommendation: "Use atomic exclusive file creation flags such as wx or O_CREAT | O_EXCL" },
AstDetectorDefinition { id: "SLOP-163", name: "no-hallucinated-cipher-primitive-v163", severity: SlopSeverity::Error, category: "crypto-auth", cwe: "CWE-327", description: "Fictional or non-standard cipher algorithm strings passed to crypto APIs", recommendation: "Enforce NIST/IETF verified cipher primitives such as aes-256-gcm or sha256" },
AstDetectorDefinition { id: "SLOP-164", name: "no-shell-template-subprocess-exec-v164", severity: SlopSeverity::Error, category: "path-injection", cwe: "CWE-78", description: "Dynamic string interpolation inside shell-enabled subprocess execution", recommendation: "Pass discrete argument arrays to subprocesses with shell execution disabled" },
AstDetectorDefinition { id: "SLOP-165", name: "no-loop-scoped-event-listener-leak-v165", severity: SlopSeverity::Error, category: "async-concurrency", cwe: "CWE-400", description: "Event listener attachment inside loop body without removal or AbortSignal", recommendation: "Register event listeners outside loop bodies or attach once/AbortSignal" },
AstDetectorDefinition { id: "SLOP-166", name: "no-phantom-ai-sdk-slopsquatting-v166", severity: SlopSeverity::Error, category: "supply-chain", cwe: "CWE-1357", description: "Hallucinated AI wrapper packages in package manifests enabling slopsquatting", recommendation: "Pin verified official AI SDK packages (@anthropic-ai/sdk, openai, google-genai)" },
AstDetectorDefinition { id: "SLOP-167", name: "no-unbounded-mpsc-queue-sprawl-v167", severity: SlopSeverity::Error, category: "async-concurrency", cwe: "CWE-400", description: "Unbounded channel or async queue allocation lacking backpressure capacity", recommendation: "Replace unbounded_channel with bounded mpsc::channel(N) to enforce backpressure" },
AstDetectorDefinition { id: "SLOP-168", name: "no-truncated-hex-entropy-keygen-v168", severity: SlopSeverity::Error, category: "crypto-auth", cwe: "CWE-326", description: "Truncated hex buffer slicing or Math.random key generation reducing entropy", recommendation: "Use full 32-byte CSPRNG buffers or Argon2id/HKDF key derivation" },
AstDetectorDefinition { id: "SLOP-169", name: "no-async-iterator-unclosed-break-v169", severity: SlopSeverity::Error, category: "async-concurrency", cwe: "CWE-775", description: "Early break or return inside for-await-of loop without stream closure", recommendation: "Wrap async stream iteration in try/finally and call stream.destroy()" },
AstDetectorDefinition { id: "SLOP-170", name: "no-compound-namespace-slopsquat-v170", severity: SlopSeverity::Error, category: "supply-chain", cwe: "CWE-829", description: "Synthetic compound package names in dependency manifests", recommendation: "Verify package existence on canonical registry and pin exact version" },
AstDetectorDefinition { id: "SLOP-171", name: "no-unvalidated-ssrf-redirect-chain-v171", severity: SlopSeverity::Error, category: "path-injection", cwe: "CWE-918", description: "Unvalidated dynamic host interpolation in outbound HTTP client requests", recommendation: "Validate target hostnames against strict allowlist before dispatching requests" },
AstDetectorDefinition { id: "SLOP-172", name: "no-non-atomic-toctou-fs-mutation-v172", severity: SlopSeverity::Error, category: "path-injection", cwe: "CWE-367", description: "Non-atomic file existence check prior to destructive file write", recommendation: "Use atomic exclusive file creation flags such as wx or O_CREAT | O_EXCL" },
AstDetectorDefinition { id: "SLOP-173", name: "no-hallucinated-cipher-primitive-v173", severity: SlopSeverity::Error, category: "crypto-auth", cwe: "CWE-327", description: "Fictional or non-standard cipher algorithm strings passed to crypto APIs", recommendation: "Enforce NIST/IETF verified cipher primitives such as aes-256-gcm or sha256" },
AstDetectorDefinition { id: "SLOP-174", name: "no-shell-template-subprocess-exec-v174", severity: SlopSeverity::Error, category: "path-injection", cwe: "CWE-78", description: "Dynamic string interpolation inside shell-enabled subprocess execution", recommendation: "Pass discrete argument arrays to subprocesses with shell execution disabled" },
AstDetectorDefinition { id: "SLOP-175", name: "no-loop-scoped-event-listener-leak-v175", severity: SlopSeverity::Error, category: "async-concurrency", cwe: "CWE-400", description: "Event listener attachment inside loop body without removal or AbortSignal", recommendation: "Register event listeners outside loop bodies or attach once/AbortSignal" },
AstDetectorDefinition { id: "SLOP-176", name: "no-phantom-ai-sdk-slopsquatting-v176", severity: SlopSeverity::Error, category: "supply-chain", cwe: "CWE-1357", description: "Hallucinated AI wrapper packages in package manifests enabling slopsquatting", recommendation: "Pin verified official AI SDK packages (@anthropic-ai/sdk, openai, google-genai)" },
AstDetectorDefinition { id: "SLOP-177", name: "no-unbounded-mpsc-queue-sprawl-v177", severity: SlopSeverity::Error, category: "async-concurrency", cwe: "CWE-400", description: "Unbounded channel or async queue allocation lacking backpressure capacity", recommendation: "Replace unbounded_channel with bounded mpsc::channel(N) to enforce backpressure" },
AstDetectorDefinition { id: "SLOP-178", name: "no-truncated-hex-entropy-keygen-v178", severity: SlopSeverity::Error, category: "crypto-auth", cwe: "CWE-326", description: "Truncated hex buffer slicing or Math.random key generation reducing entropy", recommendation: "Use full 32-byte CSPRNG buffers or Argon2id/HKDF key derivation" },
AstDetectorDefinition { id: "SLOP-179", name: "no-async-iterator-unclosed-break-v179", severity: SlopSeverity::Error, category: "async-concurrency", cwe: "CWE-775", description: "Early break or return inside for-await-of loop without stream closure", recommendation: "Wrap async stream iteration in try/finally and call stream.destroy()" },
AstDetectorDefinition { id: "SLOP-180", name: "no-compound-namespace-slopsquat-v180", severity: SlopSeverity::Error, category: "supply-chain", cwe: "CWE-829", description: "Synthetic compound package names in dependency manifests", recommendation: "Verify package existence on canonical registry and pin exact version" },
AstDetectorDefinition { id: "SLOP-181", name: "no-unvalidated-ssrf-redirect-chain-v181", severity: SlopSeverity::Error, category: "path-injection", cwe: "CWE-918", description: "Unvalidated dynamic host interpolation in outbound HTTP client requests", recommendation: "Validate target hostnames against strict allowlist before dispatching requests" },
AstDetectorDefinition { id: "SLOP-182", name: "no-non-atomic-toctou-fs-mutation-v182", severity: SlopSeverity::Error, category: "path-injection", cwe: "CWE-367", description: "Non-atomic file existence check prior to destructive file write", recommendation: "Use atomic exclusive file creation flags such as wx or O_CREAT | O_EXCL" },
AstDetectorDefinition { id: "SLOP-183", name: "no-hallucinated-cipher-primitive-v183", severity: SlopSeverity::Error, category: "crypto-auth", cwe: "CWE-327", description: "Fictional or non-standard cipher algorithm strings passed to crypto APIs", recommendation: "Enforce NIST/IETF verified cipher primitives such as aes-256-gcm or sha256" },
AstDetectorDefinition { id: "SLOP-184", name: "no-shell-template-subprocess-exec-v184", severity: SlopSeverity::Error, category: "path-injection", cwe: "CWE-78", description: "Dynamic string interpolation inside shell-enabled subprocess execution", recommendation: "Pass discrete argument arrays to subprocesses with shell execution disabled" },
AstDetectorDefinition { id: "SLOP-185", name: "no-loop-scoped-event-listener-leak-v185", severity: SlopSeverity::Error, category: "async-concurrency", cwe: "CWE-400", description: "Event listener attachment inside loop body without removal or AbortSignal", recommendation: "Register event listeners outside loop bodies or attach once/AbortSignal" },
AstDetectorDefinition { id: "SLOP-186", name: "no-phantom-ai-sdk-slopsquatting-v186", severity: SlopSeverity::Error, category: "supply-chain", cwe: "CWE-1357", description: "Hallucinated AI wrapper packages in package manifests enabling slopsquatting", recommendation: "Pin verified official AI SDK packages (@anthropic-ai/sdk, openai, google-genai)" },
AstDetectorDefinition { id: "SLOP-187", name: "no-unbounded-mpsc-queue-sprawl-v187", severity: SlopSeverity::Error, category: "async-concurrency", cwe: "CWE-400", description: "Unbounded channel or async queue allocation lacking backpressure capacity", recommendation: "Replace unbounded_channel with bounded mpsc::channel(N) to enforce backpressure" },
AstDetectorDefinition { id: "SLOP-188", name: "no-truncated-hex-entropy-keygen-v188", severity: SlopSeverity::Error, category: "crypto-auth", cwe: "CWE-326", description: "Truncated hex buffer slicing or Math.random key generation reducing entropy", recommendation: "Use full 32-byte CSPRNG buffers or Argon2id/HKDF key derivation" },
AstDetectorDefinition { id: "SLOP-189", name: "no-async-iterator-unclosed-break-v189", severity: SlopSeverity::Error, category: "async-concurrency", cwe: "CWE-775", description: "Early break or return inside for-await-of loop without stream closure", recommendation: "Wrap async stream iteration in try/finally and call stream.destroy()" },
AstDetectorDefinition { id: "SLOP-190", name: "no-compound-namespace-slopsquat-v190", severity: SlopSeverity::Error, category: "supply-chain", cwe: "CWE-829", description: "Synthetic compound package names in dependency manifests", recommendation: "Verify package existence on canonical registry and pin exact version" },
AstDetectorDefinition { id: "SLOP-191", name: "no-unvalidated-ssrf-redirect-chain-v191", severity: SlopSeverity::Error, category: "path-injection", cwe: "CWE-918", description: "Unvalidated dynamic host interpolation in outbound HTTP client requests", recommendation: "Validate target hostnames against strict allowlist before dispatching requests" },
AstDetectorDefinition { id: "SLOP-192", name: "no-non-atomic-toctou-fs-mutation-v192", severity: SlopSeverity::Error, category: "path-injection", cwe: "CWE-367", description: "Non-atomic file existence check prior to destructive file write", recommendation: "Use atomic exclusive file creation flags such as wx or O_CREAT | O_EXCL" },
AstDetectorDefinition { id: "SLOP-193", name: "no-hallucinated-cipher-primitive-v193", severity: SlopSeverity::Error, category: "crypto-auth", cwe: "CWE-327", description: "Fictional or non-standard cipher algorithm strings passed to crypto APIs", recommendation: "Enforce NIST/IETF verified cipher primitives such as aes-256-gcm or sha256" },
AstDetectorDefinition { id: "SLOP-194", name: "no-shell-template-subprocess-exec-v194", severity: SlopSeverity::Error, category: "path-injection", cwe: "CWE-78", description: "Dynamic string interpolation inside shell-enabled subprocess execution", recommendation: "Pass discrete argument arrays to subprocesses with shell execution disabled" },
AstDetectorDefinition { id: "SLOP-195", name: "no-loop-scoped-event-listener-leak-v195", severity: SlopSeverity::Error, category: "async-concurrency", cwe: "CWE-400", description: "Event listener attachment inside loop body without removal or AbortSignal", recommendation: "Register event listeners outside loop bodies or attach once/AbortSignal" },
AstDetectorDefinition { id: "SLOP-196", name: "no-phantom-ai-sdk-slopsquatting-v196", severity: SlopSeverity::Error, category: "supply-chain", cwe: "CWE-1357", description: "Hallucinated AI wrapper packages in package manifests enabling slopsquatting", recommendation: "Pin verified official AI SDK packages (@anthropic-ai/sdk, openai, google-genai)" },
AstDetectorDefinition { id: "SLOP-197", name: "no-unbounded-mpsc-queue-sprawl-v197", severity: SlopSeverity::Error, category: "async-concurrency", cwe: "CWE-400", description: "Unbounded channel or async queue allocation lacking backpressure capacity", recommendation: "Replace unbounded_channel with bounded mpsc::channel(N) to enforce backpressure" },
AstDetectorDefinition { id: "SLOP-198", name: "no-truncated-hex-entropy-keygen-v198", severity: SlopSeverity::Error, category: "crypto-auth", cwe: "CWE-326", description: "Truncated hex buffer slicing or Math.random key generation reducing entropy", recommendation: "Use full 32-byte CSPRNG buffers or Argon2id/HKDF key derivation" },
AstDetectorDefinition { id: "SLOP-199", name: "no-async-iterator-unclosed-break-v199", severity: SlopSeverity::Error, category: "async-concurrency", cwe: "CWE-775", description: "Early break or return inside for-await-of loop without stream closure", recommendation: "Wrap async stream iteration in try/finally and call stream.destroy()" },
];
#[allow(dead_code)]
pub static ACTIVE_CONTRACTS: &[crate::types::AstDetectorContract] = &[
    package_hallucinations::CONTRACT,
    async_loop_traps::CONTRACT,
    insecure_crypto::CONTRACT,
    path_traversal_deep::CONTRACT,
    unsafe_deserialization::CONTRACT,
    ide_protocol::CONTRACT,
    ffi_speculative_apis::CONTRACT,
];
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
    if config.rules.get("no-unsafe-deserialization").map(|s| s != "off").unwrap_or(true) || config.rules.get("no-polymorphic-deserialization").map(|s| s != "off").unwrap_or(true) {
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
    if config.rules.get("no-insecure-crypto").map(|s| s != "off").unwrap_or(true) || config.rules.get("no-insecure-prng-tokens").map(|s| s != "off").unwrap_or(true) {
        findings.extend(detect_insecure_crypto_in_diff(diff_files));
    }
    if config.rules.get("no-hallucinated-speculative-apis").map(|s| s != "off").unwrap_or(true) {
        findings.extend(detect_speculative_apis_in_diff(diff_files));
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
    if config.rules.get("no-unawaited-async-in-loops").map(|s| s != "off").unwrap_or(true) {
        findings.extend(detect_async_loop_traps_in_diff(diff_files));
    }
    if config.rules.get("no-hallucinated-package-manifest").map(|s| s != "off").unwrap_or(true) {
        findings.extend(detect_package_hallucinations_in_diff(diff_files));
    }
    if config.rules.get("no-hardcoded-jwt-secrets").map(|s| s != "off").unwrap_or(true) {
        findings.extend(detect_jwt_leakage_in_diff(diff_files));
    }
    if config.rules.get("no-unscoped-idor-queries").map(|s| s != "off").unwrap_or(true) {
        findings.extend(detect_broken_access_control_in_diff(diff_files));
    }
    if config.rules.get("no-unsafe-xml-or-ssrf").map(|s| s != "off").unwrap_or(true) {
        findings.extend(detect_xxe_ssrf_in_diff(diff_files));
    }
    if config.rules.get("no-speculative-orm-methods").map(|s| s != "off").unwrap_or(true) {
        findings.extend(detect_speculative_orm_in_diff(diff_files));
    }
    if config.rules.get("no-deep-path-traversal").map(|s| s != "off").unwrap_or(true) || config.rules.get("no-zip-slip-traversal").map(|s| s != "off").unwrap_or(true) {
        findings.extend(detect_deep_path_traversal_in_diff(diff_files));
    }
    if config.rules.get("no-async-pipeline-races").map(|s| s != "off").unwrap_or(true) {
        findings.extend(detect_async_race_traps_in_diff(diff_files));
    }
    if config.rules.get("no-phantom-cloud-sdks").map(|s| s != "off").unwrap_or(true) {
        findings.extend(detect_phantom_cloud_sdks_in_diff(diff_files));
    }
    if config.rules.get("no-ide-protocol-hallucinations").map(|s| s != "off").unwrap_or(true) {
        findings.extend(detect_ide_protocol_hallucinations_in_diff(diff_files));
    }
    if config.rules.get("no-speculative-ffi-wasm").map(|s| s != "off").unwrap_or(true) {
        findings.extend(detect_ffi_speculative_apis_in_diff(diff_files));
    }
    if config.rules.get("no-slopsquat-compound-namespace").map(|s| s != "off").unwrap_or(true) {
        findings.extend(crate::detectors::package_hallucinations::detect_compound_slopsquat_in_diff(diff_files));
    }
    if config.rules.get("no-async-generator-unhandled-break").map(|s| s != "off").unwrap_or(true) {
        findings.extend(crate::detectors::async_loop_traps::detect_async_generator_unhandled_break_in_diff(diff_files));
    }
    if config.rules.get("no-insecure-direct-crypto-keygen").map(|s| s != "off").unwrap_or(true) {
        findings.extend(crate::detectors::insecure_crypto::detect_insecure_direct_crypto_keygen_in_diff(diff_files));
    }
    if config.rules.get("no-unbounded-channel-sprawl").map(|s| s != "off").unwrap_or(true) {
        findings.extend(crate::detectors::unbounded_concurrency::detect_unbounded_channel_sprawl_in_diff(diff_files));
    }
    if config.rules.get("no-speculative-subprocess-injection").map(|s| s != "off").unwrap_or(true) {
        findings.extend(crate::detectors::command_injection::detect_speculative_subprocess_injection_in_diff(diff_files));
    }
    if config.rules.get("no-unbounded-event-listener-loop").map(|s| s != "off").unwrap_or(true) {
        findings.extend(crate::detectors::async_loop_traps::detect_unbounded_event_listener_loop_in_diff(diff_files));
    }
    if config.rules.get("no-phantom-ai-wrappers").map(|s| s != "off").unwrap_or(true) {
        findings.extend(crate::detectors::package_hallucinations::detect_phantom_ai_wrappers_in_diff(diff_files));
    }
    if config.rules.get("no-speculative-file-race-toctou").map(|s| s != "off").unwrap_or(true) {
        findings.extend(crate::detectors::path_traversal::detect_speculative_file_race_toctou_in_diff(diff_files));
    }
    if config.rules.get("no-hallucinated-crypto-algorithm").map(|s| s != "off").unwrap_or(true) {
        findings.extend(crate::detectors::insecure_crypto::detect_hallucinated_crypto_algorithm_in_diff(diff_files));
    }
    if config.rules.get("no-speculative-ssrf-interpolation").map(|s| s != "off").unwrap_or(true) {
        findings.extend(crate::detectors::command_injection::detect_speculative_ssrf_interpolation_in_diff(diff_files));
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
    findings.extend(detect_speculative_apis_in_content(file_path, content));
    findings.extend(detect_async_loop_traps_in_content(file_path, content));
    findings.extend(detect_package_hallucinations_in_content(file_path, content));
    findings.extend(detect_jwt_leakage_in_content(file_path, content));
    findings.extend(detect_broken_access_control_in_content(file_path, content));
    findings.extend(detect_xxe_ssrf_in_content(file_path, content));
    findings.extend(detect_speculative_orm_in_content(file_path, content));
    findings.extend(detect_deep_path_traversal_in_content(file_path, content));
    findings.extend(detect_async_race_traps_in_content(file_path, content));
    findings.extend(detect_phantom_cloud_sdks_in_content(file_path, content));
    findings.extend(detect_ide_protocol_hallucinations_in_content(file_path, content));
    findings.extend(detect_ffi_speculative_apis_in_content(file_path, content));
    findings.extend(crate::detectors::package_hallucinations::detect_compound_slopsquat_in_content(file_path, content));
    findings.extend(crate::detectors::async_loop_traps::detect_async_generator_unhandled_break_in_content(file_path, content));
    findings.extend(crate::detectors::insecure_crypto::detect_insecure_direct_crypto_keygen_in_content(file_path, content));
    findings.extend(crate::detectors::unbounded_concurrency::detect_unbounded_channel_sprawl_in_content(file_path, content));
    findings.extend(crate::detectors::command_injection::detect_speculative_subprocess_injection_in_content(file_path, content));
    findings.extend(crate::detectors::async_loop_traps::detect_unbounded_event_listener_loop_in_content(file_path, content));
    findings.extend(crate::detectors::package_hallucinations::detect_phantom_ai_wrappers_in_content(file_path, content));
    findings.extend(crate::detectors::path_traversal::detect_speculative_file_race_toctou_in_content(file_path, content));
    findings.extend(crate::detectors::insecure_crypto::detect_hallucinated_crypto_algorithm_in_content(file_path, content));
    findings.extend(crate::detectors::command_injection::detect_speculative_ssrf_interpolation_in_content(file_path, content));
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
