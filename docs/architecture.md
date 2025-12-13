# Axity Architecture

## Overview
- Implementation Language: Rust
- Execution Target: Native on Windows 10/11 and Linux x86-64
- Phases: Lexer → Parser → Type Checker → Interpreter

## Modules
- error: error types and source spans; interface: `AxityError`, `Span`
- token: lexical token model; interface: `TokenKind`, `Token`
- lexer: tokenization; interface: `lex(&str) -> Result<Vec<Token>, AxityError>`
- ast: abstract syntax tree; interface: `Program`, `Item`, `Stmt`, `Expr`, `Function`
- types: type system core; interface: `Type`
- parser: parsing to AST; interface: `parse(&[Token]) -> Result<Program, AxityError>`
- type_checker: static typing; interface: `check(&Program) -> Result<(), AxityError>`
- runtime: scoped environment; interface: `Runtime`, `Value`
- interpreter: execution; interface: `execute(&Program, &mut Runtime, &mut String) -> Result<(), AxityError>`
- import resolver: merges imported programs before type checking; `run_file(path)` resolves imports relative to the source file

## Responsibilities
- Lexer: convert source to tokens; no parsing or type logic
- Parser: build AST; no type or execution logic
- Type Checker: enforce types and signatures; no execution
- Interpreter: execute typed AST; no syntax or type analysis
- Runtime: manage scopes, variables, function frames, arrays, maps, and objects

## Interfaces and Boundaries
- The public library API is `run_source(&str) -> Result<String, AxityError>` in `src/lib.rs`
- `run_file(path)` loads a file, resolves imports, then runs phases
- Each module exposes only its domain types and functions; cross-module access goes through `lib.rs`
- No global mutable state; all execution state is in `Runtime`
- Arrays are `Rc<RefCell<Vec<Value>>>`; maps are `Rc<RefCell<HashMap<String, Value>>>`

## Extensibility
- New syntax: update token, parser, AST, and type checker in isolation
- New types: extend `Type`, type checker rules, and interpreter semantics
- Future codegen/JIT: add a `codegen` crate or module without changing front-end contracts
- Pretty printing uses a formatter that walks array/map/object values recursively with depth limiting

