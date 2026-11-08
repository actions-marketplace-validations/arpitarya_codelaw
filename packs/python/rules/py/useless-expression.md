---
id: py/useless-expression
title: "No expression statement that does nothing"
languages: [python]
fix: suggest
fix_reason: "the choices are deleting the statement and, when it ends a function, returning it; reading an attribute can run a property, so neither is automatic or the default. A statement sharing its line is manual"
fix_exception: restructure
stability: preview
ported_from: [ruff/B018, pylint/pointless-statement, pylint/expression-not-assigned]
since: 0.1.0
---

A statement isn't a bare name, number, attribute access or container literal whose value is thrown away.

**Why:** It has no effect; usually something was meant to be returned, assigned or called.

```codelaw
native: py/useless-expression
```

## How to fix

1. Return, assign or call the value as intended, or remove the line.

## Examples

### Python

#### Don't

```py
def name(user):
    user.name
    return user
```

#### Fixed

```py
def name(user):
    return user
```

#### Do

```py
def name(user):
    return user.name
```

## Differences from the source

Attribute access is reported only on plain `a.b.c` chains; Ruff also reports `f().attr`.
