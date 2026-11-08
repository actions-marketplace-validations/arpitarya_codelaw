---
id: py/consider-using-enumerate
title: "enumerate, not range(len(...))"
languages: [python]
fix: manual
fix_reason: "the item's name is the author's"
fix_exception: value
stability: preview
ported_from: [pylint/consider-using-enumerate]
since: 0.5.0
---

`for i in range(len(xs)):` whose body reads `xs[i]` iterates `enumerate(xs)` instead.

**Why:** `enumerate` gives the index and the item together, without indexing.

```codelaw
match:
  kind: for_statement
  all:
    - has: {field: left, kind: identifier, pattern: $I}
    - has: {field: right, pattern: range(len($L))}
    - has: {field: body, has: {kind: subscript, stopBy: end, pattern: '$L[$I]'}}
message: "Iterate `enumerate($L)`"
```

## How to fix

1. Write `for i, item in enumerate(xs):` and use `item` for `xs[i]`.

## Examples

### Python

#### Don't

```python
for i in range(len(names)):
    print(i, names[i])
```

#### Do

```python
for i, name in enumerate(names):
    print(i, name)
```
