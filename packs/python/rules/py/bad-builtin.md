---
id: py/bad-builtin
title: "No map or filter"
languages: [python]
fix: manual
fix_reason: "the call becomes a comprehension over the same items"
fix_exception: restructure
stability: preview
ported_from: [pylint/bad-builtin]
since: 0.5.0
---

The builtins `map` and `filter` are not called; a comprehension says the same (Pylint's opt-in `deprecated_builtins` extension).

**Why:** A comprehension reads in one pass and needs no lambda.

```codelaw
match:
  kind: call
  has: {field: function, kind: identifier, regex: '^(map|filter)$', pattern: $F}
where:
  $F: builtin
message: "Write a comprehension"
```

## How to fix

1. Rewrite `map(f, xs)` as `(f(x) for x in xs)` and `filter(p, xs)` as `(x for x in xs if p(x))`.

## Examples

### Python

#### Don't

```python
names = list(map(str.upper, words))
```

#### Do

```python
names = [w.upper() for w in words]
```
