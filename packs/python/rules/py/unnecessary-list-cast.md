---
id: py/unnecessary-list-cast
title: "No list() around a loop's iterable"
languages: [python]
fix: suggest
fix_reason: "without the copy, changing the iterable inside the loop affects the loop"
stability: preview
ported_from: [ruff/PERF101]
since: 0.5.0
---

`for x in list(items):` over a list, tuple or set literal or comprehension is written `for x in items:`.

**Why:** The copy costs memory and time for nothing.

```codelaw
match:
  kind: call
  pattern: list($X)
  inside: {kind: for_statement, field: right}
constraints:
  X: {any: [{kind: list}, {kind: tuple}, {kind: set}, {kind: list_comprehension}]}
message: "Iterate `$X` directly"
fix: "$X"
```

## Examples

### Python

#### Don't

```python
for x in list([1, 2, 3]):
    print(x)
```

#### Do

```python
for x in [1, 2, 3]:
    print(x)
```
