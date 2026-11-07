---
id: py/type-check-without-type-error
title: "A failed type check raises TypeError"
languages: [python]
fix: suggest
fix_reason: "a caller that catches the old exception no longer catches this one"
stability: preview
ported_from: [ruff/TRY004]
since: 0.5.0
---

An `if` whose test is an `isinstance`, `issubclass` or `callable` check, and whose body only raises, raises `TypeError` rather than another builtin exception.

**Why:** `TypeError` is the exception for a value of the wrong type; callers catch it by that name.

```codelaw
match:
  kind: identifier
  pattern: $E
  regex: '^(Exception|ValueError|RuntimeError|AssertionError|LookupError|KeyError|IndexError|AttributeError|ArithmeticError|NotImplementedError)$'
  any:
    - inside: {kind: raise_statement}
    - inside: {kind: call, field: function, inside: {kind: raise_statement}}
  inside:
    kind: raise_statement
    stopBy: end
    inside:
      kind: block
      not: {has: {nthChild: 2, not: {kind: comment}}}
      inside:
        any: [{kind: if_statement}, {kind: elif_clause}]
        has: {field: condition, has: {kind: call, stopBy: end, has: {field: function, regex: '^(isinstance|issubclass|callable)$'}}}
message: "Raise `TypeError` for a value of the wrong type"
fix: TypeError
```

## Examples

### Python

#### Don't

```python
def area(r):
    if not isinstance(r, Rect):
        raise ValueError("not a rectangle")
    return r.w * r.h
```

#### Do

```python
def area(r):
    if not isinstance(r, Rect):
        raise TypeError("not a rectangle")
    return r.w * r.h
```
