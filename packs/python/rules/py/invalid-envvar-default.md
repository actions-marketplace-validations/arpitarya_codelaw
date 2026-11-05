---
id: py/invalid-envvar-default
title: "os.getenv's default is a string"
languages: [python]
fix: suggest
fix_reason: "the default becomes a string, like every value read from the environment; code that used it as a number needs a conversion"
stability: preview
ported_from: [ruff/PLW1508]
since: 0.5.0
---

The default given to `os.getenv` or `os.environ.get` is a string or `None`, not a number (Ruff also reports other literal types; numbers are covered).

**Why:** Environment values are strings; a non-string default makes the result's type depend on the environment.

```codelaw
match:
  kind: call
  all:
  - has: {field: function, regex: ^os\.(getenv|environ\.get)$}
  - has:
      field: arguments
      has:
        nthChild: 2
        any:
        - {kind: integer}
        - {kind: float}
        pattern: $D
message: Give a string default; environment values are strings
fix:
- replace: {target: $D, with: '"$D"'}
```

## Examples

### Python

#### Don't

```python
n = os.getenv("WORKERS", 4)
```

#### Do

```python
n = os.getenv("WORKERS", "4")
```
