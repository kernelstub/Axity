# Axity Benchmarks Guide

## Overview

- Runs a set of benchmarks in both Python and Axity, printing identical outputs followed by per-run timings.
- Windows uses `benchmarks\run_benchmarks.bat`. Linux/macOS use `benchmarks/run_benchmarks.sh`.
- Axity executable preference: `target/release/axity.exe` (or `axity` on Linux), fallback to `target/debug`.

## Prerequisites

- Python: `python3` or `python` available on PATH.
- Rust toolchain installed to build Axity.

## Build Axity (Release)

- Windows: `cargo build --release`
- Linux/macOS: `cargo build --release`

## Run on Windows

- From project root:
- `benchmarks\run_benchmarks.bat`
- The script:
  - Detects Python (`python3`, then `python`, else `py -3`)
  - Runs each Python benchmark, prints output and `Time: X.XXXs`
  - Runs each Axity benchmark, prints output and `Time: X.XXXs`

## Run on Linux/macOS

- Make executable:
- `chmod +x benchmarks/run_benchmarks.sh`
- Run:
- `./benchmarks/run_benchmarks.sh`
- The script:
  - Detects Axity release binary at `target/release/axity` (fallback to `target/debug/axity`)
  - Detects Python (`python3` then `python`)
  - Prints outputs and timings for each benchmark pair

## Benchmarks Included

- `ackermann`, `dict_ops`, `fibonacci`, `list_ops`, `loops`, `math`, `matrix`, `nested_loops`, `primes`, `recursion`, `sorting`, `string_ops`, `strings`

## Troubleshooting

- Missing Axity binary:
  - Build with `cargo build --release`
  - Scripts will skip Axity runs if the binary is absent
- Python not found:
  - Ensure `python3` or `python` is available on PATH

