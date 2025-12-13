# Axity Semantics

## Lexical Elements
- Identifiers: `[A-Za-z_][A-Za-z0-9_]*`
- Integers: base-10, 64-bit signed
- Keywords: `let`, `fn`, `return`, `print`, `while`, `if`, `else`, `import`, `int`, `string`, `bool`, `array`, `map`, `true`, `false`
- Operators: `+ - * / < <= > >= == != = -> . [ ] , && || !`

## Grammar (informal)
- Program → { Item }
- Item → Function | Statement | Import
- Import → `import` StringLit `;`?
- Function → `fn` Ident `(` Params? `)` `->` Type `{` { Statement } `}`
- Params → Param { `,` Param }
- Param → Ident `:` Type
- Type → `int` | `string` | `bool` | `array< Type >` | `map< Type >` | ClassName
- Statement → Let | Assign | MemberAssign | Print | While | IfElse | Return | ExprStmt
- Let → `let` Ident `:` Type `=` Expr `;`
- Assign → Ident `=` Expr `;`
- MemberAssign → Expr `.` Ident `=` Expr `;`
- Print → `print` `(` Expr `)` `;`
- While → `while` Expr `{` { Statement } `}`
- IfElse → `if` Expr `{` { Statement } `}` (`else` `{` { Statement } `}`)?
- Return → `return` Expr `;`
- ExprStmt → Expr `;`
- Expr → Or
- Or → And { `||` And }
- And → Eq { `&&` Eq }
- Eq → Rel { (`==` | `!=`) Rel }
- Rel → Add { (`<` | `<=` | `>` | `>=`) Add }
- Add → Mul { (`+` | `-`) Mul }
- Mul → Unary { (`*` | `/`) Unary }
- Unary → `!` Unary | Primary
- Primary → IntLit | StringLit | `true` | `false` | Ident | Ident `(` Args? `)` | `new` ClassName `(` Args? `)`? | `(` Expr `)` | Primary `.` Ident | Primary `[` Expr `]`
- Args → Expr { `,` Expr }

## Types
- `int` (`i64`)
- `string`, `bool`
- `array<T>`, `map<T>` (string keys)
- `class` types

## Operational Semantics
- Variables: `let x: T = e;` evaluates `e`, binds result in current scope
- Assignment: `x = e;` evaluates `e`, updates the nearest scope where `x` is defined
- Member assignment: `obj.f = e;` updates field on receiver; numeric updates on `f = f (+,-,*,/) rhs` are optimized to avoid mismatched intermediate forms
- Print: `print(e);` evaluates `e`, writes formatted value plus newline to output buffer
- While: evaluate condition `c`; if `int`, nonzero is true; if `bool`, true is true; run body in a new scope each iteration
- IfElse: evaluate condition with same truth rules as `while`, run appropriate branch in a new scope
- Functions: call creates a new scope; parameters bound left-to-right; `return e;` terminates function with evaluated value
- Expression values: `int`, `string`, `bool`, arrays, maps, and object references
- Relational operators on `int`/`string` produce `int` truth values: true → `1`, false → `0`
- Logical `&& ||` operate on `bool` and produce `bool`; unary `!` operates on `bool`
- `string + string` produces `string`; arithmetic operators otherwise require `int`
- Built-ins mutate arrays/maps by reference; `len` works on arrays and strings

## Precedence and Associativity
- Highest to lowest: `!` > `* /` > `+ -` > `< <= > >=` > `== !=` > `&&` > `||`
- All binary operators are left-associative

## Scoping Rules
- Lexical, block-based scopes; while bodies and function bodies introduce a new scope
- Variable lookup searches innermost to outermost scope
- Method calls require a `self` receiver of the defining class; argument count must match parameters after `self`

