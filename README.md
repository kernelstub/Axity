<!-- PROJECT LOGO -->
<br />
<div align="center">

  <img src="https://github.com/user-attachments/assets/88b1822f-f8c7-42e7-a0be-36afaee88016" alt="Axity Logo" width="120" />

  <h3 align="center">Axity</h3>

  <p align="center">
    A compact, statically typed language written in Rust 
    <br />
    <a href="docs/architecture.md"><strong>Explore the docs »</strong></a>
    <br />
    <br />
    <a href="examples/func.ax">View Example</a>
    ·
    <a href="#contributing">Report Bug</a>
    ·
    <a href="#contributing">Contribute</a>
  </p>
</div>

<!-- TABLE OF CONTENTS -->
<details>
  <summary>Table of Contents</summary>
  <ol>
    <li><a href="#about">About The Project</a></li>
    <li>
      <a href="#installation">Getting Started</a>
      <ul>
        <li><a href="#installation">Installation</a></li>
        <li><a href="#usage">Usage</a></li>
      </ul>
    </li>
    <li><a href="#features">Features</a></li>
    <li><a href="#extending">Extending Axity</a></li>
    <li><a href="#contributing">Contributing</a></li>
  </ol>
</details>

<br />
<center><h1 align="left" id="about">About</h1></center>

Axity is a readable, testable language designed for clear compiler front end architecture and a deterministic interpreter. It favors correctness, clarity, and verifiability over cleverness, and it is structured to enable future code generation or JIT without compromising current design.

Key goals:
- Static typing with explicit `int`
- Clean separation: lexing → parsing → type checking → execution
- Deterministic semantics with explicit scopes and stack based calls
- Friendly error messages with source locations

<br />
<center><h1 align="left" id="installation">Installation</h1></center>

Prerequisites:
- Rust stable
- Cargo

Install and build:

```sh
cargo build
```

Optional (format and lint):

```sh
cargo fmt
cargo clippy
```

<br />
<center><h1 align="left" id="usage">Usage</h1></center>

Run an Axity program:

```sh
# Windows
cargo run -- .\examples\func.ax

# Unix
cargo run -- examples/func.ax
```

CLI usage (after build):

```sh
target/debug/axity <file.ax>
```

Library usage:

```rust
use axity::run_source;

fn main() -> Result<(), axity::AxityError> {
    let src = "let x: int = 1; print(x);";
    let out = run_source(src)?;
    assert_eq!(out, "1\n");
    Ok(())
}
```

Language snapshot:

```text
// variables
let x: int = 10;

// functions
fn add(a: int, b: int) -> int {
    return a + b;
}

// printing
print(add(2, 3));

// loops
while x < 10 {
    x = x + 1;
}
```

<br />
<center><h1 align="left" id="features">Features</h1></center>

- Static typing: explicit `int`, strict assignment and call signatures
- Deterministic interpreter: left to right evaluation, explicit truth values (1/0)
- Clean architecture: modules for tokenization, parsing, typing, runtime, execution
- Friendly errors: `AxityError` with line/column spans
- Extensible design: prepared for future backends (codegen/JIT)

<br />
<center><h1 align="left" id="extending">Extending Axity</h1></center>

Add new syntax or features by evolving front end modules:
- Tokens: `src/token/mod.rs`
- Parser rules: `src/parser/mod.rs`
- AST nodes: `src/ast/mod.rs`
- Types and typing rules: `src/types/mod.rs`, `src/type_checker/mod.rs`
- Runtime semantics: `src/runtime/mod.rs`, `src/interpreter/mod.rs`

Documentation:
- Architecture: `docs/architecture.md`
- Semantics: `docs/semantics.md`
- Invariants: `docs/invariants.md`
- Error Model: `docs/error_model.md`
- Language Guides: `docs/language/`

<br />
<center><h1 align="left" id="contributing">Contributing</h1></center>

- Open issues or proposals describing new language features or bugs
- Keep modules isolated and follow the phase separation
- Maintain `cargo test` coverage for lexer, parser, type checker, and execution


