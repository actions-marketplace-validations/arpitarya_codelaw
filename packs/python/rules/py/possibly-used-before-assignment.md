---
id: py/possibly-used-before-assignment
title: "A name is bound on every branch before it is read"
languages: [python]
fix: manual
fix_reason: "what the skipped branch should leave in the name is the author's"
fix_exception: value
stability: preview
ported_from: [pylint/possibly-used-before-assignment]
since: 0.5.0
---

A name is read after an `if`, `elif` or `match` in which some branches bind it and others, or the missing `else`, do not. A branch that ends in `return`, `raise`, `continue`, `break` or `sys.exit()` is not a way to reach the read. A loop that may not run, and a `try` whose handler does not bind the name, are not reported (Pylint does not either). A later `if` with the same condition text as the one that bound the name is not reported, since the two agree.

**Why:** When the skipped branch is the one taken, the read raises `UnboundLocalError` or `NameError`.

```codelaw
native: py/possibly-used-before-assignment
```

## How to fix

1. Give the name a value before the `if`, or
2. add an `else` that binds it, or
3. move the read into the branch that binds it.

## Examples

### Python

#### Don't

```python
def label(count):
    if count > 0:
        text = 'some'
    return text
```

#### Do

```python
def label(count):
    text = 'none'
    if count > 0:
        text = 'some'
    return text
```
