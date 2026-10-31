use crate::types::{DiffFile, DiffLineType, SlopCategory, SlopFinding, SlopSeverity};
use regex::Regex;
fn is_supported_ext(p: &str) -> bool {
    let l = p.to_lowercase();
    l.ends_with(".js") || l.ends_with(".ts") || l.ends_with(".jsx") || l.ends_with(".tsx") || l.ends_with(".py") || l.ends_with(".rs")
}
pub fn detect_speculative_orm_in_diff(diff_files: &[DiffFile]) -> Vec<SlopFinding> {
    let mut findings = Vec::new();
    let prisma_speculative = Regex::new(r#"(?i)\bprisma\.[a-zA-Z0-9_]+\.(findOrCreate|upsertWhere|findFirstOrThrowByName|batchUpdate)\b"#).unwrap();
    let typeorm_speculative = Regex::new(r#"(?i)\.(findOrCreateByName|findByAttributes|bulkUpdateWhere)\s*\("#).unwrap();
    let mongoose_speculative = Regex::new(r#"(?i)\b[A-Z][a-zA-Z0-9_]*\.(findAndModifyOne|upsertDoc|findOrCreate)\s*\("#).unwrap();
    let sqlalchemy_speculative = Regex::new(r#"(?i)\.(get_or_create_by|filter_by_slug|first_or_create|update_where)\s*\("#).unwrap();
    let diesel_seaorm_speculative = Regex::new(r#"::(find_or_create|filter_by_column)\s*\("#).unwrap();
    for df in diff_files {
        let path = if !df.new_path.is_empty() { &df.new_path } else { &df.old_path };
        if !is_supported_ext(path) {
            continue;
        }
        for chunk in &df.chunks {
            for dl in &chunk.lines {
                if dl.line_type != DiffLineType::Add {
                    continue;
                }
                let line_str = dl.content.trim();
                let line_num = dl.new_line_number.unwrap_or(0);
                if prisma_speculative.is_match(line_str)
                    || typeorm_speculative.is_match(line_str)
                    || mongoose_speculative.is_match(line_str)
                    || sqlalchemy_speculative.is_match(line_str)
                    || diesel_seaorm_speculative.is_match(line_str)
                {
                    findings.push(SlopFinding {
                        file: path.clone(),
                        line: line_num,
                        column: 1,
                        rule_id: "no-speculative-orm-methods".to_string(),
                        category: SlopCategory::SpeculativeApi,
                        severity: SlopSeverity::Error,
                        message: "Speculative ORM/ODM method hallucination detected.".to_string(),
                        snippet: line_str.to_string(),
                        fixable: false,
                        suggestion: Some("Use standard ORM methods (e.g. findUnique, findOneBy, get_or_create, find/filter).".to_string()),
                    });
                }
            }
        }
    }
    findings
}
pub fn detect_speculative_orm_in_content(file_path: &str, content: &str) -> Vec<SlopFinding> {
    let mut findings = Vec::new();
    if !is_supported_ext(file_path) {
        return findings;
    }
    let prisma_speculative = Regex::new(r#"(?i)\bprisma\.[a-zA-Z0-9_]+\.(findOrCreate|upsertWhere|findFirstOrThrowByName|batchUpdate)\b"#).unwrap();
    let typeorm_speculative = Regex::new(r#"(?i)\.(findOrCreateByName|findByAttributes|bulkUpdateWhere)\s*\("#).unwrap();
    let mongoose_speculative = Regex::new(r#"(?i)\b[A-Z][a-zA-Z0-9_]*\.(findAndModifyOne|upsertDoc|findOrCreate)\s*\("#).unwrap();
    let sqlalchemy_speculative = Regex::new(r#"(?i)\.(get_or_create_by|filter_by_slug|first_or_create|update_where)\s*\("#).unwrap();
    let diesel_seaorm_speculative = Regex::new(r#"::(find_or_create|filter_by_column)\s*\("#).unwrap();
    for (idx, line) in content.lines().enumerate() {
        let line_str = line.trim();
        let line_num = idx + 1;
        if prisma_speculative.is_match(line_str)
            || typeorm_speculative.is_match(line_str)
            || mongoose_speculative.is_match(line_str)
            || sqlalchemy_speculative.is_match(line_str)
            || diesel_seaorm_speculative.is_match(line_str)
        {
            findings.push(SlopFinding {
                file: file_path.to_string(),
                line: line_num,
                column: 1,
                rule_id: "no-speculative-orm-methods".to_string(),
                category: SlopCategory::SpeculativeApi,
                severity: SlopSeverity::Error,
                message: "Speculative ORM/ODM method hallucination detected.".to_string(),
                snippet: line_str.to_string(),
                fixable: false,
                suggestion: Some("Use standard ORM methods (e.g. findUnique, findOneBy, get_or_create, find/filter).".to_string()),
            });
        }
    }
    findings
}