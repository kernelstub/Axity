# Axity Error Model

## Categories
- Lexical errors: invalid characters or malformed tokens
- Parse errors: unexpected tokens or missing syntactic elements
- Type errors: undefined variables, type mismatches, argument count/type mismatches, missing returns
- Runtime errors: reading or assigning undefined variables, invalid operations (e.g., division by zero if added later)
- Internal errors: unexpected states (should be unreachable and treated as bugs)

## Structure
- All errors are `AxityError { kind, span }`
- `kind`: one of Lex, Parse, Type, Runtime
- `span`: `Some(Span{ line, col })` for source-related errors; may be `None` for runtime conditions without a single location

## Formatting
- `lex error at L:C: message`
- `parse error at L:C: message`
- `type error at L:C: message`
- `runtime error: message`

## Propagation
- Each phase returns `Result<_, AxityError>`
- `run_source` composes phases; the first error short-circuits execution and is reported

## Examples
- Lex: `unexpected '!'`
- Parse: `unexpected token`
- Type: `undefined variable`, `type mismatch`, `argument count mismatch`
- Runtime: `read of undefined variable`, `assign to undefined variable`

