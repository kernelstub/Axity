# Axity Cheatsheet

## Types
- `int`, `string`, `bool`, `array<T>`, `map<T>`, classes

## Operators
- Arithmetic: `+ - * /` (int); `string + string`
- Comparisons: `< <= > >= == !=` → `int` (1/0)
- Logical: `!` (unary), `&&`, `||` (bool)
- Member: `.`
- Index: `[ ]`

## Statements
- `let name: T = expr;`
- `name = expr;`
- `obj.field = expr;`
- `print(expr);`
- `while cond { ... }`
- `if cond { ... } else { ... }`
- `return expr;`
- `import "file.ax";`
- `match expr { case value: { ... } default: { ... } }`

## Arrays
- Literals: `[1, 2, 3]`
- Built-ins: `len(xs)`, `push(xs, v)`, `pop(xs)`, `set(xs, i, v)`, `slice(xs, s, l)`, `range(s, e)`
- Indexing: `xs[i]`

## Maps
- `map_new_int()`, `map_new_string()`
- `map_set(m, k, v)`, `map_get(m, k)`, `map_has(m, k)`, `map_keys(m)`
- `map_remove(m, k)`, `map_clear(m)`, `map_size(m)`

## Strings
- Escapes: `\" \\ \n \t`
- `strlen(s)`, `substr(s, start, len)`, `index_of(s, sub)`
- `string_replace(s, from, to)`, `string_split(s, sep)`
- `to_int(s)`, `to_string(i)`

## IO
- Files: `read_file`, `write_file`, `exists`, `mkdir`
- JSON: `read_json`, `write_json`, `json_get`, `json_set`
- TOML: `read_toml`, `write_toml`, `toml_get`, `toml_set`
- ENV: `read_env`, `write_env`, `env_get`, `env_set`

## CLI
- `axity <file.ax>` run a file (imports resolved relative to it)
- `axity repl` interactive mode; commands: `:load path`, `:env`, `:quit`
- Flags: `--dump-tokens`, `--dump-ast`

