---
id: py/redundant-final-literal
title: "No Final[Literal[...]] beside its own value"
languages: [python]
fix: auto
stability: preview
ported_from: [ruff/PYI064]
since: 0.5.0
---

`x: Final[Literal[1]] = 1` is written `x: Final = 1`: `Final` with a literal value already has that literal type.

**Why:** The annotation repeats the value.

```codelaw
match:
  kind: assignment
  pattern: '$N: Final[Literal[$V]] = $V'
  has: {field: type, pattern: $T}
message: "Write `Final` alone"
fix:
  - replace: {target: $T, with: Final}
```

## Examples

### Python

#### Don't

```python
LIMIT: Final[Literal[10]] = 10
```

#### Do

```python
LIMIT: Final = 10
```
