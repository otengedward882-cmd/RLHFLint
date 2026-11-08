//! Core linting rules for AI training & RLHF annotation records.

use crate::dedup::Deduplicator;
use crate::schema::{LintSummary, LintViolation, RLHFRecord, ViolationSeverity};

pub struct RLHFLinter {
    deduplicator: Deduplicator,
    pub min_prompt_chars: usize,
    pub max_prompt_chars: usize,
    pub min_response_chars: usize,
}

impl RLHFLinter {
    pub fn new() -> Self {
        Self {
            deduplicator: Deduplicator::new(),
            min_prompt_chars: 5,
            max_prompt_chars: 8000,
            min_response_chars: 5,
        }
    }

    /// Parses a JSON line extracting prompt, chosen, and rejected fields.
    pub fn parse_line(&self, line_num: usize, line_str: &str) -> Result<RLHFRecord, String> {
        let trimmed = line_str.trim();
        if trimmed.is_empty() {
            return Err("Empty line".to_string());
        }

        // Lightweight standard JSON string field extractor
        let prompt = Self::extract_field(trimmed, "prompt");
        let chosen = Self::extract_field(trimmed, "chosen");
        let rejected = Self::extract_field(trimmed, "rejected");

        if prompt.is_none() && chosen.is_none() && rejected.is_none() {
            return Err("Missing standard RLHF fields (expected 'prompt', 'chosen', 'rejected')".to_string());
        }

        Ok(RLHFRecord {
            line: line_num,
            prompt: prompt.unwrap_or_default(),
            chosen: chosen.unwrap_or_default(),
            rejected: rejected.unwrap_or_default(),
            raw_json: trimmed.to_string(),
        })
    }

    pub fn lint_record(&mut self, record: &RLHFRecord) -> Vec<LintViolation> {
        let mut violations = Vec::new();

        // Rule 1: Non-empty prompt
        if record.prompt.trim().is_empty() {
            violations.push(LintViolation {
                line_number: record.line,
                rule: "NO_EMPTY_PROMPT".to_string(),
                severity: ViolationSeverity::Critical,
                message: "Prompt field is completely empty or whitespace only".to_string(),
            });
        } else if record.prompt.len() < self.min_prompt_chars {
            violations.push(LintViolation {
                line_number: record.line,
                rule: "PROMPT_TOO_SHORT".to_string(),
                severity: ViolationSeverity::Warning,
                message: format!("Prompt length ({} chars) is under minimum threshold ({})", record.prompt.len(), self.min_prompt_chars),
            });
        }

        // Rule 2: Chosen vs Rejected response presence
        if record.chosen.trim().is_empty() {
            violations.push(LintViolation {
                line_number: record.line,
                rule: "NO_EMPTY_CHOSEN".to_string(),
                severity: ViolationSeverity::Critical,
                message: "Chosen response is empty; cannot compute policy preference".to_string(),
            });
        }

        if record.rejected.trim().is_empty() {
            violations.push(LintViolation {
                line_number: record.line,
                rule: "NO_EMPTY_REJECTED".to_string(),
                severity: ViolationSeverity::Critical,
                message: "Rejected response is empty; preference pair incomplete".to_string(),
            });
        }

        // Rule 3: Identical chosen & rejected (Zero-margin preference)
        if !record.chosen.is_empty() && record.chosen == record.rejected {
            violations.push(LintViolation {
                line_number: record.line,
                rule: "IDENTICAL_PAIR".to_string(),
                severity: ViolationSeverity::Critical,
                message: "Chosen and rejected responses are identical; preference loss gradient will be zero".to_string(),
            });
        }

        // Rule 4: Leaked Annotation Placeholders
        let placeholders = ["TODO", "FIXME", "[INSERT RESPONSE]", "[EMPTY]", "lorem ipsum"];
        for placeholder in placeholders {
            if record.prompt.contains(placeholder) || record.chosen.contains(placeholder) || record.rejected.contains(placeholder) {
                violations.push(LintViolation {
                    line_number: record.line,
                    rule: "LEAKED_PLACEHOLDER".to_string(),
                    severity: ViolationSeverity::Critical,
                    message: format!("Found unrendered annotation placeholder: '{}'", placeholder),
                });
                break;
            }
        }

        // Rule 5: Duplicate Prompt Check
        if !record.prompt.is_empty() && self.deduplicator.is_duplicate(&record.prompt) {
            violations.push(LintViolation {
                line_number: record.line,
                rule: "DUPLICATE_PROMPT".to_string(),
                severity: ViolationSeverity::Warning,
                message: "Exact or normalized duplicate prompt detected across dataset".to_string(),
            });
        }

        violations
    }

    /// Simple JSON field value extractor without heavy dependencies
    fn extract_field(json_str: &str, field_name: &str) -> Option<String> {
        let pattern = format!("\"{}\"", field_name);
        if let Some(pos) = json_str.find(&pattern) {
            let rest = &json_str[pos + pattern.len()..];
            if let Some(colon_pos) = rest.find(':') {
                let value_part = rest[colon_pos + 1..].trim_start();
                if value_part.starts_with('"') {
                    // Extract quoted string
                    let mut in_escape = false;
                    let mut end_idx = None;
                    for (i, ch) in value_part[1..].char_indices() {
                        if in_escape {
                            in_escape = false;
                        } else if ch == '\\' {
                            in_escape = true;
                        } else if ch == '"' {
                            end_idx = Some(i + 1);
                            break;
                        }
                    }
                    if let Some(end) = end_idx {
                        return Some(value_part[1..end].replace("\\n", "\n").replace("\\\"", "\""));
                    }
                }
            }
        }
        None
    }
}
