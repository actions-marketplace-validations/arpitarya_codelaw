---
id: py/redefined-loop-name
title: "No reassigning the loop variable"
languages: [python]
fix: manual
fix_reason: "the new name is the author's"
fix_exception: value
stability: preview
ported_from: [ruff/PLW2901]
since: 0.5.0
---

A `for` loop's variable is not reassigned inside the loop body.

**Why:** The next iteration overwrites it anyway, and readers lose track of which value it holds.

```codelaw
match:
  kind: assignment
  has: {field: left, kind: identifier, pattern: $X}
  inside:
    kind: block
    stopBy: end
    inside: {kind: for_statement, has: {field: left, kind: identifier, pattern: $X}}
  not: {inside: {kind: function_definition, stopBy: {kind: for_statement}}}
message: "`$X` is the loop variable; assign to a new name"
```

## How to fix

1. Bind the new value to a different name.

## Examples

### Python

#### Don't

```python
for line in lines:
    line = line.strip()
    print(line)
```

#### Do

```python
for line in lines:
    text = line.strip()
    print(text)
```
