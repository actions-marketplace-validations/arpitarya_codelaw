---
id: py/unnecessary-map
title: "A generator, not map(lambda ...)"
languages: [python]
fix: suggest
fix_reason: "the generator evaluates lazily like `map`; review a lambda that captured a loop variable"
stability: preview
ported_from: [ruff/C417]
since: 0.5.0
---

`map(lambda x: expr, y)` is written `(expr for x in y)`.

**Why:** The generator says the same without a function call per item.

```codelaw
match:
  pattern: "map(lambda $X: $E, $Y)"
constraints:
  X: {regex: '^\w+$'}
message: "Write `($E for $X in $Y)`"
fix: "($E for $X in $Y)"
```

## Examples

### Python

#### Don't

```python
doubled = map(lambda n: n * 2, nums)
```

#### Do

```python
doubled = (n * 2 for n in nums)
```
