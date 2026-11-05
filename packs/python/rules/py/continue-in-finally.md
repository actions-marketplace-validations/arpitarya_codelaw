---
id: py/continue-in-finally
title: "No continue in finally"
languages: [python]
fix: manual
fix_reason: "where the loop should continue is the author's"
fix_exception: restructure
stability: preview
ported_from: [ruff/PLE0116]
since: 0.5.0
---

A `finally` block inside a loop does not `continue`.

**Why:** It silently drops any exception or return in flight (and is a SyntaxError before Python 3.8).

```codelaw
match:
  kind: continue_statement
  inside: {kind: finally_clause, stopBy: {any: [{kind: for_statement}, {kind: while_statement}, {kind: function_definition}]}}
message: "`continue` in `finally` swallows exceptions"
```

## How to fix

1. Move the `continue` after the `try` statement.

## Examples

### Python

#### Don't

```python
for x in xs:
    try:
        go(x)
    finally:
        continue
```

#### Do

```python
for x in xs:
    try:
        go(x)
    finally:
        cleanup()
```
