---
id: py/list-reverse-copy
title: "x.reverse(), not x = x[::-1]"
languages: [python]
fix: suggest
fix_reason: "`reverse` changes the list in place, which other references to it now see"
stability: preview
ported_from: [ruff/FURB187]
since: 0.5.0
---

`items = items[::-1]` is written `items.reverse()`.

**Why:** Reversing in place does not copy the list.

```codelaw
match:
  kind: expression_statement
  has: {kind: assignment, pattern: '$X = $X[::-1]'}
constraints:
  X: {kind: identifier}
message: "Write `$X.reverse()`"
fix: "$X.reverse()"
```

## Examples

### Python

#### Don't

```python
items = items[::-1]
```

#### Do

```python
items.reverse()
```
