#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_rlhf_record() {
        let json_line = r#"{"prompt": "What is reinforcement learning?", "chosen": "RL is a machine learning paradigm based on rewards.", "rejected": "I do not know."}"#;
        let mut linter = rlhflint::linter::RLHFLinter::new();
        let record = linter.parse_line(1, json_line).expect("Should parse");
        let violations = linter.lint_record(&record);
        assert!(violations.is_empty(), "Valid record should have no violations");
    }

    #[test]
    fn test_empty_prompt_violation() {
        let json_line = r#"{"prompt": "   ", "chosen": "Valid response", "rejected": "Other response"}"#;
        let mut linter = rlhflint::linter::RLHFLinter::new();
        let record = linter.parse_line(2, json_line).expect("Should parse");
        let violations = linter.lint_record(&record);
        assert!(violations.iter().any(|v| v.rule == "NO_EMPTY_PROMPT"));
    }

    #[test]
    fn test_identical_preference_pair() {
        let json_line = r#"{"prompt": "Calculate 2+2", "chosen": "4", "rejected": "4"}"#;
        let mut linter = rlhflint::linter::RLHFLinter::new();
        let record = linter.parse_line(3, json_line).expect("Should parse");
        let violations = linter.lint_record(&record);
        assert!(violations.iter().any(|v| v.rule == "IDENTICAL_PAIR"));
    }

    #[test]
    fn test_leaked_placeholder_detection() {
        let json_line = r#"{"prompt": "Write an intro", "chosen": "TODO: add intro here", "rejected": "Bad intro"}"#;
        let mut linter = rlhflint::linter::RLHFLinter::new();
        let record = linter.parse_line(4, json_line).expect("Should parse");
        let violations = linter.lint_record(&record);
        assert!(violations.iter().any(|v| v.rule == "LEAKED_PLACEHOLDER"));
    }
}
