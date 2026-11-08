---
id: py/possibly-unused-variable
title: "A variable the function may read through locals() is used"
languages: [python]
fix: manual
fix_reason: "whether the value is read through `locals()` on purpose is the author's"
fix_exception: value
stability: preview
ported_from: [pylint/possibly-unused-variable]
since: 0.5.0
---

A function assigns a plain local (a name with no leading underscore, not `unused_*`) that nothing reads, and calls `locals()` after the assignment, so the value may be read that way. The same variable with no `locals()` call after it is an ordinary unused variable.

**Why:** The name looks dead, but `locals()` may expose it, so removing it needs a check of what consumes the dictionary.

```codelaw
native: py/possibly-unused-variable
```

## How to fix

1. Read the variable, or
2. build the dictionary `locals()` returned by hand, or
3. remove the variable if nothing uses the dictionary's key.

## Examples

### Python

#### Don't

```python
def render():
    title = 'home'
    return locals()
```

#### Do

```python
def render():
    title = 'home'
    return {'title': title}
```
