---
id: py/load-before-global-declaration
title: "global comes before the name is used"
languages: [python]
fix: manual
fix_reason: "moving the declaration reorders the function's statements"
fix_exception: restructure
stability: preview
ported_from: [ruff/PLE0118]
since: 0.5.0
---

A name is not used in a function before the function's `global` statement for it.

**Why:** That is a SyntaxError.

```codelaw
match:
  kind: global_statement
  has: {kind: identifier, pattern: $N}
  follows: {stopBy: end, has: {stopBy: end, kind: identifier, pattern: $N}}
  inside: {kind: block, inside: {kind: function_definition}}
message: "`$N` is used before this `global` declaration"
```

## How to fix

1. Move the `global` statement to the top of the function.

## Examples

### Python

#### Don't

```python
def bump():
    print(count)
    global count
    count += 1
```

#### Do

```python
def bump():
    global count
    print(count)
    count += 1
```
