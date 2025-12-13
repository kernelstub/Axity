# Arrays

## Overview
- Type: `array<T>`
- Literal: `[e1, e2, ...]` (non-empty)
- Indexed by zero-based integers: `xs[i]`

## Built-ins
- `len(xs) -> int` (arrays and strings)
- `push(xs, v) -> int` (returns new length)
- `pop(xs) -> T` (last element)
- `set(xs, i, v) -> int` (index updated)

## Example
```
let xs: array<int> = [1, 2, 3];
print(len(xs));    // 3
print(xs[1]);      // 2
push(xs, 4);
set(xs, 0, 10);
print(xs);         // [10, 2, 3, 4]
```

