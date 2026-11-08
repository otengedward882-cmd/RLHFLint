//! Schema definitions for RLHF and conversational AI annotation datasets.

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ViolationSeverity {
    Critical, // Rejects the record
    Warning,  // Quality flaw requiring review
    Info,
}

#[derive(Debug, Clone)]
pub struct LintViolation {
    pub line_number: usize,
    pub rule: String,
    pub severity: ViolationSeverity,
    pub message: String,
}

#[derive(Debug, Clone, Default)]
pub struct RLHFRecord {
    pub line: usize,
    pub prompt: String,
    pub chosen: String,
    pub rejected: String,
    pub raw_json: String,
}

#[derive(Debug, Clone, Default)]
pub struct LintSummary {
    pub total_records: usize,
    pub valid_records: usize,
    pub flagged_records: usize,
    pub quality_score: f64,
    pub duplicates_found: usize,
    pub violations: Vec<LintViolation>,
}
