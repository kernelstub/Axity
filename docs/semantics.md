Axity Semantics

Lexical Elements
- Identifiers: `[A-Za-z_][A-Za-z0-9_]*`
- Integers: base-10, 64-bit signed
- Keywords: `let`, `fn`, `return`, `print`, `while`, `int`
- Operators: `+ - * / < <= > >= == != = ->`

Grammar (informal)
- Program → { Item }
- Item → Function | Statement
- Function → `fn` Ident `(` Params? `)` `->` Type `{` { Statement } `}`
- Params → Param { `,` Param }
- Param → Ident `:` Type
- Type → `int`
- Statement → Let | Assign | Print | While | Return | CallStmt
- Let → `let` Ident `:` Type `=` Expr `;`
- Assign → Ident `=` Expr `;`
- Print → `print` `(` Expr `)` `;`
- While → `while` Expr `{` { Statement } `}`
- Return → `return` Expr `;`
- CallStmt → Ident `(` Args? `)` `;` (shorthand prints of calls are parsed into `Print(Call)`)
- Expr → Eq
- Eq → Rel { (`==` | `!=`) Rel }
- Rel → Add { (`<` | `<=` | `>` | `>=`) Add }
- Add → Mul { (`+` | `-`) Mul }
- Mul → Primary { (`*` | `/`) Primary }
- Primary → IntLit | Ident | Ident `(` Args? `)` | `(` Expr `)`
- Args → Expr { `,` Expr }

Types
- Only `int` exists initially (backed by `i64`)

Operational Semantics
- Variables: `let x: int = e;` evaluates `e`, binds result in current scope
- Assignment: `x = e;` evaluates `e`, updates the nearest scope where `x` is defined
- Print: `print(e);` evaluates `e`, writes decimal value plus newline to output buffer
- While: evaluate condition `c`; if nonzero, execute body in a new scope; repeat until `c` is zero
- Functions: call creates a new scope; parameters bound left-to-right; `return e;` terminates function with evaluated value
- Expression values: all expressions evaluate to `int`
- Relational operators produce `int` truth values: true → `1`, false → `0`

Precedence and Associativity
- Highest to lowest: `* /` > `+ -` > `< <= > >=` > `== !=`
- All binary operators are left-associative

Scoping Rules
- Lexical, block-based scopes; while bodies and function bodies introduce a new scope
- Variable lookup searches innermost to outermost scope

