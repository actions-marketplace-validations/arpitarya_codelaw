---
id: py/too-many-boolean-expressions
title: "An if condition joins at most `max` Boolean terms"
languages: [python]
fix: manual
fix_reason: "splitting the code is a design choice: which parts belong together is the author's"
fix_exception: restructure
stability: preview
options:
  max: { type: "integer", default: 5, description: "the most terms an `if` or `elif` condition may join with `and`/`or`" }
ported_from: [ruff/PLR0916, pylint/too-many-boolean-expressions]
since: 0.5.0
---

An `if` or `elif` whose condition joins more than `max` terms with `and` and `or` (5 by default), counted through nested and parenthesised groups, is reported on the condition. Conditions elsewhere (a `while`, an assignment) are not read, as in Ruff.

**Why:** A long condition hides which case it is for; named parts (`is_admin = …`) or a helper function say it.

```codelaw
native: py/too-many-boolean-expressions
```

## How to fix

1. Name parts of the condition, or move it into a function with a name.

## Examples

### Python

#### Don't

```python options={"max":3}
if a and b and (c or d):
    go()
```

#### Do

```python options={"max":3}
ready = a and b
if ready and (c or d):
    go()
```
