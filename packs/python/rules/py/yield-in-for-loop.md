---
id: py/yield-in-for-loop
title: "yield from, not a loop that yields each item"
languages: [python]
fix: suggest
fix_reason: "`yield from` also forwards `send()` and `throw()` to the inner iterator"
stability: preview
ported_from: [ruff/UP028, pylint/use-yield-from]
since: 0.5.0
---

`for x in y: yield x` is written `yield from y`.

**Why:** `yield from` delegates in one statement.

```codelaw
match:
  kind: for_statement
  pattern: "for $X in $Y:\n    yield $X\n"
  all:
  - not:
      has: {kind: else_clause}
  - not:
      inside: {kind: function_definition, stopBy: end, regex: ^async\s}
constraints:
  X: {kind: identifier}
message: Write `yield from $Y`
fix: yield from $Y
```

## Examples

### Python

#### Don't

```python
def gen(items):
    for item in items:
        yield item
```

#### Do

```python
def gen(items):
    yield from items
```
