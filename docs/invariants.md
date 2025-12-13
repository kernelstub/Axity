Axity Invariants

Core Program Invariants
- No variable read before initialization
- Types match exactly on assignment
- Function calls match declared signatures
- Deterministic execution with explicit scopes
- No global mutable state outside the runtime context

Enforcement
- Parser: rejects syntactic violations and malformed constructs
- Type Checker: rejects undeclared variables, type mismatches, arity mismatches, and missing returns
- Interpreter: respects scopes, evaluates in order, and never performs type inference

Proof Obligations
- For each statement, type checker must guarantee evaluated expressions have the expected type
- For each function, type checker must find at least one `return` when return type is `int`
- For variable access, runtime lookup must resolve to the nearest scope binding
- For loops, condition evaluation must yield `int`; interpreter treats nonzero as true

Safety Obligations
- No panic-based control flow in core modules
- Errors use `Result` with `AxityError`
- No `unsafe`; memory safety provided by Rust ownership and borrowing

