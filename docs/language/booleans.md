# Booleans

## Overview
- Type: `bool`
- Literals: `true`, `false`

## Operators
- Unary `!` (negation)
- Logical `&&`, `||` (and/or)

## Conditions
- `if` / `while` accept `bool` or `int` (nonzero is true)

## Example
```
let a: bool = true;
let b: bool = false;
print(a && !b);   // true

let x: int = 1;
if x { print(1); } else { print(0); }
```

