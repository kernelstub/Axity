# Functions

## Definition
- Syntax: `fn name(param1: int, param2: int) -> int { ... }`
- Parameters are ordered and typed; return type is explicit and currently only `int`

## Body and Return
- At least one `return` statement is required for `int` functions (enforced by type checker)
- `return e;` evaluates `e` and returns immediately

## Calls
- Syntax: `name(arg1, arg2)`
- Arity and types must match the declared signature
- Calls are allowed in expressions and statements

## Example
```
fn add(a: int, b: int) -> int {
    return a + b;
}

print(add(2, 3));
```

## Execution Model
- A call creates a new scope; parameters are bound to evaluated arguments
- After `return`, the call frame is popped and the value is passed to the caller

