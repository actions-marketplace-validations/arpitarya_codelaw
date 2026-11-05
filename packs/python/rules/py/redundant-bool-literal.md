---
id: py/redundant-bool-literal
title: "bool, not Literal[True, False]"
languages: [python]
fix: auto
stability: preview
ported_from: [ruff/RUF038]
since: 0.5.0
---

`Literal[True, False]` is written `bool`.

**Why:** Both values of `bool` are all of `bool`.

```codelaw
match:
  any: [{kind: subscript}, {kind: generic_type}]
  regex: '^(typing\.)?Literal\[\s*(True\s*,\s*False|False\s*,\s*True)\s*\]$'
message: "`Literal[True, False]` is `bool`"
fix: "bool"
```

## Examples

### Python

#### Don't

```python
def f(flag: Literal[True, False]) -> None:
    pass
```

#### Do

```python
def f(flag: bool) -> None:
    pass
```
