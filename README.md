<!-- PROJECT LOGO -->
<br />
<div align="center">
  <img src="https://github.com/user-attachments/assets/cf995745-1778-48e1-89d0-eaf66de62f5a" alt="Axity Logo" width="120" />
  <h3 align="center">Axity</h3>
  <p align="center">
    A compact, statically-typed language with a clean Rust implementation
    <br />
    <a href="docs/architecture.md"><strong>Explore the docs »</strong></a>
    <br />
    <br />
    <a href="examples/basic.ax">View Example</a>
    ·
    <a href="#contributing">Report Bug</a>
    ·
    <a href="#contributing">Request Feature</a>
  </p>
</div>

<details>
  <summary>Table of Contents</summary>
  <ol>
    <li><a href="#about">About</a></li>
    <li><a href="#installation">Installation</a></li>
    <li><a href="#usage">Usage</a></li>
    <li><a href="#features">Features</a></li>
    <li><a href="#built-ins">Built-ins</a></li>
    <li><a href="#examples">Examples</a></li>
    <li><a href="#docs">Docs</a></li>
    <li><a href="#contributing">Contributing</a></li>
  </ol>
</details>

<h1 id="about">About</h1>

- Readable, testable language with clear phase separation: lex → parse → type-check → interpret
- Deterministic interpreter, explicit scopes, stack-based calls
- Designed for extensibility (maps, imports, IO) and future codegen/JIT

<h1 id="installation">Installation</h1>

- Requires Rust stable and Cargo
- Build: `cargo build`
- Optional: `cargo fmt`, `cargo clippy`

<h1 id="usage">Usage</h1>

- Run an Axity program:
  - Windows: `cargo run -- .\examples\basic.ax`
  - Unix: `cargo run -- examples/basic.ax`
- Run with imports: `cargo run -- examples/import_main.ax`
- REPL: `cargo run -- repl` (commands: `:load path`, `:env`, `:quit`)
- Library: `axity::run_source(&str)` or `axity::run_file(path)`

<h1 id="features">Features</h1>

- Types: `int`, `string`, `bool`, `array<T>`, `map<T>`, `class`
- Expressions:
  - Arithmetic `+ - * /` on `int`
  - `string + string` concatenation
  - Comparisons on `int`/`string` return `int` (1/0)
  - Logical `! && ||` on `bool`
  - Calls, member access, indexing, `new Class(args?)`
- Statements: `let`, assignment, member assignment, `print(expr);`, expression statements, `while`, `if/else`, `return`
- Imports: `import "relative.ax"` resolved relative to the source file
- Pretty printing: arrays `[a, b]`, maps `{k: v}`, objects `Class{field: val}`

<h1 id="built-ins">Built-ins</h1>

- Arrays: `len(xs)`, `push(xs, v)`, `pop(xs)`, `set(xs, i, v)`, `xs[i]`
- Strings: `strlen(s)`, `substr(s, start, len)`, `index_of(s, sub)`, `to_int(s)`, `to_string(i)`
- Files: `read_file(path)`, `write_file(path, content)`, `exists(path)`, `mkdir(path)`
- JSON: `read_json(path)`, `write_json(path, content)`, `json_get(json, key)`, `json_set(json, key, value)`
- TOML: `read_toml(path)`, `write_toml(path, content)`, `toml_get(toml, "key" | "section.key")`, `toml_set(toml, "key" | "section.key", value)`
- ENV: `read_env(path)`, `write_env(path, content)`, `env_get(env, key)`, `env_set(env, key, value)`
- Maps: `map_new_int()`, `map_new_string()`, `map_set(m, k, v)`, `map_get(m, k)`, `map_has(m, k)`, `map_keys(m)`

<h1 id="examples">Examples</h1>

- Core: `examples/variables.ax`, `examples/functions.ax`, `examples/loops.ax`, `examples/conditionals.ax`
- Data: `examples/arrays_index_len.ax`, `examples/strings.ax`, `examples/maps.ax`
- OOP: `examples/classes_methods.ax`
- Imports: `examples/import_main.ax` + `examples/import_functions.ax`
- IO: `examples/io_files.ax`, `examples/io_json.ax`, `examples/io_toml_env.ax`
- Pretty printing: `examples/pretty.ax`

<h1 id="docs">Docs</h1>

- Architecture: `docs/architecture.md`
- Semantics: `docs/semantics.md`
- Cheatsheet: `docs/cheatsheet.md`
- Invariants: `docs/invariants.md`
- Error Model: `docs/error_model.md`
- Language Guides: `docs/language/` (variables, functions, loops, printing, arrays, maps, classes, booleans, imports, io)

<h1 id="contributing">Contributing</h1>

- Open issues or proposals describing new language features or bugs
- Keep modules isolated and follow the phase separation
- Maintain `cargo test` coverage for lexer, parser, type checker, and execution

