---
id: py/invalid-star-assignment-target
title: "A starred name is not the whole assignment target"
languages: [python]
fix: suggest
fix_reason: "the target becomes a one-item tuple, which binds a list; whether a list was what the author wanted is for them to accept"
stability: preview
ported_from: [pylint/invalid-star-assignment-target]
since: 0.5.0
---

`*rest = items` is a `SyntaxError` when the module is compiled, though the parser accepts it. A starred target must be inside a tuple or list: `(*rest,) = items` or `[*rest] = items`.

**Why:** The module does not import at all; the error shows up only when it runs.

```codelaw
match:
  kind: assignment
  has: {field: left, kind: list_splat_pattern, pattern: $T}
message: "A starred target must be inside a tuple or list"
fix:
  - replace: { target: $T, with: "($T,)" }
```

## Examples

### Python

#### Don't

```python
*rest = load()
```

#### Do

```python
(*rest,) = load()
```
