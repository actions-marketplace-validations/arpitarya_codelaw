---
id: py/non-empty-stub-body
title: "A stub function has no body"
languages: [python]
files: ["**/*.pyi"]
fix: auto
stability: preview
ported_from: [ruff/PYI010]
since: 0.5.0
---

A stub function's body is only `...`.

**Why:** A stub declares a signature; a body is never run and only misleads.

```codelaw
match:
  kind: block
  all:
  - inside: {kind: function_definition, field: body}
  - not: {regex: '^(\.\.\.|pass|"""[\s\S]*""")$'}
  - not:
      has: {kind: expression_statement, regex: '^(\.\.\.|"""[\s\S]*""")$', nthChild: 1}
message: A stub's body is `...`
fix: '...'
```

## Examples

### Python

#### Don't

```python path=api.pyi
def f(x: int) -> int:
    return x
```

#### Do

```python path=api.pyi
def f(x: int) -> int:
    ...
```
