---
id: py/unrecognized-platform-name
title: "A platform check in a stub names a known platform"
languages: [python]
files: ["**/*.pyi"]
fix: manual
fix_reason: "which platform was meant is the author's"
fix_exception: value
stability: preview
ported_from: [ruff/PYI008]
since: 0.5.0
---

`sys.platform == "…"` in a stub names one of the platforms type checkers know: `linux`, `win32`, `cygwin`, `darwin`.

**Why:** A misspelt or unknown platform makes the branch dead for every checker.

```codelaw
match:
  kind: string
  pattern: $S
  not: {regex: '^["''](linux|win32|cygwin|darwin)["'']$'}
  inside:
    kind: comparison_operator
    has: {nthChild: 1, regex: '^sys\.platform$'}
    regex: '^sys\.platform\s*(==|!=)'
message: "`$S` is not a platform type checkers know"
```

## How to fix

1. Use one of `linux`, `win32`, `cygwin` or `darwin`.

## Examples

### Python

#### Don't

```python path=api.pyi
if sys.platform == "linux2":
    def f() -> None: ...
```

#### Do

```python path=api.pyi
if sys.platform == "linux":
    def f() -> None: ...
```
