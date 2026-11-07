---
id: py/unrecognized-platform-check
title: "A platform check in a stub is == or !="
languages: [python]
files: ["**/*.pyi"]
fix: manual
fix_reason: "the test is rewritten as one equality per platform"
fix_exception: restructure
stability: preview
ported_from: [ruff/PYI007]
since: 0.5.0
---

A stub tests `sys.platform` only with `==` or `!=` against a string: not `sys.platform.startswith(...)`, not `<` or `in`.

**Why:** Type checkers understand only an equality test of `sys.platform`.

```codelaw
match:
  any:
    - kind: comparison_operator
      has: {nthChild: 1, regex: '^sys\.platform$'}
      not: {regex: '^sys\.platform\s*(==|!=)\s*["'']'}
    - kind: call
      has: {field: function, regex: '^sys\.platform\.\w+$'}
message: "Test `sys.platform` with `==` or `!=`"
```

## How to fix

1. Rewrite the test as `sys.platform == "<platform>"` (or `!=`), one comparison per platform, joined with `or` in separate branches if needed.

## Examples

### Python

#### Don't

```python path=api.pyi
if sys.platform.startswith("linux"):
    def f() -> None: ...
```

#### Do

```python path=api.pyi
if sys.platform == "linux":
    def f() -> None: ...
```
