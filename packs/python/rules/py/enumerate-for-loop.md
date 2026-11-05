---
id: py/enumerate-for-loop
title: "enumerate(), not a hand-kept counter"
languages: [python]
fix: manual
fix_reason: "the counter's start and its other uses decide the rewrite"
fix_exception: restructure
stability: preview
ported_from: [ruff/SIM113]
since: 0.5.0
---

A counter set to `0` before a `for` loop and incremented by one at the end of its body is replaced by `enumerate`.

**Why:** `enumerate` keeps the index with the item and can't be skipped by a `continue`.

```codelaw
match:
  kind: for_statement
  all:
    - has: {field: body, has: {kind: expression_statement, nthChild: {position: 1, reverse: true}, has: {kind: augmented_assignment, pattern: $I += 1}}}
    - follows: {kind: expression_statement, has: {kind: assignment, pattern: $I = 0}}
    - not: {has: {stopBy: end, kind: continue_statement}}
constraints:
  I: {kind: identifier}
message: "Use `enumerate` instead of the counter `$I`"
```

## How to fix

1. Write `for $I, item in enumerate(items):` and remove the counter's assignment and increment.

## Examples

### Python

#### Don't

```python
i = 0
for name in names:
    print(i, name)
    i += 1
```

#### Do

```python
for i, name in enumerate(names):
    print(i, name)
```
