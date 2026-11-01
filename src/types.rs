use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
#[allow(dead_code)]
pub const RULE_NO_HALLUCINATED_PACKAGE_MANIFEST: &str = "no-hallucinated-package-manifest";
#[allow(dead_code)]
pub const RULE_NO_HALLUCINATED_IMPORTS: &str = "no-hallucinated-imports";
#[allow(dead_code)]
pub const RULE_NO_UNAWAITED_ASYNC_IN_LOOPS: &str = "no-unawaited-async-in-loops";
#[allow(dead_code)]
pub const RULE_NO_INSECURE_PRNG_TOKENS: &str = "no-insecure-prng-tokens";
#[allow(dead_code)]
pub const RULE_NO_DEEP_PATH_TRAVERSAL: &str = "no-deep-path-traversal";
#[allow(dead_code)]
pub const RULE_NO_UNSAFE_DESERIALIZATION: &str = "no-unsafe-deserialization";
#[allow(dead_code)]
pub const RULE_NO_IDE_PROTOCOL_HALLUCINATIONS: &str = "no-ide-protocol-hallucinations";
#[allow(dead_code)]
pub const RULE_NO_SPECULATIVE_FFI_WASM: &str = "no-speculative-ffi-wasm";
#[allow(dead_code)]
pub const RULE_NO_ZIP_SLIP_TRAVERSAL: &str = "no-zip-slip-traversal";
#[allow(dead_code)]
pub const RULE_NO_POLYMORPHIC_DESERIALIZATION: &str = "no-polymorphic-deserialization";
#[allow(dead_code)]
pub const RULE_NO_ASYNC_GENERATOR_UNHANDLED_BREAK: &str = "no-async-generator-unhandled-break";
#[allow(dead_code)]
pub const RULE_NO_INSECURE_DIRECT_CRYPTO_KEYGEN: &str = "no-insecure-direct-crypto-keygen";
#[allow(dead_code)]
pub const RULE_NO_SLOPSQUAT_COMPOUND_NAMESPACE: &str = "no-slopsquat-compound-namespace";
#[allow(dead_code)]
pub const RULE_NO_UNBOUNDED_CHANNEL_SPRAWL: &str = "no-unbounded-channel-sprawl";
#[allow(dead_code)]
pub const RULE_NO_SPECULATIVE_SUBPROCESS_INJECTION: &str = "no-speculative-subprocess-injection";
#[allow(dead_code)]
pub const RULE_NO_UNBOUNDED_EVENT_LISTENER_LOOP: &str = "no-unbounded-event-listener-loop";
#[allow(dead_code)]
pub const RULE_NO_PHANTOM_AI_WRAPPERS: &str = "no-phantom-ai-wrappers";
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum SlopCategory {
Placeholder,
Duplicate,
TypeDegradation,
GhostDependency,
ScopeDrift,
AiBloat,
SilentCatch,
GhostEnv,
Complexity,
Security,
AsyncAntiPattern,
LogicBypass,
SpeculativeApi,
CryptoFlaw,
SupplyChainPoisoning,
AsyncIterationHazard,
AuthLeakage,
BrokenAccessControl,
SsrfHazard,
PathTraversalSecurity,
AsyncResourceLeak,
CryptographicEntropy,
IdePluginProtocolHallucination,
PolymorphicUnsafeDeserialization,
UnawaitedAsyncLoopTrap,
InsecureCryptographicEntropy,
SpeculativeNativeFfi,
AsyncGeneratorLeak,
TruncatedCryptoEntropy,
CompoundSlopsquatHazard,
UnboundedChannelSprawl,
SpeculativeSubprocessInjection,
UnboundedEventListenerLoop,
PhantomAiWrapper,
}
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum SlopSeverity {
Error,
Warning,
Info,
}
#[derive(Clone, Copy)]
#[allow(dead_code)]
pub struct AstDetectorContract {
pub rule_id: &'static str,
pub severity: SlopSeverity,
pub category: SlopCategory,
pub target_languages: &'static [&'static str],
pub analyze_diff: fn(&[DiffFile]) -> Vec<SlopFinding>,
pub analyze_content: fn(&str, &str) -> Vec<SlopFinding>,
}
#[allow(dead_code)]
pub type SlopContract = AstDetectorContract;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SlopFinding {
pub file: String,
pub line: usize,
pub column: usize,
pub rule_id: String,
pub category: SlopCategory,
pub severity: SlopSeverity,
pub message: String,
pub snippet: String,
pub fixable: bool,
pub suggestion: Option<String>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SlopSummary {
pub total_issues: usize,
pub errors: usize,
pub warnings: usize,
pub score: usize,
pub findings: Vec<SlopFinding>,
pub scanned_files: usize,
pub duration_ms: u128,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SlopConfig {
pub ignore: Vec<String>,
pub rules: HashMap<String, String>,
pub max_drift_files: usize,
pub allowed_ghost_deps: Vec<String>,
}
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct DiffLine {
pub line_type: DiffLineType,
pub content: String,
pub old_line_number: Option<usize>,
pub new_line_number: Option<usize>,
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DiffLineType {
Add,
Del,
Normal,
}
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct DiffChunk {
pub old_start: usize,
pub old_lines: usize,
pub new_start: usize,
pub new_lines: usize,
pub lines: Vec<DiffLine>,
}
#[derive(Debug, Clone)]
pub struct DiffFile {
pub old_path: String,
pub new_path: String,
pub chunks: Vec<DiffChunk>,
pub is_new: bool,
pub is_deleted: bool,
}
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct FunctionSignature {
pub name: String,
pub file: String,
pub line: usize,
pub body_snippet: String,
pub token_set: HashSet<String>,
}