# Benchmark Statistics: Axity vs Python

## Environment

| Item    | Value                        |
|---------|------------------------------|
| OS      | Windows (PowerShell timing)  |
| Axity   | `target/release/axity.exe`   |
| Python  | `python3` or `python`        |
| Metric  | Per-script seconds (X.XXXs)  |

## Results (Per Benchmark)

| Benchmark     | Python (s) | Axity (s) | Axity/Python | Winner  |
|---------------|------------|-----------|--------------|---------|
| ackermann     | 0.025      | 0.010     | 2.50×        | Axity   |
| dict_ops      | 0.025      | 0.010     | 2.50×        | Axity   |
| fibonacci     | 1.422      | 0.009     | 158.00×      | Axity   |
| list_ops      | 0.025      | 0.009     | 2.78×        | Axity   |
| loops         | 0.031      | 0.009     | 3.44×        | Axity   |
| math          | 0.025      | 0.009     | 2.78×        | Axity   |
| matrix        | 0.026      | 0.014     | 1.86×        | Axity   |
| nested_loops  | 0.040      | 0.013     | 3.08×        | Axity   |
| primes        | 0.025      | 0.009     | 2.78×        | Axity   |
| recursion     | 0.036      | 0.009     | 4.00×        | Axity   |
| sorting       | 0.025      | 0.009     | 2.78×        | Axity   |
| string_ops    | 0.026      | 0.009     | 2.89×        | Axity   |
| strings       | 0.025      | 0.014     | 1.79×        | Axity   |

## Category Highlights

| Category     | Benchmarks                         | Avg Axity/Python | Winner  |
|--------------|------------------------------------|------------------|---------|
| Strings      | string_ops, strings                | ~2.34×           | Axity   |
| Collections  | list_ops, sorting                  | ~2.78×           | Axity   |
| Numeric      | math, matrix, primes               | ~2.47×           | Axity   |
| Recursion    | recursion, ackermann, fibonacci    | ~54.83×          | Axity   |
| Control Flow | loops, nested_loops                | ~3.26×           | Axity   |

## Notes

| Note |
|------|
| Timings vary by machine; re-run the scripts to regenerate values. |
| Stats now recorded with per-run elapsed seconds. Windows uses PowerShell Stopwatch; Linux/macOS uses `date +%s%N`. |
| Set `REPEAT` to >1 to compute avg/min/max and write `benchmarks/results.csv` automatically. |
