---
id: py/unnecessary-double-cast-or-process
title: "No list() or tuple() inside a call that takes any iterable"
languages: [python]
fix: auto
stability: preview
ported_from: [ruff/C414]
since: 0.5.0
---

`list`, `tuple`, `set` and `sorted` are not called on the result of `list(x)` or `tuple(x)`: `set(list(x))` is `set(x)`.

**Why:** The inner call copies the iterable only for the outer one to copy it again.

```codelaw
match:
  pattern: $F($G($X))
constraints:
  F: {regex: '^(list|tuple|set|sorted)$'}
  G: {regex: '^(list|tuple)$'}
  X: {not: {any: [{kind: keyword_argument}, {kind: list_splat}, {kind: dictionary_splat}]}}
message: "Drop the inner `$G()`"
fix: "$F($X)"
```

## Examples

### Python

#### Don't

```python
unique = set(list(items))
```

#### Do

```python
unique = set(items)
```
