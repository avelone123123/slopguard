use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
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
}
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum SlopSeverity {
    Error,
    Warning,
    Info,
}
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