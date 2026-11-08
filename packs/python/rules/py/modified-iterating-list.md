---
id: py/modified-iterating-list
title: "A list is not changed while it is iterated"
languages: [python]
fix: manual
fix_reason: "the loop builds a new list, or iterates a copy"
fix_exception: restructure
stability: preview
ported_from: [pylint/modified-iterating-list]
since: 0.5.0
---

A `for x in items:` loop does not `append`, `remove`, `insert`, `extend` or `pop` on `items` in its body.

**Why:** The loop then skips or repeats items; with `append`, it may never end.

```codelaw
match:
  kind: for_statement
  all:
    - has: {field: right, kind: identifier, pattern: $L}
    - has:
        field: body
        has:
          stopBy: {any: [{kind: function_definition}, {kind: lambda}]}
          kind: call
          any: [{pattern: '$L.append($$$A)'}, {pattern: '$L.remove($$$A)'}, {pattern: '$L.insert($$$A)'}, {pattern: '$L.extend($$$A)'}, {pattern: '$L.pop($$$A)'}]
message: "`$L` changes while it is iterated"
```

## How to fix

1. Build a new list (a comprehension) or iterate over a copy (`items[:]`).

## Examples

### Python

#### Don't

```python
for x in items:
    if not x:
        items.remove(x)
```

#### Do

```python
items = [x for x in items if x]
```
