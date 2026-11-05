---
id: py/global-statement
title: "No global statement"
languages: [python]
fix: manual
fix_reason: "passing the value in and out, or a class holding the state, is a design choice"
fix_exception: restructure
stability: preview
ported_from: [ruff/PLW0603]
since: 0.5.0
---

A function does not rebind module variables with `global`.

**Why:** Global state changed from inside functions makes the program's behaviour depend on call order.

```codelaw
match:
  kind: global_statement
  inside: {kind: function_definition, stopBy: end}
message: "Pass the value in and return it instead of using `global`"
```

## How to fix

1. Pass the value as a parameter and return the new value, or keep the state on an object.

## Examples

### Python

#### Don't

```python
def bump():
    global count
    count += 1
```

#### Do

```python
def bump(count):
    return count + 1
```
