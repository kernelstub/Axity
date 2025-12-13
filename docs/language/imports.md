# Imports

## Overview
- Import other `.ax` files with a relative path
- Imports are resolved relative to the importing file

## Syntax
```
import "other.ax";
```

## Execution
- The import resolver loads and merges items before type checking and interpretation
- Circular imports are ignored after the first load

## Example
```
// examples/import_functions.ax
fn add(a: int, b: int) -> int { return a + b; }

// examples/import_main.ax
import "import_functions.ax";
print(add(2, 3));   // 5
```

