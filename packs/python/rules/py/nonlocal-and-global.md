---
id: py/nonlocal-and-global
title: "No name both nonlocal and global"
languages: [python]
fix: manual
fix_reason: "which scope was meant is the author's"
fix_exception: value
stability: preview
ported_from: [ruff/PLE0115]
since: 0.5.0
---

A function does not declare the same name `nonlocal` and `global`.

**Why:** That is a SyntaxError.

```codelaw
match:
  kind: identifier
  pattern: $N
  inside: {kind: nonlocal_statement, inside: {kind: block, has: {kind: global_statement, has: {kind: identifier, pattern: $N}}}}
message: "`$N` cannot be both nonlocal and global"
```

## How to fix

1. Keep the declaration for the scope you mean and remove the other.

## Examples

### Python

#### Don't

```python
def outer():
    x = 1
    def inner():
        global x
        nonlocal x
```

#### Do

```python
def outer():
    x = 1
    def inner():
        nonlocal x
```
