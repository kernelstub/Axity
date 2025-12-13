# REPL

## Overview
- Interactive mode to evaluate expressions and statements
- Environment persists across commands

## Start
```
axity repl
```

## Commands
- `:load path` — load and run a file (imports resolved)
- `:env` — print current environment (variables in scopes)
- `:quit` — exit

## Example
```
axity> let x: int = 1;
axity> print(x);
1
axity> :env
scope 0:
  x = 1
axity> :quit
```
