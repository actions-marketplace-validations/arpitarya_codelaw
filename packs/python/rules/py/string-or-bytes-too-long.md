---
id: py/string-or-bytes-too-long
title: "No long string literals in stubs"
languages: [python]
files: ["**/*.pyi"]
fix: auto
stability: preview
ported_from: [ruff/PYI053]
since: 0.5.0
---

A string or bytes literal longer than 50 characters in a stub is written `...`.

**Why:** Long values are implementation detail, not interface.

```codelaw
match:
  kind: string
  regex: ^.{53,}$
  all:
  - not:
      inside:
        any:
        - {kind: type}
        - {kind: decorator}
        - {kind: subscript}
  - not:
      inside: {kind: expression_statement, nthChild: 1}
message: Write `...`
fix: '...'
```

## Examples

### Python

#### Don't

```python path=api.pyi
def f(x: str = "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa") -> None: ...
```

#### Do

```python path=api.pyi
def f(x: str = ...) -> None: ...
```
