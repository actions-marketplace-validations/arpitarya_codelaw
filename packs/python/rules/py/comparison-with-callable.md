---
id: py/comparison-with-callable
title: "A function is called before it is compared"
languages: [python]
fix: manual
fix_reason: "the arguments of the call are the author's"
fix_exception: value
stability: preview
ported_from: [pylint/comparison-with-callable]
since: 0.5.0
---

`==` or `!=` between the name of a plain module-level function of this file (one binding, no decorator) and a literal. The function object is never equal to a literal. A comparison of two names, or with something that is not a literal, is not read.

**Why:** `if is_ready == True:` compares the function object, not its result, and is always false.

```codelaw
native: py/comparison-with-callable
```

## How to fix

1. Call the function: `is_ready()`.

## Examples

### Python

#### Don't

```python
def is_ready():
    return True


if is_ready == True:
    pass
```

#### Do

```python
def is_ready():
    return True


if is_ready() == True:
    pass
```
