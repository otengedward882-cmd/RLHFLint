mod dedup;
mod linter;
mod reporter;
mod schema;

use linter::RLHFLinter;
use reporter::Reporter;
use schema::{LintSummary, ViolationSeverity};
use std::env;
use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::process;

const VERSION: &str = "1.0.0";

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.contains(&"--version".to_string()) || args.contains(&"-v".to_string()) {
        println!("RLHFLint version {} (Rust)", VERSION);
        process::exit(0);
    }

    let file_arg = args.iter().position(|a| a == "--file" || a == "-f")
        .and_then(|idx| args.get(idx + 1))
        .or_else(|| args.get(1).filter(|a| !a.starts_with('-')));

    let file_path = match file_arg {
        Some(path) => path,
        None => {
            eprintln!("Error: Missing JSONL dataset file argument.");
            eprintln!("Usage: rlhflint --file dataset.jsonl [--markdown] [--min-score 85.0]");
            process::exit(2);
        }
    };

    let is_markdown = args.contains(&"--markdown".to_string());
    let min_score: f64 = args.iter().position(|a| a == "--min-score")
        .and_then(|idx| args.get(idx + 1))
        .and_then(|s| s.parse().ok())
        .unwrap_or(80.0);

    let file = match File::open(file_path) {
        Ok(f) => f,
        Err(e) => {
            eprintln!("Failed to open file '{}': {}", file_path, e);
            process::exit(1);
        }
    };

    let reader = BufReader::new(file);
    let mut linter = RLHFLinter::new();

    let mut total_records = 0;
    let mut valid_records = 0;
    let mut flagged_records = 0;
    let mut duplicates = 0;
    let mut all_violations = Vec::new();

    for (index, line_result) in reader.lines().enumerate() {
        let line_num = index + 1;
        let line = match line_result {
            Ok(l) => l,
            Err(e) => {
                eprintln!("Error reading line {}: {}", line_num, e);
                continue;
            }
        };

        if line.trim().is_empty() {
            continue;
        }

        total_records += 1;

        match linter.parse_line(line_num, &line) {
            Ok(record) => {
                let violations = linter.lint_record(&record);
                if violations.is_empty() {
                    valid_records += 1;
                } else {
                    flagged_records += 1;
                    for v in &violations {
                        if v.rule == "DUPLICATE_PROMPT" {
                            duplicates += 1;
                        }
                    }
                    all_violations.extend(violations);
                }
            }
            Err(err_msg) => {
                flagged_records += 1;
                all_violations.push(schema::LintViolation {
                    line_number: line_num,
                    rule: "PARSE_ERROR".to_string(),
                    severity: ViolationSeverity::Critical,
                    message: err_msg,
                });
            }
        }
    }

    let quality_score = if total_records > 0 {
        (valid_records as f64 / total_records as f64) * 100.0
    } else {
        0.0
    };

    let summary = LintSummary {
        total_records,
        valid_records,
        flagged_records,
        quality_score,
        duplicates_found: duplicates,
        violations: all_violations,
    };

    if is_markdown {
        print!("{}", Reporter::render_markdown(&summary));
    } else {
        print!("{}", Reporter::render_terminal(&summary));
    }

    if quality_score < min_score {
        eprintln!("Dataset quality score ({:.1}%) is below minimum required threshold ({:.1}%)", quality_score, min_score);
        process::exit(1);
    }
}
