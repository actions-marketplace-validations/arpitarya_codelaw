---
id: py/unnecessary-assign-before-yield
title: "Yield the value, not a name assigned just before"
languages: [python]
fix: auto
stability: preview
ported_from: [ruff/RUF070]
since: 0.5.0
---

`x = value` immediately followed by `yield x` (and no other use of `x`) is written `yield value`.

**Why:** The name holds the value for one line and adds nothing.

```codelaw
match:
  kind: expression_statement
  has: {kind: yield, pattern: yield $X}
  follows: {kind: expression_statement, pattern: $ASSIGN, has: {kind: assignment, pattern: $X = $V}}
  not: {precedes: {stopBy: end, has: {stopBy: end, kind: identifier, pattern: $X}}}
constraints:
  X: {kind: identifier}
message: "Yield the value directly"
fix:
  - remove: $ASSIGN
  - replace: "yield $V"
```

## Examples

### Python

#### Don't

```python
def gen():
    item = make()
    yield item
```

#### Do

```python
def gen():
    yield make()
```
