# Maps

## Overview
- Type: `map<T>` with string keys
- Mutable by built-ins; shared by reference

## Built-ins
- `map_new_int() -> map<int>`
- `map_new_string() -> map<string>`
- `map_set(m, "key", value) -> int`
- `map_get(m, "key") -> T`
- `map_has(m, "key") -> bool`
- `map_keys(m) -> array<string>`

## Example
```
let m: map<int> = map_new_int();
map_set(m, "a", 10);
map_set(m, "b", 20);
print(map_get(m, "a"));   // 10
print(map_has(m, "c"));   // false
print(map_keys(m));       // ["a", "b"]
```

