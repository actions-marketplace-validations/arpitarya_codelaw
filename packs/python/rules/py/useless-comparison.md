---
id: py/useless-comparison
title: "No comparison whose result is thrown away"
languages: [python]
fix: suggest
fix_reason: "the choices are an assignment (`==` written for `=`), an `assert`, and deleting the statement; each changes what runs (an assignment, a check, or no `__eq__` call), so none is the default"
stability: preview
ported_from: [ruff/B015]
since: 0.1.0
---

A comparison isn't written as a statement on its own.

**Why:** Its result is discarded; usually `=` or `assert` was meant.

```codelaw
native: py/useless-comparison
```

## How to fix

1. Use `=` if an assignment was meant, or `assert` if a check was.
2. Otherwise remove the line.

## Examples

### Python

#### Don't

```py
def reset(state):
    state.count == 0
```

#### Fixed (assign)

```py
def reset(state):
    state.count = 0
```

#### Fixed (assert)

```py
def reset(state):
    assert state.count == 0
```

#### Fixed (delete)

```py
def reset(state):
    pass
```

#### Do

```py
def reset(state):
    state.count = 0
```
