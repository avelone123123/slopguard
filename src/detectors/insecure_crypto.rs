use crate::types::{DiffFile, DiffLineType, SlopCategory, SlopFinding, SlopSeverity};
use regex::Regex;
fn is_js_ts(path: &str) -> bool {
let lower = path.to_lowercase();
lower.ends_with(".ts") || lower.ends_with(".tsx") || lower.ends_with(".js") || lower.ends_with(".jsx") || lower.ends_with(".mjs") || lower.ends_with(".cjs")
}
fn is_py(path: &str) -> bool {
path.to_lowercase().ends_with(".py")
}
fn is_rs(path: &str) -> bool {
path.to_lowercase().ends_with(".rs")
}
fn is_go(path: &str) -> bool {
path.to_lowercase().ends_with(".go")
}
fn get_sensitive_identifier_regex() -> Regex {
Regex::new(r#"(?i)\b[a-zA-Z0-9_$]*(?:token|secret|salt|nonce|api_?key|session|password|passwd|auth|csrf|jwt|private_key|auth_token|reset|otp|pin|iv)[a-zA-Z0-9_$]*\b"#).unwrap()
}
fn get_js_weak_prng_regex() -> Regex {
Regex::new(r#"(?:\bMath\.random|\bcrypto\.pseudoRandomBytes)\s*\("#).unwrap()
}
fn get_py_weak_prng_regex() -> Regex {
Regex::new(r#"\brandom\.(?:random|choice|choices|randint|sample|randrange)\s*\("#).unwrap()
}
fn get_rs_weak_prng_regex() -> Regex {
Regex::new(r#"\b(?:rand::thread_rng|rand::random(?:::<[^>]+>)?)\s*(?:\.|\()"#).unwrap()
}
fn get_go_weak_prng_regex() -> Regex {
Regex::new(r#"\brand\.(?:Intn|Float64|Int|Int31|Int63|Uint32|Uint64)\s*\("#).unwrap()
}
fn get_weak_hash_patterns() -> Vec<(Regex, &'static str)> {
vec![
(Regex::new(r#"(?i)\bcreateHash\s*\(\s*['"](?:md5|sha1)['"]\s*\)"#).unwrap(), "crypto.createHash initialized with cryptographically broken digest (MD5/SHA-1)"),
(Regex::new(r#"(?i)\bhashlib\.(?:md5|sha1)\s*\("#).unwrap(), "hashlib invoked with broken hash algorithm (MD5/SHA-1)"),
(Regex::new(r#"(?i)\bCryptoJS\.(?:MD5|SHA1)\s*\("#).unwrap(), "CryptoJS invoked with broken digest (MD5/SHA-1)"),
(Regex::new(r#"\b(?:md5|sha1)\.New\s*\(\s*\)"#).unwrap(), "Go crypto digest initialized with broken hash algorithm (MD5/SHA-1)"),
(Regex::new(r#"(?i)\b(?:md5|sha1)\s*\(\s*\$(?:password|pass|passwd|pwd|token|secret|salt|auth|pin|code|key)\b"#).unwrap(), "PHP weak hash invoked on credential or token variable"),
]
}
#[allow(dead_code)]
pub const CONTRACT: crate::types::AstDetectorContract = crate::types::AstDetectorContract {
rule_id: "no-insecure-prng-tokens",
severity: SlopSeverity::Error,
category: SlopCategory::InsecureCryptographicEntropy,
target_languages: &["js", "ts", "py", "rs", "go"],
analyze_diff: detect_insecure_crypto_in_diff,
analyze_content: detect_insecure_crypto_in_content,
};
pub fn detect_insecure_crypto_in_content(file_path: &str, content: &str) -> Vec<SlopFinding> {
let mut findings = Vec::new();
let sensitive_re = get_sensitive_identifier_regex();
let js_prng_re = get_js_weak_prng_regex();
let py_prng_re = get_py_weak_prng_regex();
let rs_prng_re = get_rs_weak_prng_regex();
let go_prng_re = get_go_weak_prng_regex();
let static_seed_re = Regex::new(r#"(?:SeedableRng::seed_from_u64\s*\(\s*0\s*\)|StdRng::seed_from_u64\s*\(\s*[0-9]+\s*\)|random\.seed\s*\(\s*(?:0|42|1234|1337)\s*\)|srand\s*\([^)]*\))"#).unwrap();
let hash_patterns = get_weak_hash_patterns();
let is_js = is_js_ts(file_path);
let is_python = is_py(file_path);
let is_rust = is_rs(file_path);
let is_golang = is_go(file_path);
for (idx, line) in content.lines().enumerate() {
let line_num = idx + 1;
let trimmed = line.trim();
if static_seed_re.is_match(trimmed) {
findings.push(SlopFinding {
file: file_path.to_string(),
line: line_num,
column: 1,
rule_id: "no-insecure-prng-tokens".to_string(),
category: SlopCategory::InsecureCryptographicEntropy,
severity: SlopSeverity::Error,
message: "Deterministic PRNG static seed undermines cryptographic entropy".to_string(),
snippet: trimmed.to_string(),
fixable: false,
suggestion: Some("Seed PRNG from OS entropy using rand::rngs::OsRng or secrets module".to_string()),
});
continue;
}
if js_prng_re.is_match(trimmed) && sensitive_re.is_match(trimmed) {
findings.push(SlopFinding {
file: file_path.to_string(),
line: line_num,
column: 1,
rule_id: "no-insecure-prng-tokens".to_string(),
category: SlopCategory::CryptoFlaw,
severity: SlopSeverity::Error,
message: "Insecure pseudo-random number generator (Math.random) used for security-sensitive token/credential (CWE-330)".to_string(),
snippet: trimmed.to_string(),
fixable: true,
suggestion: Some("Use crypto.randomBytes(n), crypto.randomUUID(), or crypto.getRandomValues()".to_string()),
});
continue;
}
if (is_python || (!is_js && !is_rust && !is_golang && trimmed.contains("def "))) && py_prng_re.is_match(trimmed) && sensitive_re.is_match(trimmed) {
findings.push(SlopFinding {
file: file_path.to_string(),
line: line_num,
column: 1,
rule_id: "no-insecure-prng-tokens".to_string(),
category: SlopCategory::CryptoFlaw,
severity: SlopSeverity::Error,
message: "Insecure pseudo-random generator (Python random module) used for security token/credential (CWE-330)".to_string(),
snippet: trimmed.to_string(),
fixable: true,
suggestion: Some("Use the standard secrets module (secrets.token_hex(), secrets.token_urlsafe(), secrets.choice())".to_string()),
});
continue;
}
if (is_rust || (!is_js && !is_python && !is_golang && trimmed.contains("fn "))) && rs_prng_re.is_match(trimmed) && sensitive_re.is_match(trimmed) {
findings.push(SlopFinding {
file: file_path.to_string(),
line: line_num,
column: 1,
rule_id: "no-insecure-prng-tokens".to_string(),
category: SlopCategory::CryptoFlaw,
severity: SlopSeverity::Error,
message: "Insecure PRNG (rand::thread_rng/rand::random) used for security token/credential without CSPRNG guarantees (CWE-330)".to_string(),
snippet: trimmed.to_string(),
fixable: true,
suggestion: Some("Use rand::rngs::OsRng or ring::rand::SystemRandom for cryptographic secrets".to_string()),
});
continue;
}
if (is_golang || (!is_js && !is_python && !is_rust && trimmed.contains("func "))) && go_prng_re.is_match(trimmed) && sensitive_re.is_match(trimmed) {
findings.push(SlopFinding {
file: file_path.to_string(),
line: line_num,
column: 1,
rule_id: "no-insecure-prng-tokens".to_string(),
category: SlopCategory::CryptoFlaw,
severity: SlopSeverity::Error,
message: "Insecure PRNG (math/rand) used for security token/credential without CSPRNG guarantees (CWE-330)".to_string(),
snippet: trimmed.to_string(),
fixable: true,
suggestion: Some("Use crypto/rand (crypto/rand.Read) for cryptographically secure values".to_string()),
});
continue;
}
for (re, desc) in &hash_patterns {
if re.is_match(trimmed) {
findings.push(SlopFinding {
file: file_path.to_string(),
line: line_num,
column: 1,
rule_id: "no-insecure-crypto".to_string(),
category: SlopCategory::Security,
severity: SlopSeverity::Error,
message: format!("Broken hash algorithm detected (CWE-328): {desc}"),
snippet: trimmed.to_string(),
fixable: false,
suggestion: Some("Migrate to SHA-256/SHA-512 for digests, or use Argon2id/bcrypt/scrypt for credential hashing".to_string()),
});
break;
}
}
}
findings
}
pub fn detect_insecure_crypto_in_diff(diff_files: &[DiffFile]) -> Vec<SlopFinding> {
let mut findings = Vec::new();
let sensitive_re = get_sensitive_identifier_regex();
let js_prng_re = get_js_weak_prng_regex();
let py_prng_re = get_py_weak_prng_regex();
let rs_prng_re = get_rs_weak_prng_regex();
let go_prng_re = get_go_weak_prng_regex();
let static_seed_re = Regex::new(r#"(?:SeedableRng::seed_from_u64\s*\(\s*0\s*\)|StdRng::seed_from_u64\s*\(\s*[0-9]+\s*\)|random\.seed\s*\(\s*(?:0|42|1234|1337)\s*\)|srand\s*\([^)]*\))"#).unwrap();
let hash_patterns = get_weak_hash_patterns();
for file in diff_files {
if file.is_deleted {
continue;
}
let target_path = if !file.new_path.is_empty() { &file.new_path } else { &file.old_path };
let is_js = is_js_ts(target_path);
let is_python = is_py(target_path);
let is_rust = is_rs(target_path);
let is_golang = is_go(target_path);
for chunk in &file.chunks {
for line in &chunk.lines {
if line.line_type != DiffLineType::Add {
continue;
}
let trimmed = line.content.trim();
let line_num = line.new_line_number.unwrap_or(1);
if static_seed_re.is_match(trimmed) {
findings.push(SlopFinding {
file: target_path.to_string(),
line: line_num,
column: 1,
rule_id: "no-insecure-prng-tokens".to_string(),
category: SlopCategory::InsecureCryptographicEntropy,
severity: SlopSeverity::Error,
message: "Deterministic PRNG static seed undermines cryptographic entropy".to_string(),
snippet: trimmed.to_string(),
fixable: false,
suggestion: Some("Seed PRNG from OS entropy using rand::rngs::OsRng or secrets module".to_string()),
});
continue;
}
if js_prng_re.is_match(trimmed) && sensitive_re.is_match(trimmed) {
findings.push(SlopFinding {
file: target_path.to_string(),
line: line_num,
column: 1,
rule_id: "no-insecure-prng-tokens".to_string(),
category: SlopCategory::CryptoFlaw,
severity: SlopSeverity::Error,
message: "Insecure pseudo-random number generator (Math.random) used for security-sensitive token/credential (CWE-330)".to_string(),
snippet: trimmed.to_string(),
fixable: true,
suggestion: Some("Use crypto.randomBytes(n), crypto.randomUUID(), or crypto.getRandomValues()".to_string()),
});
continue;
}
if (is_python || (!is_js && !is_rust && !is_golang && trimmed.contains("def "))) && py_prng_re.is_match(trimmed) && sensitive_re.is_match(trimmed) {
findings.push(SlopFinding {
file: target_path.to_string(),
line: line_num,
column: 1,
rule_id: "no-insecure-prng-tokens".to_string(),
category: SlopCategory::CryptoFlaw,
severity: SlopSeverity::Error,
message: "Insecure pseudo-random generator (Python random module) used for security token/credential (CWE-330)".to_string(),
snippet: trimmed.to_string(),
fixable: true,
suggestion: Some("Use the standard secrets module (secrets.token_hex(), secrets.token_urlsafe(), secrets.choice())".to_string()),
});
continue;
}
if (is_rust || (!is_js && !is_python && !is_golang && trimmed.contains("fn "))) && rs_prng_re.is_match(trimmed) && sensitive_re.is_match(trimmed) {
findings.push(SlopFinding {
file: target_path.to_string(),
line: line_num,
column: 1,
rule_id: "no-insecure-prng-tokens".to_string(),
category: SlopCategory::CryptoFlaw,
severity: SlopSeverity::Error,
message: "Insecure PRNG (rand::thread_rng/rand::random) used for security token/credential without CSPRNG guarantees (CWE-330)".to_string(),
snippet: trimmed.to_string(),
fixable: true,
suggestion: Some("Use rand::rngs::OsRng or ring::rand::SystemRandom for cryptographic secrets".to_string()),
});
continue;
}
if (is_golang || (!is_js && !is_python && !is_rust && trimmed.contains("func "))) && go_prng_re.is_match(trimmed) && sensitive_re.is_match(trimmed) {
findings.push(SlopFinding {
file: target_path.to_string(),
line: line_num,
column: 1,
rule_id: "no-insecure-prng-tokens".to_string(),
category: SlopCategory::CryptoFlaw,
severity: SlopSeverity::Error,
message: "Insecure PRNG (math/rand) used for security token/credential without CSPRNG guarantees (CWE-330)".to_string(),
snippet: trimmed.to_string(),
fixable: true,
suggestion: Some("Use crypto/rand (crypto/rand.Read) for cryptographically secure values".to_string()),
});
continue;
}
for (re, desc) in &hash_patterns {
if re.is_match(trimmed) {
findings.push(SlopFinding {
file: target_path.to_string(),
line: line_num,
column: 1,
rule_id: "no-insecure-crypto".to_string(),
category: SlopCategory::Security,
severity: SlopSeverity::Error,
message: format!("Broken hash algorithm detected (CWE-328): {desc}"),
snippet: trimmed.to_string(),
fixable: false,
suggestion: Some("Migrate to SHA-256/SHA-512 for digests, or use Argon2id/bcrypt/scrypt for credential hashing".to_string()),
});
break;
}
}
}
}
}
findings
}
#[allow(dead_code)]
pub fn detect_weak_prng_in_credentials(content: &str, file_path: &str) -> Vec<crate::types::SlopFinding> {
let mut findings = Vec::new();
let math_random_token = regex::Regex::new(r#"(token|auth|secret|session|csrf|salt|nonce|key)\s*[:=].*Math\.random\(\)"#).unwrap();
for (idx, line) in content.lines().enumerate() {
if math_random_token.is_match(line) {
findings.push(crate::types::SlopFinding {
rule_id: "SLOP-051".to_string(),
file: file_path.to_string(),
line: idx + 1,
column: 1,
category: crate::types::SlopCategory::InsecureCryptographicEntropy,
message: "Insecure PRNG Math.random() used in credential or token generation context".to_string(),
severity: crate::types::SlopSeverity::Error,
snippet: line.trim().to_string(),
fixable: true,
suggestion: Some("crypto.randomBytes(32).toString('hex')".to_string()),
});
}
}
findings
}
fn get_keygen_patterns() -> Vec<Regex> {
vec![
Regex::new(r#"(?i)(?:const|let|var|\$|auto|)\s*(?:key|aes_key|secret_key|encryption_key|hmac_key|aesKey|encryptionKey|secretKey|hmacKey)\s*=\s*(?:Math\.random\(\)\.toString\(36\)|crypto\.randomBytes\((?:[1-9]|1[0-5])\)|crypto\.pseudoRandomBytes)"#).unwrap(),
Regex::new(r#"(?i)(?:key|aes_key|secret_key|encryption_key|aesKey|secretKey|encryptionKey)\s*=\s*random\.(?:choices|sample)\([^)]+\)"#).unwrap(),
Regex::new(r#"(?i)crypto\.randomBytes\(\d+\)\.toString\(['"]hex['"]\)\.(?:slice|substring)\(\s*0\s*,\s*(?:[1-9]|[12][0-9]|3[01])\s*\)"#).unwrap(),
Regex::new(r#"(?i)(?:const|let|var|\$|auto|)\s*(?:key|aes_key|secret_key|encryption_key|hmac_key|aesKey|encryptionKey)\s*=\s*Math\.random\(\)\.toString\(36\)(?:\.(?:substring|slice)\(\d+\))?"#).unwrap(),
Regex::new(r#"(?i)(?:const|let|var|\$|auto|)\s*(?:key|aes_key|secret_key|encryption_key|hmac_key|aesKey|encryptionKey)\s*=\s*crypto\.randomBytes\((?:[1-9]|1[0-5])\)(?:\.toString\([^)]*\))?"#).unwrap(),
]
}
pub fn detect_insecure_direct_crypto_keygen_in_content(file_path: &str, content: &str) -> Vec<SlopFinding> {
let mut findings = Vec::new();
let patterns = get_keygen_patterns();
for (idx, line) in content.lines().enumerate() {
let trimmed = line.trim();
if trimmed.starts_with("//") || trimmed.starts_with('#') || trimmed.starts_with('*') || trimmed.contains("Regex::new") {
continue;
}
for pat in &patterns {
if pat.is_match(trimmed) {
findings.push(SlopFinding {
file: file_path.to_string(),
line: idx + 1,
column: 1,
rule_id: "no-insecure-direct-crypto-keygen".to_string(),
category: SlopCategory::TruncatedCryptoEntropy,
severity: SlopSeverity::Error,
message: "Direct truncated pseudo-random key generation or insufficient bit entropy (CWE-326)".to_string(),
snippet: trimmed.to_string(),
fixable: false,
suggestion: Some("Derive cryptographic keys using CSPRNG with KDF (Argon2id, PBKDF2, HKDF) with >=256-bit entropy".to_string()),
});
break;
}
}
}
findings
}
pub fn detect_insecure_direct_crypto_keygen_in_diff(diff_files: &[DiffFile]) -> Vec<SlopFinding> {
let mut findings = Vec::new();
let patterns = get_keygen_patterns();
for df in diff_files {
let path = if !df.new_path.is_empty() { &df.new_path } else { &df.old_path };
for chunk in &df.chunks {
for dl in &chunk.lines {
if dl.line_type != DiffLineType::Add {
continue;
}
let trimmed = dl.content.trim();
if trimmed.starts_with("//") || trimmed.starts_with('#') || trimmed.starts_with('*') || trimmed.contains("Regex::new") {
continue;
}
for pat in &patterns {
if pat.is_match(trimmed) {
findings.push(SlopFinding {
file: path.to_string(),
line: dl.new_line_number.unwrap_or(chunk.new_start),
column: 1,
rule_id: "no-insecure-direct-crypto-keygen".to_string(),
category: SlopCategory::TruncatedCryptoEntropy,
severity: SlopSeverity::Error,
message: "Direct truncated pseudo-random key generation or insufficient bit entropy (CWE-326)".to_string(),
snippet: trimmed.to_string(),
fixable: false,
suggestion: Some("Derive cryptographic keys using CSPRNG with KDF (Argon2id, PBKDF2, HKDF) with >=256-bit entropy".to_string()),
});
break;
}
}
}
}
}
findings
}
fn get_hallucinated_crypto_patterns() -> Vec<(Regex, &'static str)> {
vec![
(Regex::new(r#"(?i)[\"'](?:aes-256-turbo|aes256_turbo|aes-128-turbo|aes-512|aes-512-gcm)[\"']"#).unwrap(), "Hallucinated or non-standard AES cipher algorithm variant"),
(Regex::new(r#"(?i)[\"'](?:sha256-fast|sha256_fast|sha-128|sha128|sha256-turbo|sha3-turbo)[\"']"#).unwrap(), "Hallucinated cryptographic hash algorithm name"),
(Regex::new(r#"(?i)[\"'](?:blowfish-gcm|chacha20-fast|des-ede3|rsa-512|rsa-768)[\"']"#).unwrap(), "Broken, deprecated or hallucinated asymmetric/symmetric algorithm variant"),
(Regex::new(r#"(?i)hashlib\.new\s*\(\s*[\"'](?:sha256-fast|sha-128|md6|sha256_fast)[\"']\s*\)"#).unwrap(), "Python hashlib initialized with hallucinated digest name"),
(Regex::new(r#"(?i)crypto\.createCipheriv\s*\(\s*[\"'](?:aes-256-turbo|blowfish-gcm)[\"']"#).unwrap(), "Node.js crypto cipher initialized with fictional cipher variant"),
]
}
pub fn detect_hallucinated_crypto_algorithm_in_content(file_path: &str, content: &str) -> Vec<SlopFinding> {
let mut findings = Vec::new();
let patterns = get_hallucinated_crypto_patterns();
for (idx, line) in content.lines().enumerate() {
let trimmed = line.trim();
if trimmed.starts_with("//") || trimmed.starts_with('#') || trimmed.starts_with('*') || trimmed.contains("Regex::new") {
continue;
}
for (pat, desc) in &patterns {
if pat.is_match(trimmed) {
findings.push(SlopFinding {
file: file_path.to_string(),
line: idx + 1,
column: 1,
rule_id: "no-hallucinated-crypto-algorithm".to_string(),
category: SlopCategory::HallucinatedCryptoAlgorithm,
severity: SlopSeverity::Error,
message: format!("Hallucinated or non-standard cryptographic algorithm: {desc}"),
snippet: trimmed.to_string(),
fixable: false,
suggestion: Some("Enforce standard NIST/IETF primitives (aes-256-gcm, chacha20-poly1305, sha256)".to_string()),
});
break;
}
}
}
findings
}
pub fn detect_hallucinated_crypto_algorithm_in_diff(diff_files: &[DiffFile]) -> Vec<SlopFinding> {
let mut findings = Vec::new();
let patterns = get_hallucinated_crypto_patterns();
for df in diff_files {
let path = if !df.new_path.is_empty() { &df.new_path } else { &df.old_path };
for chunk in &df.chunks {
for dl in &chunk.lines {
if dl.line_type != DiffLineType::Add {
continue;
}
let trimmed = dl.content.trim();
if trimmed.starts_with("//") || trimmed.starts_with('#') || trimmed.starts_with('*') || trimmed.contains("Regex::new") {
continue;
}
for (pat, desc) in &patterns {
if pat.is_match(trimmed) {
findings.push(SlopFinding {
file: path.to_string(),
line: dl.new_line_number.unwrap_or(chunk.new_start),
column: 1,
rule_id: "no-hallucinated-crypto-algorithm".to_string(),
category: SlopCategory::HallucinatedCryptoAlgorithm,
severity: SlopSeverity::Error,
message: format!("Hallucinated or non-standard cryptographic algorithm: {desc}"),
snippet: trimmed.to_string(),
fixable: false,
suggestion: Some("Enforce standard NIST/IETF primitives (aes-256-gcm, chacha20-poly1305, sha256)".to_string()),
});
break;
}
}
}
}
}
findings
}