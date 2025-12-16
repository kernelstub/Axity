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
| ackermann     | 0.026      | 0.011     | 2.36×        | Axity   |
| dict_ops      | 0.027      | 0.013     | 2.08×        | Axity   |
| fibonacci     | 1.395      | 0.009     | 155.00×      | Axity   |
| list_ops      | 0.025      | 0.010     | 2.50×        | Axity   |
| loops         | 0.031      | 0.010     | 3.10×        | Axity   |
| math          | 0.027      | 0.010     | 2.70×        | Axity   |
| matrix        | 0.027      | 0.016     | 1.69×        | Axity   |
| nested_loops  | 0.040      | 0.018     | 2.22×        | Axity   |
| primes        | 0.025      | 0.010     | 2.50×        | Axity   |
| recursion     | 0.035      | 0.009     | 3.89×        | Axity   |
| sorting       | 0.025      | 0.009     | 2.78×        | Axity   |
| string_ops    | 0.025      | 0.010     | 2.50×        | Axity   |
| strings       | 0.026      | 0.019     | 1.37×        | Axity   |

## Category Highlights

| Category     | Benchmarks                         | Avg Axity/Python | Winner  |
|--------------|------------------------------------|------------------|---------|
| Strings      | string_ops, strings                | ~1.94×           | Axity   |
| Collections  | list_ops, sorting                  | ~2.64×           | Axity   |
| Numeric      | math, matrix, primes               | ~2.30×           | Axity   |
| Recursion    | recursion, ackermann, fibonacci    | ~53.75×          | Axity   |
| Control Flow | loops, nested_loops                | ~2.66×           | Axity   |

## Notes

| Note |
|------|
| Timings vary by machine; re-run the scripts to regenerate values. |
| Stats now recorded with per-run elapsed seconds. Windows uses PowerShell Stopwatch; Linux/macOS uses `date +%s%N`. |
| Set `REPEAT` to >1 to compute avg/min/max and write `benchmarks/results.csv` automatically. |

