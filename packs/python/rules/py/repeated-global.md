---
id: py/repeated-global
title: "One global statement"
languages: [python]
fix: auto
stability: preview
ported_from: [ruff/FURB154]
since: 0.5.0
---

Consecutive `global` (or `nonlocal`) statements are merged into one.

**Why:** One statement lists the names together.

```codelaw
match:
  kind: global_statement
  has: {kind: identifier, pattern: $N}
  regex: '^global\s+\w+$'
  follows: {kind: global_statement, pattern: $PREV}
message: "Merge into the `global` above"
fix:
  - remove: $$MATCH
  - replace: {target: $PREV, with: "$PREV, $N"}
```

## Examples

### Python

#### Don't

```python
def f():
    global a
    global b
```

#### Do

```python
def f():
    global a, b
```
