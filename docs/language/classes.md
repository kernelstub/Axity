# Classes

## Overview
- Define fields and methods
- Construct with `new ClassName(args?)`
- Optional constructor: `fn init(self: Class, ...) -> int` auto-called on `new`

## Syntax
```
class Point {
    let x: int;
    let y: int;
    fn move(self: Point, dx: int, dy: int) -> int {
        self.x = self.x + dx;
        self.y = self.y + dy;
        return 0;
    }
}
```

## Method Calls
- `obj.method(args...)`
- `self` is the first parameter and must match the class
- Arguments must match parameters after `self`

## Example
```
let p: Point = new Point;
p.move(2, 3);
print(p.x);   // 2
print(p.y);   // 3
```

