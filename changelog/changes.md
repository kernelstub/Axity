# Changelog

## 2025-12-14
- Added string interpolation in `print("...")` via `!{name}`.
- Expanded return type support to include `str`, `flt`, `bool`, `array<T>`, `map<T>`, `obj`, `buffer`, and `class` instances.
- Implemented lambda functions and immediately-invoked function expressions (IIFE).
- Introduced `buffer` type with built-ins: `buffer_new`, `buffer_len`, `buffer_get`, `buffer_set`, `buffer_push`, `buffer_from_string`, `buffer_to_string`.
- Added exceptions: `try`, `catch`, `throw` with propagation across control structures and functions.
- Added dynamic `any` type and updated type checker to treat it as a wildcard in comparisons and member access.
- Implemented comment support: single-line `//` and multi-line block `/// ... ///`.
- Parser improvements for chained calls and member access (`mk().x`, call/member/index chaining).
- Documentation:
  - Consolidated language guide with comprehensive examples.
  - Added architecture overview.
  - Added formal semantics, invariants, and error model docs.
- Examples and tests added for interpolation, returns, lambdas/IIFE, buffers, exceptions, `any`, and comments.

