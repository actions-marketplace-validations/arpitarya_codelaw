---
id: py/redefined-variable-type
title: "A name keeps one type in its scope"
languages: [python]
fix: manual
fix_reason: "which of the two types the name should have is the author's"
fix_exception: restructure
stability: preview
ported_from: [pylint/redefined-variable-type]
since: 0.5.0
---

A name (or `obj.attr`) is assigned a value of one type and later, in the same scope, a value of another, where the file states both: a literal, a comprehension, a ternary whose arms agree, a call of a builtin type or of a class of this file, or a call of a function of this file whose every `return` states the same type. Scopes are the module, each function and each class body. The branches of an `if`/`elif`/`else`, `match`, `try`/`except` and loop bodies each start from the state before them, so `if c: x = 'a'` / `else: x = 1` is not reported; after them a name holds what the last branch that assigned it gave it. `None`, tuple targets, augmented and annotated assignments, a bare `_`, and anything the file does not state are not read.

**Why:** A reader (and a type checker) expects `total` to mean one kind of thing; `total = 0` then `total = '0'` is usually one of them being wrong.

```codelaw
native: py/redefined-variable-type
```

## How to fix

1. Use a second name for the second value, or
2. convert the value so the name keeps one type.

## Examples

### Python

#### Don't

```python
def parse(raw):
    count = 0
    count = "none"
    return count
```

#### Do

```python
def parse(raw):
    count = 0
    label = "none"
    return count, label
```
