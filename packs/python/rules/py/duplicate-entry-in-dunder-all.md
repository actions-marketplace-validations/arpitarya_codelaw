---
id: py/duplicate-entry-in-dunder-all
title: "No name twice in __all__"
languages: [python]
fix: auto
stability: preview
ported_from: [ruff/RUF068]
since: 0.5.0
---

A name appears once in `__all__`.

**Why:** The duplicate adds nothing and suggests a different name was meant.

```codelaw
match:
  kind: string
  pattern: $S
  follows: {stopBy: end, kind: string, pattern: $S}
  inside:
    any: [{kind: list}, {kind: tuple}]
    inside: {kind: assignment, has: {field: left, regex: '^__all__$'}}
message: "$S is already in `__all__`"
fix: [remove]
```

## Examples

### Python

#### Don't

```python
__all__ = ["load", "save", "load"]
```

#### Do

```python
__all__ = ["load", "save"]
```
