# Loops

## While Loop
- Syntax: `while expr { ... }`
- Condition `expr` evaluates to `int`; nonzero is true, zero is false
- Body executes in a new scope each iteration

## Example
```
let i: int = 0;
while i < 3 {
    print(i);
    i = i + 1;
}
```

