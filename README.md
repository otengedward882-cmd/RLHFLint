# ⚡ RLHFLint

[![Rust](https://img.shields.io/badge/Rust-2021%20Edition-DEA584.svg?style=flat&logo=rust)](https://www.rust-lang.org/)
[![License: MIT](https://img.shields.io/badge/License-MIT-green.svg)](LICENSE)
[![Focus](https://img.shields.io/badge/domain-RLHF%20%26%20AI%20Annotation%20QA-blue.svg)](#why-rlhflint)
[![Zero Dependencies](https://img.shields.io/badge/dependencies-0-brightgreen.svg)](#zero-dependency-architecture)

> **Blazing-fast RLHF and AI training dataset linter, schema validator, and deduplicator written in Rust.**  
> Scans millions of JSONL conversation records in milliseconds, flags empty turns, leaked placeholders, zero-margin pairs, and duplicate prompts.

```text
=================================================================
  ⚡ RLHFLint — AI Dataset Quality & Schema Audit Report
=================================================================

• Total Records Scanned:     7
• Clean Passing Records:     3
• Flagged Records:           4
• Duplicates Detected:       1
• Overall Quality Score:     42.9%

-----------------------------------------------------------------
  Rule Violations Breakdown:
-----------------------------------------------------------------
  [❌ CRITICAL] Line 3: [LEAKED_PLACEHOLDER] Found unrendered annotation placeholder: 'TODO'
  [⚠️  WARNING ] Line 4: [DUPLICATE_PROMPT] Exact or normalized duplicate prompt detected across dataset
  [❌ CRITICAL] Line 6: [NO_EMPTY_PROMPT] Prompt field is completely empty or whitespace only
  [❌ CRITICAL] Line 7: [IDENTICAL_PAIR] Chosen and rejected responses are identical; preference loss gradient will be zero

=================================================================
```

---

## ⚡ Why RLHFLint?

In remote AI data annotation and LLM alignment (RLHF, DPO, PPO), dirty datasets lead to catastrophic model training failures:
- **Zero-Margin Pairs**: When an evaluator marks identical text as both `chosen` and `rejected`, the loss gradient collapses to zero during preference optimization.
- **Leaked Placeholders**: Annotators often leave unfinished tags (`TODO`, `[INSERT]`, `FIXME`) in candidate responses.
- **Duplicate Prompt Flooding**: Repeated prompts skew reward models toward memorization over generalization.
- **Scale Bottlenecks**: Python dataset cleaners frequently hit Out-Of-Memory (OOM) crashes when streaming multi-gigabyte JSONL files.

**RLHFLint** is engineered in **Rust** to stream and validate millions of dataset rows with zero memory overhead, sub-millisecond execution, and strict quality assertion scores.

---

## ✨ Key Features

- **Blazing-Fast Rust Engine**: Streams JSONL records sequentially without loading entire datasets into memory.
- **Core RLHF Quality Rules**:
  - `NO_EMPTY_PROMPT`: Rejects missing or whitespace-only queries.
  - `IDENTICAL_PAIR`: Detects when chosen and rejected responses are identical.
  - `LEAKED_PLACEHOLDER`: Catches unfinished tags (`TODO`, `FIXME`, `[INSERT RESPONSE]`, `lorem ipsum`).
  - `DUPLICATE_PROMPT`: Fast FNV-1a 64-bit hash table to detect exact and whitespace-normalized duplicates.
  - `LENGTH_BOUNDS`: Flags truncated or under-length candidate pairs.
- **Quality Health Score (0–100%)**: Computes dataset health and enforces minimum acceptance gates (`--min-score 85.0`) for CI/CD pipelines.
- **Zero External Dependencies**: Pure Rust standard library. Compiles to a single lightweight, standalone native binary.
- **Markdown & Terminal Formats**: Outputs formatted tables for pull request descriptions and JSON for automated data pipelines.

---

## 🚀 Quick Start

### 1. Installation
Clone and build with `cargo`:
```bash
git clone https://github.com/otengedward882-cmd/RLHFLint.git
cd RLHFLint
cargo build --release
```

### 2. Lint an RLHF JSONL Dataset
```bash
./target/release/rlhflint --file examples/rlhf_samples.jsonl
```

### 3. Enforce a Minimum Quality Threshold for CI
```bash
# Exits with code 1 if dataset quality score is below 90%
./target/release/rlhflint --file dataset.jsonl --min-score 90.0
```

### 4. Export Markdown Quality Report
```bash
./target/release/rlhflint --file dataset.jsonl --markdown > dataset_audit.md
```

---

## 📖 CLI Flags

| Flag | Default | Description |
|---|---|---|
| `--file <PATH>`, `-f` | `""` | Path to target JSONL dataset file |
| `--min-score <PCT>` | `80.0` | Minimum passing dataset quality score (0–100) |
| `--markdown` | `false` | Render summary report wrapped in a Markdown table |
| `--version`, `-v` | `false` | Display version information |

---

## 🧪 Running Unit Tests

Execute the test suite using `cargo test`:
```bash
cargo test
```

---

## 📄 License

This project is licensed under the **MIT License** — see the [LICENSE](LICENSE) file for details.
