# Axity Language Guide

## Table of Contents
1. Overview
2. Types
3. Variables
4. Expressions and Operators
5. Control Flow
6. Functions
7. Lambdas and IIFE
8. Classes and Objects
9. Dynamic Objects (`obj`)
10. Arrays and Maps
11. Strings
12. Buffers
13. IO: Files, JSON, TOML, ENV
14. Imports
15. REPL and Debug
16. Printing
17. Examples

## Key features:
- Types: `int`, `str`, `flt`, `bool`, `array<T>`, `map<T>`, `obj`, `buffer`, `class`
- Dynamic: `any` type for flexible values
- Control flow: `if/else`, `while`, `do { } while`, `for init; cond; post`, `for var in collection`, `match/case/default`, `retry`, `try/catch/throw`
- Operators: arithmetic, logical (with `and`/`or` aliases), bitwise, modulo, postfix `++/--`
- Strings: interpolation with `!{name}`
- Lambdas and IIFE: `fn (...) -> T { ... }` and `fn (...) -> T { ... }(args)`
- Buffers: byte arrays with conversion to/from strings

---

## Types
- `int` (64-bit)
- `str` (`string`)
- `flt` (fixed-point, six fractional digits; `1.5`, `1.5e10`)
- `bool`
- `array<T>`
- `map<T>` (string keys)
- `obj` (dynamic object; string keys, any values)
- `any` (dynamic type; accepts any value)
- `buffer` (mutable byte vector)
- `class` (user-defined types)

### int
```
let n: int = 42;
n = n + 8;
print(n);
```

### str / string
```
let a: str = "hello";
let b: string = " world";
print(a + b);
print("greet !{a}");
```

### flt
```
let x: flt = 1.5;
let y: flt = 1.5e2;
let z: flt = -3.25;
print(x);
print(y);
print(z);
```

### bool
```
let ok: bool = true;
print(ok && false);
print(!ok);
```

### array<T>
```
let xs: array<int> = [1, 2, 3];
push(xs, 4);
print(len(xs));
print(xs[2]);
set(xs, 0, 9);
print(xs[0]);
```

### map<T>
```
let m: map<str> = map_new_string();
map_set(m, "name", "Alice");
print(map_get(m, "name"));
print(map_has(m, "name"));
print(map_size(m));
let keys: array<str> = map_keys(m);
print(len(keys));
```
```
let mi: map<int> = map_new_int();
map_set(mi, "count", 1);
print(map_get(mi, "count"));
```

### obj
```
let user: obj = {
  "name": "Alice",
  "meta": {
    "age": 30,
    "city": "NY"
  }
};
print(user.name);
print(user.meta.city);
```

### any
```
let v: any = 1;
print(v);
v = "hi";
print(v);
v = { "k": "v" };
print(v.k);
```

### buffer
```
let buf: buffer = buffer_new(3);
buffer_set(buf, 0, 65);
buffer_set(buf, 1, 66);
buffer_set(buf, 2, 67);
print(buffer_len(buf));
print(buffer_to_string(buf));
```

### class
```
class Box {
  let x: int;
  fn inc(self: Box) -> int {
    self.x = self.x + 1;
    return 0;
  }
}
let b: Box = new Box();
b.inc();
print(b.x);
```

---

## Variables
- Declare and initialize: `let name: T = expr;`
- Assign: `name = expr;`
- Scope: lexical, block-based

```
let a: int = 1;
let b: flt = 1.5e1;
let s: str = "hello";
let flag: bool = true;
a = a + 2;
print(a);
print(b);
print(s);
print(flag);
```

---

## Expressions and Operators
- Arithmetic `+ - * / %` with `int` and `flt`
- Unary `-` (`int`/`flt`), `!` (`bool`), `~` (`int`)
- Comparisons: `< <= > >= == !=`
- Logical: `&& ||` and keyword aliases `and or`
- Bitwise: `& | ^ << >>` on `int`
- Indexing: `arr[i]`
- Member access: `obj.field`, `object.field`
- Method calls: `object.method(args)`
- Construction: `new Class()`
- Useful groups:
  - Arithmetic: `+ - * / %`
  - Unary: `- ! ~`
  - Comparison: `< <= > >= == !=`
  - Logical: `&& ||` (`and`, `or`)
  - Bitwise: `& | ^ << >>`
  - String concatenation: `+`

### Arithmetic
```
print(5 + 3);
print(10 - 4);
print(6 * 7);
print(15 / 3);
print(10 % 3);
```

### Logical
```
print(true && false);
print(true or false);
```

### Comparisons
```
print(3 < 5);
print(5 <= 5);
print(7 > 3);
print(5 >= 4);
print(5 == 5);
print(5 != 3);
```

### Bitwise
```
print(~5);
print(5 & 1);
print(5 | 1);
print(5 ^ 1);
print(5 << 1);
print(5 >> 1);
```

### Strings
```
let s1: str = "hi";
let s2: str = " there";
print(s1 + s2);
```

### Postfix ++ / --
```
let x: int = 1;
x++;
print(x);
let y: int = 2;
y--;
print(y);
```

### Numerics and Trig
```
let a: flt = 1.5e1;
print(a);
print(sin(0));
print(cos(0));
print(tan(0));
let neg: flt = -3.25;
print(neg);
```

---

## Control Flow
- `if cond { ... } else { ... }`
- `while cond { ... }`
- `do { ... } while cond;`
- `for init; cond; post { ... }`
- `for var in array { ... }`
- `for key in map { ... }`
- `match expr { ... }`
- `retry;`
- `try { ... } catch err { ... }`
- `throw expr;`
- `return expr;`

### If / Else
```
if 5 == 5 {
  print(true);
} else {
  print(false);
}
```

### While
```
let x: int = 0;
while x < 3 {
  print(x);
  x = x + 1;
}
```

### Do-While
```
do {
  print(x);
  x = x + 1;
} while x < 5;
```

### For (C-Style)
```
for let i: int = 0; i < 3; i++ {
  print(i);
}
```

### Foreach (Array)
```
let xs: array<int> = [1, 2, 3, 4];
for n in xs {
  if n == 2 { retry; }
  print(n);
}
```

### Foreach (Map Keys)
```
let m: map<str> = map_new_string();
map_set(m, "a", "1");
map_set(m, "b", "2");
for k in m {
  print(k);
}
```

### Match
```
match x {
  case 5: {
    print("five");
  }
  default: {
    print("other");
  }
}
```

### Exceptions
```
try {
  throw "boom";
} catch err {
  print(err);
}
```

---

## Functions
- `fn name(params) -> Ret { ... }`
- Return any type (int, str, flt, bool, array, map, obj, class)
- `main()` return value is printed automatically

```
fn add(a: int, b: int) -> int {
  return a + b;
}

let r: int = add(2, 3);
print(r);

fn greet(name: str) -> str {
  return "hello " + name;
}

print(greet("George"));

fn num() -> flt {
  return 1.5;
}

print(num());
```

### Return Types
#### bool
```
fn flag() -> bool { return true; }
print(flag());
```

#### array<int>
```
fn arr() -> array<int> { return [1, 2]; }
print(arr());
```

#### map<str>
```
fn mk() -> map<str> {
  let m: map<str> = map_new_string();
  map_set(m, "k", "v");
  return m;
}
let m: map<str> = mk();
print(map_get(m, "k"));
```

#### obj
```
fn make(name: str) -> obj {
  return { "name": name };
}
let o: obj = make("Alice");
print(o.name);
```

#### buffer
```
fn make_buf() -> buffer {
  return buffer_from_string("hi");
}
let b: buffer = make_buf();
print(buffer_to_string(b));
```

#### class
```
class Box { let x: int; }
fn mk() -> Box {
  let z: Box = new Box();
  return z;
}
let b: Box = mk();
print(b.x);
```

#### any
```
fn id(x: any) -> any { return x; }
print(id(42));
print(id("hello"));
print(id({ "a": 1 }).a);
```

---

## Lambdas and IIFE
- Lambda: `fn (a: int) -> int { return a + 1; }`
- Assign and call:

```
let inc: obj = fn (a: int) -> int {
  return a + 1;
};

print(inc(2));
```

- IIFE:
```
let iife: int =
  fn (a: int, b: int) -> int {
    return a + b;
  }(2, 3);

print(iife);
```

---

## Classes and Objects
### Class Definition and Methods
```
class Box {
  let x: int;

  fn init(self: Box) -> int {
    return 0;
  }

  fn inc(self: Box) -> int {
    self.x = self.x + 1;
    return 0;
  }
}

let b: Box = new Box();
b.inc();
print(b.x);
```

---

## Dynamic Objects (`obj`)
### Object Literals and Nested Access
```
let user: obj = {
  "name": "Alice",
  "meta": {
    "age": 30,
    "city": "NY"
  }
};

print(user.name);
print(user.meta.city);

let names: obj = {
  "name":  "George",
  "name2": "John"
};

print(names.name);
print(names.name2);
```

---

## Arrays and Maps
### Arrays
```
let arr: array<int> = [1, 2, 3];
print(len(arr));
push(arr, 4);
print(arr[2]);
set(arr, 1, 5);
print(arr[1]);
let last: int = pop(arr);
print(last);
```

### Maps
```
let m: map<str> = map_new_string();
map_set(m, "k", "v");
print(map_get(m, "k"));
```

#### Map Operations
```
let m: map<str> = map_new_string();
map_set(m, "name", "Alice");
print(map_has(m, "name"));

let keys: array<str> = map_keys(m);
print(len(keys));
print(map_size(m));

map_remove(m, "name");
print(map_has(m, "name"));

map_clear(m);
print(map_size(m));
```

---

## Strings
### Interpolation and Utilities
```
let name: str = "George";
print("hello !{name}");
print("hello " + name);
print(strlen(name));
print(substr(name, 0, 3));
print(index_of(name, "or"));
print(string_replace(name, "George", "Greg"));

let parts: array<str> = string_split("a,b,c", ",");
print(len(parts));

print(to_int("42"));
print(to_string(7));
```

---

## Buffers
Mutable byte arrays with conversion helpers:
- `buffer_new(size)`, `buffer_len(buf)`
- `buffer_get(buf, i)`, `buffer_set(buf, i, b)`, `buffer_push(buf, b)`
- `buffer_from_string(s)`, `buffer_to_string(buf)`

```
let buf: buffer = buffer_new(3);
buffer_set(buf, 0, 65);
buffer_set(buf, 1, 66);
buffer_set(buf, 2, 67);
print(buffer_len(buf));
print(buffer_to_string(buf));

let buf2: buffer = buffer_from_string("hi");
buffer_push(buf2, 33);
print(buffer_to_string(buf2));
print(buffer_get(buf2, 0));
```

---

## IO: Files, JSON, TOML, ENV
Domains and functions:
- Files: `read_file`, `write_file`, `exists`, `mkdir`
- JSON: `read_json`, `write_json`, `json_get`, `json_set`
- TOML: `read_toml`, `write_toml`, `toml_get`, `toml_set`
- ENV: `read_env`, `write_env`, `env_get`, `env_set`

```
write_file("out.txt", "hello");
let content: str = read_file("out.txt");
print(content);

let j = read_json("data.json");
print(json_get(j, "key"));
json_set(j, "key", "new");
write_json("data.json", j);

let t = read_toml("conf.toml");
print(toml_get(t, "db.port"));
toml_set(t, "db.port", 5433);
write_toml("conf.toml", t);

let e = read_env(".env");
print(env_get(e, "TOKEN"));
env_set(e, "TOKEN", "abc123");
write_env(".env", e);
```

---

## Imports
```
import "import_functions.ax";
print(add(2, 3));
```

Use `run_file(path)` to execute a file with imports.

---

## REPL and Debug
- REPL: `cargo run -- repl` (commands: `:load`, `:env`, `:quit`)
- Debug flags: `--dump-tokens`, `--dump-ast`

```
fn main() -> int { return 0; }
```

---

## Printing
- Arrays: `[1, 2]`
- Maps: `{key: value}`
- Objects: `Class{field: value}`
- Floats: six fractional digits
- Buffers: `<buffer len=N>`

```
print([1, 2]);

let o: obj = {
  "x": 1
};
print(o);

let f: flt = 1.5;
print(f);

let mm: map<str> = map_new_string();
map_set(mm, "k", "v");
print(mm);

let oo: obj = {
  "user": {
    "name": "Alice"
  },
  "nums": [1, 2]
};
print(oo);
```

---

## Comments
Single-line:
```
// this is a comment
let x: int = 1; // inline comment after code
print(x);
```

Multi-line block using triple slashes:
```
///
this is a multi-line
comment block
///
let y: int = 2;
print(y);
```

---

## Examples
See `examples/`:
- `functions_returns.ax`
- `objects.ax`
- `operators.ax`
- `comparisons.ax`
- `bitwise.ax`
- `logical.ax`
- `loops_do_for.ax`
- `iife_and_buffers.ax`
- `floats_trig.ax`
