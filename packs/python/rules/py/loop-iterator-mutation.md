---
id: py/loop-iterator-mutation
title: "No changing a list while looping over it"
languages: [python]
fix: manual
fix_reason: "whether to loop over a copy or build a new list is the author's"
fix_exception: restructure
stability: preview
ported_from: [ruff/B909]
since: 0.5.0
---

A `for` loop's iterable is not appended to, removed from or cleared inside the loop.

**Why:** Changing a list while iterating skips or repeats items.

```codelaw
match:
  kind: call
  has:
    field: function
    kind: attribute
    all:
    - has: {field: object, pattern: $L}
    - has: {field: attribute, regex: ^(append|extend|insert|remove|pop|clear|discard|add|update|popitem|setdefault)$}
  inside:
    kind: block
    stopBy: end
    inside:
      kind: for_statement
      has: {field: right, pattern: $L}
  not:
    inside:
      kind: function_definition
      stopBy: {kind: for_statement}
constraints:
  L:
    any:
    - {kind: identifier}
    - {kind: attribute}
message: '`$L` is changed while the loop iterates over it'
```

## How to fix

1. Loop over a copy (`for x in list($L):`) or build a new list.

## Examples

### Python

#### Don't

```python
for item in items:
    if item < 0:
        items.remove(item)
```

#### Do

```python
items = [item for item in items if item >= 0]
```
