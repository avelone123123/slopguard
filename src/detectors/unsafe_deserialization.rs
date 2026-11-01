use crate::types::{DiffFile, DiffLineType, SlopCategory, SlopFinding, SlopSeverity};
use regex::Regex;
fn is_py(p: &str) -> bool {
p.to_lowercase().ends_with(".py")
}
fn is_js_ts(p: &str) -> bool {
let l = p.to_lowercase();
l.ends_with(".ts") || l.ends_with(".tsx") || l.ends_with(".js") || l.ends_with(".jsx") || l.ends_with(".mjs") || l.ends_with(".cjs")
}
fn is_php(p: &str) -> bool {
p.to_lowercase().ends_with(".php")
}
fn is_rb(p: &str) -> bool {
p.to_lowercase().ends_with(".rb")
}
fn is_java(p: &str) -> bool {
let l = p.to_lowercase();
l.ends_with(".java") || l.ends_with(".cs")
}
fn is_unsafe_match(line: &str, re: &Regex, safe_opt: Option<&'static str>) -> bool {
re.is_match(line) && !safe_opt.map_or(false, |s| line.contains(s))
}
#[allow(dead_code)]
pub const CONTRACT: crate::types::AstDetectorContract = crate::types::AstDetectorContract {
rule_id: "no-unsafe-deserialization",
severity: SlopSeverity::Error,
category: SlopCategory::PolymorphicUnsafeDeserialization,
target_languages: &["py", "js", "ts", "rb", "php", "java", "rs"],
analyze_diff: detect_unsafe_deserialization_in_diff,
analyze_content: detect_unsafe_deserialization_in_content,
};
fn get_deserialization_patterns(file_path: &str, has_read_object: bool) -> Vec<(Regex, Option<&'static str>, &'static str, &'static str)> {
let mut patterns = Vec::new();
let specific = is_py(file_path) || is_js_ts(file_path) || is_php(file_path) || is_rb(file_path) || is_java(file_path) || file_path.to_lowercase().ends_with(".rs");
if !specific || is_py(file_path) {
patterns.push((Regex::new(r#"(?i)\b(?:pickle|_pickle|cPickle)\.(?:loads|load)\s*\("#).unwrap(), None, "Python pickle deserialization allows arbitrary code execution", "Replace pickle with json.loads, msgpack, or validated schemas (pydantic/msgspec)"));
patterns.push((Regex::new(r#"(?i)\byaml\.(?:unsafe_load|load)\s*\("#).unwrap(), Some("SafeLoader"), "Unsafe YAML load allows arbitrary object instantiation", "Use yaml.safe_load(stream) or pass Loader=yaml.SafeLoader"));
patterns.push((Regex::new(r#"(?i)\byaml\.load\s*\([^)]*Loader\s*=\s*(?:yaml\.)?(?:UnsafeLoader|Loader|CLoader)\b"#).unwrap(), None, "PyYAML unsafe Loader allows arbitrary Python object instantiation", "Use yaml.safe_load(stream) or pass Loader=yaml.SafeLoader"));
patterns.push((Regex::new(r#"(?i)\bmarshal\.(?:loads|load)\s*\("#).unwrap(), None, "Python marshal deserialization allows arbitrary code execution", "Use structured JSON or Protocol Buffers instead of marshal"));
patterns.push((Regex::new(r#"(?i)\bshelve\.open\s*\("#).unwrap(), None, "Python shelve storage uses pickle internally", "Use SQLite, Redis, or encrypted key-value stores with validated schemas"));
}
if !specific || is_js_ts(file_path) {
patterns.push((Regex::new(r#"(?i)\b(?:node-serialize|serialize)\.unserialize\s*\("#).unwrap(), None, "node-serialize unserialize vulnerability allows RCE via IIFE gadget payloads", "Replace node-serialize with JSON.parse and strict runtime type validation"));
patterns.push((Regex::new(r#"(?i)\b(?:serialize-javascript|serializeJavascript)\.(?:unserialize|deserialize)\s*\("#).unwrap(), None, "serialize-javascript deserialization allows remote code execution", "Parse untrusted state with JSON.parse and validated schemas"));
patterns.push((Regex::new(r#"(?i)\bvm\.(?:runInThisContext|runInNewContext|runInContext)\s*\("#).unwrap(), None, "vm dynamic context execution evaluates arbitrary code", "Avoid vm execution on untrusted code or run in isolated worker"));
patterns.push((Regex::new(r#"(?i)\beval\s*\("#).unwrap(), None, "Dynamic eval execution evaluates arbitrary scripts", "Use JSON.parse(payload) instead of eval"));
}
if !specific || is_php(file_path) {
patterns.push((Regex::new(r#"(?i)\bunserialize\s*\("#).unwrap(), Some("allowed_classes"), "PHP unserialize allows object injection and POP chain exploitation", "Use json_decode($data, true) or pass ['allowed_classes' => false] to unserialize"));
}
if !specific || is_rb(file_path) {
patterns.push((Regex::new(r#"(?i)\bMarshal\.load\s*\("#).unwrap(), None, "Ruby Marshal.load allows arbitrary object instantiation and RCE", "Use JSON.parse or secure serialization instead of Marshal.load"));
patterns.push((Regex::new(r#"(?i)\bYAML\.load\s*\("#).unwrap(), Some("permitted_classes"), "Ruby YAML.load without permitted_classes allows remote code execution", "Use YAML.safe_load(yaml_str, permitted_classes: [...])"));
}
if !specific || is_java(file_path) {
patterns.push((Regex::new(r#"(?i)\bXMLDecoder\s*\("#).unwrap(), None, "Java XMLDecoder allows arbitrary object instantiation", "Use Jackson or Gson with strict type validation"));
patterns.push((Regex::new(r#"(?i)\b(?:BinaryFormatter|NetDataContractSerializer)\b"#).unwrap(), None, ".NET deserializer allows arbitrary type instantiation and RCE", "Use System.Text.Json with typed DTOs"));
if has_read_object {
patterns.push((Regex::new(r#"(?i)\.readObject\s*\("#).unwrap(), None, "Java ObjectInputStream deserialization allows gadget chain execution", "Implement serial filtering (ObjectInputFilter) before calling readObject"));
} else {
patterns.push((Regex::new(r#"(?i)\bObjectInputStream\s*\("#).unwrap(), None, "Java ObjectInputStream deserialization allows gadget chain execution", "Implement serial filtering (ObjectInputFilter) before calling readObject"));
}
}
patterns
}
pub fn detect_unsafe_deserialization_in_content(file_path: &str, content: &str) -> Vec<SlopFinding> {
let mut findings = Vec::new();
let has_read_object = content.contains("readObject");
let patterns = get_deserialization_patterns(file_path, has_read_object);
for (i, line) in content.lines().enumerate() {
for (re, safe_opt, desc, sugg) in &patterns {
if is_unsafe_match(line, re, *safe_opt) {
findings.push(SlopFinding {
file: file_path.to_string(),
line: i + 1,
column: 1,
rule_id: "no-unsafe-deserialization".to_string(),
category: SlopCategory::PolymorphicUnsafeDeserialization,
severity: SlopSeverity::Error,
message: format!("Insecure deserialization vulnerability: {desc}"),
snippet: line.trim().to_string(),
fixable: false,
suggestion: Some(sugg.to_string()),
});
break;
}
}
}
findings
}
pub fn detect_unsafe_deserialization_in_diff(diff_files: &[DiffFile]) -> Vec<SlopFinding> {
let mut findings = Vec::new();
for f in diff_files {
let path = if !f.new_path.is_empty() { &f.new_path } else { &f.old_path };
for c in &f.chunks {
let chunk_str = c.lines.iter().map(|l| l.content.as_str()).collect::<Vec<_>>().join("\n");
let has_read_object = chunk_str.contains("readObject");
let patterns = get_deserialization_patterns(path, has_read_object);
for l in &c.lines {
if l.line_type == DiffLineType::Add {
for (re, safe_opt, desc, sugg) in &patterns {
if is_unsafe_match(&l.content, re, *safe_opt) {
findings.push(SlopFinding {
file: path.to_string(),
line: l.new_line_number.unwrap_or(1),
column: 1,
rule_id: "no-unsafe-deserialization".to_string(),
category: SlopCategory::PolymorphicUnsafeDeserialization,
severity: SlopSeverity::Error,
message: format!("Insecure deserialization vulnerability: {desc}"),
snippet: l.content.trim().to_string(),
fixable: false,
suggestion: Some(sugg.to_string()),
});
break;
}
}
}
}
}
}
findings
}
#[allow(dead_code)]
pub fn detect_polyglot_deserialization_gadgets(content: &str, file_path: &str) -> Vec<crate::types::SlopFinding> {
let mut findings = Vec::new();
let gadgets = [
("pickle.loads(", "SLOP-052", "Insecure Python pickle deserialization leads to arbitrary code execution"),
("yaml.unsafe_load(", "SLOP-052", "Unsafe YAML loading permits arbitrary object instantiation"),
("unserialize(", "SLOP-052", "PHP object injection via untrusted unserialize invocation"),
("BinaryFormatter", "SLOP-052", ".NET BinaryFormatter enables arbitrary payload deserialization"),
("NetDataContractSerializer", "SLOP-052", ".NET NetDataContractSerializer enables arbitrary payload deserialization"),
("XMLDecoder", "SLOP-052", "Java XMLDecoder allows remote code execution via object graph reconstruction")
];
for (idx, line) in content.lines().enumerate() {
for (pattern, rule, msg) in gadgets {
if line.contains(pattern) && !line.contains("test") {
findings.push(crate::types::SlopFinding {
rule_id: rule.to_string(),
file: file_path.to_string(),
line: idx + 1,
column: 1,
category: crate::types::SlopCategory::PolymorphicUnsafeDeserialization,
message: msg.to_string(),
severity: crate::types::SlopSeverity::Error,
snippet: line.trim().to_string(),
fixable: false,
suggestion: None,
});
}
}
}
findings
}