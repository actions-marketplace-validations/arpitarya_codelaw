---
id: py/docstring-in-stub
title: "No docstrings in stubs"
languages: [python]
files: ["**/*.pyi"]
fix: auto
stability: preview
ported_from: [ruff/PYI021]
since: 0.5.0
---

A stub's functions and classes have no docstrings.

**Why:** Docstrings belong with the implementation; tools read them from there.

```codelaw
match:
  kind: expression_statement
  has: {kind: string, nthChild: 1}
  not: {has: {nthChild: 2}}
  nthChild: 1
  inside: {kind: block, has: {nthChild: 2}, inside: {any: [{kind: function_definition}, {kind: class_definition}]}}
message: "Remove the docstring from the stub"
fix: [remove]
```

## Examples

### Python

#### Don't

```python path=api.pyi
def f(x: int) -> int:
    """Do it."""
    ...
```

#### Do

```python path=api.pyi
def f(x: int) -> int:
    ...
```
