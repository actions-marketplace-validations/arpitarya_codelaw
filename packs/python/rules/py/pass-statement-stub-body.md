---
id: py/pass-statement-stub-body
title: "`...`, not pass, in a stub body"
languages: [python]
files: ["**/*.pyi"]
fix: auto
stability: preview
ported_from: [ruff/PYI009]
since: 0.5.0
---

A stub function's body is `...`, not `pass`.

**Why:** `...` is the stub convention; `pass` reads as an implementation.

```codelaw
match:
  kind: pass_statement
  inside: {kind: block, not: {has: {nthChild: 2}}, inside: {kind: function_definition}}
message: "Write `...`"
fix: "..."
```

## Examples

### Python

#### Don't

```python path=api.pyi
def f(x: int) -> int:
    pass
```

#### Do

```python path=api.pyi
def f(x: int) -> int:
    ...
```
