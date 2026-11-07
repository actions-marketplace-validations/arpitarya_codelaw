---
id: py/useless-try-except
title: "No handler that only re-raises"
languages: [python]
fix: manual
fix_reason: "removing the handler, and the `try` when nothing else is left, re-indents its body"
fix_pending: "suggest; needs computed-edit (W-162)"
stability: preview
ported_from: [ruff/TRY203, pylint/try-except-raise]
since: 0.5.0
---

An `except` handler whose body is only `raise` (or `raise e`, its own exception) is reported when every handler of the `try` does just that.

**Why:** Catching an exception to raise it again unchanged does nothing but add a level of nesting.

```codelaw
utils:
  reraise:
    kind: except_clause
    has: {kind: block, all: [{has: {nthChild: 1, kind: raise_statement, regex: '^raise(\s+\w+)?\s*$'}}, {not: {has: {nthChild: 2}}}]}
match:
  matches: reraise
  inside:
    kind: try_statement
    not: {has: {kind: except_clause, not: {matches: reraise}}}
message: "The handler only re-raises; remove it"
```

## How to fix

1. Remove the handler; if the `try` has no other handler, `else` or `finally`, remove the `try` too and dedent its body.

## Examples

### Python

#### Don't

```python
try:
    run()
except ValueError:
    raise
```

#### Do

```python
run()
```
