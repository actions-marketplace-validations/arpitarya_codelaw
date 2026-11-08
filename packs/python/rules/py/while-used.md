---
id: py/while-used
title: "No while loop"
languages: [python]
fix: manual
fix_reason: "the loop is rewritten over what it iterates"
fix_exception: restructure
stability: preview
ported_from: [pylint/while-used]
since: 0.5.0
---

Code iterates with `for`, not `while` (Pylint's opt-in `while_used` extension).

**Why:** A `for` loop states what it iterates and cannot forget to advance.

```codelaw
match:
  kind: while_statement
message: "Iterate with `for`"
```

## How to fix

1. Rewrite the loop as a `for` over a range, a sequence or an iterator.

## Examples

### Python

#### Don't

```python
i = 0
while i < n:
    i += 1
```

#### Do

```python
for i in range(n):
    pass
```
