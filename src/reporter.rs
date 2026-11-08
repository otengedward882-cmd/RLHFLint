//! Summary reporter and quality score generator.

use crate::schema::{LintSummary, ViolationSeverity};

pub struct Reporter;

impl Reporter {
    pub fn render_terminal(summary: &LintSummary) -> String {
        let mut out = String::new();
        out.push_str("=================================================================\n");
        out.push_str("  ⚡ RLHFLint — AI Dataset Quality & Schema Audit Report\n");
        out.push_str("=================================================================\n\n");

        out.push_str(&format!("• Total Records Scanned:     {}\n", summary.total_records));
        out.push_str(&format!("• Clean Passing Records:     {}\n", summary.valid_records));
        out.push_str(&format!("• Flagged Records:           {}\n", summary.flagged_records));
        out.push_str(&format!("• Duplicates Detected:       {}\n", summary.duplicates_found));
        out.push_str(&format!("• Overall Quality Score:     {:.1}%\n\n", summary.quality_score));

        out.push_str("-----------------------------------------------------------------\n");
        out.push_str("  Rule Violations Breakdown:\n");
        out.push_str("-----------------------------------------------------------------\n");

        if summary.violations.is_empty() {
            out.push_str("  ✨ No violations found! Dataset is 100% compliant and production-ready.\n");
        } else {
            let display_limit = summary.violations.len().min(8);
            for v in &summary.violations[..display_limit] {
                let badge = match v.severity {
                    ViolationSeverity::Critical => "[❌ CRITICAL]",
                    ViolationSeverity::Warning => "[⚠️  WARNING ]",
                    ViolationSeverity::Info => "[ℹ️  INFO    ]",
                };
                out.push_str(&format!("  {} Line {}: [{}] {}\n", badge, v.line_number, v.rule, v.message));
            }
            if summary.violations.len() > display_limit {
                out.push_str(&format!("  ... and {} more violations.\n", summary.violations.len() - display_limit));
            }
        }

        out.push_str("\n=================================================================\n");
        out
    }

    pub fn render_markdown(summary: &LintSummary) -> String {
        let mut out = String::new();
        out.push_str("### ⚡ RLHFLint Dataset Quality Report\n\n");
        out.push_str(&format!("**Quality Health Score:** `{:.1}%`\n\n", summary.quality_score));
        out.push_str("| Metric | Value |\n|:---|:---|\n");
        out.push_str(&format!("| **Total Records** | `{}` |\n", summary.total_records));
        out.push_str(&format!("| **Valid Records** | `{}` |\n", summary.valid_records));
        out.push_str(&format!("| **Flagged Records** | `{}` |\n", summary.flagged_records));
        out.push_str(&format!("| **Duplicates** | `{}` |\n\n", summary.duplicates_found));

        if !summary.violations.is_empty() {
            out.push_str("#### Flagged Violations\n\n");
            out.push_str("| Line | Severity | Rule | Issue Description |\n|:---|:---|:---|:---|\n");
            for v in summary.violations.iter().take(10) {
                let sev = match v.severity {
                    ViolationSeverity::Critical => "❌ Critical",
                    ViolationSeverity::Warning => "⚠️ Warning",
                    ViolationSeverity::Info => "ℹ️ Info",
                };
                out.push_str(&format!("| `{}` | {} | `{}` | {} |\n", v.line_number, sev, v.rule, v.message));
            }
        }
        out
    }
}
