---
id: py/bad-exception-cause
title: "An exception is chained from an exception or None"
languages: [python]
fix: manual
fix_reason: "which exception caused it is the author's"
fix_exception: value
stability: preview
ported_from: [pylint/bad-exception-cause]
since: 0.5.0
---

`raise X from cause` names an exception (or `None`) as its cause, not a string, a number or another literal.

**Why:** Python raises `TypeError` at the `raise` when the cause is not an exception.

```codelaw
match:
  kind: raise_statement
  has: {field: cause, any: [{kind: string}, {kind: integer}, {kind: float}, {kind: 'true'}, {kind: 'false'}, {kind: list}, {kind: dictionary}, {kind: tuple}]}
message: "The cause must be an exception or `None`"
```

## How to fix

1. Chain from the caught exception (`from err`) or `from None`.

## Examples

### Python

#### Don't

```python
raise ValueError("bad") from "parse"
```

#### Do

```python
raise ValueError("bad") from err
```
