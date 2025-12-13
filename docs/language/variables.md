Variables

Declaration
- Syntax: `let name: int = expr;`
- Variables must be initialized at declaration

Assignment
- Syntax: `name = expr;`
- Type must match the declared type exactly (`int`)

Scope
- Variables live in the current scope; inner scopes shadow outer ones
- Lookup searches from innermost to outermost scope

Example
```
let x: int = 1;
x = x + 2;
print(x);
```

